// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CString;
use super::JSString;
use sys;

impl JSString {
    /// Convert this `JSString` to a `String`.
    pub fn to_string(&self) -> String {
        unsafe {
            let max_size = sys::JSStringGetMaximumUTF8CStringSize(self.raw);
            let mut buffer: Vec<u8> = Vec::with_capacity(max_size);
            let actual_size = sys::JSStringGetUTF8CString(
                self.raw,
                buffer.as_mut_ptr() as *mut ::std::os::raw::c_char,
                max_size,
            );
            buffer.set_len(actual_size - 1);
            String::from_utf8(buffer).unwrap()
        }
    }
}

impl Drop for JSString {
    fn drop(&mut self) {
        unsafe { sys::JSStringRelease(self.raw) }
    }
}

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

impl<'s> From<&'s JSString> for String {
    fn from(s: &'s JSString) -> Self {
        s.to_string()
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

        let e: String = (&d).into();
        assert_eq!(e, "abcdef");
    }
}
