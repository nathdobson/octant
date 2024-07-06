// use crate::arc::{Arc2, ArcRef};
// use crate::rc::{Rc2, RcfRef};
//
// #[test]
// fn test_arc() {
//     let x: Arc2<u32> = Arc2::new(42);
//     trait Foo {
//         fn foo(self: &ArcRef<Self>) -> u32;
//     }
//     impl Foo for u32 {
//         fn foo(self: &ArcRef<Self>) -> u32 {
//             **self.arc()
//         }
//     }
//     assert_eq!(x.foo(), 42);
// }
//
// #[test]
// fn test_rc() {
//     let x: Rc2<u32> = Rc2::new(42);
//     trait Foo {
//         fn foo(self: &RcfRef<Self>) -> u32;
//     }
//     impl Foo for u32 {
//         fn foo(self: &RcfRef<Self>) -> u32 {
//             **self.rc()
//         }
//     }
//     assert_eq!(x.foo(), 42);
// }
