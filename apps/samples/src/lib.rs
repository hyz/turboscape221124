extern crate cfg_if;
extern crate wasm_bindgen;
mod utils;

use cfg_if::cfg_if;
use serde::Serialize;
use serde_wasm_bindgen::*;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__"], js_name = "invoke")]
    async fn js_invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen( js_namespace = ["window", "__TAURI__", "fs"], js_name = "exists")]
    async fn js_exists(cmd: &str) -> JsValue;

    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "fs"], js_name = "createDir")]
    async fn js_create_dir(cmd: &str, opts: JsValue) -> Result<JsValue, JsValue>;

    // #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "fs"], js_name = "createDir")]
    // fn createDir(dir: &str, options: JsValue) -> js_sys::Promise;
}

#[wasm_bindgen]
pub async fn greet(name: &str) -> Result<JsValue, JsValue> {
    // let win = web_sys::window().unwrap();
    // _ = dbg!(win.alert_with_message(&format!("Greet,{}", name)));
    // _ = dbg!(alert(&format!("Greet,{}", name)));

    #[derive(Serialize)]
    struct InvokeArgs {
        name: String,
    }
    //  let val = json!({ "name": name }); // !
    let val = InvokeArgs { name: name.into() };
    js_invoke("greet", to_value(&val).unwrap()).await
}

#[wasm_bindgen]
pub async fn create_dir(path: &str) -> Result<JsValue, JsValue> {
    #[derive(Serialize)]
    struct Options {
        recursive: bool,
    }
    let opts = Options { recursive: true };
    js_create_dir(path, to_value(&opts).unwrap()).await
}
#[wasm_bindgen]
pub async fn exists(path: &str) -> JsValue {
    js_exists(path).await
}
