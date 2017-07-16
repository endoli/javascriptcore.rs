// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::Deref;
use super::{JSObject, JSString, JSValue};
use sys;

impl JSObject {
    /// Gets an iterator over the names of an object's enumerable properties.
    ///
    /// ```
    /// # use javascriptcore::{JSObject, JSString};
    /// # fn get_property_names(obj: JSObject) {
    /// let names: Vec<JSString> = obj.property_names().collect();
    /// # }
    /// ```
    pub fn property_names(&self) -> JSObjectPropertyNameIter {
        JSObjectPropertyNameIter {
            raw: unsafe { sys::JSObjectCopyPropertyNames(self.value.ctx, self.raw) },
            idx: 0,
        }
    }
}

impl Deref for JSObject {
    type Target = JSValue;

    fn deref(&self) -> &JSValue {
        &self.value
    }
}

pub struct JSObjectPropertyNameIter {
    raw: sys::JSPropertyNameArrayRef,
    idx: usize,
}

impl Iterator for JSObjectPropertyNameIter {
    type Item = JSString;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < unsafe { sys::JSPropertyNameArrayGetCount(self.raw) } {
            let name = unsafe { sys::JSPropertyNameArrayGetNameAtIndex(self.raw, self.idx) };
            self.idx += 1;
            Some(JSString { raw: name })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::JSPropertyNameArrayGetCount(self.raw) };
        (sz - self.idx, Some(sz))
    }
}

#[cfg(test)]
mod tests {
    use super::super::{JSContext, JSValue};

    #[test]
    fn can_get_property_names() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("value");
        let o = v.as_object().expect("object");
        let names = o.property_names().collect::<Vec<_>>();
        assert_eq!(names.len(), 1);
        assert_eq!(names[0], "id".into());
    }

    #[test]
    fn can_use_as_jsvalue_via_deref() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("value");
        let o = v.as_object().expect("object");
        assert!(v.is_object());
        assert!(o.is_object());
    }
}
