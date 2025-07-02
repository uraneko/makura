<h1>makura</h1>

This is the workspace repo for the multi-base encoding/decoding makura crates.

[<img alt="crates.io" src="https://img.shields.io/crates/v/makura.svg?style=for-the-badge&color=E40046&logo=rust&labelColor=3a3a3a" height="25">](https://crates.io/crates/makura) 
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-makura-495c9f?style=for-the-badge&logo=docsdotrs&labelColor=3a3a3a" height="25">](https://docs.rs/makura) 
[<img alt="crates.io" src="https://img.shields.io/crates/v/makura_cli.svg?style=for-the-badge&color=E40046&logo=rust&labelColor=3a3a3a" height="25">](https://crates.io/crates/makura_cli) 
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-makura_cli-495c9f?style=for-the-badge&logo=docsdotrs&labelColor=3a3a3a" height="25">](https://docs.rs/makura_cli) 
[<img alt="build:test" src="https://img.shields.io/github/actions/workflow/status/uraneko/makura/rust-ci.yml?branch=main&style=for-the-badge&labelColor=3a3a3a" height="25">](https://github.com/uraneko/makura/actions?query=branch%3Amain)
[<img alt="license" src="https://img.shields.io/github/license/uraneko/makura?style=for-the-badge&labelColor=3a3a3a&color=ECD53F" height="25">](https://github.com/uraneko/makura/blob/main/LICENSE)

##
## Contents
- [Crates](#Crates)
- [MSRV](#MSRV)
- [License](#License)

###
### Crates
* <a href="makura">makura</a>
* <a href="makura_cli">makura_cli</a>

###
### MSRV
Although the msrv is `rustc/cargo 1.85.0` , this crate should functionally work with earlier versions, as it shouldn't be using any new-ish rust features. 
The high MSRV is due to the crate edition being `2024`. 
If you need to run it on an older version of rust - I highly don't recommend it - then lowering the rust `package.edition` key in the Cargo.toml manifest should make it work (I didn't test this).

###
### License
<a href="LICENSE">MIT</a> only 

