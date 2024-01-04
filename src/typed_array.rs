// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{sys, JSException, JSObject, JSTypedArray, JSTypedArrayType, JSValue};
use std::{ptr, slice};

impl JSTypedArray {
    /// Create a new [`Self`] from its raw pointer directly.
    ///
    /// # Safety
    ///
    /// Ensure `raw` is valid, and represents a typed array.
    pub(crate) const unsafe fn from_raw(ctx: sys::JSContextRef, raw: sys::JSObjectRef) -> Self {
        Self { raw, ctx }
    }

    /// Returns a value of type [`JSTypedArrayType`] that identifies value's
    /// Typed Array type, or `JSTypedArrayType::None` if the value is not a Typed Array
    /// object.
    ///
    /// ```rust
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    /// let array = evaluate_script(&ctx, "new Uint8Array([1, 2, 3, 4, 5])", None, "foo.js", 1)
    ///     .unwrap()
    ///     .as_typed_array()
    ///     .unwrap();
    /// assert_eq!(array.ty().unwrap(), JSTypedArrayType::Uint8Array);
    /// ```
    pub fn ty(&self) -> Result<JSTypedArrayType, JSException> {
        let mut exception: sys::JSValueRef = ptr::null_mut();
        let value = unsafe { sys::JSValueGetTypedArrayType(self.ctx, self.raw, &mut exception) };

        if !exception.is_null() {
            Err(unsafe { JSValue::from_raw(self.ctx, exception) }.into())
        } else {
            Ok(value)
        }
    }

    /// Returns the length of the Typed Array.
    ///
    /// ```rust
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    /// let array = evaluate_script(&ctx, "new Uint8Array([1, 2, 3, 4, 5])", None, "foo.js", 1)
    ///     .unwrap()
    ///     .as_typed_array()
    ///     .unwrap();
    /// assert_eq!(array.len().unwrap(), 5);
    /// ```
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> Result<usize, JSException> {
        let mut exception: sys::JSValueRef = ptr::null_mut();
        let value = unsafe { sys::JSObjectGetTypedArrayLength(self.ctx, self.raw, &mut exception) };

        if !exception.is_null() {
            Err(unsafe { JSValue::from_raw(self.ctx, exception) }.into())
        } else {
            Ok(value)
        }
    }

    /// Returns the byte offset of the Typed Array.
    ///
    /// The _byte offset_ is the offset used when a Typed Array is created from
    /// another Typed Array, it's a “subview” that can start from an offset, up to a
    /// certain length, which is the byte length.
    ///
    /// ```rust
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    /// let array = evaluate_script(&ctx, "const array = new Uint8Array([1, 2, 3, 4, 5]); new Uint8Array(array.buffer, 3)", None, "foo.js", 1)
    ///     .unwrap()
    ///     .as_typed_array()
    ///     .unwrap();
    /// assert_eq!(array.byte_offset().unwrap(), 3);
    /// ```
    pub fn byte_offset(&self) -> Result<usize, JSException> {
        let mut exception: sys::JSValueRef = ptr::null_mut();
        let offset =
            unsafe { sys::JSObjectGetTypedArrayByteOffset(self.ctx, self.raw, &mut exception) };

        if !exception.is_null() {
            Err(unsafe { JSValue::from_raw(self.ctx, exception) }.into())
        } else {
            Ok(offset)
        }
    }

    /// Returns the byte length of the Typed Array.
    ///
    /// The _byte length_ is the length used when a Typed Array is created from
    /// another Typed Array, it's a “subview” that can start from an offset, up to a
    /// certain length, which is the byte length.
    ///
    /// ```rust
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    /// let array = evaluate_script(&ctx, "const array = new Uint8Array([1, 2, 3, 4, 5]); new Uint8Array(array.buffer, 1, 2)", None, "foo.js", 1)
    ///     .unwrap()
    ///     .as_typed_array()
    ///     .unwrap();
    /// assert_eq!(array.byte_length().unwrap(), 2);
    /// ```
    pub fn byte_length(&self) -> Result<usize, JSException> {
        let mut exception: sys::JSValueRef = ptr::null_mut();
        let length =
            unsafe { sys::JSObjectGetTypedArrayByteLength(self.ctx, self.raw, &mut exception) };

        if !exception.is_null() {
            Err(unsafe { JSValue::from_raw(self.ctx, exception) }.into())
        } else {
            Ok(length)
        }
    }

