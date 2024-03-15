#![feature(proc_macro_hygiene)]

use bitcoin::hashes::hex::ToHex;
use bitcoin_script::bitcoin_script;

#[test]
fn fixture() {
    let foo = vec![1, 2, 3, 4];
    let script = bitcoin_script! {
        OP_HASH160
        1234
        255
        -1
        -255
        0xabcd
        <1 + 1>
        <foo>
    };

    assert_eq!(
        script.to_bytes(),
        vec![169, 2, 210, 4, 2, 255, 0, 79, 2, 255, 128, 2, 171, 205, 82, 4, 1, 2, 3, 4]
    );
}

#[test]
fn lnhance() {
    let s = hex::decode("a2f0e0d0c0b0a0908070605040302010").unwrap();

    let script = bitcoin_script! {
        OP_CHECKTEMPLATEVERIFY
        0x0102030405060708090a0b0c0d0e0f2a
        OP_CHECKSIGFROMSTACKVERIFY
        <s>
        OP_CLTV
    };

    assert_eq!(
        script.to_hex(),
        "b3100102030405060708090a0b0c0d0e0f2ab410a2f0e0d0c0b0a0908070605040302010b1"
    );
}
