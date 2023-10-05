// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{JSClass, JSContext, JSException, JSObject, JSString, JSType, JSValue};
use crate::sys;
use std::ptr;

impl JSValue {
    /// Creates a JavaScript value of the `undefined` type.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns the unique `undefined` value.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_undefined(&ctx);
    /// assert!(v.is_undefined());
    /// ```
    pub fn new_undefined(ctx: &JSContext) -> Self {
        JSValue {
            raw: unsafe { sys::JSValueMakeUndefined(ctx.raw) },
            ctx: ctx.raw,
        }
    }

    /// Creates a JavaScript value of the `null` type.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns the unique `null` value.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_null(&ctx);
    /// assert!(v.is_null());
    /// ```
    pub fn new_null(ctx: &JSContext) -> Self {
        JSValue {
            raw: unsafe { sys::JSValueMakeNull(ctx.raw) },
            ctx: ctx.raw,
        }
    }

    /// Creates a JavaScript value of the `boolean` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `boolean`: The `bool` to assign to the newly created `JSValue`.
    ///
    /// Returns a `JSValue` of the `boolean` type, representing the value of `boolean`.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_boolean(&ctx, true /* or false */);
    /// assert!(v.is_boolean());
    /// ```
    pub fn new_boolean(ctx: &JSContext, boolean: bool) -> Self {
        JSValue {
            raw: unsafe { sys::JSValueMakeBoolean(ctx.raw, boolean) },
            ctx: ctx.raw,
        }
    }

    /// Creates a JavaScript value of the `number` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `number`: The `f64` to assign to the newly created `JSValue`.
    ///
    /// Returns a `JSValue` of the `number` type, representing the value of `number`.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_number(&ctx, 3.0f64);
    /// assert!(v.is_number());
    ///
    /// let v = JSValue::new_number(&ctx, 3.0f32 as f64);
    /// assert!(v.is_number());
    ///
    /// let v = JSValue::new_number(&ctx, 3 as f64);
    /// assert!(v.is_number());
    /// ```
    pub fn new_number(ctx: &JSContext, number: f64) -> Self {
        JSValue {
            raw: unsafe { sys::JSValueMakeNumber(ctx.raw, number) },
            ctx: ctx.raw,
        }
    }

    /// Creates a JavaScript value of the `string` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `string`: A value that can be converted into a [`JSString`] to assign
    ///   to the newly created `JSValue`. The newly created `JSValue` retains
    ///   `string`, and releases it upon garbage collection.
    ///
    /// Returns a `JSValue` of the `string` type, representing the value of `string`.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_string(&ctx, "abc");
    /// assert!(v.is_string());
    /// ```
    pub fn new_string<S: Into<JSString>>(ctx: &JSContext, string: S) -> Self {
        Self::new_string_inner(ctx.raw, string)
    }

    pub(crate) fn new_string_inner<S: Into<JSString>>(
        ctx: *const sys::OpaqueJSContext,
        string: S,
    ) -> Self {
        JSValue {
            raw: unsafe { sys::JSValueMakeString(ctx, string.into().raw) },
            ctx,
        }
    }

    /// Creates a JavaScript value of the `symbol` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `description`: A value that can be converted into a [`JSString`] to
    ///   assign to the newly created `JSValue`.
    ///
    /// Returns a `JSValue` of the `symbol` type, whose description matches the one provided.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_symbol(&ctx, "abc");
    /// assert!(v.is_symbol());
    /// ```
    pub fn new_symbol<S: Into<JSString>>(ctx: &JSContext, description: S) -> Self {
        JSValue {
            raw: unsafe { sys::JSValueMakeSymbol(ctx.raw, description.into().raw) },
            ctx: ctx.raw,
        }
    }

    /// Creates a JavaScript value from a JSON formatted string.
    ///
    /// * `ctx`: The execution context to use.
    /// * `string`: A value that can be converted into a [`JSString`] containing
    ///   the JSON string to be parsed.
    ///
    /// Returns an `Option` with the `JSValue` containing the parsed value, or `None`
    /// if the input is invalid.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_from_json(&ctx, "true").expect("value");
    /// assert!(v.is_boolean());
    /// ```
    pub fn new_from_json<S: Into<JSString>>(ctx: &JSContext, string: S) -> Option<Self> {
        let v = unsafe { sys::JSValueMakeFromJSONString(ctx.raw, string.into().raw) };
        if v.is_null() {
            None
        } else {
            Some(JSValue {
                raw: v,
                ctx: ctx.raw,
            })
        }
    }

