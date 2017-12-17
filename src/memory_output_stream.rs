// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>


#[cfg(all(test,any(feature = "v2_36", feature = "dox")))]
mod tests {
    use *;

    #[test]
    fn steal_empty() {
        let strm = MemoryOutputStream::new_resizable();
        assert_eq!(strm.get_data_size(), 0);

        assert!(strm.close(None).is_ok());
        assert_eq!(strm.steal_as_bytes().unwrap(), [].as_ref());
    }

    #[test]
    fn steal() {
        let strm = MemoryOutputStream::new_resizable();

        assert!(strm.write(&[1, 2, 3], None).is_ok());
        assert_eq!(strm.get_data_size(), 3);

        assert!(strm.write(&[4, 5], None).is_ok());
        assert_eq!(strm.get_data_size(), 5);

        assert!(strm.close(None).is_ok());
        assert_eq!(strm.steal_as_bytes().unwrap(), [1, 2, 3, 4, 5].as_ref());
    }
}
