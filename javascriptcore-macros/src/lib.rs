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
    let function_name = &function.sig.ident;

    quote! {
        unsafe extern "C" fn #function_name(
            raw_ctx: javascriptcore_sys::JSContextRef,
            function: javascriptcore_sys::JSObjectRef,
            this_object: javascriptcore_sys::JSObjectRef,
            argument_count: usize,
            arguments: *const javascriptcore_sys::JSValueRef,
            exception: *mut javascriptcore_sys::JSValueRef,
        ) -> *const javascriptcore_sys::OpaqueJSValue {
            use core::{mem::ManuallyDrop, ptr, slice};
            use javascriptcore::{JSContext, JSObject, JSValue};
            use javascriptcore_sys::JSValueRef;

            // This should never happen, it's simply a paranoid precaution.
            if raw_ctx.is_null() {
                return ptr::null();
            }

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

                #function_name
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
