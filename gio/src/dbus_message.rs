// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DBusMessage;
use std::fmt;

impl fmt::Display for DBusMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print(0))
    }
}
