extern crate cfg_if;
extern crate flatbuffers;
extern crate wasm_bindgen;

// extern crate greet;

//#[path = "../../../src/protocols/query_generated.rs"]
mod query_generated;

mod utils;

use crate::query_generated::query::Query;

use cfg_if::cfg_if;
use gloo_console::console_dbg;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub async fn collect(bytes: &[u8]) {
    console_dbg!("--------=___  ___=----------", bytes.len());
    let q = flatbuffers::root::<Query>(bytes).unwrap();

    console_dbg!(q, "--------=___ collect ___=----------");
}

#[cfg(test)]
#[test]
fn test_main() {
    // flatbuffers_examples()
}
