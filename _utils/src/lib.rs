#![allow(non_snake_case)]
#![allow(unused)]

use rand::distributions::uniform::SampleRange;
use rand::Rng;
use std::ops::{Range, RangeBounds, RangeInclusive};
use std::ptr::addr_of_mut;

pub fn generate_random_bytes(num_bytes: usize) -> Vec<u8> {
    let lol = generate_random_bytes_in_range(10, 0..=10);
    (0..num_bytes).map(|_| rand::random()).collect()
}

pub fn generate_random_bytes_in_range<R>(num_bytes: usize, value_range: R) -> Vec<u8>
    where
        R: SampleRange<u8>,
        R: Clone,
{
    (0..num_bytes)
        .map(|_| rand::thread_rng().gen_range(value_range.clone()))
        .collect()
}
