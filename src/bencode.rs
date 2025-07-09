use eyre::{OptionExt, Result};

pub enum Bencode {
    Integer(i64),
    String(String),
    List(Vec<Bencode>),
    Dictionary(std::collections::HashMap<String, Bencode>),
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

fn decode_list(s: String) -> Result<(Bencode, String)> {
    unimplemented!()
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
fn decode_dictionary(s: String) -> Result<(Bencode, String)> {
    unimplemented!()
}
