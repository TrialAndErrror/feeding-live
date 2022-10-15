use yew::prelude::*;
use crate::page::signals::Msg;
use crate::page::html_input::TextInput;
use crate::page::results_panel::ResultsDiv;

use crate::calc::convert_units::convert_ml_to_tbsp;
use crate::calc::convert_units::Result;



#[function_component(App)]
pub fn app() -> Html {
    html!(
            <div>
                {"Hi!"}
            </div>
        )
}

pub struct Model {
    total_ml: u16,
    current_result: Result,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            total_ml: 0,
            current_result: Result {
                tablespoons: 0,
                teaspoons: 0,
                leftovers: 0.0,
                scoop_portions: 0.0,
            },
        }
    }


    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetValue(value) => {
                self.total_ml = value;
            }
            Msg::ResetValue => self.total_ml = 0,
        };
        if self.total_ml != 0 {
            self.current_result = convert_ml_to_tbsp(self.total_ml);
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(Msg::SetValue);
        let reset_click = ctx.link().callback(|_| Msg::ResetValue);

        match self.total_ml {
            0 => html!{
                <div class="container">
                    <div class="jumbotron bg-transparent">
                        <h1 class="display">{"Formula Calculator"}</h1>
                        <p class="lead">{"This is a simple Rust-based application for calculating the amount of powder to mix with water to prepare Nutramigen formula at 22 calories per ounce."}</p>
                        <hr class="my-4" />
                        <p>{"Enter the desired amount of prepared formula in the box below to calculate the amount of powdered formula necessary."}</p>
                    </div>
                    <div class="row">
                        <div class="col-12 p-4">
                            <div class="input-group input-group-lg rounded-lg border border-white">
                                <div class="input-group-prepend">
                                    <div class="btn btn-dark">{"mLs of Prepared Formula:"}</div>
                                </div>

                                <TextInput value={0} {on_change} class="form-control"/>
                                <div class="input-group-append btn-dark rounded">

                                    <button type={"button"} onclick={reset_click} class="btn btn-dark btn-lg">{"Reset"}</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            },
            _ => html!{
                <div class="container">
                    <div class="bg-transparent">
                            <h1 class="display">{"Formula Calculator"}</h1>
                                        <p>{"Enter the desired amount of prepared formula in the box below to calculate the amount of powdered formula necessary."}</p>

                    </div>
                    <div class="row">
                        <div class="col-12 p-4">
                            <div class="input-group input-group-lg rounded-lg border border-white">
                               <div class="input-group-prepend">
                                    <div class="btn btn-dark">{"mLs of Prepared Formula:"}</div>
                                </div>

                                <TextInput value={self.total_ml.clone()} {on_change} class="form-control"/>
                                <div class="input-group-append btn-dark rounded">

                                    <button type={"button"} onclick={reset_click} class="btn btn-dark btn-lg">{"Reset"}</button>
                                </div>
                            </div>
                        </div>
                    </div>

                    <ResultsDiv result={self.current_result.clone()}/>
                </div>
            }
        }

    }
}