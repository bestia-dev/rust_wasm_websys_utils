extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use rust_wasm_websys_utils::websysmod::*;

#[wasm_bindgen_test]
fn prepare_json_msg_receivers_for_one02() {
    assert_eq!("[123]", prepare_json_msg_receivers_for_one(123));
}

#[wasm_bindgen_test]
fn utf8_truncate02() {
    let mut x = "abcdefg".to_string();
    utf8_truncate(&mut x, 3);
    assert_eq!("abc", &x);
}
