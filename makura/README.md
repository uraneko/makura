<h1>makura</h1>

makura is mostly an implementation of the following 2 rfcs: 
- [The Base16, Base32, and Base64 Data Encodings](https://datatracker.ietf.org/doc/html/rfc4648) 
- [The Base45 Data Encoding](https://datatracker.ietf.org/doc/html/rfc9285).

[<img alt="crates.io" src="https://img.shields.io/crates/v/makura.svg?style=for-the-badge&color=E40046&logo=rust&labelColor=3a3a3a" height="25">](https://crates.io/crates/makura) 
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-makura-495c9f?style=for-the-badge&logo=docsdotrs&labelColor=3a3a3a" height="25">](https://docs.rs/makura) 
[<img alt="build:test" src="https://img.shields.io/github/actions/workflow/status/uraneko/makura/rust-ci.yml?branch=main&style=for-the-badge&labelColor=3a3a3a" height="25">](https://github.com/uraneko/makura/actions?query=branch%3Amain)
[<img alt="license" src="https://img.shields.io/github/license/uraneko/makura?style=for-the-badge&labelColor=3a3a3a&color=ECD53F" height="25">](https://github.com/uraneko/makura/blob/main/LICENSE)

##
## Contents
- [Features](#Features)
- [Usage](#Usage)
- [Examples](#Examples)
- [MSRV](#MSRV)
- [License](#License)

###
### Features

|  base  |  encoding  |  decoding  |
| :----- | :--------: | :--------: |
| 64	 | ✓ | ✓ |
| 64 url | ✓ | ✓ |
| 45	 | ✓ | ✓ |
| 32	 | ✓ | ✓ |
| 32 hex | ✓ | ✓ |
| 16	 | ✓ | ✓ |
| custom | ✗ | ✗ |

###
### Usage

```sh
# add makura as a dependency in your cargo project
cargo add makura
```

> [!WARNING]
> The documentation is unavailable as this crate is still unpublished.

Check out <a href="docs.rs/makura/latest/features">the crate documentation</a> for a list of available features.

###
### Examples

#### makura_cli
Check out the <a href="https://github.com/uraneko/makura/blob/binary/makura_cli/src/main.rs">makura_cli crate src code</a> (just main.rs) for an example usage of this crate's functionalities.

#### docs.rs

> [!WARNING]
> The documentation is unavailable as this crate is still unpublished.

Or you can take a look at <a href="docs.rs/makura">the crate documentation</a>.

#### integration tests
Alternatively, this crate's <a href = "tests">intergration tests</a> may also provide some insight into its usage.

###
### MSRV
Although the msrv is `rustc/cargo 1.85.0` , this crate should functionally work with earlier versions, as it shouldn't be using any new-ish rust features. 
The high MSRV is due to the crate edition being `2024`. 
If you need to run it on an older version of rust - I highly don't recommend it - then lowering the rust `package.edition` key in the Cargo.toml manifest should make it work (I didn't test this).

###
### License
<a href="LICENSE">MIT</a> only 
