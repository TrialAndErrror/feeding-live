use yew::{html, Callback, MouseEvent, Html};
use crate::calc::convert_units::Result;

use super::components::text_input_block::TextInputBlock;
use super::components::results_div::ResultsDiv;


pub fn render_0_ml(on_change: &Callback<u16>, reset_click: &Callback<MouseEvent>) -> Html {
    return html! {
        <div class="container">
            <div class="jumbotron bg-transparent">
                <h1 class="display">{"Formula Calculator"}</h1>
                <p class="lead">{"This is a simple Rust-based application for calculating the amount of powder to mix with water to prepare Nutramigen formula at 22 calories per ounce."}</p>
                <hr class="my-4" />
                <p>{"Enter the desired amount of prepared formula in the box below to calculate the amount of powdered formula necessary."}</p>
            </div>

            <TextInputBlock value={0} {on_change} {reset_click} />
        </div>
    };
}


pub fn render_some_ml(value: u16, result: Result, on_change: &Callback<u16>, reset_click: &Callback<MouseEvent>) -> Html {
    return html! {
        <div class="container">
            <div class="bg-transparent">
                <h1 class="display">{"Formula Calculator"}</h1>
                <p>{"Enter the desired amount of prepared formula in the box below to calculate the amount of powdered formula necessary."}</p>

            </div>

            <TextInputBlock {value} {on_change} {reset_click} />

            <ResultsDiv {result} />
        </div>
    };
}