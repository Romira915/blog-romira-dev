use std::rc::Rc;

use yew::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ExternalService {
    Twitter,
    GitHub,
}

#[derive(PartialEq, Properties)]
pub(crate) struct ExternalServiceIconProps {
    pub(crate) href: String,
    pub(crate) service: ExternalService,
}

#[function_component]
pub(crate) fn ExternalServiceIcon(props: &ExternalServiceIconProps) -> Html {
    let ExternalServiceIconProps { href, service } = props;

    let common_a_class =
        "relative inline-block rounded-[50%] border w-[50px] h-[50px] text-3xl duration-500";
    let common_span_class =
        "fa-brands absolute top-[50%] left-[50%] -translate-x-[50%] -translate-y-[50%]";

    let (a_class, span_class) = match service {
        ExternalService::Twitter => ("border-twitter-brand text-twitter-brand hover:text-white 
            bg-twitter-light-bg dark:bg-twitter-dark-bg hover:bg-twitter-brand dark:hover:bg-twitter-brand", "fa-twitter")
        ,ExternalService::GitHub => ("border-github-brand 
            text-github-brand dark:text-white
            hover:text-white dark:hover:text-github-brand
            bg-github-white dark:bg-github-brand
            hover:bg-github-brand dark:hover:bg-white", "fa-github")
    };

    html! {
        <a href={href.clone()} target="_blank" rel="noopener noreferrer"
            class={classes!(common_a_class, a_class)}>
            <span class={classes!(common_span_class, span_class)}></span>
        </a>
    }
}
