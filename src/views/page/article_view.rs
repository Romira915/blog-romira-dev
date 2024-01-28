use yew::prelude::*;

use crate::app::{controllers::article_controller, models::cms_article::CMSArticle};

#[derive(PartialEq, Properties, Clone)]
pub(crate) struct ArticleViewProps {
    pub(crate) article_id: String,
    #[prop_or_default]
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
        use_prepared_state!((), async move |_| -> Option<CMSArticle> {
            let article = if is_preview {
                article_controller::fetch_preview_article(&article_id).await
            } else {
                article_controller::fetch_public_article(&article_id).await
            };
            match article {
                Ok(article) => Some(article),
                Err(e) => {
                    log::warn!("{:#} article_id: {}", e, article_id);
                    None
                }
            }
        })?
    }
    .unwrap();

    let (title, body) = if let Some(article) = article.as_ref() {
        let body = article.body.clone().unwrap_or_default();
        (
            article.title.as_str(),
            if cfg!(target_arch = "wasm32") {
                html! {
                    <h1>{""}</h1>
                }
            } else {
                Html::from_html_unchecked(body.into())
            },
        )
    } else {
        (
            "",
            html! {
                <h1>{"Not Found"}</h1>
            },
        )
    };

    Ok(html! {
        <div class="container mx-auto max-w-4xl px-3 pb-5
        flex flex-col items-center
        bg-light-content-background text-light-text
        dark:bg-dark-content-background dark:text-dark-text">
            <h2 class="m-5 p-5 font-bold w-full text-center
            bg-light-content-inner-background dark:bg-dark-content-inner-background
            text-2xl md:text-4xl lg:text-4xl">{title}</h2>
            <div class="markdown-body container p-5">
                {
                    body
                }
            </div>
        </div>
    })
}
