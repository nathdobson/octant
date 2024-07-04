use marshal::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKeyCredentialRpEntity {
    pub name: String,
    pub icon: Option<String>,
    pub id: Option<String>,
}
