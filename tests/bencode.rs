use torrust::bencode;

#[test]
fn test_decode_positive_integer() {
    let encoded = "i42e".to_string().into_bytes();
    let decoded = bencode::decode(encoded).unwrap();
    match decoded {
        bencode::Bencode::Integer(n) => assert_eq!(n, 42),
        _ => panic!("Decoded value is not an integer"),
    }
}

#[test]
fn test_decode_negative_integer() {
    let encoded = "i-42e".to_string().into_bytes();
    let decoded = bencode::decode(encoded).unwrap();
    match decoded {
        bencode::Bencode::Integer(n) => assert_eq!(n, -42),
        _ => panic!("Decoded value is not an integer"),
    }
}

#[test]
fn test_decode_string() {
    let encoded = "4:spam".to_string().into_bytes();
    let decoded = bencode::decode(encoded).unwrap();
    match decoded {
        bencode::Bencode::String(s) => assert_eq!(s, b"spam"),
        _ => panic!("Decoded value is not a string"),
    }
}

#[test]
fn test_decode_list() {
    let encoded = "l4:spami42ee".to_string().into_bytes();
    let decoded = bencode::decode(encoded).unwrap();
    match decoded {
        bencode::Bencode::List(l) => {
            assert_eq!(l.len(), 2);
            match &l[0] {
                bencode::Bencode::String(s) => assert_eq!(s, b"spam"),
                _ => panic!("First element is not a string"),
            }
            match &l[1] {
                bencode::Bencode::Integer(n) => assert_eq!(*n, 42),
                _ => panic!("Second element is not an integer"),
            }
        }
        _ => panic!("Decoded value is not a list"),
    }
}

#[test]
fn test_decode_dictionary() {
    let encoded = "d3:cow3:moo4:spam4:eggse".to_string().into_bytes();
    let decoded = bencode::decode(encoded).unwrap();
    match decoded {
        bencode::Bencode::Dictionary(d) => {
            assert_eq!(d.len(), 2);
            assert_eq!(
                d.get("cow").unwrap(),
                &bencode::Bencode::String(b"moo".to_vec())
            );
            assert_eq!(
                d.get("spam").unwrap(),
                &bencode::Bencode::String(b"eggs".to_vec())
            );
        }
        _ => panic!("Decoded value is not a dictionary"),
    }
}

#[test]
fn test_encode_positive_integer() {
    let bencode = bencode::Bencode::Integer(42);
    let encoded = bencode::encode(&bencode);
    assert_eq!(encoded, b"i42e");
}

#[test]
fn test_encode_negative_integer() {
    let bencode = bencode::Bencode::Integer(-42);
    let encoded = bencode::encode(&bencode);
    assert_eq!(encoded, b"i-42e");
}

#[test]
fn test_encode_string() {
    let bencode = bencode::Bencode::String(b"spam".to_vec());
    let encoded = bencode::encode(&bencode);
    assert_eq!(encoded, b"4:spam");
}

#[test]
fn test_encode_list() {
    let bencode = bencode::Bencode::List(vec![
        bencode::Bencode::String(b"spam".to_vec()),
        bencode::Bencode::Integer(42),
    ]);
    let encoded = bencode::encode(&bencode);
    assert_eq!(encoded, b"l4:spami42ee");
}

#[test]
fn test_encode_dictionary() {
    use std::collections::HashMap;
    let mut d = HashMap::new();
    d.insert(
        "spam".to_string(),
        bencode::Bencode::String(b"eggs".to_vec()),
    );
    d.insert("cow".to_string(), bencode::Bencode::String(b"moo".to_vec()));
    let bencode = bencode::Bencode::Dictionary(d);
    let encoded = bencode::encode(&bencode);
    assert_eq!(encoded, b"d3:cow3:moo4:spam4:eggse");
}

#[test]
fn test_decode_zero_integer() {
    let encoded = "i0e".to_string().into_bytes();
    let decoded = bencode::decode(encoded).unwrap();
    assert_eq!(decoded, bencode::Bencode::Integer(0));
}

#[test]
fn test_decode_empty_string() {
    let encoded = "0:".to_string().into_bytes();
    let decoded = bencode::decode(encoded).unwrap();
    assert_eq!(decoded, bencode::Bencode::String(b"".to_vec()));
}

#[test]
fn test_decode_empty_list() {
    let encoded = "le".to_string().into_bytes();
    let decoded = bencode::decode(encoded).unwrap();
    assert_eq!(decoded, bencode::Bencode::List(vec![]));
}

#[test]
fn test_decode_empty_dictionary() {
    let encoded = "de".to_string().into_bytes();
    let decoded = bencode::decode(encoded).unwrap();
    assert_eq!(
        decoded,
        bencode::Bencode::Dictionary(std::collections::HashMap::new())
    );
}

#[test]
fn test_encode_nested_list() {
    let bencode = bencode::Bencode::List(vec![
        bencode::Bencode::String(b"spam".to_vec()),
        bencode::Bencode::List(vec![
            bencode::Bencode::String(b"a".to_vec()),
            bencode::Bencode::String(b"b".to_vec()),
        ]),
    ]);
    let encoded = bencode::encode(&bencode);
    assert_eq!(encoded, b"l4:spaml1:a1:bee");
}

#[test]
fn test_decode_invalid_integer_leading_zero() {
    let encoded = "i03e".to_string().into_bytes();
    assert!(bencode::decode(encoded).is_err());
}

#[test]
fn test_decode_invalid_integer_negative_zero() {
    let encoded = "i-0e".to_string().into_bytes();
    assert!(bencode::decode(encoded).is_err());
}

#[test]
fn test_decode_invalid_string_length() {
    let encoded = "10:abc".to_string().into_bytes();
    assert!(bencode::decode(encoded).is_err());
}

#[test]
fn test_decode_unsorted_dictionary_keys() {
    let encoded = "d4:spami42e3:cow3:mooe".to_string().into_bytes();
    assert!(bencode::decode(encoded).is_err());
}

#[test]
fn test_extra_data() {
    let encoded = "i42e_extra_data".to_string().into_bytes();
    assert!(bencode::decode(encoded).is_err());
}

#[test]
fn test_round_trip_complex() {
    use std::collections::HashMap;
    let mut dict = HashMap::new();
    dict.insert(
        "publisher".to_string(),
        bencode::Bencode::String(b"bob".to_vec()),
    );
    dict.insert(
        "publisher-webpage".to_string(),
        bencode::Bencode::String(b"www.example.com".to_vec()),
    );
    let bencode = bencode::Bencode::Dictionary(dict);

    let encoded = bencode::encode(&bencode);
    let decoded = bencode::decode(encoded).unwrap();
    assert_eq!(bencode, decoded);
}
