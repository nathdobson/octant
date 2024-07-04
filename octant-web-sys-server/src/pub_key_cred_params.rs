use marshal::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PubKeyCredParams {
    #[marshal(rename = "type")]
    pub typ: String,
    pub alg: i32,
}
