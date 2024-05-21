#![feature(trait_upcasting)]
#![deny(unused_must_use)]
#![allow(unused_imports)]
#![feature(macro_metavar_expr)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(hint_must_use)]

use std::{
    marker::Unsize,
    ops::CoerceUnsized
    ,
};

pub mod any_value;
pub mod document;
pub mod element;
pub mod global;
pub mod html_div_element;
pub mod html_element;
pub mod js_value;
pub mod node;
pub mod object;
// pub mod prompt;
pub mod text;
pub mod window;
