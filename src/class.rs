// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{ffi::CString, ptr};

use crate::{sys, JSClass, JSContext, JSException, JSObject, JSValue};
use thiserror::Error;

#[derive(Debug, Error)]
enum JSClassError {
    #[error("classname was invalid (e.g. it contains a NULL character)")]
    InvalidName,

    #[error("class could not be created")]
    FailedToCreateClass,

    #[error("class could not be retained")]
    FailedToRetainClass,
}

impl JSClass {
    /// Create a new builder to build a [`Self`].
    ///
    /// ```rust
    /// # use javascriptcore::*;
    /// /// Declare a class constructor.
    /// #[constructor_callback]
    /// fn foo(
    ///     ctx: &JSContext,
    ///     constructor: &JSObject,
    ///     _arguments: &[JSValue],
    /// ) -> Result<JSValue, JSException> {
    ///     /// Declare a function.
    ///     #[function_callback]
    ///     fn bar(
    ///         ctx: &JSContext,
    ///         _function: Option<&JSObject>,
    ///         _this_object: Option<&JSObject>,
    ///         _arguments: &[JSValue],
    ///     ) -> Result<JSValue, JSException> {
    ///         Ok(JSValue::new_number(&ctx, 42.))
    ///     }
    ///
    ///     constructor.set_property("bar", JSValue::new_function(ctx, "bar", Some(bar)))?;
    ///
    ///     Ok(constructor.into())
    /// }
    ///
    /// let ctx = JSContext::default();
    /// let class = JSClass::builder(&ctx, "Foo")
    ///     .unwrap()
    ///     .constructor(Some(foo))
    ///     .build()
    ///     .unwrap();
    ///
    /// // We have a class! Now, let's populate it inside the global object, just for fun.
    ///
    /// let object = class.new_object();
    /// let global_object = ctx.global_object().unwrap();
    /// global_object.set_property("Foo", object.into()).unwrap();
    ///
    /// let result = evaluate_script(&ctx, "const foo = new Foo(); foo.bar()", None, "test.js", 1).unwrap();
    ///
    /// assert_eq!(result.as_number().unwrap(), 42.);
    //// ```
    pub fn builder<N>(ctx: &JSContext, name: N) -> Result<JSClassBuilder, JSException>
    where
        N: Into<Vec<u8>>,
    {
        let Ok(name) = CString::new(name) else {
            return Err(JSValue::new_string(ctx, JSClassError::InvalidName.to_string()).into());
        };

        let class_definition = sys::JSClassDefinition {
            className: name.as_ptr(),
            ..Default::default()
        };

        Ok(JSClassBuilder {
            ctx,
            name,
            class_definition,
        })
    }

    /// Create a new [`Self`] from its raw pointer directly.
    ///
    /// # Safety
    ///
    /// Ensure `raw` is valid.
    unsafe fn from_raw(ctx: sys::JSContextRef, raw: sys::JSClassRef, name: CString) -> Self {
        Self { ctx, raw, name }
    }

    /// Transform the `Self` into a [`JSObject`].
    ///
    /// Note that it doesn't instantiate the class. To do a proper instantiation, one has to
    /// call [`JSObject::call_as_constructor`].
    ///
    /// ```rust
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    /// let class = JSClass::builder(&ctx, "Foo").unwrap().build().unwrap();
    /// let object = class.new_object();
    ///
    /// assert!(object.is_object_of_class(&class));
    /// ```
    pub fn new_object(&self) -> JSObject {
        unsafe {
            JSObject::from_raw(
                self.ctx,
                sys::JSObjectMake(self.ctx, self.raw, ptr::null_mut()),
            )
        }
    }
}

impl Drop for JSClass {
    fn drop(&mut self) {
        unsafe { sys::JSClassRelease(self.raw) }
    }
}

/// A builder for [`JSClass`].
///
/// Get an instance of `Self` with [`JSClass::builder`].
#[must_use]
pub struct JSClassBuilder<'a> {
    /// The context.
    ctx: &'a JSContext,

    /// The class name.
    name: CString,

    /// The class definition.
    class_definition: sys::JSClassDefinition,
}

impl JSClassBuilder<'_> {
    /// Set a class constructor, called by [the `new` operator in JavaScript][new].
    ///
    /// The easiest way to generate a [`JSObjectCallAsConstructorCallback`] is by using the
    /// [`crate::constructor_callback`] procedural macro.
    ///
    /// [`JSObjectCallAsConstructorCallback`]: sys::JSObjectCallAsConstructorCallback
    /// [new]: https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/new
    pub fn constructor(mut self, constructor: sys::JSObjectCallAsConstructorCallback) -> Self {
        self.class_definition.callAsConstructor = constructor;

        self
    }

    /// Build a [`JSClass`].
    pub fn build(self) -> Result<JSClass, JSException> {
        let class = unsafe { sys::JSClassCreate(&self.class_definition) };

        if class.is_null() {
            return Err(JSValue::new_string(
                self.ctx,
                JSClassError::FailedToCreateClass.to_string(),
            )
            .into());
        }

        let class = unsafe { sys::JSClassRetain(class) };

        if class.is_null() {
            return Err(JSValue::new_string(
                self.ctx,
                JSClassError::FailedToRetainClass.to_string(),
            )
            .into());
        }

        Ok(unsafe { JSClass::from_raw(self.ctx.raw, class, self.name) })
    }
}

#[cfg(test)]
mod tests {
    use crate::{constructor_callback, evaluate_script, function_callback};

    use super::*;

    #[test]
    fn class_with_no_constructor() -> Result<(), JSException> {
        let ctx = JSContext::default();
        let class = JSClass::builder(&ctx, "Foo")?.build()?;
        let object = class.new_object();

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
                Ok(JSValue::new_number(ctx, 42.))
            }

            constructor.set_property("bar", JSValue::new_function(ctx, "bar", Some(bar)))?;

            Ok(constructor.into())
        }

        let ctx = JSContext::default();
        let class = JSClass::builder(&ctx, "Foo")?
            .constructor(Some(foo_ctor))
            .build()?;
        let object = class.new_object();

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

        // Let's try in a script.
        let global_object = ctx.global_object()?;
        global_object.set_property("Foo", object.into())?;

        let result = evaluate_script(&ctx, "const foo = new Foo(); foo.bar()", None, "test.js", 1);

        assert_eq!(result?.as_number()?, 42.);

        Ok(())
    }
}
