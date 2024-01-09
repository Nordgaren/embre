#![allow(non_snake_case)]
#![allow(unused)]

use rand::distributions::uniform::SampleRange;
use rand::Rng;
use std::ops::{Range, RangeBounds, RangeInclusive};
use std::ptr::addr_of_mut;
use windows_sys::core::PCWSTR;
use windows_sys::Win32::Foundation::GetLastError;
use windows_sys::Win32::Security::Cryptography::{CryptAcquireContextW, CryptCreateHash, CryptDecrypt, CryptDeriveKey, CryptDestroyHash, CryptDestroyKey, CryptEncrypt, CryptGenKey, CryptGetKeyParam, CryptHashData, CryptReleaseContext, CryptSetKeyParam, ALG_CLASS_DATA_ENCRYPT, ALG_CLASS_HASH, ALG_SID_AES_256, ALG_SID_SHA_256, ALG_TYPE_ANY, ALG_TYPE_BLOCK, CRYPT_VERIFYCONTEXT, KP_BLOCKLEN, KP_IV, KP_KEYLEN, PROV_RSA_AES, CALG_SHA_256, CALG_AES_128};
use embre_crypt::aes::{AESCrypter, DefaultAesCrypter};

pub(crate) fn get_key_len() -> usize {
    DefaultAesCrypter::default().get_cipher().key_len()
}

pub(crate) fn get_key_len_old() -> usize {
    unsafe {
        let mut hProv = 0;
        if CryptAcquireContextW(
            addr_of_mut!(hProv),
            0 as PCWSTR,
            0 as *const u16,
            PROV_RSA_AES,
            CRYPT_VERIFYCONTEXT,
        ) == 0
        {
            panic!("Panic at CryptAcquireContextW");
        }

        let mut hHash = 0;
        if CryptCreateHash(
            hProv,
            ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA_256,
            0,
            0,
            addr_of_mut!(hHash),
        ) == 0
        {
            panic!("Panic at CryptCreateHash");
        }

        if CryptHashData(hHash, 0 as *const u8, 0, 0) == 0 {
            panic!("Panic at CryptHashData");
        }

        let mut hKey = 0;
        if CryptGenKey(
            hProv,
            ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_AES_256,
            0,
            addr_of_mut!(hKey),
        ) == 0
        {
            panic!("Panic at CryptGenKey");
        }

        let mut key_len = 0u32;
        let mut len = 4u32;
        if CryptGetKeyParam(
            hKey,
            KP_KEYLEN,
            addr_of_mut!(key_len) as *mut u8,
            addr_of_mut!(len),
            0,
        ) == 0
        {
            panic!("Panic at CryptGetKeyParam");
        }

        (key_len / 8) as usize
    }
}

pub(crate) fn get_iv_len() -> usize {
    DefaultAesCrypter::default().get_cipher().iv_len().unwrap_or_default()
}

pub(crate) fn get_iv_len_old() -> usize {
    unsafe {
        let mut hProv = 0;
        if CryptAcquireContextW(
            addr_of_mut!(hProv),
            0 as PCWSTR,
            0 as *const u16,
            PROV_RSA_AES,
            CRYPT_VERIFYCONTEXT,
        ) == 0
        {
            panic!("Panic at CryptAcquireContextW");
        }

        let mut hHash = 0;
        if CryptCreateHash(
            hProv,
            ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA_256,
            0,
            0,
            addr_of_mut!(hHash),
        ) == 0
        {
            panic!("Panic at CryptCreateHash");
        }

        if CryptHashData(hHash, 0 as *const u8, 0, 0) == 0 {
            panic!();
        }

        let mut hKey = 0;
        if CryptGenKey(
            hProv,
            ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_AES_256,
            0,
            addr_of_mut!(hKey),
        ) == 0
        {
            panic!("Panic at CryptGenKey");
        }

        let mut iv = [0; 420];
        let mut len = iv.len() as u32;
        if CryptGetKeyParam(hKey, KP_IV, iv.as_mut_ptr(), addr_of_mut!(len), 0) == 0 {
            panic!("Panic at CryptGetKeyParam");
        }

        len as usize
    }
}

pub fn aes_encrypt_bytes(bytes: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    DefaultAesCrypter::default().aes_encrypt_bytes(bytes, key, Some(iv)).expect("Could not encrypt bytes")
}

