use std::{
    borrow::Cow,
    fmt::{Debug, Formatter}
    ,
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    sync::{Arc, Weak},
};

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
                resume_unwind(Box::new("arc_try_new_cyclic internal panic"));
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

impl<T: Debug> Debug for ArcOrWeak<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArcOrWeak::Arc(x) => x.fmt(f),
            ArcOrWeak::Weak(x) => x.fmt(f),
        }
    }
}
