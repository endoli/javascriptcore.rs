// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate provides raw bindings for the JavaScriptCore public
//! API. It is a pretty direct mapping of the underlying C API
//! provided by JavaScriptCore.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![warn(clippy::doc_markdown, missing_docs)]

use std::ptr;

/// A group that associates JavaScript contexts with one another.
/// Contexts in the same group may share and exchange JavaScript objects.
#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSContextGroup([u8; 0]);

/// A group that associates JavaScript contexts with one another.
/// Contexts in the same group may share and exchange JavaScript objects.
pub type JSContextGroupRef = *const OpaqueJSContextGroup;

/// A JavaScript execution context. Holds the global object and
/// other execution state.
#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSContext([u8; 0]);

/// A JavaScript execution context. Holds the global object and
/// other execution state.
pub type JSContextRef = *const OpaqueJSContext;

/// A global JavaScript execution context.
/// A [`JSGlobalContextRef`] is a [`JSContextRef`].
pub type JSGlobalContextRef = *mut OpaqueJSContext;

/// A UTF16 character buffer. The fundamental string representation
/// in JavaScript.
#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSString([u8; 0]);

/// A UTF16 character buffer. The fundamental string representation
/// in JavaScript.
pub type JSStringRef = *mut OpaqueJSString;

/// A JavaScript class.
/// Used with [`JSObjectMake`] to construct objects with custom behavior.
#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSClass([u8; 0]);

/// A JavaScript class.
/// Used with [`JSObjectMake`] to construct objects with custom behavior.
pub type JSClassRef = *mut OpaqueJSClass;

/// An array of JavaScript property names.
#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSPropertyNameArray([u8; 0]);

/// An array of JavaScript property names.
///
/// Values of this type are obtained via [`JSObjectCopyPropertyNames`].
///
/// Operations:
///
/// * [`JSPropertyNameArrayGetCount`]
/// * [`JSPropertyNameArrayGetNameAtIndex`]
/// * [`JSPropertyNameArrayRelease`]
/// * [`JSPropertyNameArrayRetain`]
pub type JSPropertyNameArrayRef = *mut OpaqueJSPropertyNameArray;

/// An ordered set used to collect the names of
/// a JavaScript object's properties.
#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSPropertyNameAccumulator([u8; 0]);

/// An ordered set used to collect the names of
/// a JavaScript object's properties.
///
/// Values of this type are passed to the [`getPropertyNames` callback].
/// Names are added to the accumulator using [`JSPropertyNameAccumulatorAddName`].
///
/// [`getPropertyNames` callback]: crate::JSObjectGetPropertyNamesCallback
pub type JSPropertyNameAccumulatorRef = *mut OpaqueJSPropertyNameAccumulator;

/// A function used to deallocate bytes passed to a Typed Array constructor.
///
/// The function should take two arguments. The first is a pointer to
/// the bytes that were originally passed to the Typed Array constructor.
/// The second is a pointer to additional information desired at the time
/// the bytes are to be freed.
pub type JSTypedArrayBytesDeallocator = ::std::option::Option<
    unsafe extern "C" fn(
        bytes: *mut ::std::os::raw::c_void,
        deallocatorContext: *mut ::std::os::raw::c_void,
    ),
>;

/// A JavaScript value.
/// The base type for all JavaScript values, and polymorphic functions on them.
#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSValue([u8; 0]);

/// A JavaScript value.
/// The base type for all JavaScript values, and polymorphic functions on them.
pub type JSValueRef = *const OpaqueJSValue;

/// A JavaScript object. A [`JSObjectRef`] is a [`JSValueRef`].
pub type JSObjectRef = *mut OpaqueJSValue;

extern "C" {
    /// Evaluates a string of JavaScript.
    ///
    /// * `ctx`: The execution context to use.
    /// * `script`: A [`JSStringRef`] containing the script to evaluate.
    /// * `thisObject`: The object to use as `this`, or `NULL` to
    ///   use the global object as `this`.
    /// * `sourceURL`: A [`JSStringRef`] containing a URL for the script's
    ///   source file. This is used by debuggers and when reporting
    ///   exceptions. Pass `NULL` if you do not care to include source
    ///   file information.
    /// * `startingLineNumber`: An integer value specifying the script's
    ///   starting line number in the file located at `sourceURL`. This
    ///   is only used when reporting exceptions. The value is one-based,
    ///   so the first line is line `1` and invalid values are clamped
    ///   to `1`.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store an
    ///   exception, if any. Pass `NULL` if you do not care to store an
    ///   exception.
    ///
    /// The [`JSValueRef`] that results from evaluating script, or `NULL` if an exception is thrown.
    ///
    /// # See also
    ///
    /// * [`JSCheckScriptSyntax()`]
    pub fn JSEvaluateScript(
        ctx: JSContextRef,
        script: JSStringRef,
        thisObject: JSObjectRef,
        sourceURL: JSStringRef,
        startingLineNumber: ::std::os::raw::c_int,
        exception: *mut JSValueRef,
    ) -> JSValueRef;

    /// Checks for syntax errors in a string of JavaScript.
    ///
    /// * `ctx`: The execution context to use.
    /// * `script`: A [`JSStringRef`] containing the script to check for
    ///   syntax errors.
    /// * `sourceURL`: A [`JSStringRef`] containing a URL for the script's
    ///   source file. This is only used when reporting exceptions.
    ///   Pass `NULL` if you do not care to include source file
    ///   information in exceptions.
    /// * `startingLineNumber`: An integer value specifying the script's
    ///   starting line number in the file located at `sourceURL`. This
    ///   is only used when reporting exceptions. The value is one-based,
    ///   so the first line is line `1` and invalid values are clamped
    ///   to `1`.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store a
    ///   syntax error exception, if any. Pass `NULL` if you do not care
    ///   to store a syntax error exception.
    ///
    /// Returns `true` if the script is syntactically correct, otherwise `false`.
    ///
    /// # See also
    ///
    /// * [`JSEvaluateScript()`]
    pub fn JSCheckScriptSyntax(
        ctx: JSContextRef,
        script: JSStringRef,
        sourceURL: JSStringRef,
        startingLineNumber: ::std::os::raw::c_int,
        exception: *mut JSValueRef,
    ) -> bool;

    /// Performs a JavaScript garbage collection.
    ///
    /// JavaScript values that are on the machine stack, in a register,
    /// protected by [`JSValueProtect`], set as the global object of an
    /// execution context, or reachable from any such value will not
    /// be collected.
    ///
    /// During JavaScript execution, you are not required to call this
    /// function; the JavaScript engine will garbage collect as needed.
    /// JavaScript values created within a context group are automatically
    /// destroyed when the last reference to the context group is released.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// # See also
    ///
    /// * [`JSValueProtect()`]
    /// * [`JSValueUnprotect()`]
    pub fn JSGarbageCollect(ctx: JSContextRef);
}

/// A constant identifying the type of a [`JSValueRef`].
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JSType {
    /// The unique `undefined` value.
    Undefined = 0,
    /// The unique `null` value.
    Null = 1,
    /// A primitive boolean value, one of `true` or `false`.
    Boolean = 2,
    /// A primitive number value.
    Number = 3,
    /// A primitive string value.
    String = 4,
    /// An object value (meaning that this [`JSValueRef`] is a [`JSObjectRef`]).
    Object = 5,
    /// A primitive symbol value.
    Symbol = 6,
}

/// A constant identifying the Typed Array type of a [`JSObjectRef`].
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JSTypedArrayType {
    /// `Int8Array`
    Int8Array = 0,
    /// `Int16Array`
    Int16Array = 1,
    /// `Int32Array`
    Int32Array = 2,
    /// `Uint8Array`
    Uint8Array = 3,
    /// `Uint8ClampedArray`
    Uint8ClampedArray = 4,
    /// `Uint16Array`
    Uint16Array = 5,
    /// `Uint32Array`
    Uint32Array = 6,
    /// `Float32Array`
    Float32Array = 7,
    /// `Float64Array`
    Float64Array = 8,
    /// `ArrayBuffer`
    ArrayBuffer = 9,
    /// Not a Typed Array
    None = 10,
    /// `BigInt64Array`
    BigInt64Array = 11,
    /// `BigUint64Array`
    BigUint64Array = 12,
}

