use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let hello = "hello";
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <h2>{ hello }</h2>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}

