#![deny(unused_must_use)]
#![feature(macro_metavar_expr)]

use std::{
    any::Any,
    fmt::{Debug, Formatter},
    hash::Hash,
    marker::PhantomData,
};

use serde::{Deserialize, Serialize};

#[doc(hidden)]
pub mod reexports {
    pub use octant_object;
    pub use paste;
    pub use serde;
}

pub use allow_credentials::*;
pub use allow_credentials_type::*;
pub use any_value::*;
pub use attestation_conveyance_preference::*;
pub use authentication_extensions_client_inputs::*;
pub use authentication_extensions_client_outputs::*;
pub use authenticator_assertion_response::*;
pub use authenticator_attachment::*;
pub use authenticator_attestation_response::*;
pub use authenticator_response::*;
pub use authenticator_selection_criteria::*;
pub use authenticator_transport::*;
pub use credential::*;
pub use credential_creation_options::*;
pub use credential_data::*;
pub use credential_request_options::*;
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
use octant_serde::{define_serde_trait, SerializeDyn};
pub use promise::*;
pub use pub_key_cred_params::*;
pub use public_key_credential::*;
pub use public_key_credential_creation_options::*;
pub use public_key_credential_request_options::*;
pub use public_key_credential_rp_entity::*;
pub use public_key_credential_user_entity::*;
pub use request::*;
pub use request_init::*;
pub use response::*;
pub use user_verification_requirement::*;
pub use value::*;
pub use window::*;

mod attestation_conveyance_preference;

mod authentication_extensions_client_outputs;
mod authenticator_attachment;
mod authenticator_attestation_response;
mod authenticator_response;
mod authenticator_selection_criteria;
mod authenticator_transport;
mod credential_creation_options;
mod credential_data;
mod credentials_container;
mod document;
mod element;
mod error;
mod global;
mod html_form_element;
mod html_input_element;
mod navigator;
mod node;
mod object;
mod promise;
mod pub_key_cred_params;
mod public_key_credential;
mod public_key_credential_creation_options;
mod public_key_credential_rp_entity;
mod public_key_credential_user_entity;
mod user_verification_requirement;
mod value;
mod window;

mod allow_credentials;
mod allow_credentials_type;
mod any_value;
mod authentication_extensions_client_inputs;
mod authenticator_assertion_response;
mod credential;
mod credential_request_options;
mod public_key_credential_request_options;
mod request;
mod request_init;
mod response;

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
    CredentialRequestOptions(
        TypedHandle<CredentialRequestOptionsTag>,
        CredentialRequestOptionsMethod,
    ),
    AnyValue(TypedHandle<AnyValueTag>, AnyValueMethod),
    Credential(TypedHandle<CredentialTag>, CredentialMethod),
    Request(TypedHandle<RequestTag>, RequestMethod),
    RequestInit(TypedHandle<RequestInitTag>, RequestInitMethod),
    Response(TypedHandle<ResponseTag>, ResponseMethod),
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
    NewDownMessage(Box<dyn NewDownMessage>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UpMessage {
    VisitPage(String),
    HtmlFormElement(TypedHandle<HtmlFormElementTag>, HtmlFormElementUpMessage),
    HtmlInputElement(TypedHandle<HtmlInputElementTag>, HtmlInputElementUpMessage),
    Promise(TypedHandle<PromiseTag>, PromiseUpMessage),
    Credential(TypedHandle<CredentialTag>, CredentialUpMessage),
    NewUpMessage(Box<dyn NewUpMessage>),
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
            DownMessage::NewDownMessage(x) => Debug::fmt(x, f),
        }
    }
}

impl<T: TypeTag> Debug for TypedHandle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub trait NewUpMessage: SerializeDyn + Debug + Send + Sync + Any {}

define_serde_trait!(NewUpMessage);

pub trait NewDownMessage: SerializeDyn + Debug + Send + Sync + Any {}
define_serde_trait!(NewDownMessage);

#[macro_export]
macro_rules! define_sys_class {
    {
        class $class:ident;
        extends $parent:path;
        wasm $wasm:path;
        $(new_client $new_client_dummy:ident;)?
        $(new_server $new_server_dummy:tt;)?
    } => {
        $crate::reexports::paste::paste! {
            #[cfg(side = "client")]
            $crate::reexports::octant_object::define_class! {
                pub class $class extends $parent {
                    [< $class:snake >]: $wasm,
                }
            }
            $(
                ${ignore($new_client_dummy)}
                #[cfg(side = "client")]
                impl [< $class Value >] {
                    pub fn new(handle: $crate::HandleId, [< $class:snake >]: $wasm) -> Self {
                        [< $class Value >] {
                            parent: <dyn $parent as $crate::reexports::octant_object::class::Class>::Value::new(handle, [< $class:snake >].clone().into()),
                            [< $class:snake >],
                        }
                    }
                }
            )?

            #[derive($crate::reexports::serde::Serialize, $crate::reexports::serde::Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
            pub struct [< $class Tag >];

            impl $crate::TypeTag for [< $class Tag >] {}

            #[cfg(side="server")]
            impl octant_gui::runtime::HasTypedHandle for [< $class Value >]{
                type TypeTag = [< $class Tag >];
            }

            #[cfg(side="client")]
            impl octant_gui_client::HasLocalType for [< $class Tag >]{
                type Local = dyn $class;
            }

            #[cfg(side = "server")]
            $crate::reexports::octant_object::define_class! {
                #[derive(Debug)]
                pub class $class extends $parent {

                }
            }
            $(
                ${ignore($new_server_dummy)}
                #[cfg(side = "server")]
                impl [< $class Value >] {
                    pub fn new(handle: ::octant_gui::handle::HandleValue) -> Self {
                        [< $class Value >] {
                            parent: <dyn $parent as $crate::reexports::octant_object::class::Class>::Value::new(handle),
                        }
                    }
                }
            )?

        }
    };
}