extern "C" {
    /// Returns a JavaScript value's type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] whose type you want to obtain.
    ///
    /// Returns a value of type [`JSType`] that identifies `value`'s type.
    pub fn JSValueGetType(ctx: JSContextRef, arg1: JSValueRef) -> JSType;

    /// Tests whether a JavaScript value's type is the `undefined` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to test.
    ///
    /// Returns `true` if `value`'s type is the `undefined` type, otherwise `false`.
    pub fn JSValueIsUndefined(ctx: JSContextRef, value: JSValueRef) -> bool;

    /// Tests whether a JavaScript value's type is the `null` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to test.
    ///
    /// Returns `true` if `value`'s type is the `null` type, otherwise `false`.
    pub fn JSValueIsNull(ctx: JSContextRef, value: JSValueRef) -> bool;

    /// Tests whether a JavaScript value's type is the `boolean` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to test.
    ///
    /// Returns `true` if `value`'s type is the `boolean` type, otherwise `false`.
    pub fn JSValueIsBoolean(ctx: JSContextRef, value: JSValueRef) -> bool;

    /// Tests whether a JavaScript value's type is the `number` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to test.
    ///
    /// Returns `true` if `value`'s type is the `number` type, otherwise `false`.
    pub fn JSValueIsNumber(ctx: JSContextRef, value: JSValueRef) -> bool;

    /// Tests whether a JavaScript value's type is the `string` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to test.
    ///
    /// Returns `true` if `value`'s type is the `string` type, otherwise `false`.
    pub fn JSValueIsString(ctx: JSContextRef, value: JSValueRef) -> bool;

    /// Tests whether a JavaScript value's type is the `symbol` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to test.
    ///
    /// Returns `true` if `value`'s type is the `symbol` type, otherwise `false`.
    pub fn JSValueIsSymbol(ctx: JSContextRef, value: JSValueRef) -> bool;

    /// Tests whether a JavaScript value's type is the `object` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to test.
    ///
    /// Returns `true` if `value`'s type is the `object` type, otherwise `false`.
    pub fn JSValueIsObject(ctx: JSContextRef, value: JSValueRef) -> bool;

    /// Tests whether a JavaScript value is an `object` with a given class in its class chain.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to test.
    /// * `jsClass`: The [`JSClassRef`] to test against.
    ///
    /// Returns `true` if `value` is an `object` and has `jsClass` in its
    /// class chain, otherwise `false`.
    pub fn JSValueIsObjectOfClass(
        ctx: JSContextRef,
        value: JSValueRef,
        jsClass: JSClassRef,
    ) -> bool;

    /// Tests whether a JavaScript value is an `array`.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to test.
    ///
    /// Returns `true` if `value` is an `array`, otherwise `false`.
    pub fn JSValueIsArray(ctx: JSContextRef, value: JSValueRef) -> bool;

    /// Tests whether a JavaScript value is a `date`.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to test.
    ///
    /// Returns `true` if `value` is a `date`, otherwise `false`.
    pub fn JSValueIsDate(ctx: JSContextRef, value: JSValueRef) -> bool;

    /// Returns a JavaScript value's Typed Array type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] whose Typed Array type to return.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns a value of type [`JSTypedArrayType`] that identifies
    /// value's Typed Array type, or `JSTypedArrayType::None` if the
    /// value is not a Typed Array object.
    pub fn JSValueGetTypedArrayType(
        ctx: JSContextRef,
        value: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSTypedArrayType;

    /// Tests whether two JavaScript values are equal, as compared by the JS `==` operator.
    ///
    /// * `ctx`: The execution context to use.
    /// * `a`: The first value to test.
    /// * `b`: The second value to test.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to
    ///   store an exception, if any. Pass `NULL` if you do
    ///   not care to store an exception.
    ///
    /// Returns `true` if the two values are equal, `false` if
    /// they are not equal or an exception is thrown.
    pub fn JSValueIsEqual(
        ctx: JSContextRef,
        a: JSValueRef,
        b: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool;

    /// Tests whether two JavaScript values are strict equal, as compared
    /// by the JS `===` operator.
    ///
    /// * `ctx`: The execution context to use.
    /// * `a`: The first value to test.
    /// * `b`: The second value to test.
    ///
    /// Returns `true` if the two values are strict equal, otherwise `false`.
    pub fn JSValueIsStrictEqual(ctx: JSContextRef, a: JSValueRef, b: JSValueRef) -> bool;

    /// Tests whether a JavaScript value is an object constructed by a
    /// given constructor, as compared by the JS `instanceof` operator.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to test.
    /// * `constructor`: The constructor to test against.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to
    ///   store an exception, if any. Pass `NULL` if you do
    ///   not care to store an exception.
    ///
    /// Returns `true` if value is an object constructed by constructor,
    /// as compared by the JS `instanceof` operator, otherwise `false`.
    pub fn JSValueIsInstanceOfConstructor(
        ctx: JSContextRef,
        value: JSValueRef,
        constructor: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> bool;

    /// Creates a JavaScript value of the `undefined` type.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns the unique `undefined` value.
    pub fn JSValueMakeUndefined(ctx: JSContextRef) -> JSValueRef;

    /// Creates a JavaScript value of the `null` type.
    ///
    /// * `ctx`: The execution context to use.
    ///
    /// Returns the unique `null` value.
    pub fn JSValueMakeNull(ctx: JSContextRef) -> JSValueRef;

    /// Creates a JavaScript value of the `boolean` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `boolean`: The `bool` to assign to the newly created [`JSValueRef`].
    ///
    /// Returns a [`JSValueRef`] of the `boolean` type, representing the value of `boolean`.
    pub fn JSValueMakeBoolean(ctx: JSContextRef, boolean: bool) -> JSValueRef;

    /// Creates a JavaScript value of the `number` type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `number`: The `f64` to assign to the newly created [`JSValueRef`].
    ///
    /// Returns a [`JSValueRef`] of the `number` type, representing the value of `number`.
    pub fn JSValueMakeNumber(ctx: JSContextRef, number: f64) -> JSValueRef;

    /// Creates a JavaScript value of the string type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `string`: The [`JSStringRef`] to assign to the newly created
    ///   [`JSValueRef`]. The newly created [`JSValueRef`] retains `string`, and
    ///   releases it upon garbage collection.
    ///
    /// Returns a [`JSValueRef`] of the `string` type, representing the value of `string`.
    pub fn JSValueMakeString(ctx: JSContextRef, string: JSStringRef) -> JSValueRef;

    /// Creates a JavaScript value of the symbol type.
    ///
    /// * `ctx`: The execution context to use.
    /// * `description`: The [`JSStringRef`] to assign to the newly created
    ///   [`JSValueRef`].
    ///
    /// Returns a [`JSValueRef`] of the `symbol` type, whose description matches the one provided.
    pub fn JSValueMakeSymbol(ctx: JSContextRef, description: JSStringRef) -> JSValueRef;

    /// Creates a JavaScript value from a JSON formatted string.
    ///
    /// * `ctx`: The execution context to use.
    /// * `string`: The [`JSStringRef`] containing the JSON string to be parsed.
    ///
    /// Returns a [`JSValueRef`] containing the parsed value, or `NULL` if the input is invalid.
    pub fn JSValueMakeFromJSONString(ctx: JSContextRef, string: JSStringRef) -> JSValueRef;

    /// Creates a JavaScript string containing the JSON serialized representation of a JS value.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The value to serialize.
    /// * `indent`: The number of spaces to indent when nesting.
    ///   If `0`, the resulting JSON will not contains newlines.
    ///   The size of the indent is clamped to `10` spaces.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to
    ///   store an exception, if any. Pass `NULL` if you do not
    ///   care to store an exception.
    ///
    /// Returns a [`JSStringRef`] with the result of serialization, or `NULL` if an exception is thrown.
    pub fn JSValueCreateJSONString(
        ctx: JSContextRef,
        value: JSValueRef,
        indent: ::std::os::raw::c_uint,
        exception: *mut JSValueRef,
    ) -> JSStringRef;

    /// Converts a JavaScript value to boolean and returns the resulting boolean.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to convert.
    ///
    /// Returns the boolean result of conversion.
    pub fn JSValueToBoolean(ctx: JSContextRef, value: JSValueRef) -> bool;

    /// Converts a JavaScript value to number and returns the resulting number.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to convert.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store an
    ///   exception, if any. Pass `NULL` if you do not care to store an
    ///   exception.
    ///
    /// Returns the numeric result of conversion, or `NaN` if an exception is thrown.
    pub fn JSValueToNumber(ctx: JSContextRef, value: JSValueRef, exception: *mut JSValueRef)
        -> f64;

    /// Converts a JavaScript value to string and copies the result into a JavaScript string.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to convert.
    /// * `exception`:  A pointer to a [`JSValueRef`] in which to store an
    ///   exception, if any. Pass `NULL` if you do not care to store an
    ///   exception.
    ///
    /// Returns a [`JSStringRef`] with the result of conversion, or `NULL`
    /// if an exception is thrown. Ownership follows the Create Rule.
    pub fn JSValueToStringCopy(
        ctx: JSContextRef,
        value: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSStringRef;

    /// Converts a JavaScript value to object and returns the resulting object.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to convert.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to store
    ///   an exception.
    ///
    /// Returns the [`JSObjectRef`] result of conversion, or `NULL` if
    /// an exception is thrown.
    pub fn JSValueToObject(
        ctx: JSContextRef,
        value: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;

    /// Protects a JavaScript value from garbage collection.
    ///
    /// Use this method when you want to store a [`JSValueRef`] in a
    /// global or on the heap, where the garbage collector will
    /// not be able to discover your reference to it.
    ///
    /// A value may be protected multiple times and must be
    /// [unprotected] an equal number of times before becoming
    /// eligible for garbage collection.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to protect.
    ///
    /// # See also
    ///
    /// * [`JSGarbageCollect()`]
    /// * [`JSValueUnprotect()`]
    ///
    /// [unprotected]: crate::JSValueUnprotect
    pub fn JSValueProtect(ctx: JSContextRef, value: JSValueRef);

    /// Unprotects a JavaScript value from garbage collection.
    ///
    /// A value may be [protected] multiple times and must be unprotected
    /// an equal number of times before becoming eligible for garbage
    /// collection.
    ///
    /// * `ctx`: The execution context to use.
    /// * `value`: The [`JSValueRef`] to unprotect.
    ///
    /// # See also
    ///
    /// * [`JSGarbageCollect()`]
    /// * [`JSValueProtect()`]
    ///
    /// [protected]: crate::JSValueProtect
    pub fn JSValueUnprotect(ctx: JSContextRef, value: JSValueRef);
}

/// Specifies that a property has no special attributes.
pub const kJSPropertyAttributeNone: ::std::os::raw::c_uint = 0;
/// Specifies that a property is read-only.
pub const kJSPropertyAttributeReadOnly: ::std::os::raw::c_uint = 2;
/// Specifies that a property should not be enumerated by `JSPropertyEnumerators` and JavaScript `for...in` loops.
pub const kJSPropertyAttributeDontEnum: ::std::os::raw::c_uint = 4;
/// Specifies that the delete operation should fail on a property.
pub const kJSPropertyAttributeDontDelete: ::std::os::raw::c_uint = 8;

/// A set of `JSPropertyAttribute`s.
///
/// Combine multiple attributes by logically ORing them together.
pub type JSPropertyAttributes = ::std::os::raw::c_uint;

/// Specifies that a class has no special attributes.
pub const kJSClassAttributeNone: ::std::os::raw::c_uint = 0;
/// Specifies that a class should not automatically generate a shared
/// prototype for its instance objects.
///
/// Use `kJSClassAttributeNoAutomaticPrototype` in combination with
/// [`JSObjectSetPrototype`] to manage prototypes manually.
pub const kJSClassAttributeNoAutomaticPrototype: ::std::os::raw::c_uint = 2;

/// A set of `JSClassAttribute`s.
///
/// Combine multiple attributes by logically ORing them together.
pub type JSClassAttributes = ::std::os::raw::c_uint;

/// The callback invoked when an object is first created.
///
/// * `ctx`: The execution context to use.
/// * `object`: The [`JSObjectRef`] being created.
///
/// If you named your function `Initialize`, you would declare it like this:
///
/// ```ignore
/// void
/// Initialize(JSContextRef ctx, JSObjectRef object);
/// ```
///
/// Unlike the other object callbacks, the initialize callback is
/// called on the least derived class (the parent class) first,
/// and the most derived class last.
pub type JSObjectInitializeCallback =
    ::std::option::Option<unsafe extern "C" fn(ctx: JSContextRef, object: JSObjectRef)>;

/// The callback invoked when an object is finalized (prepared
/// for garbage collection). An object may be finalized on any thread.
///
/// * `object`: The [`JSObjectRef`] being finalized.
///
/// If you named your function `Finalize`, you would declare it like this:
///
/// ```ignore
/// void
/// Finalize(JSObjectRef object);
/// ```
///
/// The finalize callback is called on the most derived class
/// first, and the least derived class (the parent class) last.
///
/// You must not call any function that may cause a garbage
/// collection or an allocation of a garbage collected object
/// from within a `JSObjectFinalizeCallback`. This includes
/// all functions that have a [`JSContextRef`] parameter.
pub type JSObjectFinalizeCallback =
    ::std::option::Option<unsafe extern "C" fn(object: JSObjectRef)>;

/// The callback invoked when determining whether an object has a property.
///
/// * `ctx`: The execution context to use.
/// * `object`: The [`JSObjectRef`] to search for the property.
/// * `propertyName`: A [`JSStringRef`] containing the name of the property look up.
///
/// Returns `true` if object has the property, otherwise `false`.
///
/// If you named your function `HasProperty`, you would declare it like this:
///
/// ```ignore
/// bool
/// HasProperty(JSContextRef ctx, JSObjectRef object, JSStringRef propertyName);
/// ```
///
/// If this function returns `false`, the `hasProperty` request
/// forwards to object's statically declared properties, then
/// its parent class chain (which includes the default object
/// class), then its prototype chain.
///
/// This callback enables optimization in cases where only a
/// property's existence needs to be known, not its value,
/// and computing its value would be expensive.
///
/// If this callback is `NULL`, the `getProperty` callback will be used
/// to service `hasProperty` requests.
///
/// # See also
///
/// * [`JSClassDefinition::getProperty`]
/// * [`JSClassDefinition::hasProperty`]
/// * [`JSObjectDeletePropertyCallback`]
/// * [`JSObjectGetPropertyCallback`]
/// * [`JSObjectSetPropertyCallback`]
/// * [`JSObjectHasProperty()`]
pub type JSObjectHasPropertyCallback = ::std::option::Option<
    unsafe extern "C" fn(ctx: JSContextRef, object: JSObjectRef, propertyName: JSStringRef) -> bool,
>;

/// The callback invoked when getting a property's value.
///
/// * `ctx`: The execution context to use.
/// * `object`: The [`JSObjectRef`] to search for the property.
/// * `propertyName`: A [`JSStringRef`] containing the name of the property to get.
/// * `exception`: A pointer to a [`JSValueRef`] in which to return an exception, if any.
///
/// Returns the property's value if object has the property, otherwise `NULL`.
///
/// If you named your function `GetProperty`, you would declare it like this:
///
/// ```ignore
/// JSValueRef
/// GetProperty(JSContextRef ctx, JSObjectRef object,
///             JSStringRef propertyName, JSValueRef* exception);
/// ```
///
/// If this function returns `NULL`, the get request forwards to `object`'s
/// statically declared properties, then its parent class chain (which
/// includes the default object class), then its prototype chain.
///
/// # See also
///
/// * [`JSClassDefinition::getProperty`]
/// * [`JSObjectDeletePropertyCallback`]
/// * [`JSObjectHasPropertyCallback`]
/// * [`JSObjectSetPropertyCallback`]
/// * [`JSObjectGetProperty()`]
pub type JSObjectGetPropertyCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        exception: *mut JSValueRef,
    ) -> *const OpaqueJSValue,
>;

/// The callback invoked when setting a property's value.
///
/// * `ctx`: The execution context to use.
/// * `object`: The [`JSObjectRef`] on which to set the property's value.
/// * `propertyName`: A [`JSStringRef`] containing the name of the property to set.
/// * `value`: A [`JSValueRef`] to use as the property's value.
/// * `exception`: A pointer to a [`JSValueRef`] in which to return an exception, if any.
///
/// Returns `true` if the property was set, otherwise `false`.
///
/// If you named your function `SetProperty`, you would declare it like this:
///
/// ```ignore
/// bool
/// SetProperty(JSContextRef ctx, JSObjectRef object,
///             JSStringRef propertyName, JSValueRef value,
///             JSValueRef* exception);
/// ```
///
/// If this function returns `false`, the set request forwards to
/// `object`'s statically declared properties, then its parent class
/// chain (which includes the default object class).
///
/// # See also
///
/// * [`JSClassDefinition::setProperty`]
/// * [`JSObjectDeletePropertyCallback`]
/// * [`JSObjectGetPropertyCallback`]
/// * [`JSObjectHasPropertyCallback`]
/// * [`JSObjectSetProperty()`]
pub type JSObjectSetPropertyCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        value: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool,
>;

/// The callback invoked when deleting a property.
///
/// * `ctx`: The execution context to use.
/// * `object`: The [`JSObjectRef`] in which to delete the property.
/// * `propertyName`: A [`JSStringRef`] containing the name of the property to delete.
/// * `exception`: A pointer to a [`JSValueRef`] in which to return an exception, if any.
///
/// Returns `true` if `propertyName` was successfully deleted, otherwise `false`.
///
/// If you named your function `DeleteProperty`, you would declare it like this:
///
/// ```ignore
/// bool
/// DeleteProperty(JSContextRef ctx, JSObjectRef object,
///                JSStringRef propertyName, JSValueRef* exception);
/// ```
///
/// If this function returns `false`, the delete request forwards to
/// `object`'s statically declared properties, then its parent class
/// chain (which includes the default object class).
///
/// # See also
///
/// * [`JSClassDefinition::deleteProperty`]
/// * [`JSObjectGetPropertyCallback`]
/// * [`JSObjectHasPropertyCallback`]
/// * [`JSObjectSetPropertyCallback`]
/// * [`JSObjectDeleteProperty()`]
pub type JSObjectDeletePropertyCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        exception: *mut JSValueRef,
    ) -> bool,
>;

/// The callback invoked when collecting the names of an object's properties.
///
/// * `ctx`: The execution context to use.
/// * `object`: The [`JSObjectRef`] whose property names are being collected.
/// * `propertyNames`: A JavaScript property name accumulator in which to
///   accumulate the names of object's properties.
///
/// If you named your function `GetPropertyNames`, you would declare it like this:
///
/// ```ignore
/// void
/// GetPropertyNames(JSContextRef ctx, JSObjectRef object,
///                  JSPropertyNameAccumulatorRef propertyNames);
/// ```
///
/// Property name accumulators are used by [`JSObjectCopyPropertyNames`]
/// and JavaScript `for...in` loops.
///
/// Use [`JSPropertyNameAccumulatorAddName`] to add property names to
/// accumulator. A class's `getPropertyNames` callback only needs to
/// provide the names of properties that the class vends through a
/// custom `getProperty` or `setProperty` callback. Other properties,
/// including statically declared properties, properties vended by
/// other classes, and properties belonging to object's prototype,
/// are added independently.
pub type JSObjectGetPropertyNamesCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyNames: JSPropertyNameAccumulatorRef,
    ),
>;

/// The callback invoked when an object is called as a function.
///
/// * `ctx`: The execution context to use.
/// * `function`: A [`JSObjectRef`] that is the function being called.
/// * `thisObject`: A [`JSObjectRef`] that is the `this` variable in the function's scope.
/// * `argumentCount`: An integer count of the number of arguments in `arguments`.
/// * `arguments`: A [`JSValueRef`] array of the arguments passed to the function.
/// * `exception`: A pointer to a [`JSValueRef`] in which to return an exception, if any.
///
/// Returns a [`JSValueRef`] that is the function's return value.
///
/// If you named your function `CallAsFunction`, you would declare it like this:
///
/// ```ignore
/// JSValueRef
/// CallAsFunction(JSContextRef ctx, JSObjectRef function,
///                JSObjectRef thisObject,
///                size_t argumentCount, const JSValueRef arguments[],
///                JSValueRef* exception);
/// ```
///
/// If your callback were invoked by the JavaScript expression
/// `myObject.myFunction()`, function would be set to `myFunction`,
/// and `thisObject` would be set to `myObject`.
///
/// If this callback is `NULL`, calling your object as a function
/// will throw an exception.
pub type JSObjectCallAsFunctionCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        function: JSObjectRef,
        thisObject: JSObjectRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> *const OpaqueJSValue,
>;

/// The callback invoked when an object is used as a constructor in a `new` expression.
///
/// * `ctx`: The execution context to use.
/// * `constructor`: A [`JSObjectRef`] that is the constructor being called.
/// * `argumentCount`: An integer count of the number of arguments in `arguments`.
/// * `arguments`: A [`JSValueRef`] array of the arguments passed to the function.
/// * `exception`: A pointer to a [`JSValueRef`] in which to return an exception, if any.
///
/// Returns a [`JSObjectRef`] that is the constructor's return value.
///
/// If you named your function `CallAsConstructor`, you would declare it like this:
///
/// ```ignore
/// JSObjectRef
/// CallAsConstructor(JSContextRef ctx, JSObjectRef constructor,
///                   size_t argumentCount, const JSValueRef arguments[],
///                   JSValueRef* exception);
/// ```
///
/// If your callback were invoked by the JavaScript expression
/// `new myConstructor()`, constructor would be set to `myConstructor`.
///
/// If this callback is `NULL`, using your object as a constructor in a
/// `new` expression will throw an exception.
pub type JSObjectCallAsConstructorCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        constructor: JSObjectRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> *mut OpaqueJSValue,
>;

/// The callback invoked when an object is used as the target
/// of an `instanceof` expression.
///
/// * `ctx`: The execution context to use.
/// * `constructor`: The [`JSObjectRef`] that is the target of the
///   `instanceof` expression.
/// * `possibleInstance`: The [`JSValueRef`] being tested to determine if it
///   is an instance of `constructor`.
/// * `exception`: A pointer to a [`JSValueRef`] in which to return an exception, if any.
///
/// Returns `true` if `possibleInstance` is an instance of `constructor`,
/// otherwise `false`.
///
/// If you named your function `HasInstance`, you would declare it like this:
///
/// ```ignore
/// bool
/// HasInstance(JSContextRef ctx, JSObjectRef constructor,
///             JSValueRef possibleInstance, JSValueRef* exception);
/// ```
///
/// If your callback were invoked by the JavaScript expression
/// `someValue instanceof myObject`, constructor would be set
/// to `myObject` and `possibleInstance` would be set to `someValue`.
///
/// If this callback is `NULL`, `instanceof` expressions that target
/// your object will return `false`.
///
/// Standard JavaScript practice calls for objects that implement
/// the `callAsConstructor` callback to implement the `hasInstance`
/// callback as well.
pub type JSObjectHasInstanceCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        constructor: JSObjectRef,
        possibleInstance: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool,
>;

/// The callback invoked when converting an object to a particular
/// JavaScript type.
///
/// * `ctx`: The execution context to use.
/// * `object`: The [`JSObjectRef`] to convert.
/// * `type`: A [`JSType`] specifying the JavaScript type to convert to.
/// * `exception`: A pointer to a [`JSValueRef`] in which to return an exception, if any.
///
///Returns the objects' converted value, or `NULL` if the object was not converted.
///
/// If you named your function `ConvertToType`, you would declare it like this:
///
/// ```ignore
/// JSValueRef
/// ConvertToType(JSContextRef ctx, JSObjectRef object, JSType type,
///               JSValueRef* exception);
/// ```
///
/// If this function returns `false`, the conversion request forwards
/// to object's parent class chain (which includes the default object
/// class).
///
/// This function is only invoked when converting an object to number
/// or string. An object converted to boolean is `true`. An object
/// converted to object is itself.
pub type JSObjectConvertToTypeCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        object: JSObjectRef,
        type_: JSType,
        exception: *mut JSValueRef,
    ) -> *const OpaqueJSValue,
>;

/// A statically declared value property.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSStaticValue {
    /// A null-terminated UTF8 string containing the property's name.
    pub name: *const ::std::os::raw::c_char,
    /// A [`JSObjectGetPropertyCallback`] to invoke when getting the property's value.
    pub getProperty: JSObjectGetPropertyCallback,
    /// A [`JSObjectSetPropertyCallback`] to invoke when setting the property's value.
    /// May be `NULL` if the `ReadOnly` attribute is set.
    pub setProperty: JSObjectSetPropertyCallback,
    /// A logically ORed set of [`JSPropertyAttributes`] to give to the property.
    pub attributes: JSPropertyAttributes,
}

