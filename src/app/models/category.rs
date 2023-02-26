use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Category {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_sys")]
    sys: Value,
    name: String,
    slug: String,
}
