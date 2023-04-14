use std::rc::Rc;

use yew::prelude::*;

use crate::{
    app::models::author::Author,
    views::components::external_service_icon::{ExternalService, ExternalServiceIcon},
};

#[derive(PartialEq, Properties)]
pub(crate) struct AuthorViewProps {
    pub(crate) author: Rc<Author>,
}

#[function_component]
pub(crate) fn AuthorView(props: &AuthorViewProps) -> Html {
    let AuthorViewProps { author } = props;
    let icon_src = author
        .profile_image_id
        .as_ref()
        .map(|img| img.src.clone())
        .unwrap_or_default();
    let biography = author.biography.as_deref().unwrap_or_default();

    html! {
        <div class="flex flex-col items-center my-3 p-3 space-y-2 min-w-[300px] max-w-[300px] w-fit
        rounded border-2 border-light-content-border dark:border-dark-content-border
        text-light-text dark:text-dark-text">
            <figure>
                <img src={icon_src} alt="Romira" width="100" height="100" class="rounded-full" />
            </figure>
            <h3 class="font-bold">{&author.full_name}</h3>
            <p>{biography}</p>
            <div class="flex space-x-4">
                <ExternalServiceIcon href={"https://twitter.com/Romira915".to_string()} service={ExternalService::Twitter} />
                <ExternalServiceIcon href={"https://github.com/Romira915".to_string()} service={ExternalService::GitHub} />
            </div>
        </div>
    }
}
