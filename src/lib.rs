#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use scanpw::scanpw;
use is_prime::is_prime;

#[napi]
pub fn rline() -> String {
  std::io::stdin().lines().next().unwrap().unwrap()
}

#[napi]
pub fn rline_pw() -> String {
  scanpw!(None)
}

#[napi]
pub fn isprime(n: String) -> bool {
  is_prime(&n)
}