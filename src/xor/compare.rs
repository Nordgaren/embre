#![allow(unused)]

use std::ops::RangeInclusive;

const CASE_BIT: u8 = 0x20;
const RANGE_START: u8 = b'A';
const RANGE_END: u8 = b'Z';
const CASE_RANGE: RangeInclusive<u8> = RANGE_START..=RANGE_END;
#[inline(always)]
pub fn xor_u8_cmp(buffer: &[u8], key: &[u8], other: &[u8]) -> bool {
    if buffer.len() != other.len() {
        return false;
    }

    for i in 0..buffer.len() {
        let mut val = other[i];

        val ^= key[i];
        if val != buffer[i] {
            return false;
        }
    }

    true
}
fn xor_case_insensitive_str_cmp(buffer: &[u8], key: &[u8], other: &[u8]) -> bool {
    if buffer.len() != other.len() {
        return false;
    }

    for i in 0..other.len() {
        let b = buffer[i];
        let mut v = other[i];

        if CASE_RANGE.contains(&v) {
            v ^= CASE_BIT;
        }

        v ^= key[i];
        if v != b && v != b ^ CASE_BIT {
            return false;
        };
    }

    true
}
pub fn xor_str_cmp_ignore_case(buffer: &[u8], key: &[u8], other: &[u8]) -> bool {
    if buffer.len() != other.len() {
        return false;
    }

    for i in 0..buffer.len() {
        let mut val = other[i];

        if CASE_RANGE.contains(&val) {
            val ^= CASE_BIT;
        }
        val ^= key[i];
        if val != buffer[i] {
            return false;
        }
    }

    true
}
#[inline(always)]
pub fn xor_w_str_cmp(buffer: &[u8], key: &[u8], other: &[u16]) -> bool {
    if buffer.len() != other.len() {
        return false;
    }

    for i in 0..buffer.len() {
        let mut val = other[i] as u8;

        val ^= key[i];
        if val != buffer[i] {
            return false;
        }
    }

    true
}
pub fn xor_w_str_cmp_ignore_case(buffer: &[u8], key: &[u8], other: &[u16]) -> bool {
    if buffer.len() != other.len() {
        return false;
    }

    for i in 0..buffer.len() {
        let mut val = other[i] as u8;

        if CASE_RANGE.contains(&val) {
            val ^= CASE_BIT;
        }
        val ^= key[i];
        if val != buffer[i] {
            return false;
        }
    }

    true
}
