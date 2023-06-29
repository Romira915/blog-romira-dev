use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::fields::Sys;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Category {
    #[serde(rename = "_id")]
    pub(crate) id: String,
    #[serde(rename = "_sys")]
    pub(crate) sys: Sys,
    pub(crate) name: String,
    pub(crate) slug: String,
}
