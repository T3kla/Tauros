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
struct GreetArgs<'a> {
    name: &'a str,
}

struct Model(i32);

#[function_component(App)]
pub fn app() -> Html {
    let dest_input_ref = use_node_ref();
    let file_input_ref = use_node_ref();
    let session_input_ref = use_node_ref();
    let mode_input_ref = use_node_ref();
    let name_input_ref = use_node_ref();
    let desc_input_ref = use_node_ref();

    // states
    //      sets up a state that yew is aware of, so it can re-render the component when it changes
    let state = use_state(|| Model(2));
    let name = use_state(|| String::new());
    let greet_msg = use_state(|| String::new());

    // functions
    //      functions that can be called from html
    //      this functions can change states but not execute rust code

    // returns the data from greet_input_ref
    let greet = {
        let name = name.clone();
        let greet_input_ref = dest_input_ref.clone();

        Callback::from(move |_| {
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    // function that adds 1 to the state
    let on_click = {
        // creates a clone of state because the original state is being used by the UI framework
        let state = state.clone();

        // move -> moves the ownership of stuff the closure uses to the closure and makes them inaccessible outside
        // |_|  -> callback expects a function that takes a parameter, but we don't need it
        Callback::from(move |_| state.set(Model { 0: state.0 * 2 }))
    };

    // *******************************

    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }
                    let new_msg =
                        invoke("greet", to_value(&GreetArgs { name: &*name }).unwrap()).await;
                    log(&new_msg.as_string().unwrap());
                    greet_msg.set(new_msg.as_string().unwrap());
                });

                || {}
            },
            name2,
        );
    }

    html! {
        <main class="container">

            <div class="row">
                <label id="label" for="input">{"Destination:"}</label>
                <input id="input" ref={dest_input_ref} value="\\\\192.168.1.40\\Developers\\SHARE AND DELETE\\____LOGS" />
            </div>

            <div class="row">
                <label id="label" for="input">{"Log file:"}</label>
                <input id="input" ref={file_input_ref} placeholder="C:\\SteamLibrary\\steamapps\\common\\HitNRush..." />
            </div>

            <div class="row">
                <label id="label" for="input">{"Session:"}</label>
                <input id="input" ref={session_input_ref} placeholder="00" />
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
                <button type="button" onclick={greet}>{"Send"}</button>
                <button type="button" onclick={on_click}>{" +1 "}</button>
                <p><center><b>{ state.0 }</b></center></p>
                <p><center><b>{ &*greet_msg }</b></center></p>
            </div>

        </main>
    }
}
