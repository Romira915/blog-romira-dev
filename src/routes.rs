use std::str::FromStr;

use yew_router::Routable;

use crate::settings::CONFIG;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/articles/:id")]
    Article { id: String },
    #[at("/preview/:id")]
    Preview { id: String },
    #[at("/page/:page")]
    Page { page: String },
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl FromStr for Route {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = url::Url::parse(&format!("{}{}", CONFIG.app_origin, s))?;
        let Some(path_segments) = url.path_segments() else {
            return Ok(Self::NotFound);
        };
        let path_segments = path_segments.collect::<Vec<_>>();

        if let (Some(&"articles"), Some(id)) = (path_segments.get(0), path_segments.get(1)) {
            return Ok(Self::Article { id: id.to_string() });
        }

        if let (Some(&"preview"), Some(id)) = (path_segments.get(0), path_segments.get(1)) {
            return Ok(Self::Preview { id: id.to_string() });
        }

        if let (Some(&"page"), Some(page)) = (path_segments.get(0), path_segments.get(1)) {
            return Ok(Self::Page {
                page: page.to_string(),
            });
        }

        if let Some(&"") = path_segments.get(0) {
            return Ok(Self::Home);
        }

        Ok(Self::NotFound)
    }
}
