use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::article::Image;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Author {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_sys")]
    sys: Value,
    full_name: String,
    profile_image: Option<Image>,
    biography: Option<String>,
}
