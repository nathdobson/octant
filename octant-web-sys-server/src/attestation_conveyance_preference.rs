use marshal::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum AttestationConveyancePreference {
    None,
    Indirect,
    Direct,
}
