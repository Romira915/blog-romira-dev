use std::rc::Rc;

use crate::{
    app::models::{
        article::{Article, ArticleType},
        cms_article::CMSArticle,
    },
    const_value::{HOUR, JST_TZ},
    routes::Route,
    settings::CONFIG,
};
use chrono::FixedOffset;
use yew::{classes, function_component, html, Html, Properties};
use yew_router::prelude::Link;

#[derive(PartialEq, Properties)]
pub(crate) struct ArticleHeadlineViewProps {
    pub(crate) article: Rc<Article>,
}

#[function_component]
pub(crate) fn ArticleHeadlineView(props: &ArticleHeadlineViewProps) -> Html {
    let ArticleHeadlineViewProps { article } = props;

    let first_published_at = article
        .first_published_at
        .as_ref()
        .map_or("".to_string(), |d| {
            d.date_naive().format("%Y年%m月%d日").to_string()
        });
    let (target, rel) = if article.r#type == ArticleType::CMS {
        ("_self", "")
    } else {
        ("_blank", "noopener noreferrer")
    };

    html! {
        <a href={article.href.to_string()} target={target} rel={rel} class={classes!("container", "flex", "mx-auto", "m-7", "px-5", "py-3", "max-w-4xl",
        "border-4", "rounded",
        "bg-light-content-background", "text-light-text", "border-light-content-border",
        "dark:bg-dark-content-background", "dark:text-dark-text", "dark:border-dark-content-border")}>
            <figure class="flex-none m-3">
                <img class="object-contain h-auto w-16 min-w-16 max-w-16" src={article.thumbnail_url.to_string()} alt="thumbnail" width="64" height="64" decoding="auto" loading="lazy" />
            </figure>
            <div class="flex flex-col items-start px-5 py-3">
                <h2 class="flex font-bold text-base md:text-lg lg:text-xl">{article.title.to_string()}</h2>
                <div class="flex items-center mb-auto">
                    <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-category" width="20" height="20" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                        <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                        <path d="M4 4h6v6h-6z"></path>
                        <path d="M14 4h6v6h-6z"></path>
                        <path d="M4 14h6v6h-6z"></path>
                        <path d="M17 17m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0"></path>
                    </svg>
                    <p>{article.categorie.clone().unwrap_or_default()}</p>
                </div>
                <time datetime={first_published_at.to_string()} class="flex">{first_published_at.to_string()}</time>
            </div>
        </a>
    }
}
