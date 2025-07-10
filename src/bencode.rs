use std::collections::HashMap;

use eyre::{OptionExt, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Bencode {
    Integer(i64),
    String(Vec<u8>),
    List(Vec<Bencode>),
    Dictionary(HashMap<String, Bencode>),
}

pub fn decode(data: Vec<u8>) -> Result<Bencode> {
    let (bencode, remaining) = decode_bencoded_bytes(&data)?;
    if !remaining.is_empty() {
        return Err(eyre::eyre!(
            "Extra data after bencoded value: {} bytes",
            remaining.len()
        ));
    }
    Ok(bencode)
}

pub fn encode(b: &Bencode) -> Vec<u8> {
    match b {
        Bencode::Integer(number) => format!("i{}e", number).into_bytes(),
        Bencode::String(bytes) => {
            let mut result = format!("{}:", bytes.len()).into_bytes();
            result.extend_from_slice(bytes);
            result
        }
        Bencode::List(list) => {
            let mut result = b"l".to_vec();
            for item in list {
                result.extend_from_slice(&encode(item));
            }
            result.push(b'e');
            result
        }
        Bencode::Dictionary(dict) => {
            let mut items: Vec<(String, Bencode)> =
                dict.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

            items.sort_by_key(|(k, _)| k.clone());

            let mut result = b"d".to_vec();
            for (key, value) in items {
                let encoded_key = encode(&Bencode::String(key.into_bytes()));
                let encoded_value = encode(&value);
                result.extend_from_slice(&encoded_key);
                result.extend_from_slice(&encoded_value);
            }
            result.push(b'e');
            result
        }
    }
}

fn decode_bencoded_bytes(data: &[u8]) -> Result<(Bencode, &[u8])> {
    let first_byte = data.first().ok_or_else(|| eyre::eyre!("Empty data"))?;

    match *first_byte {
        b'i' => decode_integer(data),
        b'l' => decode_list(data),
        b'd' => decode_dictionary(data),
        b'0'..=b'9' => decode_string(data),
        r => Err(eyre::eyre!("Invalid bencode format: {:?}", r)),
    }
}

fn decode_string(data: &[u8]) -> Result<(Bencode, &[u8])> {
    let colon_pos = data
        .iter()
        .position(|&b| b == b':')
        .ok_or_eyre("Invalid string: missing colon")?;

    let length_bytes = &data[..colon_pos];
    let length_string = std::str::from_utf8(length_bytes)
        .map_err(|_| eyre::eyre!("Invalid UTF-8 in string length"))?;

    let length = length_string.parse::<usize>()?;

    let content_start = colon_pos + 1;
    if content_start + length > data.len() {
        return Err(eyre::eyre!("String length exceeds remaining data"));
    }

    let decoded_bytes = &data[content_start..content_start + length];
    let remaining = &data[content_start + length..];

    Ok((Bencode::String(decoded_bytes.to_vec()), remaining))
}

fn decode_partial_list(data: &[u8]) -> Result<(Bencode, &[u8])> {
    let mut vec = Vec::new();
    let mut rest = &data[1..]; // Skip the 'l' or 'd'

    while !rest.is_empty() && rest[0] != b'e' {
        let (item, new_rest) = decode_bencoded_bytes(rest)?;
        vec.push(item);
        rest = new_rest;
    }

    if rest.is_empty() {
        return Err(eyre::eyre!("Missing 'e' terminator"));
    }

    Ok((Bencode::List(vec), &rest[1..])) // Skip the 'e'
}

fn decode_list(data: &[u8]) -> Result<(Bencode, &[u8])> {
    let first_byte = data.first().ok_or_else(|| eyre::eyre!("Empty data"))?;
    if *first_byte != b'l' {
        return Err(eyre::eyre!("Invalid list format"));
    }

    decode_partial_list(data)
}

fn decode_dictionary(data: &[u8]) -> Result<(Bencode, &[u8])> {
    let first_byte = data.first().ok_or_else(|| eyre::eyre!("Empty data"))?;
    if *first_byte != b'd' {
        return Err(eyre::eyre!("Invalid dictionary format"));
    }

    let (list, rest) = decode_partial_list(data)?;

    if let Bencode::List(list) = list {
        if list.len() % 2 != 0 {
            return Err(eyre::eyre!(
                "Dictionary List must have an even number of elements"
            ));
        }

        let keys = list.iter().step_by(2).cloned().collect::<Vec<_>>();
        if keys.iter().any(|k| !matches!(k, Bencode::String(_))) {
            return Err(eyre::eyre!("All keys in a dictionary must be strings"));
        }

        let keys = keys
            .into_iter()
            .map(|k| match k {
                Bencode::String(bytes) => String::from_utf8(bytes)
                    .map_err(|_| eyre::eyre!("Dictionary key must be valid UTF-8")),
                _ => unreachable!(), // We checked above that all keys are strings
            })
            .collect::<Result<Vec<String>>>()?;

        if keys.windows(2).any(|w| w[0] > w[1]) {
            return Err(eyre::eyre!("Keys in a bencoded dictionary must be sorted"));
        }

        let mut dict: HashMap<String, Bencode> = HashMap::new();
        for (key, value) in keys.iter().zip(list.iter().skip(1).step_by(2)) {
            dict.insert(key.clone(), value.clone());
        }

        Ok((Bencode::Dictionary(dict), rest))
    } else {
        Err(eyre::eyre!("Decoded value is not a list"))
    }
}

fn decode_integer(data: &[u8]) -> Result<(Bencode, &[u8])> {
    let first_byte = data.first().ok_or_else(|| eyre::eyre!("Empty data"))?;
    if *first_byte != b'i' {
        return Err(eyre::eyre!("Invalid integer format"));
    }

    let end_index = data
        .iter()
        .position(|&b| b == b'e')
        .ok_or_else(|| eyre::eyre!("Missing 'e' in integer"))?;

    let integer_bytes = &data[1..end_index];
    let integer_string =
        std::str::from_utf8(integer_bytes).map_err(|_| eyre::eyre!("Invalid UTF-8 in integer"))?;

    if integer_string.len() > 1 && integer_string.starts_with('0') {
        return Err(eyre::eyre!("Leading zeros are not allowed in integers"));
    }
    if integer_string.starts_with("-0") {
        return Err(eyre::eyre!("Negative zeros are not allowed"));
    }

    let integer = integer_string.parse::<i64>()?;

    let rest = &data[end_index + 1..];

    Ok((Bencode::Integer(integer), rest))
}
