// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;
use super::{JSClass, JSContext, JSException, JSObject, JSString, JSType, JSValue};
use sys;

impl JSValue {
    /// Creates a JavaScript value of the `undefined` type.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns the unique `undefined` value.
    pub fn new_undefined(ctx: &JSContext) -> Self {
        JSValue { raw: unsafe { sys::JSValueMakeUndefined(ctx.raw) } }
    }

    /// Creates a JavaScript value of the `null` type.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns the unique `null` value.
    pub fn new_null(ctx: &JSContext) -> Self {
        JSValue { raw: unsafe { sys::JSValueMakeNull(ctx.raw) } }
    }

    /// Creates a JavaScript value of the `boolean` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `boolean`: The `bool` to assign to the newly created `JSValue`.
    ///
    /// Returns a `JSValue` of the `boolean` type, representing the value of `boolean`.
    pub fn new_boolean(ctx: &JSContext, boolean: bool) -> Self {
        JSValue { raw: unsafe { sys::JSValueMakeBoolean(ctx.raw, boolean) } }
    }

    /// Creates a JavaScript value of the `number` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `number`: The `f64` to assign to the newly created `JSValue`.
    ///
    /// Returns a `JSValue` of the `number` type, representing the value of `number`.
    pub fn new_number(ctx: &JSContext, number: f64) -> Self {
        JSValue { raw: unsafe { sys::JSValueMakeNumber(ctx.raw, number) } }
    }

    /// Creates a JavaScript value of the string type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `string`: The `JSString` to assign to the newly created
    ///   JSValue`. The newly created `JSValue` retains string, and
    ///   releases it upon garbage collection.
    ///
    /// Returns a `JSValue` of the `string` type, representing the value of `string`.
    pub fn new_string<S: Into<JSString>>(ctx: &JSContext, string: S) -> Self {
        JSValue { raw: unsafe { sys::JSValueMakeString(ctx.raw, string.into().raw) } }
    }

    /// Creates a JavaScript value from a JSON formatted string.
    ///
    /// * `ctx`: The execution context to use.
    /// * `string`: The `JSString` containing the JSON string to be parsed.
    ///
    /// Returns a `JSValue` containing the parsed value, or `NULL` if the input is invalid.
    pub fn new_from_json<S: Into<JSString>>(ctx: &JSContext, string: S) -> Self {
        JSValue { raw: unsafe { sys::JSValueMakeFromJSONString(ctx.raw, string.into().raw) } }
    }

    /// Creates a JavaScript string containing the JSON serialized representation of a JS value.
    ///
    /// * `ctx`: The execution context to use.
    /// * `indent`: The number of spaces to indent when nesting.
    ///   If `0`, the resulting JSON will not contains newlines.
    ///   The size of the indent is clamped to `10` spaces.
    ///
    /// Returns either a `JSString` with the result of serialization, or an
    /// exception if one was thrown.
    pub fn to_json_string(&self, ctx: &JSContext, indent: u32) -> Result<JSString, JSException> {
        let mut e: sys::JSValueRef = ptr::null_mut();
        let v = unsafe { sys::JSValueCreateJSONString(ctx.raw, self.raw, indent, &mut e) };
        if v.is_null() {
            Err(JSException { value: JSValue { raw: e } })
        } else {
            Ok(JSString { raw: v })
        }
    }

