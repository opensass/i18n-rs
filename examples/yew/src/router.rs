use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::landing::LandingPage;
use crate::pages::struct_components::StructComponents;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    LandingPage,
    
    #[at("/struct_components")]
    StructComponents,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::LandingPage => html! { <LandingPage /> },
        Route::StructComponents => html! { <StructComponents /> },
    }
}
