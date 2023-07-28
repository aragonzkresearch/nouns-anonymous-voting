use std::ops::Deref;
use std::time::Duration;

use ethers::core::k256::U256;
use ethers::types::H256;
use ethers::utils::hex;

use cid::Cid;
use multibase::Base;
use multihash::Multihash;
use std::str::FromStr;

use nouns_protocol::{BBJJ_Ec, BBJJ_Fr, BN254_Fr, PrimeField};

/// Parses a hex string into BBJJ PrivateKey
/// Example: `1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef` of 32 bytes
pub(crate) fn parse_bbjj_prk(private_bbjj_key: &String) -> Result<BBJJ_Fr, String> {
    let key = if private_bbjj_key.starts_with("0x") {
        private_bbjj_key[2..].to_string()
    } else {
        private_bbjj_key.to_string()
    };

    let key = hex::decode(key).map_err(|e| format!("Failed to parse hex string: {}", e))?;

    Ok(BBJJ_Fr::from_be_bytes_mod_order(key.as_slice()))
}

/// Parses a duration string into a Duration
/// Example: `1d` (1 day)
/// Example: `1h` (1 hour)
/// Example: `1m` (1 minute)
pub(crate) fn parse_duration<T: Into<String>>(s: T) -> Duration {
    let s = s.into();
    let mut chars = s.chars();
    let mut number = String::new();
    let mut unit = String::new();
    while let Some(c) = chars.next() {
        if c.is_numeric() {
            number.push(c);
        } else {
            unit.push(c);
        }
    }
    let number = number.parse::<u64>().unwrap();
    let duration = match unit.as_str() {
        "d" => Duration::from_secs(number * 24 * 60 * 60),
        "h" => Duration::from_secs(number * 60 * 60),
        "m" => Duration::from_secs(number * 60),
        _ => panic!("Invalid duration unit"),
    };
    duration
}

/// Decodes a raw base32 IPFS CID into a U256
pub(crate) fn parse_ipfs_hash<T: Into<String>>(s: T) -> Result<H256, String> {
    let s = s.into();

    let s_cid = Cid::from_str(&s).map_err(|e| format!("Error parsing string as CID: {}", e))?;
    
    if s_cid.codec() != 0x55 { return Err("IPFS CID must be raw binary".to_string()); }

    let s_multihash = s_cid.hash();

    if s_multihash.code() != 0x12 { return Err("IPFS CID's underlying must be a sha2-256 hash".to_string()); }

    let s_bytes = s_multihash.digest();

    if s_bytes.len() != 32 { return Err("IPFS CID digest does not have the required length (32 bytes)".to_string());}

    let s_h256 = <[u8;32]>::try_from(s_bytes.deref()).map_err(|e| format!("Could not convert vector to byte array: {}", e))?.into();
    
    Ok(s_h256)
}

/// Parses a be TLCS Public Key string into a BBJJ_Ec
/// Example: `0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef,0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef`
pub(crate) fn parse_tlcs_pbk<T: Into<String>>(s: T) -> Result<BBJJ_Ec, String> {
    let s = s.into();
    let mut chars = s.chars();
    let mut x = String::new();
    let mut y = String::new();
    while let Some(c) = chars.next() {
        if c == ',' {
            break;
        }
        x.push(c);
    }
    while let Some(c) = chars.next() {
        if c == ' ' {
            continue;
        }

        y.push(c);
    }

    // Remove the 0x prefix
    if x.starts_with("0x") {
        x = x[2..].to_string();
    }
    if y.starts_with("0x") {
        y = y[2..].to_string();
    }

    let x = BN254_Fr::from_be_bytes_mod_order(
        &hex::decode(x).map_err(|_| "Invalid TLCS Public Key X coordinate")?,
    );
    let y = BN254_Fr::from_be_bytes_mod_order(
        &hex::decode(y).map_err(|_| "Invalid TLCS Public Key Y coordinate")?,
    );

    Ok(BBJJ_Ec { x, y })
}

/// Parses a U256
/// Supports both decimal (for small numbers) and hex (for large numbers) representations
/// Decimal example: `123456789`, the size of the number is limited to 8 bytes
/// Hex example: `0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef` (be 32 bytes)
pub(crate) fn parse_u256<T: Into<String>>(s: T) -> Result<U256, String> {
    let s = s.into();

    // Check if the string starts with 0x
    if s.len() > 2 && s.starts_with("0x") {
        // If it does, we parse it as a hex string
        let s = &s[2..];
        return Ok(U256::from_be_hex(s));
    }

    // If it doesn't, we parse it as a decimal string
    let number = s.parse::<u64>().map_err(|_| "Invalid decimal string")?;
    Ok(U256::from(number))
}

/// Parses a Private Key
/// Example: `1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef` (be 32 bytes)
pub(crate) fn parse_private_key(private_key: &String) -> Result<[u8; 32], String> {
    let private_key = if private_key.starts_with("0x") {
        private_key[2..].to_string()
    } else {
        private_key.to_string()
    };

    let tx_private_key =
        hex::decode(private_key).map_err(|e| format!("Invalid private key: {}", e))?;
    let tx_private_key = <[u8; 32]>::try_from(tx_private_key.deref())
        .map_err(|e| format!("Invalid private key: {}", e))?;
    Ok(tx_private_key)
}
