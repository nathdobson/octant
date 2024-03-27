use std::borrow::Cow;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub enum ArcOrWeak<T: ?Sized> {
    Arc(Arc<T>),
    Weak(Weak<T>),
}

impl<T: ?Sized> ArcOrWeak<T> {
    pub fn upgrade_cow<'a>(&'a self) -> Option<Cow<'a, Arc<T>>> {
        match self {
            ArcOrWeak::Arc(x) => Some(Cow::Borrowed(x)),
            ArcOrWeak::Weak(x) => x.upgrade().map(Cow::Owned),
        }
    }
}

pub fn arc_try_new_cyclic<T, E>(
    f: impl for<'a> FnOnce(&'a Weak<T>) -> Result<T, E>,
) -> Result<Arc<T>, E> {
    let mut err = None;
    match catch_unwind(AssertUnwindSafe(|| {
        Arc::new_cyclic(|x| match f(x) {
            Err(e) => {
                err = Some(e);
                panic!("unwinding from failed arc");
            }
            Ok(x) => x,
        })
    })) {
        Err(p) => {
            if let Some(err) = err {
                return Err(err);
            } else {
                resume_unwind(p)
            }
        }
        Ok(x) => Ok(x),
    }
}