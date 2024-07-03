// #![deny(unused_must_use)]
//
// use serde::Serialize;
//
// #[derive(Serialize, DeserializeWith)]
// pub struct Foo {
//     a: u32,
//     b: u32,
// }
//
// #[test]
// fn test_foo() {
//     let raw = Format::default()
//         .serialize_raw(&Foo { a: 2, b: 3 })
//         .unwrap();
//     let ctx = DeserializeContext::new();
//     let decoded = raw.deserialize_as_with::<Foo>(&ctx).unwrap();
//     assert_eq!(decoded.a, 2);
//     assert_eq!(decoded.b, 3);
// }
