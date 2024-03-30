use std::{
    mem::MaybeUninit,
    sync::{Arc, Weak},
};
use crate::util::unique_arc::UniqueArc;

pub enum ArcOrEmpty<T> {
    Arc(Arc<T>),
    Empty(UniqueArc<MaybeUninit<T>>),
}

impl<T> ArcOrEmpty<T> {
    pub fn weak(&self) -> Weak<T> {
        match self {
            ArcOrEmpty::Arc(x) => Arc::downgrade(x),
            ArcOrEmpty::Empty(x) => UniqueArc::downgrade_uninit(x),
        }
    }
}
