// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! JavaScriptCore Bindings
//!
//! Evaluate JavaScript programs from within an app, and support
//! JavaScript scripting of your app.

#![warn(clippy::doc_markdown, missing_docs)]
#![deny(
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

use std::ffi::CString;

pub use javascriptcore_macros::{constructor_callback, function_callback};
#[doc(hidden)]
pub use javascriptcore_sys as sys;

mod base;
mod class;
mod context;
mod contextgroup;
mod exception;
mod object;
mod string;
mod typed_array;
mod value;

pub use crate::sys::{JSType, JSTypedArrayType};
pub use crate::{
    base::{check_script_syntax, evaluate_script, garbage_collect},
    class::JSClassBuilder,
};

/// A JavaScript class.
///
/// The best way to create a class is by using [`JSClass::builder`].
pub struct JSClass {
    ctx: sys::JSContextRef,
    raw: sys::JSClassRef,
    #[allow(unused)]
    name: CString,
}

/// A JavaScript execution context.
///
/// Holds the global object and other execution state.
pub struct JSContext {
    raw: sys::JSGlobalContextRef,
}

/// A group that associates JavaScript contexts with one another.
///
/// Contexts in the same group may share and exchange JavaScript
/// objects. Sharing and/or exchanging JavaScript objects between
/// contexts in different groups will produce undefined behavior.
/// When objects from the same context group are used in multiple
/// threads, explicit synchronization is required.
pub struct JSContextGroup {
    raw: sys::JSContextGroupRef,
}

/// A wrapper for a [`JSValue`] that contains an exception.
#[derive(Debug)]
pub struct JSException {
    value: JSValue,
}

/// A JavaScript object.
///
/// An `JSObject` is a [`JSValue`]. This is implemented by having
/// `JSObject` implement the `Deref` trait so that anything that
/// expects a `JSValue` can receive a `JSObject` as well.
pub struct JSObject {
    raw: sys::JSObjectRef,
    value: JSValue,
}

/// A UTF16 character buffer.
///
/// The fundamental string representation in JavaScript. Since
/// this is using a UTF16 encoding and Rust strings are using
/// UTF8 encoding, converting between string representations
/// is not cheap.
///
/// In this crate, implementations of the conversion traits
/// `Into` and `From` are provided for `JSString`. This allows
/// conversion from `&str` and `String` into `JSString`:
///
/// ```
/// # use javascriptcore::JSString;
/// let j: JSString = "abc".into();
/// ```
///
/// Similarly, a `JSString` can be converted to a `String`
/// via a conversion trait or directly:
///
/// ```
/// # use javascriptcore::JSString;
/// let j: JSString = "abc".into();
/// let s: String = (&j).into(); // Requires a reference.
/// let s: String = j.to_string();
/// ```
///
/// In this crate, functions that need a `JSString` use
/// generics so that they can take anything that can be
/// converted to a `JSString` instead. This allows the
/// caller to pass an `&str` or `String`, or to cache a
/// previously converted `JSString` and pass that directly.
///
/// A `JSString` is not a [`JSValue`] and so it can not be
/// passed where a `JSValue` is expected. Instead, it must
/// be boxed using [`JSValue::new_string`].
#[derive(Eq)]
pub struct JSString {
    raw: sys::JSStringRef,
}

/// A JavaScript Typed Array.
///
/// A Typed Array is a special JavaScript object that represent a family of
/// buffer views. Learn more by [reading the documentation][doc].
///
/// [doc]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray#behavior_when_viewing_a_resizable_buffer
pub struct JSTypedArray {
    raw: sys::JSObjectRef,
    ctx: sys::JSContextRef,
}

/// A JavaScript value.
///
/// The base type for all JavaScript values, and polymorphic functions
/// on them.
///
/// All values passed between Rust and JavaScriptCore will be boxed with
/// a `JSValue`.
///
/// # Creating JS values
///
/// * [`JSValue::new_undefined()`]
/// * [`JSValue::new_null()`]
/// * [`JSValue::new_boolean()`]
/// * [`JSValue::new_number()`]
/// * [`JSValue::new_string()`]
/// * [`JSValue::new_typed_array_with_bytes()`]
/// * [`JSValue::new_function()`]
/// * [`JSValue::new_from_json()`]
///
/// # JSON
///
/// * [`JSValue::new_from_json()`]
/// * [`JSValue::to_json_string()`]
///
/// # Retrieving Rust values
///
/// * [`JSValue::as_boolean()`]
/// * [`JSValue::as_number()`]
/// * [`JSValue::as_object()`]
/// * [`JSValue::as_string()`]
/// * [`JSValue::as_typed_array()`]
#[derive(Debug)]
pub struct JSValue {
    raw: sys::JSValueRef,
    ctx: sys::JSContextRef,
}
