use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AuthorViewProps {}

#[function_component]
pub fn AuthorView(props: &AuthorViewProps) -> Html {
    let AuthorViewProps {} = props;
    html! {
        <div>
            {"Author"}
        </div>
    }
}
