use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name=rive)]
    static RIVE: JsValue;

    #[derive(PartialEq, Clone)]
    pub type Rive;

    #[wasm_bindgen(constructor)]
    pub fn new(opts: &JsValue) -> Rive;

    #[wasm_bindgen(method, js_name=stateMachineInputs)]
    pub fn state_machine_inputs(this: &Rive, name: &JsValue) -> Array;

    #[wasm_bindgen(method)]
    pub fn cleanup(this: &Rive);
}
