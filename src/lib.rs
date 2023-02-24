use std::collections::HashMap;

use gloo::history::{AnyHistory, History, MemoryHistory};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/posts/:id")]
    Post { id: u32 },
    #[at("/posts")]
    Posts,
    #[at("/authors/:id")]
    Author { id: u32 },
    #[at("/authors")]
    Authors,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
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
        </Router>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Post { id } => {
            html! { <h1>{id}</h1> }
        }
        Route::Posts => {
            html! { <h1>{"Posts"}</h1> }
        }
        Route::Author { id } => {
            html! { <h1>{"Author"}</h1>}
        }
        Route::Authors => {
            html! { <h1>{"Authors"}</h1> }
        }
        Route::Home => {
            html! { <h1>{"Home"}</h1> }
        }
        Route::NotFound => {
            html! { <h1>{"NotFound"}</h1> }
        }
    }
}
