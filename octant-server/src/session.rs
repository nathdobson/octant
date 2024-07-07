use std::{
    any::{type_name, Any, TypeId},
    rc::Rc,
};

use memo_map::MemoMap;
use url::Url;

use octant_error::OctantError;
use octant_web_sys_server::global::Global;

pub struct Session {
    global: Rc<Global>,
    data: MemoMap<TypeId, Box<dyn 'static + Any + Send + Sync>>,
}

pub trait SessionData: 'static + Sync + Send {}

impl Session {
    pub fn new(global: Rc<Global>) -> Session {
        Session {
            global,
            data: MemoMap::new(),
        }
    }
    pub fn global(&self) -> &Rc<Global> {
        &self.global
    }
    pub fn data<T: SessionData + Default>(&self) -> &T {
        self.data
            .get_or_insert(&TypeId::of::<T>(), || Box::<T>::default())
            .downcast_ref()
            .unwrap()
    }
    pub fn insert_data<T: SessionData>(&self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }
    pub fn try_data<T: SessionData>(&self) -> Result<&T, MissingData> {
        if let Some(data) = self.data.get(&TypeId::of::<T>()) {
            Ok(data.downcast_ref().unwrap())
        } else {
            Err(MissingData(type_name::<T>()))
        }
    }
}

pub struct MissingData(&'static str);
impl From<MissingData> for OctantError {
    fn from(value: MissingData) -> Self {
        OctantError::msg(format!("Cannot find data {}", value.0))
    }
}

pub struct UrlPrefix {
    url: Url,
}

impl UrlPrefix {
    pub fn new(url: Url) -> Self {
        UrlPrefix { url }
    }
}

impl SessionData for UrlPrefix {}

impl UrlPrefix {
    pub fn url(&self) -> &Url {
        &self.url
    }
}
