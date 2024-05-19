// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys::JSObjectCallAsFunctionCallback;

use crate::{
    sys, JSClass, JSContext, JSException, JSObject, JSString, JSType, JSTypedArray,
    JSTypedArrayType, JSValue,
};
use std::ptr;

impl JSValue {
    /// Create a new [`Self`] from its raw pointer directly.
    ///
    /// # Safety
    ///
    /// Ensure `raw` is valid.
    pub const unsafe fn from_raw(ctx: sys::JSContextRef, raw: sys::JSValueRef) -> Self {
        Self { raw, ctx }
    }

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
    ///
    /// # See also
    ///
    /// - [`JSValue::is_undefined()`]
    pub fn new_undefined(ctx: &JSContext) -> Self {
        unsafe { Self::from_raw(ctx.raw, sys::JSValueMakeUndefined(ctx.raw)) }
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
    ///
    /// # See also
    ///
    /// - [`JSValue::is_null()`]
    pub fn new_null(ctx: &JSContext) -> Self {
        unsafe { Self::from_raw(ctx.raw, sys::JSValueMakeNull(ctx.raw)) }
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
    ///
    /// # See also
    ///
    /// - [`JSValue::is_boolean()`]
    /// - [`JSValue::new_boolean()`]
    pub fn new_boolean(ctx: &JSContext, boolean: bool) -> Self {
        unsafe { Self::from_raw(ctx.raw, sys::JSValueMakeBoolean(ctx.raw, boolean)) }
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
    ///
    /// # See also
    ///
    /// - [`JSValue::is_number()`]
    /// - [`JSValue::new_number()`]
    pub fn new_number(ctx: &JSContext, number: f64) -> Self {
        unsafe { Self::from_raw(ctx.raw, sys::JSValueMakeNumber(ctx.raw, number)) }
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
    ///
    /// # See also
    ///
    /// - [`JSValue::is_string()`]
    /// - [`JSValue::new_string()`]
    pub fn new_string<S: Into<JSString>>(ctx: &JSContext, string: S) -> Self {
        Self::new_string_inner(ctx.raw, string)
    }

    pub(crate) fn new_string_inner<S: Into<JSString>>(
        ctx: *const sys::OpaqueJSContext,
        string: S,
    ) -> Self {
        unsafe { Self::from_raw(ctx, sys::JSValueMakeString(ctx, string.into().raw)) }
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
    ///
    /// # See also
    ///
    /// - [`JSValue::is_symbol()`]
    pub fn new_symbol<S: Into<JSString>>(ctx: &JSContext, description: S) -> Self {
        unsafe {
            Self::from_raw(
                ctx.raw,
                sys::JSValueMakeSymbol(ctx.raw, description.into().raw),
            )
        }
    }

    /// Creates a JavaScript value of the `array` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `items`: The array items as [`JSValue`]s.
    ///
    /// Returns a `JSValue` of the `array` type, otherwise an [exception](JSException).
    ///
    /// ```
    /// # use javascriptcore::{JSContext, JSValue};
    /// let ctx = JSContext::default();
    /// let value = JSValue::new_array(
    ///     &ctx,
    ///     &[JSValue::new_number(&ctx, 1.), JSValue::new_number(&ctx, 2.)]
    /// ).unwrap();
    /// assert!(value.is_array());
    /// ```
    ///
    /// # See also
    ///
    /// - [`JSValue::is_array()`]
    pub fn new_array(ctx: &JSContext, items: &[JSValue]) -> Result<Self, JSException> {
        let items = items
            .iter()
            .map(|argument| argument.raw)
            .collect::<Vec<_>>();
        let mut exception: sys::JSValueRef = ptr::null_mut();

        let result = unsafe {
            sys::JSObjectMakeArray(
                ctx.raw,
                items.len(),
                items.as_slice().as_ptr(),
                &mut exception,
            )
        };

        if !exception.is_null() {
            return Err(unsafe { Self::from_raw(ctx.raw, exception).into() });
        }

        if result.is_null() {
            return Err(Self::new_string(ctx, "Failed to make a new array").into());
        }

        Ok(unsafe { Self::from_raw(ctx.raw, result) })
    }

    /// Creates a JavaScript value of the `TypedArray` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `bytes`: The typed array bytes. The constructed `TypedArray` doesn't copy the bytes,
    ///   thus this method takes a `&mut` reference as it is possible to mutate the bytes via
    ///   `TypedArray` or via Rust.
    ///
    /// Returns a `JSValue` of the `TypedArray` type, otherwise an [exception](JSException).
    ///
    /// # Safety
    ///
    /// `bytes` can be mutated both by Rust or JavaScript. There is no lock, no mutex, no
    /// guard. Be extremely careful when using this API. `bytes` aren't copied, they are
    /// borrowed mutably by JavaScript. Dropping the value in Rust will clear them in
    /// JavaScript, and vice versa. Hence, this method is marked as `unsafe`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use javascriptcore::{JSContext, JSValue};
    /// let ctx = JSContext::default();
    /// let mut bytes = vec![1u8, 2, 3, 4, 5];
    /// let value = unsafe {
    ///     JSValue::new_typed_array_with_bytes(&ctx, bytes.as_mut_slice())
    ///         .unwrap()
    /// };
    /// ```
    ///
    /// # See also
    ///
    /// - [`JSValue::as_typed_array()`]
    /// - [`JSValue::is_typed_array()`]
    pub unsafe fn new_typed_array_with_bytes(
        ctx: &JSContext,
        // `&mut` instead of &` because the typed array borrows mutably the bytes.
        //
        // The argument is named `_bytes` instead of `bytes` to avoid a
        // `clippy::needless_pass_by_ref_mut` warning (only on Rust nightly).
        _bytes: &mut [u8],
    ) -> Result<Self, JSException> {
        let bytes = _bytes;
        let deallocator_ctx = ptr::null_mut();
        let mut exception: sys::JSValueRef = ptr::null_mut();

        let result = unsafe {
            sys::JSObjectMakeTypedArrayWithBytesNoCopy(
                ctx.raw,
                JSTypedArrayType::Uint8Array,
                bytes.as_ptr() as _,
                bytes.len(),
                None,
                deallocator_ctx,
                &mut exception,
            )
        };

        if !exception.is_null() {
            return Err(Self::from_raw(ctx.raw, exception).into());
        }

        if result.is_null() {
            return Err(Self::new_string(ctx, "Failed to make a new typed array").into());
        }

        Ok(Self::from_raw(ctx.raw, result))
    }

    /// Creates a JavaScript function where the function implementation is written in
    /// Rust.
    ///
    /// * `ctx`: The execution context to use.
    /// * `name`: The function name.
    /// * `function`: The function implementation.
    ///
    /// The easiest way to declare such a function is to use the
    /// [`crate::function_callback`] procedural macro:
    ///
    /// ```rust
    /// use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// #[function_callback]
    /// fn greet(
    ///     ctx: &JSContext,
    ///     _function: Option<&JSObject>,
    ///     _this_object: Option<&JSObject>,
    ///     arguments: &[JSValue],
    /// ) -> Result<JSValue, JSException> {
    ///     if arguments.len() != 1 {
    ///         return Err(JSValue::new_string(ctx, "must receive 1 argument").into());
    ///     }
    ///
    ///     let who = arguments[0].as_string()?;
    ///
    ///     Ok(JSValue::new_string(&ctx, format!("Hello, {who}!")))
    /// }
    ///
    /// let greet = JSValue::new_function(&ctx, "greet", Some(greet)).as_object().unwrap();
    ///
    /// let result = greet.call_as_function(
    ///     None,
    ///     &[JSValue::new_string(&ctx, "Gordon")],
    /// ).unwrap();
    ///
    /// assert_eq!(result.as_string().unwrap().to_string(), "Hello, Gordon!");
    /// ```
    pub fn new_function<N>(
        ctx: &JSContext,
        name: N,
        function: JSObjectCallAsFunctionCallback,
    ) -> Self
    where
        N: Into<JSString>,
    {
        unsafe {
            Self::from_raw(
                ctx.raw,
                sys::JSObjectMakeFunctionWithCallback(ctx.raw, name.into().raw, function),
            )
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
        let value = unsafe { sys::JSValueMakeFromJSONString(ctx.raw, string.into().raw) };

        if value.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw(ctx.raw, value) })
        }
    }

    /// Creates a JavaScript string containing the JSON serialized representation of a JS value.
    ///
    /// * `indent`: The number of spaces to indent when nesting.
    ///   If `0`, the resulting JSON will not contains newlines.
    ///   The size of the indent is clamped to `10` spaces.
    ///
    /// Returns either a [`JSString`] with the result of serialization, or an
    /// [exception](JSException) if one was thrown.
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
        let mut exception: sys::JSValueRef = ptr::null_mut();
        let value =
            unsafe { sys::JSValueCreateJSONString(self.ctx, self.raw, indent, &mut exception) };

        if value.is_null() {
            Err(unsafe { Self::from_raw(self.ctx, exception) }.into())
        } else {
            Ok(JSString { raw: value })
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
    ///
    /// # See also
    ///
    /// - [`JSValue::new_undefined()`]
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
    ///
    /// # See also
    ///
    /// - [`JSValue::new_null()`]
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
    ///
    /// # See also
    ///
    /// - [`JSValue::as_boolean()`]
    /// - [`JSValue::new_boolean()`]
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
    ///
    /// # See also
    ///
    /// - [`JSValue::as_number()`]
    /// - [`JSValue::new_number()`]
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
    ///
    /// # See also
    ///
    /// - [`JSValue::as_string()`]
    /// - [`JSValue::new_string()`]
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
    ///
    /// # See also
    ///
    /// - [`JSValue::new_symbol()`]
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
    ///
    /// # See also
    ///
    /// - [`JSValue::as_object()`]
    /// - [`JSValue::is_object_of_class()`]
    pub fn is_object(&self) -> bool {
        unsafe { sys::JSValueIsObject(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value is an `object` with a given class in its class chain.
    ///
    /// * `js_class`: The `JSClass` to test against.
    ///
    /// Returns `true` if `value` is an `object` and has `jsClass` in its
    /// class chain, otherwise `false`.
    ///
    /// # See also
    ///
    /// - [`JSValue::as_object()`]
    /// - [`JSValue::is_object()`]
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
    /// let v = JSValue::new_array(&ctx, &[JSValue::new_number(&ctx, 123.), JSValue::new_number(&ctx, 456.)]).unwrap();
    /// assert!(v.is_array());
    ///
    /// // But an object is not an array.
    /// let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("valid object");
    /// assert!(!v.is_array());
    /// ```
    pub fn is_array(&self) -> bool {
        unsafe { sys::JSValueIsArray(self.ctx, self.raw) }
    }

    /// Tests whether a JavaScript value is a Typed Array.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let value = JSValue::new_number(&ctx, 123.);
    /// assert!(!value.is_typed_array());
    ///
    /// let mut bytes = vec![1u8, 2, 3, 4, 5];
    /// let value = unsafe {
    ///     JSValue::new_typed_array_with_bytes(&ctx, bytes.as_mut_slice())
    ///         .unwrap()
    /// };
    /// assert!(value.is_typed_array());
    /// ```
    ///
    /// # See also
    ///
    /// - [`JSValue::as_typed_array()`]
    /// - [`JSValue::new_typed_array_with_bytes()`]
    pub fn is_typed_array(&self) -> bool {
        let mut exception: sys::JSValueRef = ptr::null_mut();
        let value = unsafe { sys::JSValueGetTypedArrayType(self.ctx, self.raw, &mut exception) };

        value != JSTypedArrayType::None
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
    ///
    /// # See also
    ///
    /// - [`JSValue::is_boolean()`]
    /// - [`JSValue::new_boolean()`]
    pub fn as_boolean(&self) -> bool {
        unsafe { sys::JSValueToBoolean(self.ctx, self.raw) }
    }

    /// Converts a JavaScript value to number and returns the resulting number.
    ///
    /// Returns either the numeric result of conversion, or an [exception](JSException)
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
    ///
    /// # See also
    ///
    /// - [`JSValue::is_number()`]
    /// - [`JSValue::new_number()`]
    pub fn as_number(&self) -> Result<f64, JSException> {
        let mut exception: sys::JSValueRef = ptr::null_mut();
        let number = unsafe { sys::JSValueToNumber(self.ctx, self.raw, &mut exception) };

        if number.is_nan() {
            Err(unsafe { Self::from_raw(self.ctx, exception) }.into())
        } else {
            Ok(number)
        }
    }

    /// Converts a JavaScript value to string and copies the result into a JavaScript string.
    ///
    /// Returns either [`JSString`] with the result of conversion, or an
    /// [exception](JSException) if one was thrown.  Ownership follows the Create Rule.
    ///
    /// ```
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let v = JSValue::new_string(&ctx, "Cave canem.");
    /// let s = v.as_string().expect("valid string");
    /// assert_eq!(s, "Cave canem.");
    /// ```
    ///
    /// # See also
    ///
    /// - [`JSValue::is_string()`]
    /// - [`JSValue::new_string()`]
    pub fn as_string(&self) -> Result<JSString, JSException> {
        let mut exception: sys::JSValueRef = ptr::null_mut();
        let string = unsafe { sys::JSValueToStringCopy(self.ctx, self.raw, &mut exception) };

        if string.is_null() {
            Err(unsafe { Self::from_raw(self.ctx, exception) }.into())
        } else {
            Ok(JSString { raw: string })
        }
    }

    /// Converts a JavaScript value to object and returns the resulting object.
    ///
    /// Returns either the `JSObject` result of conversion, or an [exception](JSException)
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
    ///
    /// # See also
    ///
    /// - [`JSValue::is_object()`]
    pub fn as_object(&self) -> Result<JSObject, JSException> {
        let mut exception: sys::JSValueRef = ptr::null_mut();
        let object = unsafe { sys::JSValueToObject(self.ctx, self.raw, &mut exception) };

        if object.is_null() {
            Err(unsafe { Self::from_raw(self.ctx, exception) }.into())
        } else {
            Ok(unsafe { JSObject::from_raw(self.ctx, object) })
        }
    }

    /// Converts a JavaScript value to a Typed Array object and returns the
    /// resulting object.
    ///
    /// Returns either the `JSTypedArray` result of conversion, or an [exception](JSException)
    /// if one was thrown.
    ///
    /// ```rust
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    /// let array = evaluate_script(
    ///     &ctx,
    ///     "new Uint8Array([1, 2, 3, 4, 5])",
    ///     None,
    ///     "foo.js",
    ///     1,
    /// ).unwrap();
    ///
    /// assert!(array.is_typed_array());
    ///
    /// let array = array.as_typed_array().unwrap();
    /// ````
    ///
    /// # See also
    ///
    /// - [`JSValue::is_typed_array()`]
    /// - [`JSValue::new_typed_array_with_bytes()`]
    pub fn as_typed_array(&self) -> Result<JSTypedArray, JSException> {
        if !self.is_typed_array() {
            return Err(unsafe {
                Self::from_raw(
                    self.ctx,
                    JSString::from("Value is not a Typed Array").raw as *const _,
                )
            }
            .into());
        }

        let object = self.as_object()?;

        Ok(unsafe { JSTypedArray::from_raw(object.ctx, object.raw) })
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
    /// # See also
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
    /// # See also
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

impl From<JSValue> for sys::JSValueRef {
    fn from(value: JSValue) -> Self {
        value.raw
    }
}

impl From<JSValue> for sys::JSObjectRef {
    fn from(value: JSValue) -> Self {
        value.raw as *mut _
    }
}

#[cfg(test)]
mod tests {
    use crate::{evaluate_script, function_callback, sys, JSContext, JSException, JSType, JSValue};

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
    fn array() {
        let ctx = JSContext::default();
        let va = JSValue::new_array(
            &ctx,
            &[
                JSValue::new_boolean(&ctx, true),
                JSValue::new_boolean(&ctx, false),
            ],
        )
        .unwrap();
        assert!(va.is_array());
        assert!(!va.is_null());
        assert_eq!(va.get_type(), JSType::Object); // true!
        assert!(va.as_boolean());
        assert!(va.as_number().is_err());
        let vo = va.as_object().unwrap();
        assert!(vo.get_property_at_index(0).as_boolean());
        assert!(!vo.get_property_at_index(1).as_boolean());
    }

    #[test]
    fn typed_array() -> Result<(), JSException> {
        let ctx = JSContext::default();
        let mut bytes = vec![1u8, 2, 3, 4, 5];
        let array = unsafe { JSValue::new_typed_array_with_bytes(&ctx, bytes.as_mut_slice()) }?;

        // It's a `Uint8Array.`
        assert!(array.is_typed_array());

        let array = array.as_object()?;

        assert_eq!(
            unsafe {
                array
                    .get_property("byteLength")
                    .as_number()?
                    .to_int_unchecked::<usize>()
            },
            bytes.len()
        );
        assert_eq!(
            unsafe {
                array
                    .get_property("BYTES_PER_ELEMENT")
                    .as_number()?
                    .to_int_unchecked::<usize>()
            },
            1
        );

        // Let's test the mutability of the bytes, i.e. they aren't copied but borrowed.
        array.set_property_at_index(2, JSValue::new_number(&ctx, 10.))?;

        assert_eq!(bytes, &[1u8, 2, 10, 4, 5]);

        bytes[3] = 11;

        assert_eq!(
            unsafe {
                array
                    .get_property_at_index(3)
                    .as_number()?
                    .to_int_unchecked::<u8>()
            },
            11
        );

        Ok(())
    }

    #[test]
    fn function() -> Result<(), JSException> {
        let ctx = JSContext::default();

        unsafe extern "C" fn sum(
            ctx: sys::JSContextRef,
            _function: sys::JSObjectRef,
            _this_object: sys::JSObjectRef,
            argument_count: usize,
            arguments: *const sys::JSValueRef,
            _exception: *mut sys::JSValueRef,
        ) -> *const sys::OpaqueJSValue {
            use std::{mem, slice};

            let arguments = unsafe { slice::from_raw_parts(arguments, argument_count) }
                .iter()
                .map(|value| JSValue::from_raw(ctx, *value))
                .collect::<Vec<_>>();

            #[allow(clippy::get_first)]
            let x = arguments.get(0).unwrap();
            let y = arguments.get(1).unwrap();

            let ctx = JSContext::from_raw(ctx as *mut _);
            let result = JSValue::new_number(&ctx, x.as_number().unwrap() + y.as_number().unwrap());

            mem::forget(ctx);

            result.raw
        }

        // Let's try from Rust.
        let sum = JSValue::new_function(&ctx, "awesome_sum", Some(sum));
        let result = sum.as_object()?.call_as_function(
            None,
            &[JSValue::new_number(&ctx, 1.), JSValue::new_number(&ctx, 2.)],
        )?;

        assert_eq!(result.as_number()?, 3.);

        // Let's try in a script.
        let global_object = ctx.global_object()?;
        global_object.set_property("awesome_sum", sum)?;

        let result = evaluate_script(&ctx, "awesome_sum(3, 4)", None, "test.js", 1)?;

        assert_eq!(result.as_number()?, 7.);

        Ok(())
    }

    #[test]
    fn function_with_macros() -> Result<(), JSException> {
        use crate as javascriptcore;

        let ctx = JSContext::default();

        #[function_callback]
        fn sum(
            ctx: &JSContext,
            _function: Option<&JSObject>,
            _this_object: Option<&JSObject>,
            arguments: &[JSValue],
        ) -> Result<JSValue, JSException> {
            if arguments.len() != 2 {
                return Err(JSValue::new_string(ctx, "must receive 2 arguments").into());
            }

            let x = arguments[0].as_number()?;
            let y = arguments[1].as_number()?;

            Ok(JSValue::new_number(ctx, x + y))
        }

        // Let's try from Rust.
        let sum = JSValue::new_function(&ctx, "awesome_sum", Some(sum));
        let sum_as_object = sum.as_object()?;

        // Correct call.
        {
            let result = sum_as_object.call_as_function(
                None,
                &[JSValue::new_number(&ctx, 1.), JSValue::new_number(&ctx, 2.)],
            )?;

            assert_eq!(result.as_number()?, 3.);
        }

        // Invalid call: Not enough arguments.
        {
            let result = sum_as_object.call_as_function(None, &[]);

            assert!(result.is_err());
        }

        // Let's try in a script.
        let global_object = ctx.global_object()?;
        global_object.set_property("awesome_sum", sum)?;

        let result = evaluate_script(&ctx, "awesome_sum(3, 4)", None, "test.js", 1)?;

        assert_eq!(result.as_number()?, 7.);

        Ok(())
    }

    #[test]
    fn function_with_macros_and_generics() -> Result<(), JSException> {
        use crate as javascriptcore;

        let ctx = JSContext::default();

        trait Math {
            fn double(x: f64) -> f64;
        }

        struct Double;
        struct NoDouble;

        impl Math for Double {
            fn double(x: f64) -> f64 {
                x * 2.
            }
        }

        impl Math for NoDouble {
            fn double(x: f64) -> f64 {
                x
            }
        }

        #[function_callback]
        fn do_double<M>(
            ctx: &JSContext,
            _function: Option<&JSObject>,
            _this_object: Option<&JSObject>,
            arguments: &[JSValue],
        ) -> Result<JSValue, JSException>
        where
            M: Math,
        {
            if arguments.len() != 1 {
                return Err(JSValue::new_string(ctx, "must receive 1 argument").into());
            }

            let x = arguments[0].as_number()?;

            Ok(JSValue::new_number(ctx, M::double(x)))
        }

        // Let's try with `Double` as the generic parameter.
        let do_math = JSValue::new_function(&ctx, "do_math_with_double", Some(do_double::<Double>));
        let do_math_as_object = do_math.as_object()?;

        // Correct call.
        {
            let result =
                do_math_as_object.call_as_function(None, &[JSValue::new_number(&ctx, 2.)])?;

            assert_eq!(result.as_number()?, 4.);
        }

        // Let's try with `NoDouble` as the generic parameter.
        let do_math =
            JSValue::new_function(&ctx, "do_math_with_no_double", Some(do_double::<NoDouble>));
        let do_math_as_object = do_math.as_object()?;

        // Correct call.
        {
            let result =
                do_math_as_object.call_as_function(None, &[JSValue::new_number(&ctx, 2.)])?;

            assert_eq!(result.as_number()?, 2.);
        }

        Ok(())
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
