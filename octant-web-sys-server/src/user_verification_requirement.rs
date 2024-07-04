use marshal::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum UserVerificationRequirement {
    Required,
    Preferred,
    Discouraged,
}
