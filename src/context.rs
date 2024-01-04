// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys::JSContextGetGlobalObject;

use crate::{sys, JSClass, JSContext, JSContextGroup, JSException, JSObject, JSString, JSValue};
use std::ptr;

impl JSContext {
    /// Create a new [`Self`] from its raw pointer directly.
    ///
    /// # Safety
    ///
    /// Ensure `raw` is valid.
    pub const unsafe fn from_raw(raw: sys::JSGlobalContextRef) -> Self {
        Self { raw }
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
    pub fn new() -> Self {
        Self::default()
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
        unsafe { Self::from_raw(sys::JSGlobalContextCreate(global_object_class.raw)) }
    }

    /// Gets the context group to which a JavaScript execution context belongs.
    pub fn group(&self) -> JSContextGroup {
        let group = unsafe { sys::JSContextGetGroup(self.raw) };

        unsafe {
            sys::JSContextGroupRetain(group);
        };

        JSContextGroup { raw: group }
    }

    /// Gets a copy of the name of a context.
    ///
    /// A `JSContext`'s name is exposed for remote debugging
    /// to make it easier to identify the context you would like to
    /// attach to.
    ///
    /// Returns the name for this context, if there is one.
    ///
    /// ```
    /// # use javascriptcore::JSContext;
    /// let ctx = JSContext::new();
    ///
    /// // By default, a context has no name.
    /// assert!(ctx.name().is_none());
    /// ```
    pub fn name(&self) -> Option<JSString> {
        let result = unsafe { sys::JSGlobalContextCopyName(self.raw) };

        if result.is_null() {
            None
        } else {
            Some(JSString { raw: result })
        }
    }

    /// Sets the remote debugging name for a context.
    ///
    /// * `name`: The remote debugging name to set.
    ///
    /// ```
    /// # use javascriptcore::JSContext;
    /// let ctx = JSContext::new();
    ///
    /// ctx.set_name("test thread");
    /// assert_eq!(ctx.name().unwrap(), "test thread");
    /// ```
    pub fn set_name<S: Into<JSString>>(&self, name: S) {
        unsafe { sys::JSGlobalContextSetName(self.raw, name.into().raw) }
    }

    /// Get the global object of this context.
    ///
    /// ```rust
    /// # use javascriptcore::JSContext;
    /// let ctx = JSContext::new();
    ///
    /// assert!(ctx.global_object().is_ok());
    /// ```
    pub fn global_object(&self) -> Result<JSObject, JSException> {
        let global_object = unsafe { JSContextGetGlobalObject(self.raw) };

        if global_object.is_null() {
            Err(unsafe { JSValue::from_raw(self.raw, global_object) }.into())
        } else {
            Ok(unsafe { JSObject::from_raw(self.raw, global_object) })
        }
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
        unsafe { Self::from_raw(sys::JSGlobalContextCreate(ptr::null_mut())) }
    }
}

impl Drop for JSContext {
    fn drop(&mut self) {
        unsafe { sys::JSGlobalContextRelease(self.raw) }
    }
}

#[cfg(test)]
mod tests {
    use crate::JSContext;

    #[test]
    fn context_group() {
        let ctx = JSContext::new();
        let _g = ctx.group();
        // Nothing to do with g now...
    }

    #[test]
    fn context_names() {
        let ctx = JSContext::new();
        assert!(ctx.name().is_none());

        ctx.set_name("test thread");
        assert_eq!(ctx.name().unwrap(), "test thread");
    }

    #[test]
    fn global_object() {
        let ctx = JSContext::new();
        let global_object = ctx.global_object().unwrap();

        let some_property = global_object.get_property("Array");
        assert!(!some_property.is_undefined());
    }
}