/// A statically declared function property.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSStaticFunction {
    /// A null-terminated UTF8 string containing the property's name.
    pub name: *const ::std::os::raw::c_char,
    /// A [`JSObjectCallAsFunctionCallback`] to invoke when the property
    /// is called as a function.
    pub callAsFunction: JSObjectCallAsFunctionCallback,
    /// A logically ORed set of [`JSPropertyAttributes`] to give to the property.
    pub attributes: JSPropertyAttributes,
}

/// Contains properties and callbacks that define a type of object.
///
/// All fields other than the version field are optional. Any pointer may be `NULL`.
///
/// The `staticValues` and `staticFunctions` arrays are the simplest and most
/// efficient means for vending custom properties. Statically declared
/// properties automatically service requests like `getProperty`,
/// `setProperty`, and `getPropertyNames`. Property access callbacks
/// are required only to implement unusual properties, like array
/// indexes, whose names are not known at compile-time.
///
/// If you named your getter function `GetX` and your setter function
/// `SetX`, you would declare a [`JSStaticValue`] array containing `"X"` like this:
///
/// ```ignore
/// JSStaticValue StaticValueArray[] = {
///     { "X", GetX, SetX, kJSPropertyAttributeNone },
///     { 0, 0, 0, 0 }
/// };
/// ```
///
/// Standard JavaScript practice calls for storing function objects in
/// prototypes, so they can be shared. The default [`JSClassRef`] created by
/// [`JSClassCreate`] follows this idiom, instantiating objects with a
/// shared, automatically generating prototype containing the class's
/// function objects. The [`kJSClassAttributeNoAutomaticPrototype`]
/// attribute specifies that a [`JSClassRef`] should not automatically
/// generate such a prototype. The resulting [`JSClassRef`] instantiates
/// objects with the default object prototype, and gives each instance
/// object its own copy of the class's function objects.
///
/// A `NULL` callback specifies that the default object callback
/// should substitute, except in the case of `hasProperty`, where it
/// specifies that `getProperty` should substitute.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSClassDefinition {
    /// The version number of this structure. The current version is 0.
    pub version: ::std::os::raw::c_int,
    /// A logically ORed set of [`JSClassAttributes`] to give to the class.
    pub attributes: JSClassAttributes,
    /// A null-terminated UTF8 string containing the class's name.
    pub className: *const ::std::os::raw::c_char,
    /// A [`JSClassRef`] to set as the class's parent class. Pass `NULL` use the default object class.
    pub parentClass: JSClassRef,
    /// A [`JSStaticValue`] array containing the class's statically declared
    /// value properties. Pass `NULL` to specify no statically declared
    /// value properties. The array must be terminated by a [`JSStaticValue`]
    /// whose name field is `NULL`.
    pub staticValues: *const JSStaticValue,
    /// A [`JSStaticFunction`] array containing the class's statically
    /// declared function properties. Pass `NULL` to specify no
    /// statically declared function properties. The array must be
    /// terminated by a [`JSStaticFunction`] whose name field is `NULL`.
    pub staticFunctions: *const JSStaticFunction,
    /// The callback invoked when an object is first created. Use this callback
    /// to initialize the object.
    pub initialize: JSObjectInitializeCallback,
    /// The callback invoked when an object is finalized (prepared for garbage
    /// collection). Use this callback to release resources allocated for the
    /// object, and perform other cleanup.
    pub finalize: JSObjectFinalizeCallback,
    /// The callback invoked when determining whether an object has a property.
    ///
    /// If this field is `NULL`, `getProperty` is called instead. The
    /// `hasProperty` callback enables optimization in cases where
    /// only a property's existence needs to be known, not its value,
    /// and computing its value is expensive.
    pub hasProperty: JSObjectHasPropertyCallback,
    /// The callback invoked when getting a property's value.
    pub getProperty: JSObjectGetPropertyCallback,
    /// The callback invoked when setting a property's value.
    pub setProperty: JSObjectSetPropertyCallback,
    /// The callback invoked when deleting a property.
    pub deleteProperty: JSObjectDeletePropertyCallback,
    /// The callback invoked when collecting the names of an object's properties.
    pub getPropertyNames: JSObjectGetPropertyNamesCallback,
    /// The callback invoked when an object is called as a function.
    pub callAsFunction: JSObjectCallAsFunctionCallback,
    /// The callback invoked when an object is used as a constructor in a `new` expression.
    pub callAsConstructor: JSObjectCallAsConstructorCallback,
    /// The callback invoked when an object is used as the target of an `instanceof` expression.
    pub hasInstance: JSObjectHasInstanceCallback,
    /// The callback invoked when converting an object to a particular JavaScript type.
    pub convertToType: JSObjectConvertToTypeCallback,
}

