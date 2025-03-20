use serde::{Deserialize, Serialize};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::web::events::SubmitEvent;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn unlisten(unlisten_key: JsValue) -> JsValue;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MessageWrapper {
    pub user_id: String,
    pub content: String,
    pub sent_date: Option<i64>,
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct MsgArgs<'a> {
    msg: &'a str,
}

#[component]
pub fn App() -> View {
    let name = create_signal(String::new());
    let greet_msg = create_signal(String::new());

    let msg = create_signal(String::new());

    let greet = move |e: SubmitEvent| {
        e.prevent_default();
        spawn_local_scoped(async move {
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let args = serde_wasm_bindgen::to_value(&GreetArgs {
				name: &name.get_clone()
			})
			.unwrap();
            let new_msg = invoke("greet", args).await;
            greet_msg.set(new_msg.as_string().unwrap());
        })
    };

    let send_message = move |e:SubmitEvent| {
        e.prevent_default();
        spawn_local_scoped(async move {
            let args = serde_wasm_bindgen::to_value(&MsgArgs {
                msg:  &msg.get_clone()
            })
            .unwrap();
            let _ = invoke("send_message", args).await;
        });
    };

    view! {
        main(class="container") {
            div(class="bg-red-500 text-white font-bold rounded-lg p-4") {
                p { "This should be styled with Tailwind, how does this work?" }
            }
        }
    }
}
