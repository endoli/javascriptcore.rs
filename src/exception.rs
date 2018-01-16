// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{JSException,JSString};

impl JSException {

    /// Converts a JavaScript exception to a JavaScript string.
    ///
    /// Returns either `JSString` with the result of conversion, or an
    /// exception if one was thrown.  Ownership follows the Create Rule.
    ///
    pub fn as_string(&self) -> Result<JSString, JSException> {
        self.value.as_string()
    }

}
