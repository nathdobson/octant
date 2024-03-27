#![deny(unused_must_use)]
#![feature(try_blocks)]
#![feature(trait_alias)]
#![feature(trait_upcasting)]
#![feature(ptr_metadata)]

use std::pin::Pin;
use std::sync::Arc;

use futures::sink::Sink;
use futures::Stream;

pub use global::Global;
use octant_gui_core::{DownMessageList, UpMessageList};
pub use runtime::Runtime;

pub mod document;
pub mod element;
pub mod global;
pub mod handle;
pub mod html_element;
pub mod html_form_element;
pub mod js_value;
pub mod node;
pub mod object;
pub mod runtime;
pub mod text;
pub mod window;
pub mod html_input_element;
pub mod event_loop;
pub mod builder;

pub type DownMessageSink = Pin<Box<dyn Send + Sync + Sink<DownMessageList, Error=anyhow::Error>>>;
pub type UpMessageStream = Pin<Box<dyn Send + Sync + Stream<Item=anyhow::Result<Option<UpMessageList>>>>>;
pub type Handle = Arc<dyn handle::Trait>;
pub type JsValue = Arc<dyn js_value::Trait>;
pub type Window = Arc<dyn window::Trait>;

pub type Document = Arc<dyn document::Trait>;

pub type HtmlElement = Arc<dyn html_element::Trait>;
pub type HtmlFormElement = Arc<dyn html_form_element::Trait>;
pub type HtmlInputElement = Arc<dyn html_input_element::Trait>;

pub type Element = Arc<dyn element::Trait>;

pub type Node = Arc<dyn node::Trait>;

pub type Text = Arc<dyn text::Trait>;
