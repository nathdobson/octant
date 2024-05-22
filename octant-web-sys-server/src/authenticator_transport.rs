use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthenticatorTransport {
    Usb,
    Nfc,
    Ble,
    Internal,
    Hybrid,
    Test,
    Unknown,
}
