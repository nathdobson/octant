use std::sync::{Arc, Weak};

use crate::util::unique_arc::{MaybeUninit2, UniqueArc};

pub enum ArcOrEmpty<T: ?Sized> {
    Arc(Arc<T>),
    Empty(UniqueArc<MaybeUninit2<T>>),
}

impl<T> ArcOrEmpty<T> {
    pub fn weak(&self) -> Weak<T> {
        match self {
            ArcOrEmpty::Arc(x) => Arc::downgrade(x),
            ArcOrEmpty::Empty(x) => UniqueArc::downgrade_uninit(x),
        }
    }
}
