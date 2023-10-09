use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn function_callback(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    let function = syn::parse::<syn::ItemFn>(item)
        .expect("#[function_callback] must apply on a valid function");
    let function_name = &function.sig.ident;

    quote! {
        unsafe extern "C" fn #function_name(
            __raw_ctx: javascriptcore_sys::JSContextRef,
            __raw_function: javascriptcore_sys::JSObjectRef,
            __raw_this_object: javascriptcore_sys::JSObjectRef,
            __raw_argument_count: usize,
            __raw_arguments: *const javascriptcore_sys::JSValueRef,
            __raw_exception: *mut javascriptcore_sys::JSValueRef,
        ) -> *const javascriptcore_sys::OpaqueJSValue {
            use core::{mem, ptr, slice};
            use javascriptcore::{JSContext, JSObject, JSValue};

            let __ctx = JSContext::from_raw(__raw_ctx as *mut _);
            let __function = JSObject::from_raw(__raw_ctx, __raw_function);
            let __this_object = JSObject::from_raw(__raw_ctx, __raw_this_object);

            let __function = if __raw_function.is_null() {
                None
            } else {
                Some(&__function)
            };

            let __this_object = if __raw_this_object.is_null() {
                None
            } else {
                Some(&__this_object)
            };

            let __arguments = if __raw_argument_count == 0 {
                Vec::new()
            } else {
                unsafe { slice::from_raw_parts(__raw_arguments, __raw_argument_count) }
                    .iter()
                    .map(|value| JSValue::from_raw(__raw_ctx, *value))
                    .collect::<Vec<_>>()
            };

            #function

            let func: fn(
                &JSContext,
                Option<&JSObject>,
                Option<&JSObject>,
                arguments: &[JSValue],
            ) -> Result<JSValue, JSException> = #function_name;
            let result = func(&__ctx, __function, __this_object, __arguments.as_slice());

            mem::forget(__ctx);

            match result {
                Ok(value) => value.into(),
                Err(exception) => {
                    let raw_exception: javascriptcore_sys::JSValueRef = exception.into();
                    *__raw_exception = raw_exception as *mut _;

                    ptr::null()
                }
            }
        }
    }
    .into()
}
