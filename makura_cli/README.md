<h1>makura_cli</h1>

This is a CLI tool for different bases encoding/decoding.
It uses the <a href="../makura">makura</a> crate as a backend.

[<img alt="crates.io" src="https://img.shields.io/crates/v/makura_cli.svg?style=for-the-badge&color=E40046&logo=rust&labelColor=3a3a3a" height="25">](https://crates.io/crates/makura_cli) 
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-makura_cli-495c9f?style=for-the-badge&logo=docsdotrs&labelColor=3a3a3a" height="25">](https://docs.rs/makura_cli) 
[<img alt="build:test" src="https://img.shields.io/github/actions/workflow/status/uraneko/makura/rust-ci.yml?branch=main&style=for-the-badge&labelColor=3a3a3a" height="25">](https://github.com/uraneko/makura/actions?query=branch%3Amain)
[<img alt="license" src="https://img.shields.io/github/license/uraneko/makura?style=for-the-badge&labelColor=3a3a3a&color=ECD53F" height="25">](https://github.com/uraneko/makura/blob/main/LICENSE)

## Table of Contents 
* [Features](#Features)
* [Installation](#Installation)
* [Usage](#Usage)
* [MSRV](#Usage)
* [License](#License)

### 
### Features
|  command/base  |  64  | 64url |  45  |  32  | 32hex |  16  | custom |
| :------       | :--: | :---: | :--: | :--: | :---: | :--: | :----: |
| encode | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ |
| decode | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ |
| deduce | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ |
| recast | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ |

###
### Installation 

#### Cargo

> [!WARNING]
> This can not be done yet, as the crate is still unpublished.

```bash
cargo install makura_cli --locked 
```

#### Git 

```bash
git clone https://github.com/uraneko/makura
cd makura
cargo install --path makura_cli --bin maku --locked
# this provides the `maku` cli tool 
```

### 
### Usage

```bash
# the following command decodes the input string from base32 encoding
# and outputs the result to stdout/err
maku dec -b 32 -i <some_file_or_string>
# this takes a piped string input and encodes it to base 64 (the implicit base for encoding when none is explicitly provided is base64)
cat <some_file> | maku enc -o encoded.txt
# run maku --help for a list of all command
```

### MSRV
Although the msrv is `rustc/cargo 1.85.0` , this crate should functionally work with earlier versions, as it shouldn't be using any new-ish rust features. 
The high MSRV is due to the crate edition being `2024`. 
If you need to run it on an older version of rust - I highly don't recommend it - then lowering the rust `package.edition` key in the Cargo.toml manifest should make it work (I didn't test this).

### License
<a href="LICENSE">MIT</a> only 
