// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{error, fmt};

use crate::{sys, JSException, JSString, JSValue};

impl JSException {
    /// Return the underlying value backing the exception.
    pub const fn underlying_value(&self) -> &JSValue {
        &self.value
    }

    /// Return the name of the exception. This is the value of the `name`
    /// property on the exception object.
    pub fn name(&self) -> Result<JSString, JSException> {
        self.value.as_object()?.get_property("name").as_string()
    }
}

impl fmt::Display for JSException {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.underlying_value().as_string() {
            Ok(string) => write!(formatter, "JSException (interpreted as string): {string}"),
            Err(_) => write!(formatter, "{self:?}"),
        }
    }
}

impl error::Error for JSException {}

impl From<JSValue> for JSException {
    fn from(value: JSValue) -> Self {
        Self { value }
    }
}

impl From<JSException> for sys::JSValueRef {
    fn from(value: JSException) -> Self {
        value.value.raw
    }
}
