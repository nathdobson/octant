#![deny(unused_must_use)]

use std::{
    fmt::{Debug, Formatter},
    hash::Hash,
    marker::PhantomData,
};

use serde::{Deserialize, Serialize};

use crate::{
    document::{DocumentMethod, DocumentTag},
    element::{ElementMethod, ElementTag},
    global::GlobalMethod,
    html_form_element::{HtmlFormElementMethod, HtmlFormElementTag, HtmlFormElementUpMessage},
    html_input_element::{HtmlInputElementMethod, HtmlInputElementTag, HtmlInputElementUpMessage},
    navigator::{NavigatorMethod, NavigatorTag},
    node::{NodeMethod, NodeTag},
    window::{WindowMethod, WindowTag},
};
use crate::credential_creation_options::{CredentialCreationOptionsMethod, CredentialCreationOptionsTag};
use crate::credentials_container::{CredentialsContainerMethod, CredentialsContainerTag};

pub mod credential_creation_options;
pub mod credentials_container;
pub mod document;
pub mod element;
pub mod global;
pub mod html_form_element;
pub mod html_input_element;
pub mod navigator;
pub mod node;
pub mod object;
pub mod public_key_credential_creation_options;
pub mod public_key_credential_rp_entity;
pub mod public_key_credential_user_entity;
pub mod value;
pub mod window;
pub mod pub_key_cred_params;
pub mod authenticator_selection_criteria;
pub mod authenticator_attachment;
pub mod user_verification_requirement;
pub mod attestation_conveyance_preference;

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
    CredentialsContainer(TypedHandle<CredentialsContainerTag>, CredentialsContainerMethod),
    CredentialCreationOptionsMethod(TypedHandle<CredentialCreationOptionsTag>, CredentialCreationOptionsMethod),
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
