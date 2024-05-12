#![allow(dead_code)]
#![deny(unused_must_use)]
#![feature(try_blocks)]
#![feature(trait_alias)]
#![feature(trait_upcasting)]
#![feature(ptr_metadata)]

use std::pin::Pin;

use futures::Stream;

pub use global::Global;
use octant_gui_core::UpMessageList;
pub use runtime::Runtime;

pub mod builder;
pub mod document;
pub mod element;
pub mod event_loop;
pub mod global;
pub mod handle;
pub mod html_element;
pub mod html_form_element;
pub mod html_input_element;
pub mod js_value;
pub mod node;
pub mod object;
pub mod runtime;
pub mod text;
pub mod window;

mod any_value;
mod credential;
pub mod credential_creation_options;
mod credential_request_options;
pub mod credentials_container;
pub mod navigator;
mod promise;
mod request;
mod request_init;
mod response;
pub mod sink;
//
pub type UpMessageStream =
     Pin<Box<dyn Send + Sync + Stream<Item = anyhow::Result<Option<UpMessageList>>>>>;
// pub type Handle = Arc<dyn handle::Trait>;
// pub type JsValue = Arc<dyn js_value::Trait>;
// pub type Window = Arc<dyn window::Trait>;
//
// pub type Document = Arc<dyn document::Trait>;
//
// pub type Navigator = Arc<dyn navigator::Trait>;
//
// pub type CredentialsContainer = Arc<dyn credentials_container::Trait>;
// pub type CredentialCreationOptions = Arc<dyn credential_creation_options::Trait>;
// pub type CredentialRequestOptions = Arc<dyn credential_request_options::Trait>;
// pub type RequestInit = Arc<dyn request_init::Trait>;
// pub type Request = Arc<dyn request::Trait>;
//
// pub type HtmlElement = Arc<dyn html_element::Trait>;
// pub type HtmlFormElement = Arc<dyn html_form_element::Trait>;
// pub type HtmlInputElement = Arc<dyn html_input_element::Trait>;
//
// pub type Element = Arc<dyn element::Trait>;
//
// pub type Node = Arc<dyn node::Trait>;
//
// pub type Text = Arc<dyn text::Trait>;
// pub type AnyValue = Arc<dyn any_value::Trait>;
//
// pub type Promise = Arc<dyn promise::Trait>;
//
// pub type Credential = Arc<dyn credential::Trait>;
// pub type Response = Arc<dyn response::Trait>;
