use bytes::{Bytes, BytesMut};
use ethnum::U256;
use fastrlp::{Decodable, Encodable};
use fastrlp_derive::{RlpDecodable, RlpEncodable};
use hex_literal::hex;

#[derive(Debug, PartialEq, RlpEncodable, RlpDecodable)]
struct Item {
    a: Bytes,
}

#[derive(Debug, PartialEq, RlpEncodable, RlpDecodable)]
struct Test4Numbers {
    a: u8,
    b: u128,
    c: U256,
    d: U256,
}

fn encoded<T: Encodable>(t: &T) -> BytesMut {
    let mut out = BytesMut::new();
    t.encode(&mut out);
    out
}

#[test]
fn test_encode_item() {
    let item = Item {
        a: b"dog".to_vec().into(),
    };

    let expected = vec![0xc4, 0x83, b'd', b'o', b'g'];
    let out = encoded(&item);
    assert_eq!(out, expected);

    let decoded = Decodable::decode(&mut &*expected).expect("decode failure");
    assert_eq!(item, decoded);

    let item = Test4Numbers {
        a: 0x05,
        b: 0x01deadbeefbaadcafe,
        c: U256::from_be_bytes(hex!(
            "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
        )),
        d: U256::from_be_bytes(hex!(
            "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
        )),
    };

    let expected = hex!("f84d058901deadbeefbaadcafea056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470").to_vec();
    let out = encoded(&item);
    assert_eq!(out, expected);

    let decoded = Decodable::decode(&mut &*expected).expect("decode failure");
    assert_eq!(item, decoded);
}
