use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[function_component]
pub fn EventComponent() -> Html {
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_cautious_change = {
        let input_value_handle = input_value_handle.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    html! {
        <div class="event">
        <form>
        <label for= "cautious-input" >
            { "Cautious Input" }
            <input onchange = {on_cautious_change} id ="cautious-input" type="text" value={input_value.clone()} />
        </label>
        <button type="submit"> { "Submit" } </button>
        </form>
         <h1>{input_value}</h1>

    </div>
    }
}
