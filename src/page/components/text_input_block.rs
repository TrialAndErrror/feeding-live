use yew::prelude::*;
use super::text_input::TextInput;


#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: u16,
    pub on_change: Callback<u16>,
    pub reset_click: Callback<MouseEvent>,
}

#[function_component(TextInputBlock)]
pub fn text_input_block(props: &Props) -> Html {
    return html! {
        <div class="row">
            <div class="col-12 p-4">
                <div class="input-group input-group-lg rounded-lg border border-white">
                    <div class="input-group-prepend">
                        <div class="btn btn-dark">{"mLs of Prepared Formula:"}</div>
                    </div>

                    <TextInput value={props.value} on_change={&props.on_change} class="form-control"/>
                    <div class="input-group-append btn-dark rounded">

                        <button type={"button"} onclick={&props.reset_click} class="btn btn-dark btn-lg">{"Reset"}</button>
                    </div>
                </div>
            </div>
        </div>
    };
}