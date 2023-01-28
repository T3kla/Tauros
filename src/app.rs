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
    dest: &'a str,
    file: &'a str,
    sess: &'a str,
    mode: &'a str,
    name: &'a str,
    desc: &'a str,
}

#[derive(Serialize, Deserialize, PartialEq)]
struct Counter(u32);

#[function_component(App)]
pub fn app() -> Html {
    let dest_input_ref = use_node_ref();
    let file_input_ref = use_node_ref();
    let sess_input_ref = use_node_ref();
    let mode_input_ref = use_node_ref();
    let name_input_ref = use_node_ref();
    let desc_input_ref = use_node_ref();

    let response_back = use_state(|| Counter { 0: 0u32 });
    let response_front = use_state(|| String::new());

    // when send button is pressed, response_back state is changed
    let on_send_pressed = {
        let response_back = response_back.clone();

        Callback::from(move |_| {
            response_back.set(Counter {
                0: response_back.0 + 1,
            });
        })
    };

    // hooks to the response_back state invokes a command when it changes
    {
        let response_front = response_front.clone();
        let response_back = response_back.clone();
        let response_back2 = response_back.clone();

        let dest_input_ref = dest_input_ref.clone();
        let file_input_ref = file_input_ref.clone();
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
                            dest: &dest_input_ref
                                .cast::<web_sys::HtmlInputElement>()
                                .unwrap()
                                .value(),
                            file: &file_input_ref
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

    html! {
        <main class="container">

            <div class="row">
                <label id="label" for="input">{"Destination:"}</label>
                <input id="input" ref={dest_input_ref} placeholder="\\\\192.168.1.40\\Developers\\SHARE AND DELETE\\____LOGS" />
            </div>

            <div class="row">
                <label id="label" for="input">{"Log file:"}</label>
                <input id="input" ref={file_input_ref} placeholder="C:\\SteamLibrary\\steamapps\\common\\HitNRush..." />
            </div>

            <div class="row">
                <label id="label" for="input">{"Session:"}</label>
                <input id="input" ref={sess_input_ref} placeholder="00" />
            </div>

            <div class="row">
                <label id="label" for="input">{"Mode:"}</label>
                <input id="input" ref={mode_input_ref} placeholder="2trio" />
            </div>

            <div class="row">
                <label id="label" for="input">{"Name:"}</label>
                <input id="input" ref={name_input_ref} placeholder="manu" />
            </div>

            <div class="row">
                <label id="label" for="input">{"Description:"}</label>
                <input id="input" ref={desc_input_ref} placeholder="fatal_error_before_deploy" />
            </div>

            <div class="row">
                <button type="button" onclick={on_send_pressed}>{"Send"}</button>
                <p id="response">{ &*response_front }</p>
            </div>

        </main>
    }
}
