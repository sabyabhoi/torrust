use torrust::bencode;

#[test]
fn test_decode_positive_integer() {
    let encoded = "i42e".to_string();
    let decoded = bencode::decode(encoded).unwrap();
    match decoded {
        bencode::Bencode::Integer(n) => assert_eq!(n, 42),
        _ => panic!("Decoded value is not an integer"),
    }
}

#[test]
fn test_decode_negative_integer() {
    let encoded = "i-42e".to_string();
    let decoded = bencode::decode(encoded).unwrap();
    match decoded {
        bencode::Bencode::Integer(n) => assert_eq!(n, -42),
        _ => panic!("Decoded value is not an integer"),
    }
}

#[test]
fn test_decode_string() {
    let encoded = "4:spam".to_string();
    let decoded = bencode::decode(encoded).unwrap();
    match decoded {
        bencode::Bencode::String(s) => assert_eq!(s, "spam"),
        _ => panic!("Decoded value is not a string"),
    }
}

#[test]
fn test_decode_list() {
    let encoded = "l4:spami42ee".to_string();
    let decoded = bencode::decode(encoded).unwrap();
    match decoded {
        bencode::Bencode::List(l) => {
            assert_eq!(l.len(), 2);
            match &l[0] {
                bencode::Bencode::String(s) => assert_eq!(s, "spam"),
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
    let encoded = "d3:cow3:moo4:spam4:eggse".to_string();
    let decoded = bencode::decode(encoded).unwrap();
    match decoded {
        bencode::Bencode::Dictionary(d) => {
            assert_eq!(d.len(), 2);
            assert_eq!(
                d.get("cow").unwrap(),
                &bencode::Bencode::String("moo".to_string())
            );
            assert_eq!(
                d.get("spam").unwrap(),
                &bencode::Bencode::String("eggs".to_string())
            );
        }
        _ => panic!("Decoded value is not a dictionary"),
    }
}