pub(crate) fn aes_encrypt_bytes_old(bytes: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    unsafe {
        let mut hProv = 0;
        if CryptAcquireContextW(
            addr_of_mut!(hProv),
            0 as PCWSTR,
            0 as *const u16,
            PROV_RSA_AES,
            CRYPT_VERIFYCONTEXT,
        ) == 0
        {
            panic!("Panic at CryptAcquireContextW");
        }

        let mut hHash = 0;
        if CryptCreateHash(
            hProv,
            ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA_256,
            0,
            0,
            addr_of_mut!(hHash),
        ) == 0
        {
            panic!("Panic at CryptCreateHash");
        }

        if CryptHashData(hHash, key.as_ptr(), key.len() as u32, 0) == 0 {
            panic!("Panic at CryptHashData");
        }

        let mut hKey = 0;
        if CryptDeriveKey(
            hProv,
            ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_AES_256,
            hHash,
            0,
            addr_of_mut!(hKey),
        ) == 0
        {
            panic!("Panic at CryptDeriveKey");
        }

        if CryptSetKeyParam(hKey, KP_IV, iv.as_ptr(), 0) == 0 {
            panic!("Panic at CryptSetKeyParam");
        }

        let mut block_len = 0u32;
        let mut len = 4u32;
        if CryptGetKeyParam(
            hKey,
            KP_BLOCKLEN,
            addr_of_mut!(block_len) as *mut u8,
            addr_of_mut!(len),
            0,
        ) == 0
        {
            panic!("Panic at CryptGetKeyParam");
        }

        block_len = block_len >> 3;
        let mut out = bytes.to_vec();

        let pad = block_len - (out.len() % block_len as usize) as u32;
        out.resize(out.len() + pad as usize, pad as u8);
        let mut len = out.len() as u32;
        if CryptEncrypt(hKey, 0, 0, 0, out.as_mut_ptr(), addr_of_mut!(len), len) == 0 {
            panic!("Panic at CryptEncrypt");
        }

        CryptReleaseContext(hProv, 0);
        CryptDestroyHash(hHash);
        CryptDestroyKey(hKey);

        out
    }
}

pub(crate) fn aes_decrypt_bytes(bytes: Vec<u8>, key: &[u8], iv: &[u8]) -> Vec<u8> {
    unsafe {
        let mut hProv = 0;
        if CryptAcquireContextW(
            addr_of_mut!(hProv),
            0 as PCWSTR,
            0 as *const u16,
            PROV_RSA_AES,
            CRYPT_VERIFYCONTEXT,
        ) == 0
        {
            panic!("Panic at CryptAcquireContextW");
        }

        let mut hHash = 0;
        if !CryptCreateHash(
            hProv,
            ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA_256,
            0,
            0,
            addr_of_mut!(hHash),
        ) == 0
        {
            panic!("Panic at CryptCreateHash");
        }

        if !CryptHashData(hHash, key.as_ptr(), key.len() as u32, 0) == 0 {
            panic!();
        }

        let mut hKey = 0;
        if !CryptDeriveKey(
            hProv,
            ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_BLOCK | ALG_SID_AES_256,
            hHash,
            0,
            addr_of_mut!(hKey),
        ) == 0
        {
            panic!("Panic at CryptDeriveKey");
        }

        if !CryptSetKeyParam(hKey, KP_IV, iv.as_ptr(), 0) == 0 {
            panic!("Panic at CryptSetKeyParam");
        }

        let mut len = bytes.len() as u32;
        let mut payload = bytes.to_vec();
        if !CryptDecrypt(hKey, 0, 0, 0, payload.as_mut_ptr(), addr_of_mut!(len)) == 0 {
            panic!("Panic at CryptDecrypt");
        }

        CryptReleaseContext(hProv, 0);
        CryptDestroyHash(hHash);
        CryptDestroyKey(hKey);

        let pad = get_aes_padding(&payload[..]);
        if pad > 0 {
            payload.truncate(bytes.len() - pad);
        }

        payload
    }
}

pub(crate) fn get_aes_padding(slice: &[u8]) -> usize {
    if slice.is_empty() {
        return 0;
    }

    let pad = slice[slice.len() - 1];
    for b in slice.iter().rev().take(pad as usize) {
        if b != &pad {
            return 0;
        }
    }

    pad as usize
}

pub fn xor_bytes(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    if bytes.len() != key.len() {
        panic!("Key and source len differ.")
    }

    let mut out = bytes.to_vec();

    for i in 0..bytes.len() {
        out[i] ^= key[i];
    }

    out
}

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

pub(crate) fn make_const_name(string: &str) -> String {
    let underscores = [" ", ",", "."];
    let mut const_name = string.to_uppercase();

    for pattern in underscores {
        const_name = const_name.replace(pattern, "_")
    }

    let one: Vec<char> = ('!'..',').collect();
    let two: Vec<char> = (':'..'A').collect();
    let three: Vec<char> = ('['..'_').collect();
    let four: Vec<char> = ('{'..='~').collect();

    let delete = ['\0', '!', '\"', '-', '/', '`'];

    for pattern in delete
        .iter()
        .chain(one.iter())
        .chain(two.iter())
        .chain(three.iter())
        .chain(four.iter())
    {
        const_name = const_name.replace(*pattern, "")
    }

    const_name.replace("__", "_")
}
