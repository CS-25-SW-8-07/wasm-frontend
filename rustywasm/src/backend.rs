use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::state::StateHandle;

pub async fn sleep(timeout: i32) -> Result<(), JsValue> {
    let promise = js_sys::Promise::new(&mut |res, rej| {
        let Some(window) = web_sys::window() else {
            let _ = rej.call0(&JsValue::from_str("No window"));
            return;
        };

        if let Err(val) =
            window.set_timeout_with_callback_and_timeout_and_arguments_0(&res, timeout)
        {
            let _ = rej.call0(&val);
        }
    });

    wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map(|_| ())
}

#[wasm_bindgen]
#[allow(dead_code)]
struct Backend {
    state: StateHandle,
}

#[wasm_bindgen]
impl Backend {
    #[allow(dead_code)]
    #[wasm_bindgen(constructor)]
    pub fn new(state: StateHandle) -> Self {
        Self { state }
    }

    #[allow(dead_code)]
    #[wasm_bindgen]
    pub async fn start(&mut self) -> Result<(), JsValue> {
        Ok(())
    }
}
