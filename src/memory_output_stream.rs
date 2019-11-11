// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::MemoryOutputStream;

    #[test]
    fn steal_empty() {
        let strm = MemoryOutputStream::new_resizable();
        assert_eq!(strm.get_data_size(), 0);

        assert!(strm.close(::NONE_CANCELLABLE).is_ok());
        assert_eq!(strm.steal_as_bytes().unwrap(), [].as_ref());
    }

    #[test]
    fn steal() {
        let strm = MemoryOutputStream::new_resizable();

        assert!(strm.write(&[1, 2, 3], ::NONE_CANCELLABLE).is_ok());
        assert_eq!(strm.get_data_size(), 3);

        assert!(strm.write(&[4, 5], ::NONE_CANCELLABLE).is_ok());
        assert_eq!(strm.get_data_size(), 5);

        assert!(strm.close(::NONE_CANCELLABLE).is_ok());
        assert_eq!(strm.steal_as_bytes().unwrap(), [1, 2, 3, 4, 5].as_ref());
    }
}
