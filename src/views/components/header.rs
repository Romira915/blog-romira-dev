


use yew::{
    function_component, html, Callback, Html, Properties,
    UseReducerHandle,
};



use crate::{
    state::{ColorMode, ColorModeActions},
};

#[derive(PartialEq, Properties)]
pub struct HeaderProps {
    pub(crate) color_mode: UseReducerHandle<ColorMode>,
}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    let HeaderProps { color_mode } = props;
    let toggle_color_mode = {
        let color_mode = color_mode.clone();
        Callback::from(move |_| {
            log::debug!("on click color mode toggle button.");
            color_mode.dispatch(ColorModeActions::Toggle)
        })
    };

    html! {
        <header class="bg-light-primary text-light-text
                     dark:bg-dark-primary dark:text-dark-text">
            <div class="container mx-auto
                        flex flex-wrap flex-row items-center justify-between px-1 py-5">
                <h1 class="grow font-extrabold italic text-xl md:text-2xl lg:text-4xl">
                    <a href="/">
                        {"romira's develop blog"}
                    </a>
                </h1>
                <button onclick={toggle_color_mode}>
                    {
                        if **color_mode == ColorMode::Light {
                            html! {
                                <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-brightness-up" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                    <path d="M12 12m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0"></path>
                                    <path d="M12 5l0 -2"></path>
                                    <path d="M17 7l1.4 -1.4"></path>
                                    <path d="M19 12l2 0"></path>
                                    <path d="M17 17l1.4 1.4"></path>
                                    <path d="M12 19l0 2"></path>
                                    <path d="M7 17l-1.4 1.4"></path>
                                    <path d="M6 12l-2 0"></path>
                                    <path d="M7 7l-1.4 -1.4"></path>
                                </svg>
                            }
                        } else {
                            html! {
                                <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-moon" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                    <path d="M12 3c.132 0 .263 0 .393 0a7.5 7.5 0 0 0 7.92 12.446a9 9 0 1 1 -8.313 -12.454z"></path>
                                </svg>
                            }
                        }
                    }
                </button>
            </div>
        </header>
    }
}
