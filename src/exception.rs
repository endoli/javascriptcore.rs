// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{JSException, JSString, JSValue};

impl JSException {
    /// Return the underlying value backing the exception.
    pub fn underlying_value(&self) -> &JSValue {
        &self.value
    }

    /// Return the name of the exception. This is the value of the `name`
    /// property on the exception object.
    pub fn name(&self) -> Result<JSString, JSException> {
        self.value
            .as_object()
            .unwrap()
            .get_property("name")
            .as_string()
    }
}

impl From<JSValue> for JSException {
    fn from(value: JSValue) -> Self {
        Self { value }
    }
}
