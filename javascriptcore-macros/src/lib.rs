use proc_macro::TokenStream;
use quote::quote;

/// Transforms a Rust function into a C function for being used as a JavaScript callback.
///
/// This `function_callback` procedural macro transforms a Rust function of type:
///
/// ```rust,ignore
/// fn(
///     context: &JSContext,
///     function: Option<&JSObject>,
///     this_object: Option<&JSObject>,
///     arguments: &[JSValue]
/// ) -> Result<JSValue, JSException>
/// ```
///
/// into a `javascriptcore_sys::JSObjectCallAsFunctionCallback` function.
///
/// Check the documentation of `javascriptcore::JSValue::new_function` to learn more.
#[proc_macro_attribute]
pub fn function_callback(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    let function = syn::parse::<syn::ItemFn>(item)
        .expect("#[function_callback] must apply on a valid function");
    let function_visibility = &function.vis;
    let function_name = &function.sig.ident;
    let function_generics = &function.sig.generics.params;
    let function_where_clause = &function.sig.generics.where_clause;

    quote! {
        #function_visibility unsafe extern "C" fn #function_name < #function_generics > (
            raw_ctx: javascriptcore::sys::JSContextRef,
            function: javascriptcore::sys::JSObjectRef,
            this_object: javascriptcore::sys::JSObjectRef,
            argument_count: usize,
            arguments: *const javascriptcore::sys::JSValueRef,
            exception: *mut javascriptcore::sys::JSValueRef,
        ) -> *const javascriptcore::sys::OpaqueJSValue
        #function_where_clause
        {
            use ::core::{mem::ManuallyDrop, option::Option, ops::Not, ptr, result::Result, slice};
            use ::std::vec::Vec;
            use javascriptcore::{sys::JSValueRef, JSContext, JSObject, JSValue};

            // This should never happen, it's simply a paranoid precaution.
            assert!(raw_ctx.is_null().not(), "`JSContextRef` is null");

            // First off, let's prepare the arguments. The goal is to transform the raw C pointers
            // into Rust types.

            // Let's not drop `ctx`, otherwise it will close the context.
            let ctx = ManuallyDrop::new(JSContext::from_raw(raw_ctx as *mut _));
            let function = JSObject::from_raw(raw_ctx, function);
            let this_object = JSObject::from_raw(raw_ctx, this_object);

            let function = if function.is_null() {
                None
            } else {
                Some(&function)
            };

            let this_object = if this_object.is_null() {
                None
            } else {
                Some(&this_object)
            };

            let arguments = if argument_count == 0 || arguments.is_null() {
                Vec::new()
            } else {
                unsafe { slice::from_raw_parts(arguments, argument_count) }
                    .iter()
                    .map(|value| JSValue::from_raw(raw_ctx, *value))
                    .collect::<Vec<_>>()
            };

            // Isolate the `#function` inside its own block to avoid collisions with variables.
            // Let's use also this as an opportunity to type check the function being annotated by
            // `function_callback`.
            let func: fn(
                &JSContext,
                Option<&JSObject>,
                Option<&JSObject>,
                &[JSValue],
            ) -> Result<JSValue, JSException> = {
                #function

                #function_name ::< #function_generics >
            };

            // Second, call the original function.
            let result = func(&ctx, function, this_object, arguments.as_slice());

            // Finally, let's handle the result, including the exception.
            match result {
                Ok(value) => {
                    // Ensure `exception` contains a null pointer.
                    *exception = ptr::null_mut();

                    // Return the result.
                    value.into()
                }
                Err(exc) => {
                    // Fill the exception.
                    *exception = JSValueRef::from(exc) as *mut _;

                    // Return a null pointer for the result.
                    ptr::null()
                }
            }
        }
    }
    .into()
}

/// Transforms a Rust function into a C function for being used as a JavaScript
/// constructor callback.
///
/// This `constructor_callback` procedural macro transforms a Rust function of type:
///
/// ```rust,ignore
/// fn(
///     context: &JSContext,
///     constructor: Option<&JSObject>,
///     arguments: &[JSValue]
/// ) -> Result<JSValue, JSException>
/// ```
///
/// into a `javascriptcore_sys::JSObjectCallAsConstructorCallback` function.
///
/// Check the documentation of `javascriptcore::JSClass::new` to learn more.
#[proc_macro_attribute]
pub fn constructor_callback(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    let constructor = syn::parse::<syn::ItemFn>(item)
        .expect("#[constructor_callback] must apply on a valid function");
    let constructor_visibility = &constructor.vis;
    let constructor_name = &constructor.sig.ident;
    let constructor_generics = &constructor.sig.generics.params;
    let constructor_where_clause = &constructor.sig.generics.where_clause;

    quote! {
        #constructor_visibility unsafe extern "C" fn #constructor_name < #constructor_generics >(
            raw_ctx: javascriptcore::sys::JSContextRef,
            constructor: javascriptcore::sys::JSObjectRef,
            argument_count: usize,
            arguments: *const javascriptcore::sys::JSValueRef,
            exception: *mut javascriptcore::sys::JSValueRef,
        ) -> *mut javascriptcore::sys::OpaqueJSValue
        #constructor_where_clause
        {
            use ::core::{mem::ManuallyDrop, option::Option, ops::Not, ptr, result::Result, slice};
            use ::std::vec::Vec;
            use javascriptcore::{sys::JSValueRef, JSContext, JSObject, JSValue};

            // This should never happen, it's simply a paranoid precaution.
            assert!(raw_ctx.is_null().not(), "`JSContextRef` is null");

            // First off, let's prepare the arguments. The goal is to transform the raw C pointers
            // into Rust types.

            // Let's not drop `ctx`, otherwise it will close the context.
            let ctx = ManuallyDrop::new(JSContext::from_raw(raw_ctx as *mut _));
            let constructor = JSObject::from_raw(raw_ctx, constructor);

            let arguments = if argument_count == 0 || arguments.is_null() {
                Vec::new()
            } else {
                unsafe { slice::from_raw_parts(arguments, argument_count) }
                    .iter()
                    .map(|value| JSValue::from_raw(raw_ctx, *value))
                    .collect::<Vec<_>>()
            };

            // Isolate the `#constructor` inside its own block to avoid collisions with variables.
            // Let's use also this as an opportunity to type check the constructor being annotated by
            // `constructor_callback`.
            let ctor: fn(
                &JSContext,
                &JSObject,
                &[JSValue],
            ) -> Result<JSValue, JSException> = {
                #constructor

                #constructor_name ::< #constructor_generics >
            };

            // Second, call the original constructor.
            let result = ctor(&ctx, &constructor, arguments.as_slice());

            // Finally, let's handle the result, including the exception.
            match result {
                Ok(value) => {
                    // Ensure `exception` contains a null pointer.
                    *exception = ptr::null_mut();

                    // Return the result.
                    value.into()
                }
                Err(exc) => {
                    // Fill the exception.
                    *exception = JSValueRef::from(exc) as *mut _;

                    // Return a null pointer for the result.
                    ptr::null_mut()
                }
            }
        }
    }
    .into()
}
