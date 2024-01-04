// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{sys, JSException, JSObject, JSString, JSValue};
use std::ops::Deref;
use std::ptr;

impl JSObject {
    /// Create a new [`Self`] from its raw pointer directly.
    ///
    /// # Safety
    ///
    /// Ensure `raw` is valid.
    pub const unsafe fn from_raw(ctx: sys::JSContextRef, raw: sys::JSObjectRef) -> Self {
        Self {
            raw,
            value: JSValue::from_raw(ctx, raw),
        }
    }

    /// Gets an iterator over the names of an object's enumerable properties.
    ///
    /// ```
    /// # use javascriptcore::{JSContext, JSObject, JSString, JSValue};
    /// let ctx = JSContext::default();
    /// let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("valid object");
    /// let o = v.as_object().expect("object");
    ///
    /// let names: Vec<String> = o.property_names()
    ///                           .map(|s| s.to_string())
    ///                           .collect();
    /// assert_eq!(names, vec!["id"]);
    /// ```
    pub fn property_names(&self) -> JSObjectPropertyNameIter {
        JSObjectPropertyNameIter {
            raw: unsafe { sys::JSObjectCopyPropertyNames(self.value.ctx, self.raw) },
            idx: 0,
        }
    }

    /// Tests whether an object has a given property.
    ///
    /// * `name`: A value that can be converted to a [`JSString`] containing
    ///   the property's name.
    ///
    /// Returns `true` if the object has a property whose name matches
    /// `name`, otherwise `false`.
    ///
    /// ```
    /// # use javascriptcore::{JSContext, JSObject, JSString, JSValue};
    /// let ctx = JSContext::default();
    /// let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("valid object");
    /// let o = v.as_object().expect("object");
    ///
    /// assert!(o.has_property("id"));
    /// ```
    pub fn has_property<S>(&self, name: S) -> bool
    where
        S: Into<JSString>,
    {
        unsafe { sys::JSObjectHasProperty(self.value.ctx, self.raw, name.into().raw) }
    }

    /// Gets a property from an object.
    ///
    /// * `name`: A value that can be converted to a [`JSString`] containing
    ///   the property's name.
    ///
    /// Returns the property's value if object has the property, otherwise
    /// the undefined value.
    ///
    /// ```
    /// # use javascriptcore::{JSContext, JSObject, JSString, JSValue};
    /// let ctx = JSContext::default();
    /// let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("valid object");
    /// let o = v.as_object().expect("object");
    ///
    /// let n = o.get_property("id");
    /// assert!(n.is_number());
    /// // Remember that this will be an f64 now!
    /// assert_eq!(n.as_number().expect("number"), 123.0);
    /// ```
    ///
    /// # See also
    ///
    /// * [`JSObject::get_property_at_index()`]
    /// * [`JSObject::has_property()`]
    /// * [`JSObject::set_property()`]
    /// * [`JSObject::set_property_at_index()`]
    pub fn get_property<S>(&self, name: S) -> JSValue
    where
        S: Into<JSString>,
    {
        let mut exception: sys::JSValueRef = ptr::null_mut();
        let value = unsafe {
            sys::JSObjectGetProperty(self.value.ctx, self.raw, name.into().raw, &mut exception)
        };

        unsafe { JSValue::from_raw(self.value.ctx, value) }
    }

    /// Gets a property from an object by numeric index.
    ///
    /// * `index`: An integer value that is the property's name.
    ///
    /// Returns the property's value if object has the property,
    /// otherwise the undefined value.
    ///
    /// Calling `get_property_at_index` is equivalent to calling
    /// `get_property` with a string containing `index`,
    /// but `get_property_at_index` provides optimized access to
    /// numeric properties.
    ///
    /// ```
    /// # use javascriptcore::{JSContext, JSObject, JSString, JSValue};
    /// let ctx = JSContext::default();
    /// let v = JSValue::new_from_json(&ctx, "[3, true, \"abc\"]").expect("valid array");
    /// let o = v.as_object().expect("object");
    ///
    /// let n = o.get_property_at_index(0).as_number().expect("number");
    /// let b = o.get_property_at_index(1).as_boolean();
    /// let s = o.get_property_at_index(2).as_string().expect("string");
    ///
    /// assert_eq!(n, 3.0);
    /// assert_eq!(b, true);
    /// assert_eq!(s, "abc");
    /// ```
    ///
    /// This also works with objects when the keys are strings of numeric indexes:
    ///
    /// ```
    /// # use javascriptcore::{JSContext, JSObject, JSString, JSValue};
    /// let ctx = JSContext::default();
    /// let v = JSValue::new_from_json(&ctx, "{\"a\": 3, \"1\": true, \"2\": \"abc\"}").expect("valid object");
    /// let o = v.as_object().expect("object");
    ///
    /// // There is no property "0", so this will be `undefined`:
    /// assert!(o.get_property_at_index(0).is_undefined());
    /// assert_eq!(o.get_property_at_index(1).as_boolean(), true);
    /// assert_eq!(o.get_property_at_index(2).as_string().expect("string"), "abc");
    /// ```
    ///
    /// # See also
    ///
    /// * [`JSObject::get_property()`]
    /// * [`JSObject::has_property()`]
    /// * [`JSObject::set_property()`]
    /// * [`JSObject::set_property_at_index()`]
    pub fn get_property_at_index(&self, index: u32) -> JSValue {
        let mut exception: sys::JSValueRef = ptr::null_mut();
        let value = unsafe {
            sys::JSObjectGetPropertyAtIndex(self.value.ctx, self.raw, index, &mut exception)
        };

        unsafe { JSValue::from_raw(self.value.ctx, value) }
    }

