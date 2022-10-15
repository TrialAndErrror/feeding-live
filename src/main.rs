mod app;
mod page;
mod calc;

use app::Model;

fn main() {
    yew::start_app::<Model>();
}
