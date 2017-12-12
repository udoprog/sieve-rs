use std::collections::HashMap;
use std::ops;
use std::hash;

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
    Idx: Eq
        + hash::Hash
        + Copy
        + ops::Add<Output = Idx>
        + ops::Mul<Output = Idx>,
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
                2,
                3,
                5,
                7,
                11,
                13,
                17,
                19,
                23,
                29,
                31,
                37,
                41,
                43,
                47,
                53,
                59,
                61,
                67,
                71,
                73,
                79,
                83,
                89,
                97,
                101,
                103,
                107,
                109,
                113,
                127,
                131,
                137,
                139,
                149,
                151,
                157,
                163,
                167,
                173,
                179,
                181,
                191,
                193,
                197,
                199,
                211,
                223,
                227,
                229,
                233,
                239,
                241,
                251,
                257,
                263,
                269,
                271,
                277,
                281,
                283,
                293,
                307,
                311,
                313,
                317,
                331,
                337,
                347,
                349,
                353,
                359,
                367,
                373,
                379,
                383,
                389,
                397,
                401,
                409,
                419,
                421,
                431,
                433,
                439,
                443,
                449,
                457,
                461,
                463,
                467,
                479,
                487,
                491,
                499,
                503,
                509,
                521,
                523,
                541,
            ]
        );
    }
}
