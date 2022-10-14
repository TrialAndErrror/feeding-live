use yew::prelude::*;

enum Msg {
    SetValue(value)
}

struct Model {
    total_ml: u16,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            total_ml: 0
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) {
        if Msg::SetValue {
             self.total_mL = (Msg::SetValue).value
        };
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input type="text" {onkeypress=ctx.link().callback(|_| Msg::SetValue)} />
        }
    }
}