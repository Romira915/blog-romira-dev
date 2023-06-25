use super::traits::ArticleTrait;

// #[derive(Clone, Default, PartialEq)]
pub(crate) struct Articles<A>
where
    A: ArticleTrait,
{
    pub(crate) articles: Vec<A>,
}
