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
    pub fn new_number(ctx: &JSContext, number: f64) -> Self {
        JSValue {
            raw: unsafe { sys::JSValueMakeNumber(ctx.raw, number) },
            ctx: ctx.raw,
        }
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
        JSValue {
            raw: unsafe { sys::JSValueMakeString(ctx.raw, string.into().raw) },
            ctx: ctx.raw,
        }
    }

    /// Creates a JavaScript value from a JSON formatted string.
    ///
    /// * `ctx`: The execution context to use.
    /// * `string`: A value that can be converted into a `JSString` containing
    ///   the JSON string to be parsed.
    ///
    /// Returns a `Result` with the `JSValue` containing the parsed value, or an error
    /// if the input is invalid.
    pub fn new_from_json<S: Into<JSString>>(ctx: &JSContext, string: S) -> Result<Self, ()> {
        let v = unsafe { sys::JSValueMakeFromJSONString(ctx.raw, string.into().raw) };
        if v.is_null() {
            Err(())
        } else {
            Ok(JSValue {
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
    /// Returns either a `JSString` with the result of serialization, or an
    /// exception if one was thrown.
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
    pub fn get_type(&self) -> JSType {
        unsafe { sys::JSValueGetType(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `undefined` type.
    ///
    /// Returns `true` if `value`'s type is the `undefined` type, otherwise `false`.
    pub fn is_undefined(&self) -> bool {
        unsafe { sys::JSValueIsUndefined(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `null` type.
    ///
    /// Returns `true` if `value`'s type is the `null` type, otherwise `false`.
    pub fn is_null(&self) -> bool {
        unsafe { sys::JSValueIsNull(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `boolean` type.
    ///
    /// Returns `true` if `value`'s type is the `boolean` type, otherwise `false`.
    pub fn is_boolean(&self) -> bool {
        unsafe { sys::JSValueIsBoolean(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `number` type.
    ///
    /// Returns `true` if `value`'s type is the `number` type, otherwise `false`.
    pub fn is_number(&self) -> bool {
        unsafe { sys::JSValueIsNumber(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `string` type.
    ///
    /// Returns `true` if `value`'s type is the `string` type, otherwise `false`.
    pub fn is_string(&self) -> bool {
        unsafe { sys::JSValueIsString(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value's type is the `object` type.
    ///
    /// Returns `true` if `value`'s type is the `object` type, otherwise `false`.
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
    pub fn as_boolean(&self) -> bool {
        unsafe { sys::JSValueToBoolean(self.ctx, self.raw) }
    }

    /// Converts a JavaScript value to number and returns the resulting number.
    ///
    /// Returns either the numeric result of conversion, or an exception
    /// if one was thrown.
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
    /// Returns either `JSString` with the result of conversion, or an
    /// exception if one was thrown.  Ownership follows the Create Rule.
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
            Ok(JSObject { raw: o })
        }
    }
}

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
        assert_eq!(vu.is_undefined(), true);
        assert_eq!(vu.is_null(), false);
        assert_eq!(vu.get_type(), JSType::Undefined);
        assert_eq!(vu.as_boolean(), false);
        assert_eq!(vu.as_string().unwrap(), "undefined".into());
    }

    #[test]
    fn null() {
        let ctx = JSContext::default();
        let vn = JSValue::new_null(&ctx);
        assert_eq!(vn.is_null(), true);
        assert_eq!(vn.is_undefined(), false);
        assert_eq!(vn.get_type(), JSType::Null);
        assert_eq!(vn.as_boolean(), false);
        assert_eq!(vn.as_string().unwrap(), "null".into());
    }

    #[test]
    fn boolean() {
        let ctx = JSContext::default();
        let vt = JSValue::new_boolean(&ctx, true);
        assert_eq!(vt.is_boolean(), true);
        assert_eq!(vt.is_null(), false);
        assert_eq!(vt.get_type(), JSType::Boolean);
        assert_eq!(vt.as_boolean(), true);
        assert_eq!(vt.as_number().unwrap(), 1.0);
        assert_eq!(vt.as_string().unwrap(), "true".into());

        let vf = JSValue::new_boolean(&ctx, false);
        assert_eq!(vf.is_boolean(), true);
        assert_eq!(vf.is_null(), false);
        assert_eq!(vf.get_type(), JSType::Boolean);
        assert_eq!(vf.as_boolean(), false);
        assert_eq!(vf.as_number().unwrap(), 0.0);
        assert_eq!(vf.as_string().unwrap(), "false".into());
    }

    #[test]
    fn number() {
        let ctx = JSContext::default();
        let vn = JSValue::new_number(&ctx, 30.4);
        assert_eq!(vn.is_number(), true);
        assert_eq!(vn.is_null(), false);
        assert_eq!(vn.get_type(), JSType::Number);
        assert_eq!(vn.as_boolean(), true);
        assert_eq!(vn.as_number().unwrap(), 30.4);
        assert_eq!(vn.as_string().unwrap(), "30.4".into());
    }

    #[test]
    fn string() {
        let ctx = JSContext::default();
        let vs = JSValue::new_string(&ctx, "abc");
        assert_eq!(vs.is_string(), true);
        assert_eq!(vs.is_null(), false);
        assert_eq!(vs.get_type(), JSType::String);
        assert_eq!(vs.as_boolean(), true);
        assert!(vs.as_number().is_err());
        assert_eq!(vs.as_string().unwrap(), "abc".into());
    }

    #[test]
    fn json_boolean_true() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "true").expect("value");
        assert_eq!(v.is_boolean(), true);
        assert_eq!(v.as_boolean(), true);
        assert_eq!(v.as_number().unwrap(), 1.0);
        let s = v.to_json_string(0).unwrap();
        assert_eq!(s, "true".into());
    }

    #[test]
    fn json_boolean_false() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "false").expect("value");
        assert_eq!(v.is_boolean(), true);
        assert_eq!(v.as_boolean(), false);
        assert_eq!(v.as_number().unwrap(), 0.0);
        let s = v.to_json_string(0).unwrap();
        assert_eq!(s, "false".into());
    }

    #[test]
    fn json_number_0() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "0").expect("value");
        assert_eq!(v.is_number(), true);
        assert_eq!(v.as_boolean(), false);
        assert_eq!(v.as_number().unwrap(), 0.0);
        let s = v.to_json_string(0).unwrap();
        assert_eq!(s, "0".into());
    }

    #[test]
    fn json_number_3() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "3").expect("value");
        assert_eq!(v.is_number(), true);
        assert_eq!(v.as_boolean(), true);
        assert_eq!(v.as_number().unwrap(), 3.0);
        let s = v.to_json_string(0).unwrap();
        assert_eq!(s, "3".into());
    }

    #[test]
    fn json_string() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "\"abc\"").expect("value");
        assert_eq!(v.is_string(), true);
        assert_eq!(v.as_boolean(), true);
        assert!(v.as_number().is_err());
        let s = v.to_json_string(0).unwrap();
        assert_eq!(s, "\"abc\"".into());
    }

    #[test]
    fn json_string_number() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "\"3\"").expect("value");
        assert_eq!(v.is_string(), true);
        assert_eq!(v.as_boolean(), true);
        assert_eq!(v.as_number().unwrap(), 3.0);
        let s = v.to_json_string(0).unwrap();
        assert_eq!(s, "\"3\"".into());
    }

    #[test]
    fn json_failure() {
        let ctx = JSContext::default();

        let v = JSValue::new_from_json(&ctx, "3 +");
        assert!(v.is_err());
    }
}
