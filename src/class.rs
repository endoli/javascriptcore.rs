use JSClass;
use sys::{JSClassCreate, JSClassDefinition};

impl JSClass {
    /// Return a new class object based on the passed-in class definition.
    pub fn new_with_class_definition(class_definition: &JSClassDefinition) -> JSClass {
        let class_ref = unsafe { JSClassCreate(class_definition) };
        JSClass { raw: class_ref }
    }

    /// Return a new, default class object.
    ///
    /// ```
    /// use javascriptcore::{JSClass, JSContext};
    ///
    /// let global_class = JSClass::new();
    /// let ctx = JSContext::new_with_class(&global_class);
    /// ```
    pub fn new() -> JSClass {
        let class_definition = JSClassDefinition::default();
        JSClass::new_with_class_definition(&class_definition)
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use std::ptr;
    use super::{JSClass, JSClassDefinition};
    use {JSContext, JSObject, JSValue};
    use sys::{JSContextRef, JSObjectMake, JSObjectRef, JSStaticFunction,
              JSStringCreateWithUTF8CString, JSStringRef, JSValueMakeString, JSValueMakeUndefined,
              JSValueRef, OpaqueJSValue, JSValueMakeBoolean, JSStaticValue};

    extern "C" fn ahoy_property_handler(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        exception: *mut JSValueRef,
    ) -> *const OpaqueJSValue {
        println!("###### ahoy_property_handler");
        unsafe { JSValueMakeBoolean(ctx, true) }
    }

    #[test]
    fn test_new_with_class_definition() {
        let test_class_name = "Raz";
        let mut class_definition = JSClassDefinition::default();
        class_definition.className = CString::new(test_class_name).unwrap().as_ptr();
        let static_values = [
            JSStaticValue {
                name: CString::new("ahoy").unwrap().as_ptr(),
                getProperty: Some(ahoy_property_handler),
                setProperty: None,
                attributes: 1 << 1,
            },
            JSStaticValue {
                name: ptr::null_mut(),
                getProperty: None,
                setProperty: None,
                attributes: 0,
            },
        ];
        class_definition.staticValues = static_values.as_ptr();

        let class = JSClass::new_with_class_definition(&class_definition);
        let ctx = JSContext::default();
        let raw_object = unsafe { JSObjectMake(ctx.raw, class.raw, ptr::null_mut()) };
        let o = JSObject {
            raw: raw_object,
            value: JSValue {
                raw: raw_object,
                ctx: ctx.raw,
            },
        };
        let prop_value = o.get_property("ahoy");
        assert_eq!(prop_value.as_boolean(), true);
    }

}
