pub(crate) mod articles;
pub(crate) mod components;
pub(crate) mod page;

use std::{cell::RefCell, collections::HashMap, sync::Arc};

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
    const_value::COLOR_MODE_STATE_KEY,
    routes::Route,
    state::{ColorMode, ColorModeActions},
    test_data::use_sleep,
    views::components::header::Header,
};

use self::{
    components::{article_headline_view::ArticleHeadlineView, articles_view::ArticlesView},
    page::{article_view::ArticleView, home::Home},
};

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
            color_mode.dispatch({
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
            log::debug!("Set LocalStorage key {}", COLOR_MODE_STATE_KEY);
            LocalStorage::set(COLOR_MODE_STATE_KEY, *state.clone()).expect("failed to set");
            || ()
        },
        color_mode.clone(),
    );

    html!(
        <div class={&color_mode.to_string()}>
            {
                if *color_mode == ColorMode::Light {
                    html! {
                        <link rel="stylesheet"
                            href="https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.2.0/github-markdown-light.min.css" />
                    }
                } else {
                    html! {
                        <link rel="stylesheet"
                            href="https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.2.0/github-markdown-dark.min.css" />
                    }
                }
            }
            <div class={classes!("min-w-screen", "min-h-screen",
            "bg-light-background", "dark:bg-dark-background")}>
                <Header color_mode={color_mode.clone()} />
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </div>
    )
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Article { id } => {
            let fallback = html! {<div>{"Loading..."}</div>};
            html! {
                <Suspense {fallback}>
                    <ArticleView article_id={id} />
                </Suspense>
            }
        }
        Route::Preview { id } => {
            let fallback = html! {<div>{"Loading..."}</div>};
            html! {
                <Suspense {fallback}>
                    <ArticleView article_id={id} is_preview={true} />
                </Suspense>
            }
        }
        Route::Page { page } => {
            let fallback = html! {<div>{"Loading..."}</div>};
            html! {
                <Suspense {fallback}>
                    <Home />
                </Suspense>
            }
        }
        Route::Home => {
            let fallback = html! {<div>{"Loading..."}</div>};
            html! {
                <Suspense {fallback}>
                    <Home />
                </Suspense>
            }
        }
        Route::NotFound => {
            html! { <h1>{"NotFound"}</h1> }
        }
    }
}
