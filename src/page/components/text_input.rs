use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: u16,
    pub on_change: Callback<u16>,
    pub class: String
}


fn get_value_from_input_event(e: InputEvent) -> u16 {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.value().into());
    target.value().parse::<u16>().unwrap()
}

/// Controlled Text Input Component
#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props { value, on_change , class} = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event));
    });

    html! {
        <input type="text" class={class} value={value.to_string()} {oninput} />
    }
}