    /// Set a property onto an object.
    ///
    /// This can be used to create a new property, or to update an existing property.
    ///
    /// * `index`: A value that can be converted to a [`JSString`] containing
    ///   the property's name.
    /// * `value`: A value containing the property's value.
    ///
    /// Calling `get_property_at_index` is equivalent to calling
    /// `get_property` with a string containing `index`,
    /// but `get_property_at_index` provides optimized access to
    /// numeric properties.
    ///
    /// ```
    /// # use javascriptcore::{JSContext, JSValue};
    /// let ctx = JSContext::default();
    /// let object = JSValue::new_from_json(&ctx, r#"{"a": 10}"#).expect("valid object").as_object().unwrap();
    /// object.set_property("b", JSValue::new_number(&ctx, 11.)).unwrap();
    ///
    /// assert!(object.has_property("a"));
    /// assert!(object.has_property("b"));
    /// ```
    ///
    /// # See also
    ///
    /// * [`JSObject::get_property()`]
    /// * [`JSObject::get_property_at_index()`]
    /// * [`JSObject::has_property()`]
    /// * [`JSObject::set_property_at_index()`]
    pub fn set_property<S>(&self, name: S, value: JSValue) -> Result<(), JSException>
    where
        S: Into<JSString>,
    {
        let name: JSString = name.into();
        let mut exception: sys::JSValueRef = ptr::null_mut();
        let context = self.value.ctx;

        unsafe {
            sys::JSObjectSetProperty(
                context,
                self.raw,
                name.raw,
                value.raw,
                sys::kJSPropertyAttributeNone,
                &mut exception,
            );
        }

        if !exception.is_null() {
            return Err(unsafe { JSValue::from_raw(context, exception) }.into());
        }

        Ok(())
    }

    /// Set a property onto an object by using a numeric index.
    ///
    /// This can be used to create a new property, or to update an existing property.
    ///
    /// * `index`: An integer value that is the property's name.
    /// * `value`: A value containing the property's value.
    ///
    /// Calling `set_property_at_index` is equivalent to calling `set_property` with
    /// a string containing `index`, but `set_property_at_index` provides optimized
    /// access to numeric properties.
    ///
    /// ```
    /// # use javascriptcore::{JSContext, JSValue};
    /// let ctx = JSContext::default();
    /// let object = JSValue::new_from_json(&ctx, r#"[10]"#).expect("valid array").as_object().unwrap();
    /// object.set_property_at_index(1, JSValue::new_number(&ctx, 11.)).unwrap();
    ///
    /// assert!(object.has_property("0"));
    /// assert!(object.has_property("1"));
    /// ```
    ///
    /// # See also
    ///
    /// * [`JSObject::get_property()`]
    /// * [`JSObject::get_property_at_index()`]
    /// * [`JSObject::has_property()`]
    /// * [`JSObject::set_property()`]
    pub fn set_property_at_index(&self, index: u32, value: JSValue) -> Result<(), JSException> {
        let mut exception: sys::JSValueRef = ptr::null_mut();
        let context = self.value.ctx;

        unsafe {
            sys::JSObjectSetPropertyAtIndex(context, self.raw, index, value.raw, &mut exception);
        }

        if !exception.is_null() {
            return Err(unsafe { JSValue::from_raw(context, exception) }.into());
        }

        Ok(())
    }

    /// Returns `true` if the object can be called as a constructor, otherwise `false`.
    ///
    /// ```rust
    /// # use javascriptcore::{JSContext, JSValue};
    /// let ctx = JSContext::default();
    /// let global = ctx.global_object().unwrap();
    ///
    /// let number = global.get_property("Number").as_object().unwrap();
    /// assert!(number.is_constructor());
    ///
    /// let math = global.get_property("Math").as_object().unwrap();
    /// let pow = math.get_property("pow").as_object().unwrap();
    /// assert!(!pow.is_constructor());
    /// ```
    ///
    /// # See also
    ///
    /// - [`JSObject::call_as_constructor()`]
    pub fn is_constructor(&self) -> bool {
        unsafe { sys::JSObjectIsConstructor(self.value.ctx, self.raw) }
    }

