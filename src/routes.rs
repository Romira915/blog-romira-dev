use yew_router::Routable;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/articles/:id")]
    Article { id: String },
    #[at("/page/:page")]
    Page { page: String },
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}
