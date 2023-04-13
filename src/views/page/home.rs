use std::rc::Rc;

use yew::prelude::*;

use crate::app::controllers::article_controller;
use crate::app::models::article::Articles;
use crate::views::components::articles_view::ArticlesView;
use crate::views::components::author_view::AuthorView;

#[derive(PartialEq, Properties)]
pub(crate) struct HomeProps {}

#[function_component]
pub(crate) fn Home(props: &HomeProps) -> HtmlResult {
    let HomeProps {} = props;

    let articles = use_prepared_state!(
        async move |_| -> Articles { article_controller::fetch_articles().await.unwrap() },
        ()
    )?
    .unwrap();

    Ok(html! {
        <div>
            <div class="container mx-auto max-w-4xl
            ">
                <ArticlesView articles={Rc::new(articles.items.clone())} />
                <AuthorView />
            </div>
        </div>
    })
}