    /// Returns a JavaScript value's type.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns a value of type `JSType` that identifies `value`'s type.
    pub fn get_type(&self, ctx: &JSContext) -> JSType {
        unsafe { sys::JSValueGetType(ctx.raw, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `undefined` type.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns `true` if `value`'s type is the `undefined` type, otherwise `false`.
    pub fn is_undefined(&self, ctx: &JSContext) -> bool {
        unsafe { sys::JSValueIsUndefined(ctx.raw, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `null` type.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns `true` if `value`'s type is the `null` type, otherwise `false`.
    pub fn is_null(&self, ctx: &JSContext) -> bool {
        unsafe { sys::JSValueIsNull(ctx.raw, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `boolean` type.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns `true` if `value`'s type is the `boolean` type, otherwise `false`.
    pub fn is_boolean(&self, ctx: &JSContext) -> bool {
        unsafe { sys::JSValueIsBoolean(ctx.raw, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `number` type.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns `true` if `value`'s type is the `number` type, otherwise `false`.
    pub fn is_number(&self, ctx: &JSContext) -> bool {
        unsafe { sys::JSValueIsNumber(ctx.raw, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `string` type.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns `true` if `value`'s type is the `string` type, otherwise `false`.
    pub fn is_string(&self, ctx: &JSContext) -> bool {
        unsafe { sys::JSValueIsString(ctx.raw, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `object` type.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns `true` if `value`'s type is the `object` type, otherwise `false`.
    pub fn is_object(&self, ctx: &JSContext) -> bool {
        unsafe { sys::JSValueIsObject(ctx.raw, self.raw) }
    }

    /// Tests whether a JavaScript value is an `object` with a given class in its class chain.
    ///
    /// * `ctx`: The execution context to use.
    /// * `js_class`: The `JSClass` to test against.
    ///
    /// Returns `true` if `value` is an `object` and has `jsClass` in its
    /// class chain, otherwise `false`.
    pub fn is_object_of_class(&self, ctx: &JSContext, js_class: &JSClass) -> bool {
        unsafe { sys::JSValueIsObjectOfClass(ctx.raw, self.raw, js_class.raw) }
    }

    /// Tests whether a JavaScript value is an `array`.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns `true` if `value` is an `array`, otherwise `false`.
    pub fn is_array(&self, ctx: &JSContext) -> bool {
        unsafe { sys::JSValueIsArray(ctx.raw, self.raw) }
    }

    /// Tests whether a JavaScript value is a `date`.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns `true` if `value` is a `date`, otherwise `false`.
    pub fn is_date(&self, ctx: &JSContext) -> bool {
        unsafe { sys::JSValueIsDate(ctx.raw, self.raw) }
    }

    /// Converts a JavaScript value to boolean and returns the resulting boolean.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns the boolean result of conversion.
    pub fn as_boolean(&self, ctx: &JSContext) -> bool {
        unsafe { sys::JSValueToBoolean(ctx.raw, self.raw) }
    }

    /// Converts a JavaScript value to number and returns the resulting number.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns either the numeric result of conversion, or an exception
    /// if one was thrown.
    pub fn as_number(&self, ctx: &JSContext) -> Result<f64, JSException> {
        let mut e: sys::JSValueRef = ptr::null_mut();
        let f = unsafe { sys::JSValueToNumber(ctx.raw, self.raw, &mut e) };
        if f.is_nan() {
            Err(JSException { value: JSValue { raw: e } })
        } else {
            Ok(f)
        }
    }

    /// Converts a JavaScript value to string and copies the result into a JavaScript string.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns either `JSString` with the result of conversion, or an
    /// exception if one was thrown.  Ownership follows the Create Rule.
    pub fn as_string(&self, ctx: &JSContext) -> Result<JSString, JSException> {
        let mut e: sys::JSValueRef = ptr::null_mut();
        let s = unsafe { sys::JSValueToStringCopy(ctx.raw, self.raw, &mut e) };
        if s.is_null() {
            Err(JSException { value: JSValue { raw: e } })
        } else {
            Ok(JSString { raw: s })
        }
    }

    /// Converts a JavaScript value to object and returns the resulting object.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns either the `JSObject` result of conversion, or an exception
    /// if one was thrown.
    pub fn as_object(&self, ctx: &JSContext) -> Result<JSObject, JSException> {
        let mut e: sys::JSValueRef = ptr::null_mut();
        let o = unsafe { sys::JSValueToObject(ctx.raw, self.raw, &mut e) };
        if o.is_null() {
            Err(JSException { value: JSValue { raw: e } })
        } else {
            Ok(JSObject { raw: o })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{JSContext, JSType, JSValue};

    #[test]
    fn undefined() {
        let ctx = JSContext::default();
        let vu = JSValue::new_undefined(&ctx);
        assert_eq!(vu.is_undefined(&ctx), true);
        assert_eq!(vu.is_null(&ctx), false);
        assert_eq!(vu.get_type(&ctx), JSType::kJSTypeUndefined);
        assert_eq!(vu.as_boolean(&ctx), false);
        assert_eq!(vu.as_string(&ctx).unwrap(), "undefined".into());
    }

    #[test]
    fn null() {
        let ctx = JSContext::default();
        let vn = JSValue::new_null(&ctx);
        assert_eq!(vn.is_null(&ctx), true);
        assert_eq!(vn.is_undefined(&ctx), false);
        assert_eq!(vn.get_type(&ctx), JSType::kJSTypeNull);
        assert_eq!(vn.as_boolean(&ctx), false);
        assert_eq!(vn.as_string(&ctx).unwrap(), "null".into());
    }

    #[test]
    fn boolean() {
        let ctx = JSContext::default();
        let vt = JSValue::new_boolean(&ctx, true);
        assert_eq!(vt.is_boolean(&ctx), true);
        assert_eq!(vt.is_null(&ctx), false);
        assert_eq!(vt.get_type(&ctx), JSType::kJSTypeBoolean);
        assert_eq!(vt.as_boolean(&ctx), true);
        assert_eq!(vt.as_number(&ctx).unwrap(), 1.0);
        assert_eq!(vt.as_string(&ctx).unwrap(), "true".into());

        let vf = JSValue::new_boolean(&ctx, false);
        assert_eq!(vf.is_boolean(&ctx), true);
        assert_eq!(vf.is_null(&ctx), false);
        assert_eq!(vf.get_type(&ctx), JSType::kJSTypeBoolean);
        assert_eq!(vf.as_boolean(&ctx), false);
        assert_eq!(vf.as_number(&ctx).unwrap(), 0.0);
        assert_eq!(vf.as_string(&ctx).unwrap(), "false".into());
    }

    #[test]
    fn number() {
        let ctx = JSContext::default();
        let vn = JSValue::new_number(&ctx, 30.4);
        assert_eq!(vn.is_number(&ctx), true);
        assert_eq!(vn.is_null(&ctx), false);
        assert_eq!(vn.get_type(&ctx), JSType::kJSTypeNumber);
        assert_eq!(vn.as_boolean(&ctx), true);
        assert_eq!(vn.as_number(&ctx).unwrap(), 30.4);
        assert_eq!(vn.as_string(&ctx).unwrap(), "30.4".into());
    }

    #[test]
    fn string() {
        let ctx = JSContext::default();
        let vs = JSValue::new_string(&ctx, "abc");
        assert_eq!(vs.is_string(&ctx), true);
        assert_eq!(vs.is_null(&ctx), false);
        assert_eq!(vs.get_type(&ctx), JSType::kJSTypeString);
        assert_eq!(vs.as_boolean(&ctx), true);
        assert!(vs.as_number(&ctx).is_err());
        assert_eq!(vs.as_string(&ctx).unwrap(), "abc".into());
    }

    #[test]
    fn json_boolean_true() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "true");
        assert_eq!(v.is_boolean(&ctx), true);
        assert_eq!(v.as_boolean(&ctx), true);
        assert_eq!(v.as_number(&ctx).unwrap(), 1.0);
        let s = v.to_json_string(&ctx, 0).unwrap();
        assert_eq!(s, "true".into());
    }

    #[test]
    fn json_boolean_false() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "false");
        assert_eq!(v.is_boolean(&ctx), true);
        assert_eq!(v.as_boolean(&ctx), false);
        assert_eq!(v.as_number(&ctx).unwrap(), 0.0);
        let s = v.to_json_string(&ctx, 0).unwrap();
        assert_eq!(s, "false".into());
    }

    #[test]
    fn json_number_0() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "0");
        assert_eq!(v.is_number(&ctx), true);
        assert_eq!(v.as_boolean(&ctx), false);
        assert_eq!(v.as_number(&ctx).unwrap(), 0.0);
        let s = v.to_json_string(&ctx, 0).unwrap();
        assert_eq!(s, "0".into());
    }

    #[test]
    fn json_number_3() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "3");
        assert_eq!(v.is_number(&ctx), true);
        assert_eq!(v.as_boolean(&ctx), true);
        assert_eq!(v.as_number(&ctx).unwrap(), 3.0);
        let s = v.to_json_string(&ctx, 0).unwrap();
        assert_eq!(s, "3".into());
    }

    #[test]
    fn json_string() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "\"abc\"");
        assert_eq!(v.is_string(&ctx), true);
        assert_eq!(v.as_boolean(&ctx), true);
        assert!(v.as_number(&ctx).is_err());
        let s = v.to_json_string(&ctx, 0).unwrap();
        assert_eq!(s, "\"abc\"".into());
    }

    #[test]
    fn json_string_number() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "\"3\"");
        assert_eq!(v.is_string(&ctx), true);
        assert_eq!(v.as_boolean(&ctx), true);
        assert_eq!(v.as_number(&ctx).unwrap(), 3.0);
        let s = v.to_json_string(&ctx, 0).unwrap();
        assert_eq!(s, "\"3\"".into());
    }
}
