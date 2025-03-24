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
        main(class="w-screen h-screen") {
            div(class="flex flex-grow flex-col") {
                div(class="flex flex-col flex-grow p-4") {

                }
                div(class="flex flex-row gap-4 p-4") {
                    input(class="flex-grow p-2 border rounded") {

                    }
                    button(class="px-4 py-2 bg-blue-500 text-white rounded") {
                        "Send"
                    }
                }
            }
        }
    }
}