    /// Returns a mutable slice of the underlying buffer represented by the
    /// Typed Array.
    ///
    /// # Safety
    ///
    /// The pointer of the slice returned by this function is temporary and is not
    /// guaranteed to remain valid across JavaScriptCore API calls.
    ///
    /// # Example
    ///    
    /// ```rust
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// /// Create a Typed Array from the Rust API.
    /// let mut bytes = vec![1u8, 2, 3, 4, 5];
    /// let array_as_value =
    ///     unsafe { JSValue::new_typed_array_with_bytes(&ctx, bytes.as_mut_slice()) }.unwrap();
    /// let mut array = array_as_value.as_typed_array().unwrap();
    ///
    /// ctx.global_object().unwrap().set_property("array", array_as_value).unwrap();
    ///
    /// /// Create a sub-Typed Array from `array` in JavaScript.
    /// let mut sub_array = evaluate_script(
    ///     &ctx,
    ///     "new Uint8Array(array.buffer, 1, 3)",
    ///     None,
    ///     "foo.js",
    ///     1,
    /// )
    ///     .unwrap()
    ///     .as_typed_array()
    ///     .unwrap();
    ///
    /// let sub_slice = unsafe { sub_array.as_mut_slice() }.unwrap();
    ///
    /// // Items are untouched.
    /// assert_eq!(sub_slice, &[2, 3, 4]);
    /// assert_eq!(bytes, &[1, 2, 3, 4, 5]);
    ///
    /// // Now let's mutate them.
    /// sub_slice[0] = 12;
    /// sub_slice[2] = 14;
    ///
    /// // See, they are mutated.
    /// assert_eq!(sub_slice, &[12, 3, 14]);
    /// assert_eq!(bytes, &[1, 12, 3, 14, 5]);
    /// ```
    pub unsafe fn as_mut_slice(&mut self) -> Result<&mut [u8], JSException> {
        self.as_mut_slice_impl()
    }

    unsafe fn as_mut_slice_impl(&self) -> Result<&mut [u8], JSException> {
        let offset = self.byte_offset()?;
        let length = self.len()?;

        let mut exception: sys::JSValueRef = ptr::null_mut();
        let ptr = sys::JSObjectGetTypedArrayBytesPtr(self.ctx, self.raw, &mut exception);

        if !exception.is_null() {
            Err(JSValue::from_raw(self.ctx, exception).into())
        } else {
            assert!(!ptr.is_null(), "`ptr` must not be null");

            Ok(slice::from_raw_parts_mut(
                ptr.offset(offset.try_into().unwrap()).cast::<u8>(),
                length,
            ))
        }
    }

    /// Returns a `Vec` (so a copy) of the underlying buffer represented by the
    /// Typed Array.
    ///
    /// ```rust
    /// # use javascriptcore::*;
    /// let ctx = JSContext::default();
    ///
    /// let mut array = evaluate_script(&ctx, "const array = new Uint8Array([1, 2, 3, 4, 5]); new Uint8Array(array.buffer, 1, 3)", None, "foo.js", 1)
    ///     .unwrap()
    ///     .as_typed_array()
    ///     .unwrap();
    ///
    /// assert_eq!(array.to_vec().unwrap(), &[2, 3, 4]);
    /// ```
    pub fn to_vec(&self) -> Result<Vec<u8>, JSException> {
        Ok(unsafe { self.as_mut_slice_impl() }?.to_vec())
    }
}

impl From<&JSTypedArray> for JSObject {
    fn from(array: &JSTypedArray) -> Self {
        // SAFETY: `ctx` and `raw` is valid, it's safe to use them.
        unsafe { JSObject::from_raw(array.ctx, array.raw) }
    }
}

