use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/unlock")]
    Unlock,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Unlock => html! { <h1>{ "Unlock" }</h1> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}