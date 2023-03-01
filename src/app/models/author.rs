use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::fields::Sys;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Author {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_sys")]
    sys: Sys,
    full_name: String,
    profile_image: Option<String>,
    biography: Option<String>,
}
