use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

// https://github.com/rustwasm/wasm-bindgen/issues/1727
// cargo +nightly build --target wasm32-unknown-unknown
// wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_rust.wasm --out-dir wasm/ --web --no-typescript
#[wasm_bindgen]
pub fn schedule_some_work_for_next_tick(path: String) -> js_sys::Promise {
    let future = NextTick::new()
        // Do some work...
        .and_then(move |_| {
            let data: Vec = fs::read(path.as_str()).expect("Unable to read file");
            Ok(data.len() as f64)
        })
        // And then convert the Item and Error into JsValue.
        .map(|result| JsValue::from(result))
        .map_err(|error| {
            let js_error = js_sys::Error::new(&format!("uh oh! {:?}", error));
            JsValue::from(js_error)
        });
    future_to_promise(future)
}

//  Getting rid of inlined JS #68
// https://github.com/Pauan/rust-dominator/issues/68

#[wasm_bindgen(inline_js = "
    export function set_property(obj, name, value) { obj[name] = value; }

    export function add_event(elem, name, capture, passive, f) {
        elem.addEventListener(name, f, {
            capture,
            passive,
            once: false,
        });
    }

    export function add_event_once(elem, name, f) {
        elem.addEventListener(name, f, {
            capture: true,
            passive: true,
            once: true,
        });
    }

    export function remove_event(elem, name, capture, f) {
        elem.removeEventListener(name, f, capture);
    }
")]
extern "C" {
    // TODO move this into wasm-bindgen or gloo or something
    // TODO maybe use Object for obj ?
    pub(crate) fn set_property(obj: &JsValue, name: &str, value: &JsValue);

    // TODO replace with gloo-events
    pub(crate) fn add_event(
        elem: &EventTarget,
        name: &str,
        capture: bool,
        passive: bool,
        f: &Function,
    );
    pub(crate) fn add_event_once(elem: &EventTarget, name: &str, f: &Function);
    pub(crate) fn remove_event(elem: &EventTarget, name: &str, capture: bool, f: &Function);
}

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: &str);
    #[wasm_bindgen(js_namespace = Object, js_name = defineProperty)]
    fn define_property(obj: &JsValue, prop: &str, descriptor: &JsValue);
    #[wasm_bindgen(js_namespace = Object, js_name = fromEntries)]
    fn from_entries(iterable: &JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = Object, js_name = assign)]
    fn assign(target: &JsValue, source: &JsValue);
    #[wasm_bindgen(js_namespace = Array, js_name = of)]
    fn array_of(of: &JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = Array, js_name = of)]
    fn array_of_2(of_1: &JsValue, of_2: &JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = Array, js_name = of)]
    fn array_prop_key_value(key: &str, value: &JsValue) -> JsValue;
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Solution 1: using Object.defineProperty and Object.fromEntries
    let descriptor = js_sys::Array::new();
    descriptor.push(&js_sys::Array::from_iter([
        JsValue::from("value"),
        JsValue::from(42_i32),
    ]));
    descriptor.push(&js_sys::Array::from_iter([
        JsValue::from("writable"),
        JsValue::from(true),
    ]));
    descriptor.push(&js_sys::Array::from_iter([
        JsValue::from("configurable"),
        JsValue::from(true),
    ]));
    descriptor.push(&js_sys::Array::from_iter([
        JsValue::from("enumerable"),
        JsValue::from(true),
    ]));
    let descriptor = from_entries(&descriptor);
    let obj = js_sys::Object::new();
    define_property(&obj, "foo", &descriptor);
    log(&format!("it works: {:?}", obj));

    // Solution 2: using Object.fromEntries and js_sys::Array::from_iter
    let obj = js_sys::Object::new();
    assign(
        &obj,
        &from_entries(&js_sys::Array::from_iter([&js_sys::Array::from_iter([
            JsValue::from("bar"),
            JsValue::from(42_i32),
        ])])),
    );
    log(&format!("it works: {:?}", obj));

    // Solution 3: using Object.fromEntries and Array.of and js_sys::Array::from_iter
    let obj = js_sys::Object::new();
    assign(
        &obj,
        &from_entries(&array_of(&js_sys::Array::from_iter([
            JsValue::from("baz"),
            JsValue::from(42_i32),
        ]))),
    );
    log(&format!("it works: {:?}", obj));

    // Solution 4: using Object.fromEntries and Array.of
    let obj = js_sys::Object::new();
    assign(
        &obj,
        &from_entries(&array_of(&array_of_2(
            &JsValue::from("foobar"),
            &JsValue::from(42_i32),
        ))),
    );
    log(&format!("it works: {:?}", obj));

    // Solution 5: using Object.fromEntries and Array.of and a special Array.of that takes &str in argument
    let obj = js_sys::Object::new();
    assign(
        &obj,
        &from_entries(&array_of(&array_prop_key_value(
            wasm_bindgen::intern("foobaz"),
            &JsValue::from(42_i32),
        ))),
    );
    log(&format!("it works: {:?}", obj));

    Ok(())
}
