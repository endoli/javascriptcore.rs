// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CString;
use super::JSString;
use sys;

impl PartialEq for JSString {
    fn eq(&self, other: &JSString) -> bool {
        unsafe { sys::JSStringIsEqual(self.raw, other.raw) }
    }
}

impl<'s> From<&'s str> for JSString {
    fn from(s: &'s str) -> Self {
        let c = CString::new(s.as_bytes()).unwrap();
        JSString { raw: unsafe { sys::JSStringCreateWithUTF8CString(c.as_ptr()) } }
    }
}

impl From<String> for JSString {
    fn from(s: String) -> Self {
        let c = CString::new(s.as_bytes()).unwrap();
        JSString { raw: unsafe { sys::JSStringCreateWithUTF8CString(c.as_ptr()) } }
    }
}

#[cfg(test)]
mod tests {
    use super::JSString;

    #[test]
    fn from_conversion() {
        let a: JSString = "abc".into();
        let b: JSString = "abc".to_owned().into();
        assert_eq!(a, a);
        assert_eq!(a, b);
        assert_eq!(b, b);

        let c: JSString = "def".into();
        assert_ne!(a, c);

        let d: JSString = "abcdef".into();
        assert_ne!(a, d);
    }
}
