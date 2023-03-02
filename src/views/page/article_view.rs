use yew::prelude::*;

use crate::app::{controllers::article_controller, models::article::Article};

#[derive(PartialEq, Properties)]
pub(crate) struct ArticleViewProps {
    pub(crate) article_id: String,
    pub(crate) is_preview: Option<bool>,
}

#[function_component]
pub(crate) fn ArticleView(props: &ArticleViewProps) -> HtmlResult {
    let ArticleViewProps {
        article_id,
        is_preview,
    } = props;

    let article = {
        let article_id = article_id.clone();
        let is_preview = is_preview.clone().unwrap_or_else(|| false);
        use_prepared_state!(
            async move |_| -> Option<Article> {
                let article = if is_preview {
                    article_controller::fetch_article_with_preview(&article_id).await
                } else {
                    article_controller::fetch_article_with_public(&article_id).await
                };
                match article {
                    Ok(article) => Some(article),
                    Err(e) => {
                        log::warn!("{:#?} article_id: {}", e, article_id);
                        None
                    }
                }
            },
            ()
        )?
    }
    .unwrap();

    let (title, body) = if let Some(article) = article.as_ref() {
        let body = article.body.clone().unwrap_or_default();
        (
            article.title.as_str(),
            Html::from_html_unchecked(body.into()),
        )
    } else {
        (
            "",
            html! {

                <div>{"Not Found"}</div>
            },
        )
    };

    Ok(html! {
        <div class="container mx-auto max-w-4xl px-5 pb-5
        flex flex-col items-center
        bg-light-content-background text-light-text
        dark:bg-dark-content-background dark:text-dark-text">
            <h2 class="m-5 p-5 font-bold text-2xl md:text-4xl lg:text-4xl">{title}</h2>
            <div class="markdown-body">
                {
                    body
                }
            </div>
        </div>
    })
}
