use std::io::{BufRead, IsTerminal, Write, stdin};
use std::path::PathBuf;

use clap::{Args, Parser};

use makura::{Base, Bases};
use makura::{Decoder, Encoder};

fn main() -> Result<(), CLIError> {
    let res = match Makura::parse() {
        Makura::Decode(d) => d.run(),
        Makura::Encode(e) => e.run(),
        Makura::Deduce(d) => d.run(),
        Makura::Convert(c) => c.run(),
    }?;

    let stdout = std::io::stdout().lock();
    std::io::BufWriter::new(stdout).write_all(res.as_bytes())?;

    Ok(())
}

#[derive(Debug)]
enum CLIError {
    CouldNotOpenFileForReading,
    NeedABaseToEncode,
    DecodeFailed,
    DeduceFailed,
    IOError,
}

impl std::error::Error for CLIError {}

impl std::fmt::Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<CLIError> for std::io::Error {
    fn from(value: CLIError) -> Self {
        Self::other(value.to_string())
    }
}

impl From<std::io::Error> for CLIError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError
    }
}

#[derive(Debug, Parser)]
enum Makura {
    Decode(Decode),
    Encode(Encode),
    Deduce(Deduce),
    Convert(Convert),
}

trait CommandLauncher {
    fn run(self) -> Result<String, CLIError>;
}

#[derive(Debug, Args)]
struct Decode {
    #[arg(long, short = 'f')]
    file: Option<std::path::PathBuf>,
    #[arg(long, short = 'd')]
    data: Option<String>,
    #[arg(long, short = 'b')]
    base: Option<Base>,
    #[arg(long)]
    auto: bool,
    #[arg(long, short = 'c')]
    chain: Option<u8>,
    #[arg(long, short = 'r')]
    repeat: Option<u8>,
}

fn read_file(f: std::path::PathBuf) -> std::io::Result<Vec<u8>> {
    std::fs::read(f)
}

fn read_file_str(f: std::path::PathBuf) -> std::io::Result<String> {
    std::fs::read_to_string(f)
}

fn pipe_input() -> String {
    stdin().lock().lines().flatten().collect::<String>()
}

fn extract_data(f: Option<PathBuf>, d: Option<String>) -> Result<String, CLIError> {
    let stdin = stdin().lock();
    if !stdin.is_terminal() {
        return Ok(pipe_input());
    }

    if let Some(f) = f {
        let d = read_file_str(f);
        if d.is_err() {
            return d.map_err(|_| CLIError::CouldNotOpenFileForReading);
        }

        d.map_err(|_| CLIError::CouldNotOpenFileForReading)
    } else if let Some(d) = d {
        Ok(d)
    } else {
        return Err(CLIError::CouldNotOpenFileForReading);
    }
}

impl CommandLauncher for Decode {
    fn run(self) -> Result<String, CLIError> {
        let data = extract_data(self.file, self.data)?;

        if let Some(base) = self.base {
            Decoder::decode(data, base)
                .map(|res| res.into_utf8().unwrap())
                .map_err(|e| CLIError::DecodeFailed)
        } else {
            Decoder::decode_deduce(data)
                .map(|res| res.into_utf8().unwrap())
                .map_err(|e| CLIError::DecodeFailed)
        }
    }
}

#[derive(Debug, Args)]
struct Encode {
    #[arg(long, short = 'f')]
    file: Option<std::path::PathBuf>,
    #[arg(long, short = 'd')]
    data: Option<String>,
    #[arg(long, short = 'b')]
    base: Option<Base>,
    #[arg(long, short = 'c')]
    chain: Option<u8>,
    #[arg(long, short = 'r')]
    repeat: Option<u8>,
}

impl CommandLauncher for Encode {
    fn run(self) -> Result<String, CLIError> {
        let data = extract_data(self.file, self.data)?;

        let Some(base) = self.base else {
            return Err(CLIError::NeedABaseToEncode);
        };

        Ok(<Base as Into<Encoder>>::into(base).encode(data))
        // .map(|res| res.into_utf8().unwrap())
        // .map_err(|e| CLIError::DecodeFailed)
    }
}

#[derive(Debug, Args)]
struct Deduce {
    #[arg(long, short = 'f')]
    file: Option<std::path::PathBuf>,
    #[arg(long, short = 'd')]
    data: Option<String>,
}

impl CommandLauncher for Deduce {
    fn run(self) -> Result<String, CLIError> {
        let data = extract_data(self.file, self.data)?;

        Bases::default()
            .deduce_encoding(&data)
            .map_err(|e| CLIError::DeduceFailed)
            .map(|b| b.to_string())
    }
}

#[derive(Debug, Args)]
struct Convert {
    #[arg(long, short = 'S')]
    src: Option<Base>,
    #[arg(long, short = 'D')]
    dest: Base,
    #[arg(long, short = 'f')]
    file: Option<std::path::PathBuf>,
    #[arg(long, short = 'd')]
    data: Option<String>,
    #[arg(long, short = 'b')]
    base: Option<Base>,
    #[arg(long, short = 'c')]
    chain: Option<u8>,
    #[arg(long, short = 'r')]
    repeat: Option<u8>,
}

impl CommandLauncher for Convert {
    fn run(self) -> Result<String, CLIError> {
        let data = extract_data(self.file, self.data)?;

        let input = if let Some(src_base) = self.src {
            Decoder::decode(data, src_base)
        } else {
            Decoder::decode_deduce(data)
        };

        if input.is_err() {
            return input
                .map(|_| String::new())
                .map_err(|_| CLIError::DecodeFailed);
        }
        let input = input.unwrap().into_utf8().unwrap();
        let enc: Encoder = self.dest.into();

        Ok(enc.encode(input))
    }
}
