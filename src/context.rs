// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;
use super::{JSClass, JSContext};
use sys;

impl JSContext {
    /// Creates a global JavaScript execution context and populates it
    /// with all the built-in JavaScript objects, such as `Object`,
    /// `Function`, `String`, and `Array`.
    ///
    /// In WebKit version 4.0 and later, the context is created in a
    /// unique context group. Therefore, scripts may execute in it
    /// concurrently with scripts executing in other contexts.
    /// However, you may not use values created in the context in other
    /// contexts.
    pub fn new() -> Self {
        JSContext::default()
    }

    /// Creates a global JavaScript execution context and populates it
    /// with all the built-in JavaScript objects, such as `Object`,
    /// `Function`, `String`, and `Array`.
    ///
    /// In WebKit version 4.0 and later, the context is created in a
    /// unique context group. Therefore, scripts may execute in it
    /// concurrently with scripts executing in other contexts.
    /// However, you may not use values created in the context in other
    /// contexts.
    ///
    /// * `global_object_class`: The class to use when creating the global
    ///   object.
    pub fn new_with_class(global_object_class: &JSClass) -> Self {
        JSContext { raw: unsafe { sys::JSGlobalContextCreate(global_object_class.raw) } }
    }
}

impl Default for JSContext {
    /// Creates a global JavaScript execution context and populates it
    /// with all the built-in JavaScript objects, such as `Object`,
    /// `Function`, `String`, and `Array`.
    ///
    /// In WebKit version 4.0 and later, the context is created in a
    /// unique context group. Therefore, scripts may execute in it
    /// concurrently with scripts executing in other contexts.
    /// However, you may not use values created in the context in other
    /// contexts.
    fn default() -> Self {
        JSContext { raw: unsafe { sys::JSGlobalContextCreate(ptr::null_mut()) } }
    }
}

impl Drop for JSContext {
    fn drop(&mut self) {
        unsafe { sys::JSGlobalContextRelease(self.raw) }
    }
}
