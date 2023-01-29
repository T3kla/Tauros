use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct SendArgs<'a> {
    file: &'a str,
    dest: &'a str,
    sess: &'a str,
    mode: &'a str,
    name: &'a str,
    desc: &'a str,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct Counter(u32);

#[derive(Serialize, Deserialize, PartialEq)]
struct Empty();

#[function_component(App)]
pub fn app() -> Html {
    // Input states

    let file_input_ref = use_node_ref();
    let dest_input_ref = use_node_ref();
    let sess_input_ref = use_node_ref();
    let mode_input_ref = use_node_ref();
    let name_input_ref = use_node_ref();
    let desc_input_ref = use_node_ref();

    let response_front = use_state(|| String::new());

    // Restore response front timer

    // Log file dialog button stuff

    let file_back = use_state(|| Counter { 0: 0u32 });

    let on_file_pressed = {
        let file_back = file_back.clone();

        Callback::from(move |_| {
            file_back.set(Counter { 0: file_back.0 + 1 });
        })
    };

    {
        let file_input_ref = file_input_ref.clone();
        let response_front = response_front.clone();
        let file_back = file_back.clone();
        let file_back2 = file_back.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if file_back.0 == 0 {
                        return;
                    }
                    let result = invoke("file_command", to_value(&Empty {}).unwrap()).await;
                    match result.as_string() {
                        Some(s) => {
                            response_front.set("".to_string());
                            file_input_ref
                                .cast::<web_sys::HtmlInputElement>()
                                .unwrap()
                                .set_value(&s);
                        }
                        None => {
                            response_front.set("Failed to get file".to_string());
                        }
                    };
                });
                || {}
            },
            file_back2,
        );
    }

    // Destination folder dialog button stuff

    let dest_back = use_state(|| Counter { 0: 0u32 });

    let on_dest_pressed = {
        let dest_back = dest_back.clone();

        Callback::from(move |_| {
            dest_back.set(Counter { 0: dest_back.0 + 1 });
        })
    };

    {
        let dest_input_ref = dest_input_ref.clone();
        let response_front = response_front.clone();
        let dest_back = dest_back.clone();
        let dest_back2 = dest_back.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if dest_back.0 == 0 {
                        return;
                    }
                    let result = invoke("dest_command", to_value(&Empty {}).unwrap()).await;
                    match result.as_string() {
                        Some(s) => {
                            response_front.set("".to_string());
                            dest_input_ref
                                .cast::<web_sys::HtmlInputElement>()
                                .unwrap()
                                .set_value(&s);
                        }
                        None => {
                            response_front.set("Failed to get folder".to_string());
                        }
                    };
                });
                || {}
            },
            dest_back2,
        );
    }

    // Send button stuff

    let response_back = use_state(|| Counter { 0: 0u32 });

    let on_send_pressed = {
        let response_back = response_back.clone();

        Callback::from(move |_| {
            response_back.set(Counter {
                0: response_back.0 + 1,
            });
        })
    };

    {
        let response_front = response_front.clone();
        let response_back = response_back.clone();
        let response_back2 = response_back.clone();

        let file_input_ref = file_input_ref.clone();
        let dest_input_ref = dest_input_ref.clone();
        let sess_input_ref = sess_input_ref.clone();
        let mode_input_ref = mode_input_ref.clone();
        let name_input_ref = name_input_ref.clone();
        let desc_input_ref = desc_input_ref.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if response_back.0 == 0 {
                        return;
                    }
                    let new_msg = invoke(
                        "send_command",
                        to_value(&SendArgs {
                            file: &file_input_ref
                                .cast::<web_sys::HtmlInputElement>()
                                .unwrap()
                                .value(),
                            dest: &dest_input_ref
                                .cast::<web_sys::HtmlInputElement>()
                                .unwrap()
                                .value(),
                            sess: &sess_input_ref
                                .cast::<web_sys::HtmlInputElement>()
                                .unwrap()
                                .value(),
                            mode: &mode_input_ref
                                .cast::<web_sys::HtmlInputElement>()
                                .unwrap()
                                .value(),
                            name: &name_input_ref
                                .cast::<web_sys::HtmlInputElement>()
                                .unwrap()
                                .value(),
                            desc: &desc_input_ref
                                .cast::<web_sys::HtmlInputElement>()
                                .unwrap()
                                .value(),
                        })
                        .unwrap(),
                    )
                    .await;
                    response_front.set(new_msg.as_string().unwrap());
                });
                || {}
            },
            response_back2,
        );
    }

    // Html stuff

    html! {
    <main class="grid-container">

        <label id="label" for="input">{"File:"}</label>
        <input ref={file_input_ref} placeholder="C:\\SomeFolder\\AnotherFolder\\logFile.log" />
        <button style="margin-right: 2px" type="button" onclick={on_file_pressed}>{">"}</button>

        <label id="label" for="input">{"Destination:"}</label>
        <input ref={dest_input_ref} placeholder="C:\\LoggingFolder\\AnotherFolder" />
        <button style="margin-right: 2px" type="button" onclick={on_dest_pressed}>{">"}</button>

        <label id="label" for="input">{"Session:"}</label>
        <input ref={sess_input_ref} placeholder="00" />
        <div></div>

        <label id="label" for="input">{"Mode:"}</label>
        <input ref={mode_input_ref} placeholder="1mode" />
        <div></div>

        <label id="label" for="input">{"Name:"}</label>
        <input ref={name_input_ref} placeholder="name" />
        <div></div>

        <label id="label" for="input">{"Description:"}</label>
        <input ref={desc_input_ref} placeholder="fatal error before X" />
        <div></div>

        <button type="button" onclick={on_send_pressed}>{"Send"}</button>
        <label style="text-align: center" id="label">{ &*response_front }</label>
        <div></div>

    </main>
    }
}
