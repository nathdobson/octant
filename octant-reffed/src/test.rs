use crate::{ArcRef, RcRef, Reffed};
use std::{rc::Rc, sync::Arc};

#[test]
fn test_arc() {
    let x: Arc<u32> = Arc::new(42);
    trait Foo {
        fn foo(self: ArcRef<Self>) -> u32;
    }
    impl Foo for u32 {
        fn foo(self: ArcRef<Self>) -> u32 {
            *self.arc()
        }
    }
    assert_eq!(x.reffed().foo(), 42);
}

#[test]
fn test_rc() {
    let x: Rc<u32> = Rc::new(42);
    trait Foo {
        fn foo(self: RcRef<Self>) -> u32;
    }
    impl Foo for u32 {
        fn foo(self: RcRef<Self>) -> u32 {
            *self.rc()
        }
    }
    assert_eq!(x.reffed().foo(), 42);
}
