use yew::prelude::*;
use crate::page::signals::Msg;
use crate::page::views::{render_some_ml, render_0_ml};

use crate::calc::convert_units::convert_ml_to_tbsp;
use crate::calc::result::Result;

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
            0 => render_0_ml(&on_change, &reset_click),
            _ => render_some_ml(self.total_ml.clone(), self.current_result.clone(), &on_change, &reset_click)
        }
    }
}