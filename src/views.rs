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
    classes, function_component, html, use_effect_with_deps, use_reducer, use_state, AttrValue,
    Callback, Html, Properties, Reducible, Suspense,
};
use yew_hooks::use_effect_once;
use yew_router::{BrowserRouter, Router, Switch};
use yewdux::prelude::use_store;

use crate::{
    routes::Route,
    settings::COLOR_MODE_STATE_KEY,
    state::{ColorMode, ColorModeActions},
    test_data::use_sleep,
    views::{articles::Posts, components::header::Header},
};

use self::components::headline::Headline;

#[function_component]
pub fn App() -> Html {
    let fallback = html! {<div>{"loading..."}</div>};

    html! {
        <BrowserRouter>
            <Suspense fallback={fallback}>
                <Content />
            </Suspense>
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
    let fallback = html! {<div>{"loading..."}</div>};

    html! {
        <Router history={history}>
            <Suspense fallback={fallback}>
                <Content />
            </Suspense>
        </Router>
    }
}

#[function_component]
pub fn Content() -> Html {
    let color_mode = use_reducer(|| ColorMode::Light);

    use_effect_once({
        let color_mode = color_mode.clone();

        move || {
            color_mode.clone().dispatch({
                let color_mode_with_storage =
                    LocalStorage::get(COLOR_MODE_STATE_KEY).unwrap_or_else(|_| ColorMode::Light);
                match color_mode_with_storage {
                    ColorMode::Light => ColorModeActions::SetLight,
                    ColorMode::Dark => ColorModeActions::SetDark,
                }
            });
            || ()
        }
    });

    use_effect_with_deps(
        move |state| {
            LocalStorage::set(COLOR_MODE_STATE_KEY, *state.clone()).expect("failed to set");
            || ()
        },
        color_mode.clone(),
    );

    html!(
        <div class={&color_mode.to_string()}>
            <div class={classes!("w-screen", "h-screen",
            "bg-light-background", "dark:bg-dark-background")}>
                <Header color_mode={color_mode.clone()} />
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
