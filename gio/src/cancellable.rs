// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;

#[test]
fn check_callback() {
    let c = crate::Cancellable::new();
    c.connect_cancelled(|_| {});
    c.cancel(); // if it doesn't crash at this point, then we're good to go!
}
