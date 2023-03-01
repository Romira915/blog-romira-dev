use std::rc::Rc;

use crate::{
    app::models::article::Article, views::components::article_headline_view::ArticleHeadlineView,
};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub(crate) struct ArticlesViewProps {
    pub(crate) articles: Rc<Vec<Article>>,
}

#[function_component]
pub(crate) fn ArticlesView(props: &ArticlesViewProps) -> Html {
    let ArticlesViewProps { articles } = props;

    html! {
        <div>
        {
            articles.iter().map(|article| {
                html! {
                    <ArticleHeadlineView article={Rc::new(article.clone())} />
                }
            }).collect::<Html>()
        }
        </div>
    }
}
