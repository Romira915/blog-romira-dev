use crate::{routes::Route, settings::CONFIG};
use yew::{classes, function_component, html, Html, Properties};
use yew_router::prelude::Link;

#[derive(PartialEq, Properties)]
pub struct HeadlineProps {}

#[function_component]
pub fn Headline(props: &HeadlineProps) -> Html {
    let HeadlineProps {} = props;
    let thumbnail_url = "https://blog-romira.imgix.net/df1b4b60-b983-4126-8f63-8a0cddf85254/prtimes-hackathon2023.drawio.png";
    let title = "Rustでblogサイトを作った";
    let first_published_at = "2023-02-28T12:59:46.459Z";
    let categorie = "develop";

    html! {
        <Link<Route> classes={classes!("container", "flex",
        "bg-light-content-background", "text-light-text",
        "dark:bg-dark-content-background", "dark:text-dark-text")} to={Route::Home}>
            <figure class="flex float-left">
                <img src={thumbnail_url} alt="thumbnail" height="64" width="64" decoding="auto" loading="lazy" />
            </figure>
            <div class="flex flex-col">
                <h2 class="flex">{title}</h2>
                <time datetime={first_published_at} class="flex">{first_published_at}</time>
            </div>
        </Link<Route>>
    }
}
