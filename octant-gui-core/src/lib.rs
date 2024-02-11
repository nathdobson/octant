#![deny(unused_must_use)]


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
pub enum DocumentMethod {}

#[derive(Serialize, Deserialize, Debug)]
pub enum Method {
    Log,
    Global(GlobalMethod),
    Window(WindowMethod),
    DocumentMethod(DocumentMethod),
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, Debug)]
pub struct Handle(pub usize);

#[derive(Serialize, Deserialize, Debug)]
pub enum Argument {
    Handle(Handle),
    Json(Value),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Invoke {
        assign: Option<Handle>,
        method: Method,
        arguments: Vec<Argument>,
    },
    Delete(Handle),
}
