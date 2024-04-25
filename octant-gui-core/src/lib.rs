#![deny(unused_must_use)]

use std::{
    fmt::{Debug, Formatter},
    hash::Hash,
    marker::PhantomData,
};

use serde::{Deserialize, Serialize};

pub use attestation_conveyance_preference::*;
pub use authentication_extensions_client_outputs::*;
pub use authenticator_attachment::*;
pub use authenticator_attestation_response::*;
pub use authenticator_response::*;
pub use authenticator_selection_criteria::*;
pub use authenticator_transport::*;
pub use credential::*;
pub use credential_creation_options::*;
pub use credential_promise::*;
pub use credentials_container::*;
pub use document::*;
pub use element::*;
pub use error::*;
pub use global::*;
pub use html_form_element::*;
pub use html_input_element::*;
pub use navigator::*;
pub use node::*;
pub use object::*;
pub use promise::*;
pub use pub_key_cred_params::*;
pub use public_key_credential::*;
pub use public_key_credential_creation_options::*;
pub use public_key_credential_rp_entity::*;
pub use public_key_credential_user_entity::*;
pub use registration_extensions_client_outputs::*;
pub use user_verification_requirement::*;
pub use value::*;
pub use window::*;

mod attestation_conveyance_preference;

mod authenticator_attachment;
mod authenticator_selection_criteria;
mod credential_creation_options;
mod credentials_container;
mod document;
mod element;
mod global;
mod html_form_element;
mod html_input_element;
mod navigator;
mod node;
mod object;
mod promise;
mod pub_key_cred_params;
mod public_key_credential_creation_options;
mod public_key_credential_rp_entity;
mod public_key_credential_user_entity;
mod user_verification_requirement;
mod value;
mod window;
mod credential;
mod public_key_credential;
mod credential_promise;
mod error;
mod authenticator_response;
mod authentication_extensions_client_outputs;
mod authenticator_attestation_response;
mod authenticator_transport;

mod registration_extensions_client_outputs;

#[derive(Serialize, Deserialize, Debug)]
pub struct DownMessageList {
    pub commands: Vec<DownMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Method {
    Log,
    Global(GlobalMethod),
    Window(TypedHandle<WindowTag>, WindowMethod),
    Navigator(TypedHandle<NavigatorTag>, NavigatorMethod),
    Document(TypedHandle<DocumentTag>, DocumentMethod),
    Element(TypedHandle<ElementTag>, ElementMethod),
    Node(TypedHandle<NodeTag>, NodeMethod),
    HtmlFormElement(TypedHandle<HtmlFormElementTag>, HtmlFormElementMethod),
    HtmlInputElement(TypedHandle<HtmlInputElementTag>, HtmlInputElementMethod),
    CredentialsContainer(
        TypedHandle<CredentialsContainerTag>,
        CredentialsContainerMethod,
    ),
    CredentialCreationOptions(
        TypedHandle<CredentialCreationOptionsTag>,
        CredentialCreationOptionsMethod,
    ),
    Promise(TypedHandle<PromiseTag>, PromiseMethod),
    CredentialPromise(TypedHandle<CredentialPromiseTag>, CredentialPromiseMethod),
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct HandleId(pub usize);

pub trait TypeTag:
    'static
    + Serialize
    + for<'de> Deserialize<'de>
    + Copy
    + Clone
    + Eq
    + Ord
    + PartialEq
    + PartialOrd
    + Hash
{
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct TypedHandle<T: TypeTag>(pub HandleId, pub PhantomData<T>);

#[derive(Serialize, Deserialize)]
pub enum DownMessage {
    Invoke { assign: HandleId, method: Method },
    Delete(HandleId),
    Fail(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UpMessage {
    VisitPage(String),
    HtmlFormElement(TypedHandle<HtmlFormElementTag>, HtmlFormElementUpMessage),
    HtmlInputElement(TypedHandle<HtmlInputElementTag>, HtmlInputElementUpMessage),
    CredentialPromise(
        TypedHandle<CredentialPromiseTag>,
        CredentialPromiseUpMessage,
    ),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpMessageList {
    pub commands: Vec<UpMessage>,
}

impl Debug for HandleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}

impl Debug for DownMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DownMessage::Invoke { assign, method } => {
                write!(f, "{:?} := ", assign)?;
                write!(f, "{:?}", method)?;
                Ok(())
            }
            DownMessage::Delete(handle) => {
                write!(f, "delete {:?}", handle)?;
                Ok(())
            }
            DownMessage::Fail(_) => {
                write!(f, "fail")
            }
        }
    }
}

impl<T: TypeTag> Debug for TypedHandle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
