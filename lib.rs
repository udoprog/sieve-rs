//! [<img alt="github" src="https://img.shields.io/badge/github-udoprog/sieve--rs-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/udoprog/sieve-rs)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/sieve.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/sieve)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-sieve-66c2a5?style=for-the-badge&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/sieve)
//! [<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/udoprog/sieve-rs/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/udoprog/sieve-rs/actions?query=branch%3Amain)
//!
//! A simple, growable prime-sieve for Rust.
//!
//! <br>
//!
//! ## Examples
//!
//! All the primes (if you have the memory):
//!
//! ```rust
//! for prime in sieve::Sieve::infinite::<u32>().take(100) {
//!     println!("prime = {}", prime);
//! }
//! ```
//!
//! Only primes below a certain value:
//!
//! ```rust
//! for prime in sieve::Sieve::bounded(1_000_000u64) {
//!     println!("prime = {}", prime);
//! }
//! ```

use std::collections::HashMap;
use std::hash;
use std::ops;

pub trait Same<T> {}
impl<T> Same<T> for T {}

pub struct Sieve<Idx, I> {
    composite: HashMap<Idx, Idx>,
    iter: I,
}

impl<Idx, I> Sieve<Idx, I>
where
    Idx: PartialEq + Eq + hash::Hash,
{
    pub fn size(&self) -> usize {
        self.composite.len()
    }
}

impl<Idx> Sieve<Idx, ops::RangeFrom<Idx>> {
    pub fn infinite<U: Same<Idx>>() -> Sieve<Idx, ops::RangeFrom<Idx>>
    where
        Idx: From<u32> + Eq + hash::Hash,
    {
        Sieve {
            iter: 2.into()..,
            composite: HashMap::new(),
        }
    }

    pub fn bounded(upper: Idx) -> Sieve<Idx, ops::Range<Idx>>
    where
        Idx: From<u32> + Eq + hash::Hash,
    {
        Sieve {
            iter: 2.into()..upper,
            composite: HashMap::new(),
        }
    }
}

impl<Idx, I> Iterator for Sieve<Idx, I>
where
    I: Iterator<Item = Idx>,
    Idx: Eq + hash::Hash + Copy + ops::Add<Output = Idx> + ops::Mul<Output = Idx>,
{
    type Item = Idx;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(n) = self.iter.next() {
            if let Some(value) = self.composite.remove(&n) {
                let mut key = n + value;

                while self.composite.contains_key(&key) {
                    key = key + value;
                }

                self.composite.insert(key, value);
            } else {
                self.composite.insert(n * n, n);
                return Some(n);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut sieve = Sieve::infinite::<u32>();
        let primes = sieve.by_ref().take(100).collect::<Vec<u32>>();

        assert_eq!(
            primes,
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257,
                263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353,
                359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449,
                457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541,
            ]
        );
    }
}
