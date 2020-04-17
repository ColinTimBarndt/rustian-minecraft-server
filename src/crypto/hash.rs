// Copyright (C) 2019 RoccoDev
// Licensed under the MIT license.
// <https://opensource.org/licenses/MIT>

// Bench results:
// First hash: 152ms
// Second hash: 1ms
// Third hash: 0ms

extern crate crypto; // Tested with 0.2.36
extern crate num_bigint; // Tested with 0.2
//extern crate rustc_serialize; // Tested with ^0.3
extern crate regex; // Tested with 1

use regex::Regex;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

use std::iter;

const LEADING_ZERO_REGEX: &str = r#"^0+"#;

pub fn hash(input: &Vec<&str>) -> String {
    let mut hasher = Sha1::new();
    for msg in input {
        hasher.input_str(msg);
    }
    let mut hex: Vec<u8> = iter::repeat(0).take((hasher.output_bits() + 7)/8).collect();
    hasher.result(&mut hex);

    let negative = (hex[0] & 0x80) == 0x80;

    let regex = Regex::new(LEADING_ZERO_REGEX).unwrap();

    if negative {
        two_complement(&mut hex);
        format!("-{}", regex.replace(to_hex(hex.as_slice()).as_str(), "").to_string())
    }
    else {
        regex.replace(to_hex(hex.as_slice()).as_str(), "").to_string()
    }
}

fn two_complement(bytes: &mut Vec<u8>) {
    let mut carry = true;
    for i in (0..bytes.len()).rev() {
        bytes[i] = !bytes[i] & 0xff;
        if carry {
            carry = bytes[i] == 0xff;
            bytes[i] = bytes[i] + 1;
        }
    }
}

fn to_hex(bytes: &[u8]) -> String {
    use std::iter::FromIterator;
    let tmp = bytes.iter().map(|a| {
        if *a<0x10 {
            format!("0{:x}", a)
        } else {
            format!("{:x}", a)
        }
    });
    String::from_iter(tmp)
}
