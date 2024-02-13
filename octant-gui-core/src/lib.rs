#![deny(unused_must_use)]

use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandList {
    pub commands: Vec<Command>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GlobalMethod {
    Window,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WindowMethod {
    Document,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentMethod {
    Body,
    CreateTextNode,
    CreateElement,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ElementMethod {
    AppendChild,
    SetAttribute,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Method {
    Log,
    Global(GlobalMethod),
    Window(WindowMethod),
    Document(DocumentMethod),
    Element(ElementMethod),
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Handle(pub usize);

#[derive(Serialize, Deserialize)]
pub enum Argument {
    Handle(Handle),
    Json(Value),
}

#[derive(Serialize, Deserialize)]
pub enum Command {
    Invoke {
        assign: Option<Handle>,
        method: Method,
        arguments: Vec<Argument>,
    },
    Delete(Handle),
}

impl Debug for Handle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Invoke {
                assign,
                method,
                arguments,
            } => {
                if let Some(assign) = assign {
                    write!(f, "{:?} := ", assign)?;
                }
                write!(f, "{:?} {:?}", method, arguments)?;
                Ok(())
            }
            Command::Delete(handle) => {
                write!(f, "delete {:?}", handle)?;
                Ok(())
            }
        }
    }
}

impl Debug for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::Handle(x) => write!(f, "{:?}", x),
            Argument::Json(x) => write!(f, "{:?}", x),
        }
    }
}
