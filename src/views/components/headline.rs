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
        <Link<Route> classes={classes!("container", "flex", "mx-auto", "px-5", "py-3", "max-w-lg",
        "bg-light-content-background", "text-light-text",
        "dark:bg-dark-content-background", "dark:text-dark-text")} to={Route::Home}>
            <figure class="flex p-3">
                <img src={thumbnail_url} alt="thumbnail" height="64" width="64" decoding="auto" loading="lazy" />
            </figure>
            <div class="flex flex-col items-start px-5 py-3">
                <h2 class="flex text-xl">{title}</h2>
                <div class="flex items-center">
                    <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-category" width="20" height="20" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                        <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                        <path d="M4 4h6v6h-6z"></path>
                        <path d="M14 4h6v6h-6z"></path>
                        <path d="M4 14h6v6h-6z"></path>
                        <path d="M17 17m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0"></path>
                    </svg>
                    <p>{categorie}</p>
                </div>
                <time datetime={first_published_at} class="flex">{first_published_at}</time>
            </div>
        </Link<Route>>
    }
}
