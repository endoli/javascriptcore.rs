// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{ffi::CString, ptr};

use sys::{
    JSClassCreate, JSClassDefinition, JSClassRetain, JSObjectCallAsConstructorCallback,
    JSObjectMake,
};

use crate::{sys, JSClass, JSContext, JSObject};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JSClassError {
    #[error("classname was invalid (e.g. it contains a NULL character)")]
    InvalidName,

    #[error("class could not be created")]
    FailedToCreateClass,

    #[error("class could not be retained")]
    FailedToRetainClass,
}

impl JSClass {
    pub fn new<N>(
        name: N,
        constructor: JSObjectCallAsConstructorCallback,
    ) -> Result<Self, JSClassError>
    where
        N: Into<Vec<u8>>,
    {
        let Ok(name) = CString::new(name) else {
            return Err(JSClassError::InvalidName);
        };

        let class_definition = JSClassDefinition {
            className: name.as_ptr(),
            callAsConstructor: constructor,
            ..Default::default()
        };

        let class = unsafe { JSClassCreate(&class_definition) };

        if class.is_null() {
            return Err(JSClassError::FailedToCreateClass);
        }

        let class = unsafe { JSClassRetain(class) };

        if class.is_null() {
            return Err(JSClassError::FailedToRetainClass);
        }

        Ok(unsafe { JSClass::from_raw(class, name) })
    }

    /// Create a new [`Self`] from its raw pointer directly.
    ///
    /// # Safety
    ///
    /// Ensure `raw` is valid.
    unsafe fn from_raw(raw: sys::JSClassRef, name: CString) -> Self {
        Self { raw, name }
    }

    pub fn instantiate(&self, ctx: &JSContext) -> JSObject {
        unsafe { JSObject::from_raw(ctx.raw, JSObjectMake(ctx.raw, self.raw, ptr::null_mut())) }
    }
}

impl Drop for JSClass {
    fn drop(&mut self) {
        unsafe { sys::JSClassRelease(self.raw) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{constructor_callback, function_callback, JSException};

    use super::*;

    #[test]
    fn class_with_no_constructor() -> Result<(), JSException> {
        let ctx = JSContext::default();
        let class = JSClass::new("Foo", None).unwrap();
        let object = class.instantiate(&ctx);

        assert!(object.is_object_of_class(&class));

        // It's failing because there is no constructor.
        assert!(object.call_as_constructor(&[]).is_err());

        Ok(())
    }

    #[test]
    fn class_with_constructor() -> Result<(), JSException> {
        use crate as javascriptcore;

        #[constructor_callback]
        fn foo_ctor(
            ctx: &JSContext,
            constructor: &JSObject,
            _arguments: &[JSValue],
        ) -> Result<JSValue, JSException> {
            #[function_callback]
            fn bar(
                ctx: &JSContext,
                _function: Option<&JSObject>,
                _this_object: Option<&JSObject>,
                _arguments: &[JSValue],
            ) -> Result<JSValue, JSException> {
                Ok(JSValue::new_number(&ctx, 42.))
            }

            constructor.set_property("bar", JSValue::new_function(ctx, "bar", Some(bar)))?;

            Ok(constructor.into())
        }

        let ctx = JSContext::default();
        let class = JSClass::new("Foo", Some(foo_ctor)).unwrap();
        let object = class.instantiate(&ctx);

        assert!(object.is_object_of_class(&class));

        let object = object.call_as_constructor(&[])?.as_object()?;

        assert_eq!(
            object
                .get_property("bar")
                .as_object()?
                .call_as_function(None, &[])?
                .as_number()?,
            42.
        );

        Ok(())
    }
}
