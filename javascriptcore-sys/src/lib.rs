// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate provides raw bindings for the JavaScriptCore public
//! API. It is a pretty direct mapping of the underlying C API
//! provided by JavaScriptCore.

#![allow(non_camel_case_types, non_snake_case)]

mod javascriptcore_sys;

pub use javascriptcore_sys::*;