    /// Call this object considering it is a valid object constructor.
    ///
    /// ```rust
    /// # use javascriptcore::{JSContext, JSValue};
    /// let ctx = JSContext::default();
    /// let global = ctx.global_object().unwrap();
    /// let number = global.get_property("Number").as_object().unwrap();
    ///
    /// let result = number.call_as_constructor(&[JSValue::new_string(&ctx, "42")]).unwrap();
    ///
    /// assert!(result.is_object());
    /// assert_eq!(result.as_number().unwrap(), 42.);
    /// ```
    ///
    /// # See also
    ///
    /// - [`JSObject::call_as_function()`]
    /// - [`JSObject::is_constructor()`]
    pub fn call_as_constructor(&self, arguments: &[JSValue]) -> Result<JSValue, JSException> {
        let arguments = arguments
            .iter()
            .map(|argument| argument.raw)
            .collect::<Vec<_>>();
        let mut exception: sys::JSValueRef = ptr::null_mut();
        let context = self.value.ctx;

        let result = unsafe {
            sys::JSObjectCallAsConstructor(
                context,
                self.raw,
                arguments.len(),
                arguments.as_slice().as_ptr(),
                &mut exception,
            )
        };

        if !exception.is_null() {
            return Err(unsafe { JSValue::from_raw(context, exception) }.into());
        }

        if result.is_null() {
            return Err(JSValue::new_string_inner(
                context,
                "Cannot call this object as a constructor: it is not a valid constructor",
            )
            .into());
        }

        Ok(unsafe { JSValue::from_raw(context, result) })
    }

    /// Returns `true` if the object can be called as a constructor, otherwise `false`.
    ///
    /// ```rust
    /// # use javascriptcore::{JSContext, JSValue};
    /// let ctx = JSContext::default();
    /// let global = ctx.global_object().unwrap();
    ///
    /// let number = global.get_property("Number").as_object().unwrap();
    /// assert!(number.is_function());
    ///
    /// let math = global.get_property("Math").as_object().unwrap();
    /// let pow = math.get_property("pow").as_object().unwrap();
    /// assert!(pow.is_function());
    ///
    /// let pi = math.get_property("PI").as_object().unwrap();
    /// assert!(!pi.is_function());
    /// ```
    ///
    /// # See also
    ///
    /// - [`JSObject::call_as_function()`]
    pub fn is_function(&self) -> bool {
        unsafe { sys::JSObjectIsFunction(self.value.ctx, self.raw) }
    }

    /// Call this object considering it is a valid function.
    ///
    /// ```rust
    /// # use javascriptcore::{JSContext, JSValue};
    /// let ctx = JSContext::default();
    /// let global = ctx.global_object().unwrap();
    /// let math = global.get_property("Math").as_object().unwrap();
    /// let pow = math.get_property("pow").as_object().unwrap();
    ///
    /// let result = pow.call_as_function(
    ///     None,
    ///     &[JSValue::new_number(&ctx, 2.), JSValue::new_number(&ctx, 3.)],
    /// ).unwrap();
    ///
    /// assert_eq!(result.as_number().unwrap(), 8.);
    /// ```
    ///
    /// # See also
    ///
    /// - [`JSObject::call_as_constructor()`]
    /// - [`JSObject::is_function()`]
    pub fn call_as_function(
        &self,
        this: Option<&JSObject>,
        arguments: &[JSValue],
    ) -> Result<JSValue, JSException> {
        let arguments = arguments
            .iter()
            .map(|argument| argument.raw)
            .collect::<Vec<_>>();
        let mut exception: sys::JSValueRef = ptr::null_mut();
        let context = self.value.ctx;

        let result = unsafe {
            sys::JSObjectCallAsFunction(
                context,
                self.raw,
                this.map_or_else(ptr::null_mut, |this| this.raw),
                arguments.len(),
                arguments.as_slice().as_ptr(),
                &mut exception,
            )
        };

        if !exception.is_null() {
            return Err(unsafe { JSValue::from_raw(context, exception).into() });
        }

        if result.is_null() {
            return Err(JSValue::new_string_inner(
                context,
                "Cannot call this object as a function: it is not a valid function",
            )
            .into());
        }

        Ok(unsafe { JSValue::from_raw(context, result) })
    }
}

/// A `JSObject` can be dereferenced to return the underlying `JSValue`.
///
/// This lets a `JSObject` instance be used where a `JSValue` instance is
/// expected.
impl Deref for JSObject {
    type Target = JSValue;

    fn deref(&self) -> &JSValue {
        &self.value
    }
}

