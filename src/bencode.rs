use std::collections::HashMap;

use eyre::{OptionExt, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Bencode {
    Integer(i64),
    String(String),
    List(Vec<Bencode>),
    Dictionary(HashMap<String, Bencode>),
}

pub fn decode(s: String) -> Result<Bencode> {
    let (bencode, remaining) = decode_bencoded_string(s)?;
    if !remaining.is_empty() {
        return Err(eyre::eyre!(
            "Extra data after bencoded value: {}",
            remaining
        ));
    }
    Ok(bencode)
}

fn decode_bencoded_string(s: String) -> Result<(Bencode, String)> {
    let first_char = s
        .chars()
        .next()
        .ok_or_else(|| eyre::eyre!("Empty string"))?;

    match first_char {
        'i' => decode_integer(s),
        'l' => decode_list(s),
        'd' => decode_dictionary(s),
        '0'..='9' => decode_string(s),
        _ => Err(eyre::eyre!("Invalid bencode format")),
    }
}

fn decode_string(s: String) -> Result<(Bencode, String)> {
    let (length_string, rest) = s.split_once(':').ok_or_eyre("Invalid string")?;

    let length = length_string.parse::<usize>()?;
    if length > rest.len() {
        return Err(eyre::eyre!("String length exceeds remaining data"));
    }

    let (decoded_string, rest) = rest.split_at(length);

    Ok((
        Bencode::String(decoded_string.to_string()),
        rest.to_string(),
    ))
}

fn decode_partial_list(s: String) -> Result<(Bencode, String)> {
    let mut vec = Vec::new();
    let mut rest = s[1..].to_string();
    while rest.len() > 0 && rest.chars().next() != Some('e') {
        let (item, new_rest) = decode_bencoded_string(rest)?;
        vec.push(item);
        rest = new_rest;
    }

    Ok((Bencode::List(vec), rest[1..].to_string()))
}

fn decode_list(s: String) -> Result<(Bencode, String)> {
    let first_char = s
        .chars()
        .next()
        .ok_or_else(|| eyre::eyre!("Empty string"))?;
    if first_char != 'l' {
        return Err(eyre::eyre!("Invalid list format"));
    }

    decode_partial_list(s)
}

fn decode_dictionary(s: String) -> Result<(Bencode, String)> {
    let first_char = s
        .chars()
        .next()
        .ok_or_else(|| eyre::eyre!("Empty string"))?;
    if first_char != 'd' {
        return Err(eyre::eyre!("Invalid dictionary format"));
    }

    let (list, rest) = decode_partial_list(s)?;

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
                Bencode::String(s) => s,
                _ => unreachable!(), // We checked above that all keys are strings
            })
            .collect::<Vec<String>>();

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

fn decode_integer(s: String) -> Result<(Bencode, String)> {
    let first_char = s
        .chars()
        .next()
        .ok_or_else(|| eyre::eyre!("Empty string"))?;
    if first_char != 'i' {
        return Err(eyre::eyre!("Invalid integer format"));
    }

    let end_index = s
        .find('e')
        .ok_or_else(|| eyre::eyre!("Missing 'e' in integer"))?;

    let integer_string = &s[1..end_index];
    let integer = integer_string.parse::<i64>()?;

    let rest = s[end_index + 1..].to_string();

    Ok((Bencode::Integer(integer), rest))
}
