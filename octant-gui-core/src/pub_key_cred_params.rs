use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PubKeyCredParams {
    #[serde(rename = "type")]
    pub typ: String,
    pub alg: i64,
}
