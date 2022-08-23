use yew_router::prelude::*;
use yew::prelude::*;

use crate::router::{Route, switch};

pub mod components;
pub mod router;
pub mod language;
pub mod themes; 

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}