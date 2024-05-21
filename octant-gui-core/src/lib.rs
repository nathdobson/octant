#![deny(unused_must_use)]
#![feature(macro_metavar_expr)]
#![feature(unsize)]

use std::{
    any::Any,
    fmt::{Debug, Formatter},
    hash::Hash,
};

use serde::{Deserialize, Serialize};

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
pub use handle::*;
pub use html_form_element::*;
pub use html_input_element::*;
pub use navigator::*;
pub use node::*;
pub use object::*;
use octant_object::class::Class;
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

#[doc(hidden)]
pub mod reexports {
    pub use anyhow;
    pub use catalog;
    pub use paste;
    pub use serde;

    pub use octant_object;
    pub use octant_serde;
}

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
mod handle;
mod public_key_credential_request_options;
mod request;
mod request_init;
mod response;

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
    Promise(TypedHandle<PromiseTag>, PromiseUpMessage),
    Credential(TypedHandle<CredentialTag>, CredentialUpMessage),
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

pub trait NewUpMessage: SerializeDyn + Debug + Send + Sync + Any {}
define_serde_trait!(NewUpMessage);

#[macro_export]
macro_rules! define_sys_class {
    {
        class $class:ident;
        extends $parent:path;
        wasm $wasm:path;
        $(new_client $new_client_dummy:tt;)?
        $(new_server $new_server_dummy:tt;)?
        $(client_field $client_field:ident : $client_field_type:ty ;)*
        $(server_field $server_field:ident : $server_field_type:ty ;)*
    } => {
        $crate::reexports::paste::paste! {
            #[cfg(side = "client")]
            $crate::reexports::octant_object::define_class! {
                #[derive(Debug)]
                pub class $class extends $parent implements ::std::fmt::Debug{
                    [< $class:snake >]: $wasm,
                    $($client_field : $client_field_type, )*
                }
            }

            #[cfg(any($(all() ${ignore($new_client_dummy)} )?))]
            #[cfg(side = "client")]
            impl $crate::FromHandle for dyn $class {
                type Builder = $wasm;
                fn from_handle(handle: $crate::NewTypedHandle<Self>, [< $class:snake >]: Self::Builder) -> [< $class Value >]  {
                    [< $class Value >] {
                        parent: <dyn $parent as $crate::FromHandle>::from_handle(handle.unsize(), [< $class:snake >].clone().into()),
                        [< $class:snake >],
                        $($client_field : ::std::default::Default::default(), )*
                    }
                }
            }

            #[cfg(side="client")]
            impl [< $class Value >] {
                pub fn native(&self) -> &$wasm{
                    &self.[< $class:snake >]
                }
            }

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
                    $($server_field : $server_field_type, )*
                }
            }
            #[cfg(any($(all() ${ignore($new_server_dummy)} )?))]
            #[cfg(side = "server")]
            impl [< $class Value >] {
                pub fn new(handle: ::octant_gui::handle::HandleValue) -> Self {
                    [< $class Value >] {
                        parent: <dyn $parent as $crate::reexports::octant_object::class::Class>::Value::new(handle),
                        $($server_field : ::std::default::Default::default(), )*
                    }
                }
            }

            impl $crate::reexports::serde::Serialize for dyn $class {
                fn serialize<S>(&self, s: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: $crate::reexports::serde::Serializer,
                {
                    #[cfg(side = "server")]
                    return self.handle().handle().serialize(s);
                    #[cfg(side = "client")]
                    return self.raw_handle().serialize(s);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_sys_rpc {
    {
        fn $name:ident($runtime:ident $(, $input_name:ident: $input:ty)*) -> ( $( $output:ident, )* ) { $($imp:tt)* }
    } => {
        $crate::reexports::paste::paste!{
            #[cfg(side = "server")]
            fn $name(
                runtime: &::std::sync::Arc<::octant_gui::Runtime>
                $(, $input_name: $input)*
            ) -> (
                $(
                    ::std::sync::Arc<dyn $output>
                ),*
            ) {
                $(
                    let [< output_ ${index()} >] = ::std::sync::Arc::new(<dyn $output as $crate::reexports::octant_object::class::Class>::Value::new(runtime.add_uninit()));
                )*
                runtime.new_send(Box::<[< $name:camel Request >]>::new([< $name:camel Request >] {
                    $($input_name,)*
                    $(
                        ${ignore($output)}
                        [< output_ ${index()} >]: $crate::NewTypedHandle::new(
                            ::octant_gui::runtime::HasTypedHandle::typed_handle(&*[< output_ ${index()} >]).0
                        ),
                    )*
                }));
                ( $(
                    ${ignore($output)}
                    [< output_ ${index()} >]
                ),* )
            }

            #[derive($crate::reexports::serde::Serialize,Debug)]
            pub struct [< $name:camel Request >] {
                $($input_name: $input,)*
                $(
                    [< output_ ${index()} >]: $crate::NewTypedHandle<dyn $output>,
                )*
            }

            impl<'de> $crate::reexports::octant_serde::DeserializeWith<'de> for [< $name:camel Request >] {
                fn deserialize_with<D:$crate::reexports::serde::Deserializer<'de>>(ctx: &$crate::reexports::octant_serde::TypeMap, d:D)->Result<Self,D::Error>{
                    todo!();
                }
            }

            #[cfg(side="client")]
            $crate::reexports::octant_serde::define_serde_impl!([< $name:camel Request >]: octant_gui_client::ClientDownMessage);
            #[cfg(side="client")]
            impl octant_gui_client::ClientDownMessage for [< $name:camel Request >] {}

            #[cfg(side="server")]
            $crate::reexports::octant_serde::define_serde_impl!([< $name:camel Request >]: octant_gui::ServerDownMessage);
            #[cfg(side="server")]
            impl octant_gui::ServerDownMessage for [< $name:camel Request >] {}

            #[cfg(side = "client")]
            #[$crate::reexports::catalog::register(octant_gui_client::DOWN_MESSAGE_HANDLER_REGISTRY)]
            fn [<handle_ $name>]() -> octant_gui_client::DownMessageHandler<[< $name:camel Request >]> {
                |runtime: &::std::sync::Arc<::octant_gui_client::Runtime>, req: [< $name:camel Request >]| {
                    let runtime = runtime.clone();
                    let result = [<impl_ $name>](&runtime $(, req.$input_name)*)?;
                    $(
                        runtime.add_new(req.[< output_ ${index()} >], ::std::sync::Arc::new(<dyn $output as $crate::FromHandle>::from_handle(req.[< output_ ${index()} >], result.${index()})));
                    )*
                    Ok(())
                }
            }

            #[cfg(side="client")]
            fn [<impl_ $name>](
                $runtime: &::std::sync::Arc<octant_gui_client::Runtime>,
                $($input_name: $input,)*
            ) -> $crate::reexports::anyhow::Result<
                ($(
                    <dyn $output as $crate::FromHandle>::Builder,
                )*)
            >{
                $($imp)*
            }
        }
    };
}

pub trait FromHandle: Class {
    type Builder;
    fn from_handle(handle: NewTypedHandle<Self>, builder: Self::Builder) -> Self::Value;
}
