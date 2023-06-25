use std::rc::Rc;

use yew::prelude::*;

use crate::app::controllers::{article_controller, author_controller};
use crate::app::models::author::Author;
use crate::app::models::cms_article::CMSArticles;
use crate::const_value::ROMIRA_CONTENT_ID;
use crate::views::components::articles_view::ArticlesView;
use crate::views::components::author_view::AuthorView;

#[derive(PartialEq, Properties)]
pub(crate) struct HomeProps {}

#[function_component]
pub(crate) fn Home(props: &HomeProps) -> HtmlResult {
    let HomeProps {} = props;

    let articles = use_prepared_state!(
        async move |_| -> CMSArticles { article_controller::fetch_articles().await.unwrap() },
        ()
    )?
    .unwrap();
    let author = use_prepared_state!(
        async move |_| -> Author {
            author_controller::fetch_author(ROMIRA_CONTENT_ID)
                .await
                .unwrap()
        },
        ()
    )?
    .unwrap();

    Ok(html! {
        <div>
            <div class="container flex flex-col items-center mx-auto max-w-4xl
            ">
                <ArticlesView articles={Rc::clone(&articles)} />
                <AuthorView author={Rc::clone(&author)} />
            </div>
        </div>
    })
}
