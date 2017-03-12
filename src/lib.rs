// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! JavaScriptCore Bindings
//!
//! Evaluate JavaScript programs from within an app, and support
//! JavaScript scripting of your app.

#![warn(missing_docs)]
#![deny(trivial_numeric_casts, unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate javascriptcore_sys as sys;

mod base;
mod context;
mod contextgroup;
mod exception;
mod string;
mod value;

pub use base::{check_script_syntax, evaluate_script, garbage_collect};
pub use sys::{JSType, JSTypedArrayType};

/// A JavaScript class.
///
/// Used with `JSObjectMake` to construct objects with custom
/// behavior.
///
/// TODO: Fix `JSObjectMake` reference once it has been wrapped.
pub struct JSClass {
    #[allow(dead_code)]
    raw: sys::JSClassRef,
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

/// A wrapper for a JSValue that contains an exception.
#[derive(Debug)]
pub struct JSException {
    value: JSValue,
}

/// A JavaScript object.
///
/// An `JSObject` is a `JSValue`.
pub struct JSObject {
    #[allow(dead_code)]
    raw: sys::JSObjectRef,
}

/// A UTF16 character buffer.
///
/// The fundamental string representation in JavaScript.
#[derive(Debug, Eq)]
pub struct JSString {
    #[allow(dead_code)]
    raw: sys::JSStringRef,
}

/// A JavaScript value.
///
/// The base type for all JavaScript values, and polymorphic functions
/// on them.
#[derive(Debug)]
pub struct JSValue {
    #[allow(dead_code)]
    raw: sys::JSValueRef,
}