impl Default for JSClassDefinition {
    fn default() -> Self {
        JSClassDefinition {
            version: 0,
            attributes: 0,
            className: ptr::null(),
            parentClass: ptr::null_mut(),
            staticValues: ptr::null(),
            staticFunctions: ptr::null(),
            initialize: None,
            finalize: None,
            hasProperty: None,
            getProperty: None,
            setProperty: None,
            deleteProperty: None,
            getPropertyNames: None,
            callAsFunction: None,
            callAsConstructor: None,
            hasInstance: None,
            convertToType: None,
        }
    }
}

extern "C" {
    /// Creates a JavaScript class suitable for use with [`JSObjectMake`].
    ///
    /// * `definition`: A [`JSClassDefinition`] that defines the class.
    ///
    /// Returns a [`JSClassRef`] with the given definition. Ownership follows
    /// the Create Rule.
    pub fn JSClassCreate(definition: *const JSClassDefinition) -> JSClassRef;

    /// Retains a JavaScript class.
    ///
    /// `jsClass`: The [`JSClassRef`] to retain.
    ///
    /// Returns a [`JSClassRef`] that is the same as `jsClass`.
    pub fn JSClassRetain(jsClass: JSClassRef) -> JSClassRef;

    /// Releases a JavaScript class.
    ///
    /// `jsClass`: The [`JSClassRef`] to release.
    pub fn JSClassRelease(jsClass: JSClassRef);

    /// Creates a JavaScript object.
    ///
    /// The default object class does not allocate storage for private data,
    /// so you must provide a non-`NULL` `jsClass` to `JSObjectMake` if you
    /// want your object to be able to store private data.
    ///
    /// `data` is set on the created object before the initialize methods in
    /// its class chain are called. This enables the initialize methods to
    /// retrieve and manipulate data through [`JSObjectGetPrivate`].
    ///
    /// * `ctx`: The execution context to use.
    /// * `jsClass`: The [`JSClassRef`] to assign to the object. Pass `NULL` to use
    ///   the default object class.
    /// * `data`: A `void*` to set as the object's private data.
    ///    Pass `NULL` to specify no private data.
    ///
    /// Returns a [`JSObjectRef`] with the given class and private data.
    pub fn JSObjectMake(
        ctx: JSContextRef,
        jsClass: JSClassRef,
        data: *mut ::std::os::raw::c_void,
    ) -> JSObjectRef;

    /// Convenience method for creating a JavaScript function with a given
    /// callback as its implementation.
    ///
    /// * `ctx`: The execution context to use.
    /// * `name`: A [`JSStringRef`] containing the function's name. This will be
    ///   used when converting the function to string. Pass `NULL` to create
    ///   an anonymous function.
    /// * `callAsFunction`: The [`JSObjectCallAsFunctionCallback`] to invoke
    ///   when the function is called.
    ///
    /// Returns a [`JSObjectRef`] that is a function. The object's prototype will be
    /// the default function prototype.
    pub fn JSObjectMakeFunctionWithCallback(
        ctx: JSContextRef,
        name: JSStringRef,
        callAsFunction: JSObjectCallAsFunctionCallback,
    ) -> JSObjectRef;

    /// Convenience method for creating a JavaScript constructor.
    ///
    /// * `ctx`: The execution context to use.
    /// * `jsClass`: A [`JSClassRef`] that is the class your constructor
    ///   will assign to the objects its constructs. `jsClass` will
    ///   be used to set the constructor's `.prototype` property, and
    ///   to evaluate `instanceof` expressions. Pass `NULL` to use
    ///   the default object class.
    /// * `callAsConstructor` A [`JSObjectCallAsConstructorCallback`] to
    ///   invoke when your constructor is used in a `new` expression.
    ///   Pass `NULL` to use the default object constructor.
    ///
    /// Returns a [`JSObjectRef`] that is a constructor. The object's
    /// prototype will be the default object prototype.
    ///
    /// The default object constructor takes no arguments and constructs
    /// an object of class `jsClass` with no private data.
    pub fn JSObjectMakeConstructor(
        ctx: JSContextRef,
        jsClass: JSClassRef,
        callAsConstructor: JSObjectCallAsConstructorCallback,
    ) -> JSObjectRef;

    /// Creates a JavaScript Array object.
    ///
    /// * `ctx`: The execution context to use.
    /// * `argumentCount`: An integer count of the number of
    ///   arguments in `arguments`.
    /// * `arguments`: A [`JSValueRef`] array of data to populate the
    ///   `Array` with. Pass `NULL` if `argumentCount` is `0`.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns a [`JSObjectRef`] that is an `Array`.
    ///
    /// The behavior of this function does not exactly match the behavior
    /// of the built-in `Array` constructor. Specifically, if one argument
    ///  is supplied, this function returns an array with one element.
    pub fn JSObjectMakeArray(
        ctx: JSContextRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;

    /// Creates a JavaScript `Date` object, as if by invoking the
    /// built-in `Date` constructor.
    ///
    /// * `ctx`: The execution context to use.
    /// * `argumentCount`: An integer count of the number of
    ///   arguments in `arguments`.
    /// * `arguments`: A [`JSValueRef`] array of arguments to pass to
    ///   the `Date` constructor. Pass `NULL` if `argumentCount` is `0`.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns a [`JSObjectRef`] that is a `Date`.
    pub fn JSObjectMakeDate(
        ctx: JSContextRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;

    /// Creates a JavaScript `Error` object, as if by invoking the
    /// built-in `Error` constructor.
    ///
    /// * `ctx`: The execution context to use.
    /// * `argumentCount`: An integer count of the number of
    ///   arguments in `arguments`.
    /// * `arguments`: A [`JSValueRef`] array of arguments to pass to
    ///   the `Error` constructor. Pass `NULL` if `argumentCount` is `0`.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns a [`JSObjectRef`] that is a `Error`.
    pub fn JSObjectMakeError(
        ctx: JSContextRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;

    /// Creates a JavaScript `RegExp` object, as if by invoking the
    /// built-in `RegExp` constructor.
    ///
    /// * `ctx`: The execution context to use.
    /// * `argumentCount`: An integer count of the number of
    ///   arguments in `arguments`.
    /// * `arguments`: A [`JSValueRef`] array of arguments to pass to
    ///   the `RegExp` constructor. Pass `NULL` if `argumentCount` is `0`.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns a [`JSObjectRef`] that is a `RegExp`.
    pub fn JSObjectMakeRegExp(
        ctx: JSContextRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;

    /// Creates a JavaScript promise object by invoking the provided executor.
    ///
    /// * `ctx`: The execution context to use.
    /// * `resolve`: A pointer to a [`JSObjectRef`] in which to store the
    ///   resolve function for the new promise. Pass `NULL` if you do not
    ///   care to store the resolve callback.
    /// * `reject`: A pointer to a [`JSObjectRef`] in which to store the
    ///   reject function for the new promise. Pass `NULL` if you do not
    ///   care to store the reject callback.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// A [`JSObjectRef`] that is a promise or `NULL` if an exception occurred.
    pub fn JSObjectMakeDeferredPromise(
        ctx: JSContextRef,
        resolve: *mut JSObjectRef,
        reject: *mut JSObjectRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;

    /// Creates a function with a given script as its body.
    ///
    /// * `ctx`: The execution context to use.
    /// * `name`: A [`JSStringRef`] containing the function's name. This
    ///   will be used when converting the function to string. Pass
    ///   `NULL` to create an anonymous function.
    /// * `parameterCount`: An integer count of the number of parameter
    ///   names in `parameterNames`.
    /// * `parameterNames`: A [`JSStringRef`] array containing the names of
    ///   the function's parameters. Pass `NULL` if `parameterCount` is `0`.
    /// * `body`: A [`JSStringRef`] containing the script to use as the
    ///   function's body.
    /// * `sourceURL` A [`JSStringRef`] containing a URL for the script's
    ///   source file. This is only used when reporting exceptions.
    ///   Pass `NULL` if you do not care to include source file
    ///   information in exceptions.
    /// * `startingLineNumber`: An integer value specifying the
    ///   script's starting line number in the file located at
    ///   `sourceURL`. This is only used when reporting exceptions.
    ///   The value is one-based, so the first line is line `1`
    ///   and invalid values are clamped to `1`.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns a [`JSObjectRef`] that is a function, or `NULL` if either
    /// body or `parameterNames` contains a syntax error. The
    /// object's prototype will be the default function prototype.
    ///
    /// Use this method when you want to execute a script repeatedly, to
    /// avoid the cost of re-parsing the script before each execution.
    pub fn JSObjectMakeFunction(
        ctx: JSContextRef,
        name: JSStringRef,
        parameterCount: ::std::os::raw::c_uint,
        parameterNames: *const JSStringRef,
        body: JSStringRef,
        sourceURL: JSStringRef,
        startingLineNumber: ::std::os::raw::c_int,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;

    /// Gets an object's prototype.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: A [`JSObjectRef`] whose prototype you want to get.
    ///
    /// Returns a [`JSValueRef`] that is the object's prototype.
    ///
    /// # See also
    ///
    /// * [`JSObjectSetPrototype()`]
    pub fn JSObjectGetPrototype(ctx: JSContextRef, object: JSObjectRef) -> JSValueRef;

    ///Sets an object's prototype.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The [`JSObjectRef`] whose prototype you want to set.
    /// * `value`: A [`JSValueRef`] to set as the object's prototype.
    ///
    /// # See also
    ///
    /// * [`JSObjectGetPrototype()`]
    pub fn JSObjectSetPrototype(ctx: JSContextRef, object: JSObjectRef, value: JSValueRef);

    /// Tests whether an object has a given property.
    ///
    /// * `object`: The [`JSObjectRef`] to test.
    /// * `propertyName`: A [`JSStringRef`] containing the property's name.
    ///
    /// Returns `true` if the object has a property whose name matches
    /// `propertyName`, otherwise `false`.
    ///
    /// # See also
    ///
    /// * [`JSClassDefinition::hasProperty`]
    /// * [`JSObjectDeleteProperty()`]
    /// * [`JSObjectGetProperty()`]
    /// * [`JSObjectHasPropertyCallback`]
    /// * [`JSObjectHasPropertyForKey()`]
    /// * [`JSObjectSetProperty()`]
    pub fn JSObjectHasProperty(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
    ) -> bool;

    /// Gets a property from an object.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The [`JSObjectRef`] whose property you want to get.
    /// * `propertyName`: A [`JSStringRef`] containing the property's name.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns the property's value if object has the property, otherwise
    /// the undefined value.
    ///
    /// # See also
    ///
    /// * [`JSClassDefinition::getProperty`]
    /// * [`JSObjectDeleteProperty()`]
    /// * [`JSObjectGetPropertyAtIndex()`]
    /// * [`JSObjectGetPropertyCallback`]
    /// * [`JSObjectGetPropertyForKey()`]
    /// * [`JSObjectHasProperty()`]
    /// * [`JSObjectSetProperty()`]
    pub fn JSObjectGetProperty(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        exception: *mut JSValueRef,
    ) -> JSValueRef;

    /// Sets a property on an object.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The [`JSObjectRef`] whose property you want to set.
    /// * `propertyName`: A [`JSStringRef`] containing the property's name.
    /// * `value`: A [`JSValueRef`] to use as the property's value.
    /// * `attributes`: A logically ORed set of [`JSPropertyAttributes`] to give to the property.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// # See also
    ///
    /// * [`JSClassDefinition::setProperty`]
    /// * [`JSObjectDeleteProperty()`]
    /// * [`JSObjectGetProperty()`]
    /// * [`JSObjectHasProperty()`]
    /// * [`JSObjectSetPropertyAtIndex()`]
    /// * [`JSObjectSetPropertyCallback`]
    /// * [`JSObjectGetPropertyForKey()`]
    pub fn JSObjectSetProperty(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        value: JSValueRef,
        attributes: JSPropertyAttributes,
        exception: *mut JSValueRef,
    );

    /// Deletes a property from an object.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The [`JSObjectRef`] whose property you want to delete.
    /// * `propertyName`: A [`JSStringRef`] containing the property's name.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns `true` if the delete operation succeeds, otherwise `false`
    /// (for example, if the property has the [`kJSPropertyAttributeDontDelete`]
    /// attribute set).
    ///
    /// # See also
    ///
    /// * [`JSClassDefinition::deleteProperty`]
    /// * [`JSObjectDeletePropertyCallback`]
    /// * [`JSObjectDeletePropertyForKey()`]
    /// * [`JSObjectGetProperty()`]
    /// * [`JSObjectHasProperty()`]
    /// * [`JSObjectSetProperty()`]
    pub fn JSObjectDeleteProperty(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        exception: *mut JSValueRef,
    ) -> bool;

    /// Tests whether an object has a given property using a [`JSValueRef`] as the property key.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The [`JSObjectRef`] to test.
    /// * `propertyKey`: A [`JSValueRef`] containing the property key
    ///   to use when looking up the property.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns `true` if the object has a property whose name matches
    /// `propertyKey`, otherwise `false`.
    ///
    /// This function is the same as performing `propertyKey in object` from JavaScript.
    ///
    /// # See also
    ///
    /// * [`JSObjectDeletePropertyForKey()`]
    /// * [`JSObjectGetPropertyForKey()`]
    /// * [`JSObjectHasProperty()`]
    /// * [`JSObjectSetPropertyForKey()`]
    pub fn JSObjectHasPropertyForKey(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyKey: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool;

    /// Gets a property from an object using a [`JSValueRef`] as the property key.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The [`JSObjectRef`] whose property you want to get.
    /// * `propertyKey`: A [`JSValueRef`] containing the property key
    ///   to use when looking up the property.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// The property's value if object has the property key, otherwise the undefined value.
    ///
    /// This function is the same as performing `object[propertyKey]` from JavaScript.
    ///
    /// # See also
    ///
    /// * [`JSObjectDeletePropertyForKey()`]
    /// * [`JSObjectGetProperty()`]
    /// * [`JSObjectGetPropertyAtIndex()`]
    /// * [`JSObjectHasPropertyForKey()`]
    /// * [`JSObjectSetPropertyForKey()`]
    pub fn JSObjectGetPropertyForKey(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyKey: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSValueRef;

    /// Sets a property on an object using a [`JSValueRef`] as the property key.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object `:The [`JSObjectRef`] whose property you want to set.
    /// * `propertyKey`: A [`JSValueRef`] containing the property key
    ///   to use when looking up the property.
    /// * `value`: A [`JSValueRef`] to use as the property's value.
    /// * `attributes`: A logically ORed set of [`JSPropertyAttributes`]
    ///   to give to the property.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// This function is the same as performing `object[propertyKey] = value` from JavaScript.
    ///
    /// # See also
    ///
    /// * [`JSObjectDeletePropertyForKey()`]
    /// * [`JSObjectGetPropertyForKey()`]
    /// * [`JSObjectHasPropertyForKey()`]
    /// * [`JSObjectSetProperty()`]
    /// * [`JSObjectSetPropertyAtIndex()`]
    pub fn JSObjectSetPropertyForKey(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyKey: JSValueRef,
        value: JSValueRef,
        attributes: JSPropertyAttributes,
        exception: *mut JSValueRef,
    );

    /// Deletes a property from an object using a [`JSValueRef`] as the property key.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The [`JSObjectRef`] whose property you want to delete.
    /// * `propertyKey`: A [`JSValueRef`] containing the property key
    ///   to use when looking up the property.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns `true` if the delete operation succeeds, otherwise `false`
    /// (for example, if the property has the `kJSPropertyAttributeDontDelete`
    /// attribute set).
    ///
    /// This function is the same as performing `delete object[propertyKey]` from JavaScript.
    ///
    /// # See also
    ///
    /// * [`JSObjectDeleteProperty()`]
    /// * [`JSObjectGetPropertyForKey()`]
    /// * [`JSObjectHasPropertyForKey()`]
    /// * [`JSObjectSetPropertyForKey()`]
    pub fn JSObjectDeletePropertyForKey(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyKey: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool;

    /// Gets a property from an object by numeric index.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The [`JSObjectRef`] whose property you want to get.
    /// * `propertyIndex`: An integer value that is the property's name.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns the property's value if object has the property,
    /// otherwise the undefined value.
    ///
    /// Calling [`JSObjectGetPropertyAtIndex`] is equivalent to calling
    /// [`JSObjectGetProperty`] with a string containing `propertyIndex`,
    /// but `JSObjectGetPropertyAtIndex` provides optimized access to
    /// numeric properties.
    ///
    /// # See also
    ///
    /// * [`JSObjectGetProperty()`]
    /// * [`JSObjectGetPropertyForKey()`]
    /// * [`JSObjectSetPropertyAtIndex()`]
    pub fn JSObjectGetPropertyAtIndex(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyIndex: ::std::os::raw::c_uint,
        exception: *mut JSValueRef,
    ) -> JSValueRef;

    /// Sets a property on an object by numeric index.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The [`JSObjectRef`] whose property you want to set.
    /// * `propertyIndex`: The property's name as a number.
    /// * `value`: A [`JSValueRef`] to use as the property's value.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Calling `JSObjectSetPropertyAtIndex` is equivalent to calling
    /// [`JSObjectSetProperty`] with a string containing `propertyIndex`,
    /// but `JSObjectSetPropertyAtIndex` provides optimized access to
    /// numeric properties.
    ///
    /// # See also
    ///
    /// * [`JSObjectGetPropertyAtIndex()`]
    /// * [`JSObjectSetProperty()`]
    /// * [`JSObjectSetPropertyForKey()`]
    pub fn JSObjectSetPropertyAtIndex(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyIndex: ::std::os::raw::c_uint,
        value: JSValueRef,
        exception: *mut JSValueRef,
    );

    /// Gets an object's private data.
    ///
    /// * `object`: A [`JSObjectRef`] whose private data you want to get.
    ///
    /// Returns a `void*` that is the object's private data, if the
    /// object has private data, otherwise `NULL`.
    ///
    /// # See also
    ///
    /// * [`JSObjectMake()`]
    /// * [`JSObjectSetPrivate()`]
    pub fn JSObjectGetPrivate(object: JSObjectRef) -> *mut ::std::os::raw::c_void;

    /// Sets a pointer to private data on an object.
    ///
    /// * `object`: The [`JSObjectRef`] whose private data you want to set.
    /// * `data`: A `void*` to set as the object's private data.
    ///
    /// Returns `true` if object can store private data, otherwise `false`.
    ///
    /// The default object class does not allocate storage for private data.
    /// Only objects created with a non-`NULL` [`JSClassRef`] can store private data.
    ///
    /// # See also
    ///
    /// * [`JSObjectGetPrivate()`]
    /// * [`JSObjectMake()`]
    pub fn JSObjectSetPrivate(object: JSObjectRef, data: *mut ::std::os::raw::c_void) -> bool;

    /// Tests whether an object can be called as a function.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The [`JSObjectRef`] to test.
    ///
    /// Returns `true` if the object can be called as a function, otherwise `false`.
    ///
    /// # See also
    ///
    /// * [`JSObjectCallAsFunction()`]
    pub fn JSObjectIsFunction(ctx: JSContextRef, object: JSObjectRef) -> bool;

    /// Calls an object as a function.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The [`JSObjectRef`] to call as a function.
    /// * `thisObject`: The object to use as `this`, or `NULL` to use the global object as `this`.
    /// * `argumentCount`: An integer count of the number of arguments in `arguments`.
    /// * `arguments`: A [`JSValueRef`] array of arguments to pass to the function.
    ///   Pass `NULL` if `argumentCount` is `0`.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns the [`JSValueRef`] that results from calling `object` as a function,
    /// or `NULL` if an exception is thrown or `object` is not a function.
    ///
    /// # See also
    ///
    /// * [`JSObjectCallAsFunction()`]
    pub fn JSObjectCallAsFunction(
        ctx: JSContextRef,
        object: JSObjectRef,
        thisObject: JSObjectRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSValueRef;

    /// Tests whether an object can be called as a constructor.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The [`JSObjectRef`] to test.
    ///
    /// Returns `true` if the object can be called as a constructor, otherwise `false`.
    pub fn JSObjectIsConstructor(ctx: JSContextRef, object: JSObjectRef) -> bool;

    /// Calls an object as a constructor.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The [`JSObjectRef`] to call as a constructor.
    /// * `argumentCount`: An integer count of the number of arguments in `arguments`.
    /// * `arguments`: A [`JSValueRef`] array of arguments to pass to the constructor.
    ///   Pass `NULL` if `argumentCount` is `0`.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns the [`JSObjectRef`] that results from calling `object` as a constructor,
    /// or `NULL` if an exception is thrown or `object` is not a constructor.
    pub fn JSObjectCallAsConstructor(
        ctx: JSContextRef,
        object: JSObjectRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;

    /// Gets the names of an object's enumerable properties.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The object whose property names you want to get.
    ///
    /// Returns a [`JSPropertyNameArrayRef`] containing the names of
    /// `object`'s enumerable properties. Ownership follows the Create
    /// Rule.
    ///
    /// # See also
    ///
    /// * [`JSPropertyNameArrayGetCount`]
    /// * [`JSPropertyNameArrayGetNameAtIndex`]
    /// * [`JSPropertyNameArrayRelease`]
    /// * [`JSPropertyNameArrayRetain`]
    pub fn JSObjectCopyPropertyNames(
        ctx: JSContextRef,
        object: JSObjectRef,
    ) -> JSPropertyNameArrayRef;

    /// Retains a JavaScript property name array.
    ///
    /// * `array`: The [`JSPropertyNameArrayRef`] to retain.
    ///
    /// Returns a [`JSPropertyNameArrayRef`] that is the same as array.
    ///
    /// # See also
    ///
    /// * [`JSPropertyNameArrayRelease()`]
    pub fn JSPropertyNameArrayRetain(array: JSPropertyNameArrayRef) -> JSPropertyNameArrayRef;

    /// Releases a JavaScript property name array.
    ///
    /// * `array` The [`JSPropertyNameArrayRef`] to release.
    ///
    /// # See also
    ///
    /// * [`JSPropertyNameArrayRetain()`]
    pub fn JSPropertyNameArrayRelease(array: JSPropertyNameArrayRef);

    /// Gets a count of the number of items in a JavaScript property name array.
    ///
    /// * `array`: The array from which to retrieve the count.
    ///
    /// Return an integer count of the number of names in `array`.
    ///
    /// # See also
    ///
    /// * [`JSObjectCopyPropertyNames()`]
    /// * [`JSPropertyNameArrayGetNameAtIndex`]
    pub fn JSPropertyNameArrayGetCount(array: JSPropertyNameArrayRef) -> usize;

    /// Gets a property name at a given index in a JavaScript property name array.
    ///
    /// * `array`: The array from which to retrieve the property name.
    /// * `index`: The index of the property name to retrieve.
    ///
    /// Returns a [`JSStringRef`] containing the property name.
    ///
    /// # See also
    ///
    /// * [`JSObjectCopyPropertyNames()`]
    /// * [`JSPropertyNameArrayGetCount`]
    pub fn JSPropertyNameArrayGetNameAtIndex(
        array: JSPropertyNameArrayRef,
        index: usize,
    ) -> JSStringRef;

    /// Adds a property name to a JavaScript property name accumulator.
    ///
    /// * `accumulator`: The accumulator object to which to add the property name.
    /// * `propertyName`: The property name to add.
    pub fn JSPropertyNameAccumulatorAddName(
        accumulator: JSPropertyNameAccumulatorRef,
        propertyName: JSStringRef,
    );

    /// Creates a JavaScript context group.
    ///
    /// [`JSContextGroupRef`] associates JavaScript contexts with one another.
    /// Contexts in the same group may share and exchange JavaScript
    /// objects. Sharing and/or exchanging JavaScript objects between
    /// contexts in different groups will produce undefined behavior.
    /// When objects from the same context group are used in multiple threads,
    /// explicit synchronization is required.
    ///
    /// Returns the created [`JSContextGroupRef`].
    pub fn JSContextGroupCreate() -> JSContextGroupRef;

    /// Retains a JavaScript context group.
    ///
    /// * `group`: The [`JSContextGroupRef`] to retain.
    ///
    /// Returns a [`JSContextGroupRef`] that is the same as group.
    pub fn JSContextGroupRetain(group: JSContextGroupRef) -> JSContextGroupRef;

    /// Releases a JavaScript context group.
    ///
    /// * `group`: The [`JSContextGroupRef`] to release.
    pub fn JSContextGroupRelease(group: JSContextGroupRef);

    /// Creates a global JavaScript execution context.
    ///
    /// `JSGlobalContextCreate` allocates a global object and populates
    /// it with all the built-in JavaScript objects, such as `Object`,
    /// `Function`, `String`, and `Array`.
    ///
    /// In WebKit version 4.0 and later, the context is created in a
    /// unique context group. Therefore, scripts may execute in it
    /// concurrently with scripts executing in other contexts. However,
    /// you may not use values created in the context in other contexts.
    ///
    /// * `globalObjectClass`: The class to use when creating the global
    ///   object. Pass `NULL` to use the default object class.
    ///
    /// Returns a [`JSGlobalContextRef`] with a global object of
    /// class `globalObjectClass`.
    pub fn JSGlobalContextCreate(globalObjectClass: JSClassRef) -> JSGlobalContextRef;

    /// Creates a global JavaScript execution context in the context
    /// group provided.
    ///
    /// `JSGlobalContextCreateInGroup` allocates a global object and
    /// populates it with all the built-in JavaScript objects, such as
    /// `Object`, `Function`, `String`, and `Array`.
    ///
    /// * `group`: The context group to use. The created global context
    ///   retains the group.  Pass `NULL` to create a unique group for
    ///   the context.
    /// * `globalObjectClass`: The class to use when creating the global
    ///   object. Pass NULL to use the default object class.
    ///
    /// Returns a [`JSGlobalContextRef`] with a global object of class
    /// `globalObjectClass` and a context group equal to `group`.
    pub fn JSGlobalContextCreateInGroup(
        group: JSContextGroupRef,
        globalObjectClass: JSClassRef,
    ) -> JSGlobalContextRef;

    /// Retains a global JavaScript execution context.
    ///
    /// * `ctx`: The [`JSGlobalContextRef`] to retain.
    ///
    /// Returns a [`JSGlobalContextRef`] that is the same as `ctx`.
    pub fn JSGlobalContextRetain(ctx: JSGlobalContextRef) -> JSGlobalContextRef;

    /// Releases a global JavaScript execution context.
    ///
    /// * `ctx` The [`JSGlobalContextRef`] to release.
    pub fn JSGlobalContextRelease(ctx: JSGlobalContextRef);

    /// Gets the global object of a JavaScript execution context.
    ///
    /// * `ctx` The [`JSContextRef`] whose global object you want to get.
    ///
    /// Returns `ctx`'s global object.
    pub fn JSContextGetGlobalObject(ctx: JSContextRef) -> JSObjectRef;

    /// Gets the context group to which a JavaScript execution context belongs.
    ///
    /// * `ctx`: The [`JSContextRef`] whose group you want to get.
    ///
    /// Returns `ctx`'s group.
    pub fn JSContextGetGroup(ctx: JSContextRef) -> JSContextGroupRef;

    /// Gets the global context of a JavaScript execution context.
    ///
    /// * `ctx`: The [`JSContextRef`] whose global context you want to get.
    ///
    /// Returns `ctx`'s global context.
    pub fn JSContextGetGlobalContext(ctx: JSContextRef) -> JSGlobalContextRef;

    /// Gets a copy of the name of a context.
    ///
    /// A [`JSGlobalContextRef`]'s name is exposed for remote debugging
    /// to make it easier to identify the context you would like to
    /// attach to.
    ///
    /// * `ctx`: The [`JSGlobalContextRef`] whose name you want to get.
    ///
    /// Returns the name for `ctx`.
    pub fn JSGlobalContextCopyName(ctx: JSGlobalContextRef) -> JSStringRef;

    /// Sets the remote debugging name for a context.
    ///
    /// * `ctx`: The [`JSGlobalContextRef`] that you want to name.
    /// * `name`: The remote debugging name to set on `ctx`.
    pub fn JSGlobalContextSetName(ctx: JSGlobalContextRef, name: JSStringRef);
}
/// A UTF-16 code unit.
///
/// One, or a sequence of two, can encode any Unicode character. As
/// with all scalar types, endianness depends on the underlying
/// architecture.
pub type JSChar = ::std::os::raw::c_ushort;
extern "C" {
    /// Creates a JavaScript string from a buffer of Unicode characters.
    ///
    /// * `chars`: The buffer of Unicode characters to copy into the
    ///   new [`JSStringRef`].
    /// * `numChars`: The number of characters to copy from the buffer
    ///   pointed to by `chars`.
    ///
    /// Returns a [`JSStringRef`] containing `chars`. Ownership follows the
    /// Create Rule.
    pub fn JSStringCreateWithCharacters(chars: *const JSChar, numChars: usize) -> JSStringRef;

    /// Creates a JavaScript string from a null-terminated UTF8 string.
    ///
    /// * `string`: The null-terminated UTF8 string to copy into the
    ///   new [`JSStringRef`].
    ///
    /// Returns a [`JSStringRef`] containing `string`. Ownership follows the
    /// Create Rule.
    pub fn JSStringCreateWithUTF8CString(string: *const ::std::os::raw::c_char) -> JSStringRef;

    /// Retains a JavaScript string.
    ///
    /// * `string`: The [`JSStringRef`] to retain.
    ///
    /// Returns a [`JSStringRef`] that is the same as `string`.
    pub fn JSStringRetain(string: JSStringRef) -> JSStringRef;

    /// Releases a JavaScript string.
    ///
    /// * `string`: The [`JSStringRef`] to release.
    pub fn JSStringRelease(string: JSStringRef);

    /// Returns the number of Unicode characters in a JavaScript string.
    ///
    /// * `string`: The [`JSStringRef`] whose length (in Unicode characters)
    ///   you want to know.
    ///
    /// Returns the number of Unicode characters stored in `string`.
    pub fn JSStringGetLength(string: JSStringRef) -> usize;

    /// Returns a pointer to the Unicode character buffer that
    /// serves as the backing store for a JavaScript string.
    ///
    /// * `string`: The [`JSStringRef`] whose backing store you want to access.
    ///
    /// Returns a pointer to the Unicode character buffer that serves
    /// as `string`'s backing store, which will be deallocated when
    /// `string` is deallocated.
    pub fn JSStringGetCharactersPtr(string: JSStringRef) -> *const JSChar;

    /// Returns the maximum number of bytes a JavaScript string will
    /// take up if converted into a null-terminated UTF8 string.
    ///
    /// * `string`: The [`JSStringRef`] whose maximum converted size (in bytes)
    ///   you want to know.
    ///
    /// Returns the maximum number of bytes that could be required to
    /// convert `string` into a null-terminated UTF8 string. The number
    /// of bytes that the conversion actually ends up requiring could
    /// be less than this, but never more.
    pub fn JSStringGetMaximumUTF8CStringSize(string: JSStringRef) -> usize;

    /// Converts a JavaScript string into a null-terminated UTF8 string,
    /// and copies the result into an external byte buffer.
    ///
    /// * `string`: The source [`JSStringRef`].
    /// * `buffer`: The destination byte buffer into which to copy a
    ///   null-terminated UTF8 representation of `string`. On return,
    ///   `buffer` contains a UTF8 string representation of `string`. If
    ///   `bufferSize` is too small, `buffer` will contain only
    ///   partial results. If `buffer` is not at least `bufferSize`
    ///   bytes in size, behavior is undefined.
    /// * `bufferSize`: The size of the external buffer in bytes.
    ///
    /// Returns the number of bytes written into buffer (including the null-terminator byte).
    pub fn JSStringGetUTF8CString(
        string: JSStringRef,
        buffer: *mut ::std::os::raw::c_char,
        bufferSize: usize,
    ) -> usize;

    /// Tests whether two JavaScript strings match.
    ///
    /// * `a`: The first [`JSStringRef`] to test.
    /// * `b`: The second [`JSStringRef`] to test.
    ///
    /// Returns `true` if the two strings match, otherwise `false`.
    pub fn JSStringIsEqual(a: JSStringRef, b: JSStringRef) -> bool;

    /// Tests whether a JavaScript string matches a null-terminated
    /// UTF8 string.
    ///
    /// * `a`: The [`JSStringRef`] to test.
    /// * `b`: The null-terminated UTF8 string to test.
    ///
    /// Returns `true` if the two strings match, otherwise `false`.
    pub fn JSStringIsEqualToUTF8CString(a: JSStringRef, b: *const ::std::os::raw::c_char) -> bool;

    /// Creates a JavaScript Typed Array object with the given number of elements.
    ///
    /// * `ctx`: The execution context to use.
    /// * `arrayType`: A value identifying the type of array to
    ///   create. If `arrayType` is `JSTypedArrayType::None` or
    ///   `JSTypedArrayType::ArrayBuffer` then `NULL` will be returned.
    /// * `length`: The number of elements to be in the new Typed Array.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns a [`JSObjectRef`] that is a Typed Array with all elements set to
    /// zero or `NULL` if there was an error.
    pub fn JSObjectMakeTypedArray(
        ctx: JSContextRef,
        arrayType: JSTypedArrayType,
        length: usize,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;

    /// Creates a JavaScript Typed Array object from an existing pointer.
    ///
    /// * `ctx`: The execution context to use.
    /// * `arrayType`: A value identifying the type of array to
    ///   create. If `arrayType` is `JSTypedArrayType::None` or
    ///   `JSTypedArrayType::ArrayBuffer` then `NULL` will be returned.
    /// * `bytes`: A pointer to the byte buffer to be used as the backing store
    ///   of the Typed Array object.
    /// * `byteLength`: The number of bytes pointed to by the parameter bytes.
    /// * `bytesDeallocator`: The allocator to use to deallocate the external
    ///    buffer when the `JSTypedArrayData` object is deallocated.
    /// * `deallocatorContext`: A pointer to pass back to the deallocator.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns a [`JSObjectRef`] Typed Array whose backing store is the same as
    /// the one pointed to by `bytes` or `NULL` if there was an error.
    ///
    /// If an exception is thrown during this function the `bytesDeallocator`
    /// will always be called.
    pub fn JSObjectMakeTypedArrayWithBytesNoCopy(
        ctx: JSContextRef,
        arrayType: JSTypedArrayType,
        bytes: *mut ::std::os::raw::c_void,
        byteLength: usize,
        bytesDeallocator: JSTypedArrayBytesDeallocator,
        deallocatorContext: *mut ::std::os::raw::c_void,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;

    /// Creates a JavaScript Typed Array object from an existing
    /// JavaScript Array Buffer object.
    ///
    /// * `ctx`: The execution context to use.
    /// * `arrayType`: A value identifying the type of array to
    ///   create. If `arrayType` is `JSTypedArrayType::None` or
    ///   `JSTypedArrayType::ArrayBuffer` then `NULL` will be returned.
    /// * `buffer`: An Array Buffer object that should be used as the
    ///   backing store for the created JavaScript Typed Array object.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns a [`JSObjectRef`] that is a Typed Array or `NULL` if there
    /// was an error. The backing store of the Typed Array will be `buffer`.
    pub fn JSObjectMakeTypedArrayWithArrayBuffer(
        ctx: JSContextRef,
        arrayType: JSTypedArrayType,
        buffer: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;

    /// Creates a JavaScript Typed Array object from an existing
    /// JavaScript Array Buffer object with the given offset and
    /// length.
    ///
    /// * `ctx`: The execution context to use.
    /// * `arrayType`: A value identifying the type of array to
    ///   create. If `arrayType` is `JSTypedArrayType::None` or
    ///   `JSTypedArrayType::ArrayBuffer` then `NULL` will be returned.
    /// * `buffer`: An Array Buffer object that should be used as the
    ///   backing store for the created JavaScript Typed Array object.
    /// * `byteOffset`: The byte offset for the created Typed Array.
    ///   `byteOffset` should aligned with the element size of `arrayType`.
    /// * `length`: The number of elements to include in the Typed Array.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns a [`JSObjectRef`] that is a Typed Array or `NULL` if there
    /// was an error. The backing store of the Typed Array will be `buffer`.
    pub fn JSObjectMakeTypedArrayWithArrayBufferAndOffset(
        ctx: JSContextRef,
        arrayType: JSTypedArrayType,
        buffer: JSObjectRef,
        byteOffset: usize,
        length: usize,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;

    /// Returns a temporary pointer to the backing store of a
    /// JavaScript Typed Array object.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The Typed Array object whose backing store pointer
    ///   to return.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns a pointer to the raw data buffer that serves as `object`'s
    /// backing store or `NULL` if object is not a Typed Array object.
    ///
    /// The pointer returned by this function is temporary and is not
    /// guaranteed to remain valid across JavaScriptCore API calls.
    pub fn JSObjectGetTypedArrayBytesPtr(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> *mut ::std::os::raw::c_void;

    /// Returns the length of a JavaScript Typed Array object.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The Typed Array object whose length to return.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns the length of the Typed Array object or `0` if the object
    /// is not a Typed Array object.
    pub fn JSObjectGetTypedArrayLength(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> usize;

    /// Returns the byte length of a JavaScript Typed Array object.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The Typed Array object whose byte length to return.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns the byte length of the Typed Array object or `0` if the
    /// object is not a Typed Array object.
    pub fn JSObjectGetTypedArrayByteLength(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> usize;

    /// Returns the byte offset of a JavaScript Typed Array object.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The Typed Array object whose byte offset to return.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns the byte offset of the Typed Array object or `0` if the
    /// object is not a Typed Array object.
    pub fn JSObjectGetTypedArrayByteOffset(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> usize;

    /// Returns the JavaScript Array Buffer object that is used as the
    /// backing of a JavaScript Typed Array object.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The [`JSObjectRef`] whose Typed Array type data pointer
    ///   to obtain.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns a [`JSObjectRef`] with a [`JSTypedArrayType`] of
    /// `JSTypedArrayType::ArrayBuffer` or `NULL` if object is not
    /// a Typed Array.
    pub fn JSObjectGetTypedArrayBuffer(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;

    /// Creates a JavaScript Array Buffer object from an existing pointer.
    ///
    /// * `ctx`: The execution context to use.
    /// * `bytes`: A pointer to the byte buffer to be used as the backing
    ///   store of the Typed Array object.
    /// * `byteLength`: The number of bytes pointed to by the parameter `bytes`.
    /// * `bytesDeallocator`: The allocator to use to deallocate the
    ///   external buffer when the Typed Array data object is deallocated.
    /// * `deallocatorContext`: A pointer to pass back to the deallocator.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns a [`JSObjectRef`] Array Buffer whose backing store is
    /// the same as the one pointed to by `bytes` or `NULL` if there
    /// was an error.
    ///
    /// If an exception is thrown during this function the `bytesDeallocator`
    /// will always be called.
    pub fn JSObjectMakeArrayBufferWithBytesNoCopy(
        ctx: JSContextRef,
        bytes: *mut ::std::os::raw::c_void,
        byteLength: usize,
        bytesDeallocator: JSTypedArrayBytesDeallocator,
        deallocatorContext: *mut ::std::os::raw::c_void,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;

    /// Returns a pointer to the data buffer that serves as the backing
    /// store for a JavaScript Typed Array object.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The Array Buffer object whose internal backing
    ///   store pointer to return.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns a pointer to the raw data buffer that serves as
    /// `object`'s backing store or `NULL` if `object` is not an
    /// Array Buffer object.
    ///
    /// The pointer returned by this function is temporary and is not
    /// guaranteed to remain valid across JavaScriptCore API calls.
    pub fn JSObjectGetArrayBufferBytesPtr(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> *mut ::std::os::raw::c_void;

    /// Returns the number of bytes in a JavaScript data object.
    ///
    /// * `ctx`: The execution context to use.
    /// * `object`: The JS Array Buffer object whose length in bytes to return.
    /// * `exception`: A pointer to a [`JSValueRef`] in which to store
    ///   an exception, if any. Pass `NULL` if you do not care to
    ///   store an exception.
    ///
    /// Returns the number of bytes stored in the data `object`.
    pub fn JSObjectGetArrayBufferByteLength(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> usize;
}
