use makura::{BASE64, Decode, Encode};
#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "serde")]
#[derive(Debug, Serialize)]
struct Pagoda {
    s: String,
    x: u128,
    are_we_pagodding: bool,
}

fn main() {
    let s = "this is a string literal";
    // let s: [u8; 8] = [2, 3, 4, 52, 3, 6, 9, 56];
    // let s = Pagoda {
    //     s: "this is a pagoda thing".into(),
    //     x: 43209,
    //     are_we_pagodding: true,
    // };
    let e = s.encode(BASE64);

    println!("{:?}", e);
}
