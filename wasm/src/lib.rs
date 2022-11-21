#![allow(dead_code)]

use rmp_serde::decode;
use serde::Deserialize;

#[derive(Debug, serde::Deserialize)]
struct MyHack {
    inner: u32,
    inner2: u16,
    inner3: u16,
    inner4: u8,
}

// Deserializing from msgpack crashes under Singlepass, works under Cranelift
#[no_mangle]
pub extern "C" fn deserialize_msgpack() {
    /*
    This slice is the messagepack equivalent of:
        {
            "inner": 1,
            "inner2": 2,
            "inner3": 3,
            "inner4": 4
        }
     */
    let slice = vec![
        132, 165, 105, 110, 110, 101, 114, 1, 166, 105, 110, 110, 101, 114, 50, 2, 166, 105, 110, 110, 101, 114, 51, 3, 166, 105, 110, 110, 101, 114, 52, 4
    ];

    let mut deserializer = decode::Deserializer::new(&slice[..]).with_human_readable();
    let value = MyHack::deserialize(&mut deserializer).unwrap();
    assert_eq!(value.inner, 1);
    assert_eq!(value.inner2, 2);
    assert_eq!(value.inner3, 3);
    assert_eq!(value.inner4, 4);
}

// Deserializing from JSON works fine with either Cranelift or Singlepass
#[no_mangle]
pub extern "C" fn deserialize_json() {
    let value: MyHack = serde_json::from_str(r#"{ "inner": 1, "inner2": 2, "inner3": 3, "inner4": 4 }"#).unwrap();
    assert_eq!(value.inner, 1);
    assert_eq!(value.inner2, 2);
    assert_eq!(value.inner3, 3);
    assert_eq!(value.inner4, 4);
}
