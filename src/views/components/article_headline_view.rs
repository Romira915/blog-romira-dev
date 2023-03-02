use std::rc::Rc;

use crate::{app::models::article::Article, routes::Route, settings::CONFIG};
use chrono::FixedOffset;
use yew::{classes, function_component, html, Html, Properties};
use yew_router::prelude::Link;

const HOUR: i32 = 3600;
/** JST (UCT+09:00) */
const JST_TZ: i32 = 9;

#[derive(PartialEq, Properties)]
pub(crate) struct ArticleHeadlineViewProps {
    pub(crate) article: Rc<Article>,
}

#[function_component]
pub(crate) fn ArticleHeadlineView(props: &ArticleHeadlineViewProps) -> Html {
    let ArticleHeadlineViewProps { article } = props;

    let id = article.id.as_str();
    let title = article.title.as_str();
    let thumbnail_url = article.cover_image.as_ref().map_or(
        "https://blog-romira.imgix.net/df46b2ca-5b5c-4847-ba48-650cbae1ae23/no-image.png"
            .to_string(),
        |img| img.src.to_string(),
    );
    let first_published_at = article
        .sys
        .raw
        .first_published_at
        .unwrap_or_default()
        .with_timezone(&FixedOffset::east_opt(JST_TZ * HOUR).unwrap())
        .date_naive()
        .format("%Y年%m月%d日")
        .to_string();
    let categorie = "develop";

    html! {
        <a href={format!("articles/{}", id)} class={classes!("container", "flex", "mx-auto", "m-7", "px-5", "py-3", "max-w-4xl",
        "bg-light-content-background", "text-light-text",
        "dark:bg-dark-content-background", "dark:text-dark-text")}>
            <figure class="flex-none m-3">
                <img class="object-contain h-auto w-16 min-w-16 max-w-16" src={thumbnail_url} alt="thumbnail" width="64" decoding="auto" loading="lazy" />
            </figure>
            <div class="flex flex-col items-start px-5 py-3">
                <h2 class="flex font-bold text-base md:text-lg lg:text-xl">{title}</h2>
                <div class="flex items-center mb-auto">
                    <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-category" width="20" height="20" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                        <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                        <path d="M4 4h6v6h-6z"></path>
                        <path d="M14 4h6v6h-6z"></path>
                        <path d="M4 14h6v6h-6z"></path>
                        <path d="M17 17m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0"></path>
                    </svg>
                    <p>{categorie}</p>
                </div>
                <time datetime={first_published_at.clone()} class="flex">{first_published_at}</time>
            </div>
        </a>
    }
}
