use std::hash::{Hash, Hasher};
use std::sync::Weak;

pub struct ByWeak<T>(pub T);

impl<T> Hash for ByWeak<Weak<T>> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state)
    }
}

impl<T> PartialEq for ByWeak<Weak<T>> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ptr() == other.0.as_ptr()
    }
}

impl<T> Eq for ByWeak<Weak<T>> {}
