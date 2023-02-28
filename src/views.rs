pub(crate) mod articles;
pub(crate) mod components;

use std::collections::HashMap;

use gloo::{
    history::{AnyHistory, History, MemoryHistory},
    storage::{LocalStorage, Storage},
};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use yew::{
    function_component, html, use_effect_with_deps, use_reducer, use_state, AttrValue, Callback,
    Html, Properties, Reducible, Suspense,
};
use yew_router::{BrowserRouter, Router, Switch};
use yewdux::prelude::use_store;

use crate::{
    routes::Route,
    settings::COLOR_MODE_STATE_KEY,
    state::ColorMode,
    views::{articles::Posts, components::header::Header},
};

use self::components::headline::Headline;

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Content />
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Content />
        </Router>
    }
}

#[function_component]
pub fn Content() -> Html {
    let (color_mode, _) = use_store::<ColorMode>();

    html!(
        <div class={&color_mode.to_string()}>
            <Header />
            <main>
                <Switch<Route> render={switch} />
            </main>
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">{ "Bulma" }</a>
                    { " and images from " }
                    <a href="https://unsplash.com">{ "Unsplash" }</a>
                </div>
            </footer>
        </div>
    )
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Post { id } => {
            html! { <h1>{id}</h1> }
        }
        Route::Posts => {
            let fallback = html! {<div>{"Loading..."}</div>};
            html! {
                <Suspense {fallback}>
                    <Posts />
                </Suspense>
            }
        }
        Route::Author { id: _ } => {
            html! { <h1>{"Author"}</h1>}
        }
        Route::Authors => {
            html! { <h1>{"Authors"}</h1> }
        }
        Route::Home => {
            html! {
               <Headline />
            }
        }
        Route::NotFound => {
            html! { <h1>{"NotFound"}</h1> }
        }
    }
}
