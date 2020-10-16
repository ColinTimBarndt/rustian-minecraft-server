// Copyright (C) 2019 RoccoDev
// Licensed under the MIT license.
// <https://opensource.org/licenses/MIT>
// Source: https://gist.github.com/RoccoDev/8fa130f1946f89702f799f89b8469bc9

// Bench results:
// First hash: 152ms
// Second hash: 1ms
// Third hash: 0ms

use regex::Regex;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

use std::iter;

use rustc_serialize::hex::ToHex;

const LEADING_ZERO_REGEX: &str = r#"^0+"#;

pub fn calc_hash(input: &[&[u8]]) -> String {
  let mut hasher = Sha1::new();
  for bytes in input {
    hasher.input(bytes);
  }
  let mut hex: Vec<u8> = iter::repeat(0)
    .take((hasher.output_bits() + 7) / 8)
    .collect();
  hasher.result(&mut hex);

  let negative = (hex[0] & 0x80) == 0x80;

  let regex = Regex::new(LEADING_ZERO_REGEX).unwrap();

  if negative {
    two_complement(&mut hex);
    format!(
      "-{}",
      regex
        .replace(hex.as_slice().to_hex().as_str(), "")
        .to_string()
    )
  } else {
    regex
      .replace(hex.as_slice().to_hex().as_str(), "")
      .to_string()
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

mod tests {
  #![allow(unused_imports)]
  use super::calc_hash;
  #[test]
  pub fn test_calc_hashes() {
    assert_eq!(
      "-7c9d5b0044c130109a5d7b5fb5c317c02b4e28c1",
      calc_hash(&[b"jeb_"])
    );
    assert_eq!(
      "4ed1f46bbe04bc756bcb17c0c7ce3e4632f06a48",
      calc_hash(&[b"Notch"])
    );
    assert_eq!(
      "88e16a1019277b15d58faf0541e11910eb756f6",
      calc_hash(&[b"simon"])
    );
  }
}
