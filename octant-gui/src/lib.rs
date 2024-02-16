#![deny(unused_must_use)]

use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use std::{iter, mem};

use atomic_refcell::AtomicRefCell;
use futures::sink::Sink;
use futures::{SinkExt, Stream, StreamExt};
use serde_json::Value;

use octant_gui_core::{
    Argument, Command, CommandList, DocumentMethod, ElementMethod, GlobalMethod, Handle, Method,
    RemoteEvent, WindowMethod,
};

type RenderSink = Pin<Box<dyn Send + Sync + Sink<CommandList, Error = anyhow::Error>>>;
type EventSource = Pin<Box<dyn Send + Sync + Stream<Item = anyhow::Result<RemoteEvent>>>>;

struct State {
    buffer: Vec<Command>,
    consumer: RenderSink,
    next_handle: usize,
}

pub struct Root(AtomicRefCell<State>);

pub struct OwnedHandle {
    root: Arc<Root>,
    handle: Handle,
}

impl Root {
    pub fn new(mut events: EventSource, consumer: RenderSink) -> Arc<Self> {
        let result = Arc::new(Root(AtomicRefCell::new(State {
            buffer: vec![],
            consumer,
            next_handle: 0,
        })));
        let weak = Arc::downgrade(&result);
        tokio::spawn(async move {
            while let Some(event) = events.next().await {
                log::info!("event {:?}", event);
            }
        });
        result
    }
    pub fn invoke(self: &Arc<Self>, method: Method, arguments: Vec<Argument>) -> OwnedHandle {
        let ref mut this = *self.0.borrow_mut();
        let handle = Handle(this.next_handle);
        this.next_handle += 1;
        this.buffer.push(Command::Invoke {
            assign: Some(handle),
            method,
            arguments,
        });
        OwnedHandle {
            root: self.clone(),
            handle,
        }
    }
    pub fn delete(&self, handle: Handle) {
        self.send(Command::Delete(handle));
    }
    pub fn send(&self, command: Command) {
        let ref mut this = *self.0.borrow_mut();
        this.buffer.push(command);
    }
    pub async fn flush(&self) -> anyhow::Result<()> {
        let ref mut this = *self.0.borrow_mut();
        this.consumer
            .send(CommandList {
                commands: mem::replace(&mut this.buffer, vec![]),
            })
            .await?;
        Ok(())
    }
    pub fn log(self: &Arc<Self>, argument: Argument) {
        self.invoke(Method::Log, vec![argument]);
    }
    pub fn window(self: &Arc<Self>) -> Window {
        Window {
            parent: Node {
                parent: Object {
                    parent: JsValue {
                        handle: self.invoke(Method::Global(GlobalMethod::Window), vec![]),
                    },
                },
            },
        }
    }
}

impl Drop for OwnedHandle {
    fn drop(&mut self) {
        self.root.delete(self.handle)
    }
}

impl OwnedHandle {
    pub fn invoke(&self, method: Method, args: Vec<Argument>) -> OwnedHandle {
        self.root.invoke(
            method,
            iter::once(Argument::Handle(self.handle))
                .chain(args.into_iter())
                .collect(),
        )
    }
    pub fn handle(&self) -> Handle {
        self.handle
    }
}

pub struct JsValue {
    handle: OwnedHandle,
}

impl JsValue {
    pub fn new(handle: OwnedHandle) -> Self {
        JsValue { handle }
    }
    pub fn handle(&self) -> &OwnedHandle {
        &self.handle
    }
}

pub struct Object {
    parent: JsValue,
}

impl Object {
    pub fn new(handle: OwnedHandle) -> Self {
        Object {
            parent: JsValue::new(handle),
        }
    }
}

impl Deref for Object {
    type Target = JsValue;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

pub struct Node {
    parent: Object,
}

impl Node {
    pub fn new(handle: OwnedHandle) -> Self {
        Node {
            parent: Object::new(handle),
        }
    }
}

impl Deref for Node {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

pub struct Window {
    parent: Node,
}

impl Window {
    pub fn new(handle: OwnedHandle) -> Self {
        Window {
            parent: Node::new(handle),
        }
    }
}

impl Deref for Window {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

pub struct Document {
    parent: Node,
}

impl Document {
    pub fn new(handle: OwnedHandle) -> Self {
        Document {
            parent: Node::new(handle),
        }
    }
}

impl Deref for Document {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

pub struct Element {
    parent: Node,
}

impl Element {
    pub fn new(handle: OwnedHandle) -> Self {
        Element {
            parent: Node::new(handle),
        }
    }
}

impl Deref for Element {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

pub struct HtmlElement {
    parent: Element,
}

impl HtmlElement {
    pub fn new(handle: OwnedHandle) -> Self {
        HtmlElement {
            parent: Element::new(handle),
        }
    }
}

impl Deref for HtmlElement {
    type Target = Element;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

pub struct Text {
    parent: Node,
}

impl Text {
    pub fn new(handle: OwnedHandle) -> Self {
        Text {
            parent: Node::new(handle),
        }
    }
}

impl Deref for Text {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

pub struct HtmlFormElement {
    parent: HtmlElement,
}

impl Deref for HtmlFormElement {
    type Target = HtmlElement;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl HtmlFormElement {
    pub fn new(handle: OwnedHandle) -> Self {
        HtmlFormElement {
            parent: HtmlElement::new(handle),
        }
    }
}

impl Window {
    fn invoke(&self, method: WindowMethod, args: Vec<Argument>) -> OwnedHandle {
        self.handle().invoke(Method::Window(method), args)
    }
    pub fn document(&self) -> Document {
        Document::new(self.invoke(WindowMethod::Document, vec![]))
    }
}

impl Document {
    fn invoke(&self, method: DocumentMethod, args: Vec<Argument>) -> OwnedHandle {
        self.handle.invoke(Method::Document(method), args)
    }
    pub fn body(&self) -> HtmlElement {
        HtmlElement::new(self.invoke(DocumentMethod::Body, vec![]))
    }
    pub fn create_text_node(&self, text: &str) -> Text {
        Text::new(self.invoke(
            DocumentMethod::CreateTextNode,
            vec![Argument::Json(Value::String(text.to_string()))],
        ))
    }
    pub fn create_element(&self, tag: &str) -> Element {
        Element::new(self.invoke(
            DocumentMethod::CreateElement,
            vec![Argument::Json(Value::String(tag.to_string()))],
        ))
    }
    pub fn create_form_element(&self) -> HtmlFormElement {
        HtmlFormElement::new(self.invoke(DocumentMethod::CreateFormElement, vec![]))
    }
}

impl Element {
    fn invoke(&self, method: ElementMethod, args: Vec<Argument>) -> OwnedHandle {
        self.handle.invoke(Method::Element(method), args)
    }
    pub fn append_child(&self, child: &Node) {
        self.invoke(
            ElementMethod::AppendChild,
            vec![Argument::Handle(child.handle().handle())],
        );
    }
    pub fn set_attribute(&self, name: &str, value: &str) {
        self.invoke(
            ElementMethod::SetAttribute,
            vec![
                Argument::Json(Value::String(name.to_string())),
                Argument::Json(Value::String(value.to_string())),
            ],
        );
    }
}