    /// Creates a JavaScript string containing the JSON serialized representation of a JS value.
    ///
    /// * `indent`: The number of spaces to indent when nesting.
    ///   If `0`, the resulting JSON will not contains newlines.
    ///   The size of the indent is clamped to `10` spaces.
    ///
    /// Returns either a [`JSString`] with the result of serialization, or an
    /// exception if one was thrown.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_boolean(&ctx, false);
    /// let s = v.to_json_string(0).unwrap();
    /// assert_eq!(s, "false");
    /// ```
    pub fn to_json_string(&self, indent: u32) -> Result<JSString, JSException> {
        let mut e: sys::JSValueRef = ptr::null_mut();
        let v = unsafe { sys::JSValueCreateJSONString(self.ctx, self.raw, indent, &mut e) };
        if v.is_null() {
            Err(JSException {
                value: JSValue {
                    raw: e,
                    ctx: self.ctx,
                },
            })
        } else {
            Ok(JSString { raw: v })
        }
    }

    /// Returns a JavaScript value's type.
    ///
    /// Returns a value of type `JSType` that identifies `value`'s type.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_from_json(&ctx, "true").expect("value");
    /// assert_eq!(v.get_type(), JSType::Boolean);
    ///
    /// let v = JSValue::new_from_json(&ctx, "5.0").expect("value");
    /// assert_eq!(v.get_type(), JSType::Number);
    ///
    /// let v = JSValue::new_from_json(&ctx, "null").expect("value");
    /// assert_eq!(v.get_type(), JSType::Null);
    /// ```
    pub fn get_type(&self) -> JSType {
        unsafe { sys::JSValueGetType(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `undefined` type.
    ///
    /// Returns `true` if `value`'s type is the `undefined` type, otherwise `false`.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_undefined(&ctx);
    /// assert!(v.is_undefined());
    /// ```
    pub fn is_undefined(&self) -> bool {
        unsafe { sys::JSValueIsUndefined(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `null` type.
    ///
    /// Returns `true` if `value`'s type is the `null` type, otherwise `false`.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_from_json(&ctx, "null").expect("value");
    /// assert!(v.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        unsafe { sys::JSValueIsNull(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `boolean` type.
    ///
    /// Returns `true` if `value`'s type is the `boolean` type, otherwise `false`.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_from_json(&ctx, "false").expect("value");
    /// assert!(v.is_boolean());
    /// ```
    pub fn is_boolean(&self) -> bool {
        unsafe { sys::JSValueIsBoolean(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `number` type.
    ///
    /// Returns `true` if `value`'s type is the `number` type, otherwise `false`.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_from_json(&ctx, "-23").expect("value");
    /// assert!(v.is_number());
    /// ```
    pub fn is_number(&self) -> bool {
        unsafe { sys::JSValueIsNumber(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `string` type.
    ///
    /// Returns `true` if `value`'s type is the `string` type, otherwise `false`.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_from_json(&ctx, "\"Pueri et puellae\"").expect("value");
    /// assert!(v.is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        unsafe { sys::JSValueIsString(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `symbol` type.
    ///
    /// Returns `true` if `value`'s type is the `symbol` type, otherwise `false`.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_symbol(&ctx, "abc");
    /// assert!(v.is_symbol());
    /// ```
    pub fn is_symbol(&self) -> bool {
        unsafe { sys::JSValueIsSymbol(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `object` type.
    ///
    /// Returns `true` if `value`'s type is the `object` type, otherwise `false`.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("valid object");
    /// assert!(v.is_object());
    /// ```
    pub fn is_object(&self) -> bool {
        unsafe { sys::JSValueIsObject(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value is an `object` with a given class in its class chain.
    ///
    /// * `js_class`: The `JSClass` to test against.
    ///
    /// Returns `true` if `value` is an `object` and has `jsClass` in its
    /// class chain, otherwise `false`.
    pub fn is_object_of_class(&self, js_class: &JSClass) -> bool {
        unsafe { sys::JSValueIsObjectOfClass(self.ctx, self.raw, js_class.raw) }
    }

    /// Tests whether a JavaScript value is an `array`.
    ///
    /// Returns `true` if `value` is an `array`, otherwise `false`.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_from_json(&ctx, "[123, 456]").expect("valid array");
    /// assert!(v.is_array());
    ///
    /// // But an object is not an array.
    /// let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("valid object");
    /// assert!(!v.is_array());
    /// ```
    pub fn is_array(&self) -> bool {
        unsafe { sys::JSValueIsArray(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value is a `date`.
    ///
    /// Returns `true` if `value` is a `date`, otherwise `false`.
    pub fn is_date(&self) -> bool {
        unsafe { sys::JSValueIsDate(self.ctx, self.raw) }
    }

    /// Converts a JavaScript value to boolean and returns the resulting boolean.
    ///
    /// Returns the boolean result of conversion.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_boolean(&ctx, false);
    /// assert_eq!(v.as_boolean(), false);
    /// ```
    pub fn as_boolean(&self) -> bool {
        unsafe { sys::JSValueToBoolean(self.ctx, self.raw) }
    }

    /// Converts a JavaScript value to number and returns the resulting number.
    ///
    /// Returns either the numeric result of conversion, or an exception
    /// if one was thrown.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_number(&ctx, 5.0);
    /// let n = v.as_number().expect("valid number");
    /// assert_eq!(n, 5.0);
    /// ```
    pub fn as_number(&self) -> Result<f64, JSException> {
        let mut e: sys::JSValueRef = ptr::null_mut();
        let f = unsafe { sys::JSValueToNumber(self.ctx, self.raw, &mut e) };
        if f.is_nan() {
            Err(JSException {
                value: JSValue {
                    raw: e,
                    ctx: self.ctx,
                },
            })
        } else {
            Ok(f)
        }
    }

    /// Converts a JavaScript value to string and copies the result into a JavaScript string.
    ///
    /// Returns either [`JSString`] with the result of conversion, or an
    /// exception if one was thrown.  Ownership follows the Create Rule.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_string(&ctx, "Cave canem.");
    /// let s = v.as_string().expect("valid string");
    /// assert_eq!(s, "Cave canem.");
    /// ```
    pub fn as_string(&self) -> Result<JSString, JSException> {
        let mut e: sys::JSValueRef = ptr::null_mut();
        let s = unsafe { sys::JSValueToStringCopy(self.ctx, self.raw, &mut e) };
        if s.is_null() {
            Err(JSException {
                value: JSValue {
                    raw: e,
                    ctx: self.ctx,
                },
            })
        } else {
            Ok(JSString { raw: s })
        }
    }

    /// Converts a JavaScript value to object and returns the resulting object.
    ///
    /// Returns either the `JSObject` result of conversion, or an exception
    /// if one was thrown.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("valid object");
    /// let o = v.as_object().expect("object");
    /// // We now have an object that we can inspect.
    /// ```
    pub fn as_object(&self) -> Result<JSObject, JSException> {
        let mut e: sys::JSValueRef = ptr::null_mut();
        let o = unsafe { sys::JSValueToObject(self.ctx, self.raw, &mut e) };
        if o.is_null() {
            Err(JSException {
                value: JSValue {
                    raw: e,
                    ctx: self.ctx,
                },
            })
        } else {
            Ok(JSObject {
                raw: o,
                value: JSValue {
                    raw: self.raw,
                    ctx: self.ctx,
                },
            })
        }
    }

    /// Protects a JavaScript value from garbage collection.
    ///
    /// Use this method when you want to store a [`JSValue`] in a
    /// global or on the heap, where the garbage collector will
    /// not be able to discover your reference to it.
    ///
    /// A value may be protected multiple times and must be
    /// [unprotected] an equal number of times before becoming
    /// eligible for garbage collection.
    ///
    /// See also:
    ///
    /// * [`garbage_collect()`]
    /// * [`JSValue::unprotect()`]
    ///
    /// [`garbage_collect()`]: crate::garbage_collect
    /// [unprotected]: crate::JSValue::unprotect
    pub fn protect(&self) {
        unsafe { sys::JSValueProtect(self.ctx, self.raw) };
    }

    /// Unprotects a JavaScript value from garbage collection.
    ///
    /// A value may be [protected] multiple times and must be unprotected
    /// an equal number of times before becoming eligible for garbage
    /// collection.
    ///
    /// See also:
    ///
    /// * [`garbage_collect()`]
    /// * [`JSValue::protect()`]
    ///
    /// [`garbage_collect()`]: crate::garbage_collect
    /// [protected]: crate::JSValue::protect
    pub fn unprotect(&self) {
        unsafe { sys::JSValueUnprotect(self.ctx, self.raw) };
    }
}

/// Implement partial equality checks for `JSValue`.
///
/// These are performed in the same manner as `===` (strict
/// equality) in JavaScript.
impl PartialEq for JSValue {
    fn eq(&self, other: &JSValue) -> bool {
        unsafe { sys::JSValueIsStrictEqual(self.ctx, self.raw, other.raw) }
    }
}

#[cfg(test)]
mod tests {
    use super::{JSContext, JSType, JSValue};

    #[test]
    fn strict_equality() {
        let ctx = JSContext::default();

        let m = JSValue::new_number(&ctx, 30.4);
        let n = JSValue::new_number(&ctx, 30.4);
        assert_eq!(m, n);

        let t = JSValue::new_boolean(&ctx, true);
        let f = JSValue::new_boolean(&ctx, false);
        let g = JSValue::new_boolean(&ctx, false);
        assert_eq!(f, g);
        assert_ne!(t, f);
    }

    #[test]
    fn undefined() {
        let ctx = JSContext::default();
        let vu = JSValue::new_undefined(&ctx);
        assert!(vu.is_undefined());
        assert!(!vu.is_null());
        assert_eq!(vu.get_type(), JSType::Undefined);
        assert!(!vu.as_boolean());
        assert_eq!(vu.as_string().unwrap(), "undefined");
    }

    #[test]
    fn null() {
        let ctx = JSContext::default();
        let vn = JSValue::new_null(&ctx);
        assert!(vn.is_null());
        assert!(!vn.is_undefined());
        assert_eq!(vn.get_type(), JSType::Null);
        assert!(!vn.as_boolean());
        assert_eq!(vn.as_string().unwrap(), "null");
    }

    #[test]
    fn boolean() {
        let ctx = JSContext::default();
        let vt = JSValue::new_boolean(&ctx, true);
        assert!(vt.is_boolean());
        assert!(!vt.is_null());
        assert_eq!(vt.get_type(), JSType::Boolean);
        assert!(vt.as_boolean());
        assert_eq!(vt.as_number().unwrap(), 1.0);
        assert_eq!(vt.as_string().unwrap(), "true");

        let vf = JSValue::new_boolean(&ctx, false);
        assert!(vf.is_boolean());
        assert!(!vf.is_null());
        assert_eq!(vf.get_type(), JSType::Boolean);
        assert!(!vf.as_boolean());
        assert_eq!(vf.as_number().unwrap(), 0.0);
        assert_eq!(vf.as_string().unwrap(), "false");
    }

    #[test]
    fn number() {
        let ctx = JSContext::default();
        let vn = JSValue::new_number(&ctx, 30.4);
        assert!(vn.is_number());
        assert!(!vn.is_null());
        assert_eq!(vn.get_type(), JSType::Number);
        assert!(vn.as_boolean());
        assert_eq!(vn.as_number().unwrap(), 30.4);
        assert_eq!(vn.as_string().unwrap(), "30.4");
    }

    #[test]
    fn string() {
        let ctx = JSContext::default();
        let vs = JSValue::new_string(&ctx, "abc");
        assert!(vs.is_string());
        assert!(!vs.is_null());
        assert_eq!(vs.get_type(), JSType::String);
        assert!(vs.as_boolean());
        assert!(vs.as_number().is_err());
        assert_eq!(vs.as_string().unwrap(), "abc");
    }

    #[test]
    fn json_boolean_true() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "true").expect("value");
        assert!(v.is_boolean());
        assert!(v.as_boolean());
        assert_eq!(v.as_number().unwrap(), 1.0);
        let s = v.to_json_string(0).unwrap();
        assert_eq!(s, "true");
    }

    #[test]
    fn json_boolean_false() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "false").expect("value");
        assert!(v.is_boolean());
        assert!(!v.as_boolean());
        assert_eq!(v.as_number().unwrap(), 0.0);
        let s = v.to_json_string(0).unwrap();
        assert_eq!(s, "false");
    }

    #[test]
    fn json_number_0() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "0").expect("value");
        assert!(v.is_number());
        assert!(!v.as_boolean());
        assert_eq!(v.as_number().unwrap(), 0.0);
        let s = v.to_json_string(0).unwrap();
        assert_eq!(s, "0");
    }

    #[test]
    fn json_number_3() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "3").expect("value");
        assert!(v.is_number());
        assert!(v.as_boolean());
        assert_eq!(v.as_number().unwrap(), 3.0);
        let s = v.to_json_string(0).unwrap();
        assert_eq!(s, "3");
    }

    #[test]
    fn json_string() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "\"abc\"").expect("value");
        assert!(v.is_string());
        assert!(v.as_boolean());
        assert!(v.as_number().is_err());
        let s = v.to_json_string(0).unwrap();
        assert_eq!(s, "\"abc\"");
    }

    #[test]
    fn json_string_number() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "\"3\"").expect("value");
        assert!(v.is_string());
        assert!(v.as_boolean());
        assert_eq!(v.as_number().unwrap(), 3.0);
        let s = v.to_json_string(0).unwrap();
        assert_eq!(s, "\"3\"");
    }

    #[test]
    fn json_failure() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "3 +");
        assert!(v.is_none());
    }
}
