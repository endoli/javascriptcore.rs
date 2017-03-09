// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{JSClass, JSContext, JSType, JSValue};
use sys;

impl JSValue {
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
}
