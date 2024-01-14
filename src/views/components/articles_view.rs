use std::rc::Rc;

use crate::{
    app::models::{
        article::{Article, Articles},
        cms_article::{CMSArticle, CMSArticles},
    },
    views::components::article_headline_view::ArticleHeadlineView,
};
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub(crate) struct ArticlesViewProps {
    pub(crate) articles: Rc<Articles>,
}

#[function_component]
pub(crate) fn ArticlesView(props: &ArticlesViewProps) -> Html {
    let ArticlesViewProps { articles } = props;

    html! {
        <div class="py-3 px-5 w-[100%]
        bg-light-content-background dark:bg-dark-content-background">
        {
            articles.items.iter().map(|article| {
                html! {
                    <ArticleHeadlineView article={Rc::new(article.clone())} />
                }
            }).collect::<Html>()
        }
        </div>
    }
}
