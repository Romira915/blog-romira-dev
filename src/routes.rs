use std::str::FromStr;

use yew_router::Routable;

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
        let url = url::Url::parse(&format!("http://localhost{}", s))?;
        let Some(path_segments) = url.path_segments() else {
            return Ok(Self::NotFound);
        };
        let path_segments = path_segments.collect::<Vec<_>>();

        if let Some(first) = path_segments.get(0) {
            match *first {
                "articles" => {
                    return Ok(Self::Article {
                        id: path_segments
                            .get(1)
                            .map(|p| *p)
                            .unwrap_or_default()
                            .to_string(),
                    })
                }
                _ => return Ok(Self::NotFound),
            }
        }

        Ok(Self::NotFound)
    }
}
