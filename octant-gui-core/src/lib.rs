#![deny(unused_must_use)]

use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::document::{DocumentMethod, DocumentTag};
use crate::element::{ElementMethod, ElementTag};
use crate::global::GlobalMethod;
use crate::html_form_element::{
    HtmlFormElementMethod, HtmlFormElementTag, HtmlFormElementUpMessage,
};
use crate::html_input_element::{
    HtmlInputElementMethod, HtmlInputElementTag, HtmlInputElementUpMessage,
};
use crate::node::{NodeMethod, NodeTag};
use crate::window::{WindowMethod, WindowTag};

pub mod document;
pub mod element;
pub mod global;
pub mod html_form_element;
pub mod html_input_element;
pub mod node;
pub mod object;
pub mod value;
pub mod window;

#[derive(Serialize, Deserialize, Debug)]
pub struct DownMessageList {
    pub commands: Vec<DownMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Method {
    Log,
    Global(GlobalMethod),
    Window(TypedHandle<WindowTag>, WindowMethod),
    Document(TypedHandle<DocumentTag>, DocumentMethod),
    Element(TypedHandle<ElementTag>, ElementMethod),
    Node(TypedHandle<NodeTag>, NodeMethod),
    HtmlFormElement(TypedHandle<HtmlFormElementTag>, HtmlFormElementMethod),
    HtmlInputElement(TypedHandle<HtmlInputElementTag>, HtmlInputElementMethod),
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
{}

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
