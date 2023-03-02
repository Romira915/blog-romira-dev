use serde::{Deserialize, Serialize};


use super::fields::Sys;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Category {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_sys")]
    sys: Sys,
    name: String,
    slug: String,
}
