use yew::prelude::*;
use crate::calc::result::Result;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub result: Result,
}


#[function_component(ResultsDiv)]
pub fn results_div(props: &Props) -> Html {
    let Props { result } = props.clone();
    let tbsp_string = String::from(format!("Tablespoons: {}", result.tablespoons));
    let tsp_string = String::from(format!("Teaspoons: {}", result.teaspoons));
    let leftovers_string = String::from(format!("Leftovers: {}", result.leftovers));
    let scoops_string = String::from(format!("Scoops: {}", result.scoop_portions));

    html! {
        <div class="row">
            <div class="col-12 p-4">
                <div class="card">
                  <div class="card-header bg-dark bg-gradient text-white p-3 mb-0">
                    <h4>{"Spoon Measurements"}</h4>
                  </div>
                  <div class="card-body">
                    <p class="card-text">{tbsp_string}</p>
                    <p class="card-text">{tsp_string}</p>
                    <p class="card-text">{leftovers_string}</p>
                  </div>
                </div>
            </div>
            <div class="col-12 p-4">
                <div class="card">
                  <div class="card-header bg-dark bg-gradient text-white p-3 mb-0">
                    <span><h4>{"Scoop Measurements"}</h4></span>
                  </div>
                  <div class="card-body">
                    <p class="card-text">{scoops_string}</p>
                  </div>
                </div>
            </div>
        </div>
    }
}