impl From<JSTypedArray> for JSObject {
    fn from(array: JSTypedArray) -> Self {
        (&array).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{evaluate_script, JSContext};

    #[test]
    fn new() -> Result<(), JSException> {
        let ctx = JSContext::default();
        let mut bytes = vec![1u8, 2, 3, 4, 5];
        let array = unsafe { JSValue::new_typed_array_with_bytes(&ctx, bytes.as_mut_slice()) }?;

        // It's a Typed Array.
        assert!(array.is_typed_array());

        let array = array.as_typed_array()?;
        // It's a `Uint8Array`.
        assert_eq!(array.ty()?, JSTypedArrayType::Uint8Array);

        // Can go to `JSObject` and `JSValue` again.
        assert!(JSValue::from(JSObject::from(array)).is_typed_array());

        Ok(())
    }

    #[test]
    fn len() -> Result<(), JSException> {
        let ctx = JSContext::default();
        let mut bytes = vec![1u8, 2, 3, 4, 5];
        let array = unsafe { JSValue::new_typed_array_with_bytes(&ctx, bytes.as_mut_slice()) }?
            .as_typed_array()?;

        assert_eq!(array.len()?, 5);

        Ok(())
    }

    #[test]
    fn byte_offset() -> Result<(), JSException> {
        let ctx = JSContext::default();
        let mut bytes = vec![1u8, 2, 3, 4, 5];
        let array = unsafe { JSValue::new_typed_array_with_bytes(&ctx, bytes.as_mut_slice()) }?
            .as_typed_array()?;

        assert_eq!(array.byte_offset()?, 0);

        // More complex.
        let array = evaluate_script(
            &ctx,
            "const array = new Uint8Array([1, 2, 3, 4, 5]); new Uint8Array(array.buffer, 2)",
            None,
            "foo.js",
            1,
        )?
        .as_typed_array()?;

        assert_eq!(array.len()?, 3);
        assert_eq!(array.byte_offset()?, 2);

        Ok(())
    }

    #[test]
    fn byte_length() -> Result<(), JSException> {
        let ctx = JSContext::default();
        let mut bytes = vec![1u8, 2, 3, 4, 5];
        let array = unsafe { JSValue::new_typed_array_with_bytes(&ctx, bytes.as_mut_slice()) }?
            .as_typed_array()?;

        assert_eq!(array.byte_length()?, 5);

        // More complex.
        let array = evaluate_script(
            &ctx,
            "const array = new Uint8Array([1, 2, 3, 4, 5]); new Uint8Array(array.buffer, 2, 2)",
            None,
            "foo.js",
            1,
        )?
        .as_typed_array()?;

        assert_eq!(array.len()?, 2);
        assert_eq!(array.byte_length()?, 2);

        Ok(())
    }

    #[test]
    fn as_mut_slice_has_correct_items() -> Result<(), JSException> {
        let ctx = JSContext::default();

        // No byte offset, no byte length.
        {
            let mut array = evaluate_script(
                &ctx,
                "const array0 = new Uint8Array([1, 2, 3, 4, 5]); new Uint8Array(array0.buffer)",
                None,
                "foo.js",
                1,
            )?
            .as_typed_array()?;

            assert_eq!(array.len()?, 5);
            assert_eq!(array.byte_offset()?, 0);
            assert_eq!(array.byte_length()?, 5);
            assert_eq!(unsafe { array.as_mut_slice()? }, &[1, 2, 3, 4, 5]);
        }

        // A byte offset, no byte length.
        {
            let mut array = evaluate_script(
                &ctx,
                "const array1 = new Uint8Array([1, 2, 3, 4, 5]); new Uint8Array(array1.buffer, 1)",
                None,
                "foo.js",
                2,
            )?
            .as_typed_array()?;

            assert_eq!(array.len()?, 4);
            assert_eq!(array.byte_offset()?, 1);
            assert_eq!(array.byte_length()?, 4);
            assert_eq!(unsafe { array.as_mut_slice()? }, &[2, 3, 4, 5]);
        }

        // A byte offset, a byte length, the typed array is length-tracking.
        {
            let mut array = evaluate_script(
                &ctx,
                "const array2 = new Uint8Array([1, 2, 3, 4, 5]); new Uint8Array(array2.buffer, 1, 3)",
                None,
                "foo.js",
                3,
            )?
            .as_typed_array()?;

            assert_eq!(array.len()?, 3);
            assert_eq!(array.byte_offset()?, 1);
            assert_eq!(array.byte_length()?, 3);
            assert_eq!(unsafe { array.as_mut_slice()? }, &[2, 3, 4]);
        }

        Ok(())
    }

    #[test]
    fn as_mut_slice_is_mutable() -> Result<(), JSException> {
        let ctx = JSContext::default();

        let mut bytes = vec![1u8, 2, 3, 4, 5];
        let array_as_value =
            unsafe { JSValue::new_typed_array_with_bytes(&ctx, bytes.as_mut_slice()) }?;
        let mut array = array_as_value.as_typed_array()?;

        ctx.global_object()?.set_property("array", array_as_value)?;

        let mut sub_array = evaluate_script(
            &ctx,
            "new Uint8Array(array.buffer, 1, 3)",
            None,
            "foo.js",
            1,
        )?
        .as_typed_array()?;

        assert_eq!(sub_array.len()?, 3);
        assert_eq!(sub_array.byte_offset()?, 1);
        assert_eq!(sub_array.byte_length()?, 3);
        let sub_slice = unsafe { sub_array.as_mut_slice() }?;

        // Items are untouched.
        assert_eq!(sub_slice, &[2, 3, 4]);
        assert_eq!(bytes, &[1, 2, 3, 4, 5]);

        // Now let's mutate them.
        sub_slice[0] = 12;
        sub_slice[2] = 14;

        // See, they are mutated.
        assert_eq!(sub_slice, &[12, 3, 14]);
        assert_eq!(bytes, &[1, 12, 3, 14, 5]);
        assert_eq!(unsafe { array.as_mut_slice() }?, &[1, 12, 3, 14, 5]);

        Ok(())
    }
}