impl From<&JSObject> for JSValue {
    fn from(object: &JSObject) -> Self {
        // SAFETY: `ctx` and `raw` is valid, it's safe to use them.
        unsafe { JSValue::from_raw(object.value.ctx, object.value.raw) }
    }
}

impl From<JSObject> for JSValue {
    fn from(object: JSObject) -> Self {
        (&object).into()
    }
}

pub struct JSObjectPropertyNameIter {
    raw: sys::JSPropertyNameArrayRef,
    idx: usize,
}

impl Iterator for JSObjectPropertyNameIter {
    type Item = JSString;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < unsafe { sys::JSPropertyNameArrayGetCount(self.raw) } {
            let name = unsafe { sys::JSPropertyNameArrayGetNameAtIndex(self.raw, self.idx) };
            self.idx += 1;
            // GetNameAtIndex doesn't retain the name, so since we're going to release it
            // when we release the property name array, but this JSString may outlive that,
            // we need to retain the string to keep it alive.
            Some(JSString {
                raw: unsafe { sys::JSStringRetain(name) },
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::JSPropertyNameArrayGetCount(self.raw) };
        (sz - self.idx, Some(sz))
    }
}

impl Drop for JSObjectPropertyNameIter {
    fn drop(&mut self) {
        unsafe { sys::JSPropertyNameArrayRelease(self.raw) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{JSContext, JSException, JSValue};

    #[test]
    fn can_has_property() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("value");
        let o = v.as_object().expect("object");
        assert!(o.has_property("id"));
        assert!(!o.has_property("no-such-value"));
    }

    #[test]
    fn can_get_property() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("value");
        let o = v.as_object().expect("object");
        assert!(o.get_property("id").is_number());
        assert!(o.get_property("no-such-value").is_undefined());
    }

    #[test]
    fn can_get_property_at_index() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "[3, true, \"abc\"]").expect("value");
        let o = v.as_object().expect("object");
        assert!(o.get_property_at_index(0).is_number());
        assert!(o.get_property_at_index(1).is_boolean());
        assert!(o.get_property_at_index(2).is_string());
        assert!(o.get_property_at_index(5).is_undefined());
    }

    #[test]
    fn can_get_property_names() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("value");
        let o = v.as_object().expect("object");
        let names = o.property_names().collect::<Vec<_>>();
        assert_eq!(names.len(), 1);
        assert_eq!(names[0], "id");
    }

    #[test]
    fn can_set_property() -> Result<(), JSException> {
        let ctx = JSContext::default();
        let object = JSValue::new_from_json(&ctx, r#"{"foo": "bar"}"#)
            .unwrap()
            .as_object()?;

        assert!(object.has_property("foo"));
        assert!(!object.has_property("baz"));

        object.set_property("baz", JSValue::new_string(&ctx, "qux"))?;

        assert!(object.has_property("baz"));
        assert_eq!(object.get_property("baz").as_string()?.to_string(), "qux");

        Ok(())
    }

    #[test]
    fn can_set_property_at_index() -> Result<(), JSException> {
        let ctx = JSContext::default();
        let object = JSValue::new_from_json(&ctx, r#"[10]"#)
            .unwrap()
            .as_object()?;

        assert!(object.has_property("0"));
        assert!(!object.has_property("1"));

        object.set_property_at_index(1, JSValue::new_number(&ctx, 11.))?;

        assert!(object.has_property("1"));
        assert_eq!(object.get_property_at_index(1).as_number()?, 11.);

        Ok(())
    }

    #[test]
    fn can_use_as_jsvalue_via_deref() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("value");
        let o = v.as_object().expect("object");
        assert!(v.is_object());
        assert!(o.is_object());
    }

    #[test]
    fn can_call_as_constructor() -> Result<(), JSException> {
        let ctx = JSContext::default();
        let global = ctx.global_object()?;
        let number = global.get_property("Number").as_object()?;

        let result = number.call_as_constructor(&[JSValue::new_string(&ctx, "42")])?;

        // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/Number#return_value
        assert!(result.is_object());
        assert!(!result.is_number());

        assert_eq!(result.as_number()?, 42.);

        Ok(())
    }

    #[test]
    fn can_call_as_function() -> Result<(), JSException> {
        let ctx = JSContext::default();
        let global = ctx.global_object()?;
        let math = global.get_property("Math").as_object()?;
        let pow = math.get_property("pow").as_object()?;

        let result = pow.call_as_function(
            None,
            &[JSValue::new_number(&ctx, 2.), JSValue::new_number(&ctx, 3.)],
        )?;

        assert_eq!(result.as_number()?, 8.);

        // Not a function, it's a constant.
        let e = math.get_property("E").as_object()?;

        assert!(e.call_as_function(None, &[]).is_err());

        Ok(())
    }
}
