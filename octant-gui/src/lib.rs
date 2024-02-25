#![deny(unused_must_use)]
#![feature(try_blocks)]
#![feature(trait_alias)]
#![feature(trait_upcasting)]

use std::pin::Pin;
use std::sync::Arc;

use futures::sink::Sink;
use futures::Stream;

pub use global::Global;
use octant_gui_core::{DownMessageList, RemoteEvent};
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

pub type RenderSink = Pin<Box<dyn Send + Sync + Sink<DownMessageList, Error=anyhow::Error>>>;
pub type EventSource = Pin<Box<dyn Send + Sync + Stream<Item=anyhow::Result<RemoteEvent>>>>;
pub type Handle = Arc<dyn handle::Trait>;
pub type JsValue = Arc<dyn js_value::Trait>;
pub type Window = Arc<dyn window::Trait>;

pub type Document = Arc<dyn document::Trait>;

pub type HtmlElement = Arc<dyn html_element::Trait>;
pub type HtmlFormElement = Arc<dyn html_form_element::Trait>;

pub type Element = Arc<dyn element::Trait>;

pub type Node = Arc<dyn node::Trait>;

pub type Text = Arc<dyn text::Trait>;
