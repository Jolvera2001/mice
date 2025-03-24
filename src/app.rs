use serde::{Deserialize, Serialize};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
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

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
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
    let msg = create_signal(String::new());    
    let chat = create_signal(Vec::<MessageWrapper>::new());

    let send_message = move |_| {
        if msg.with(|m| m.trim().is_empty()) {
            return;
        }

        spawn_local_scoped(async move {
            let args = serde_wasm_bindgen::to_value(&MsgArgs {
                msg:  &msg.get_clone()
            })
            .unwrap();
            let result =invoke("send_message", args).await;

            if let Ok(message) = serde_wasm_bindgen::from_value::<MessageWrapper>(result) {
                chat.update(|messages| {
                    messages.push(message);
                });
            } else {
                println!("Error processing message");
            }
        });

        msg.set(String::new());
    };

    view! {
        main(class="w-screen h-screen") {
            div(class="flex flex-grow flex-col h-full") {
                div(class="flex flex-col flex-grow p-4") {
                    Indexed(
                        list=chat,
                        view=|msg| view! {
                            p { (msg.content) }
                        },
                    )
                }
                div(class="flex flex-row gap-4 p-4") {
                    input(
                        class="flex-grow p-2 border rounded",
                        bind:value=msg
                    ) {

                    }
                    button(
                        class="px-4 py-2 bg-blue-500 text-white rounded",
                        on:click=send_message
                    ) {
                        "Send"
                    }
                }
            }
        }
    }
}
