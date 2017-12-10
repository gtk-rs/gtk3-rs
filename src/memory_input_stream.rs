// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>


#[cfg(test)]
mod tests {
    use std::vec::Vec;
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    use glib::Bytes;
    use *;

    #[test]
    fn new() {
        let strm = MemoryInputStream::new();
        let ret = strm.skip(1, None);
        assert!(!ret.is_err());
        assert_eq!(ret.unwrap(), 0);

        let buf = Vec::with_capacity(10);
        let ret = strm.read_all(buf, None).unwrap();
        assert_eq!(ret.0, Vec::new());
        assert!(ret.1.is_none());
    }

    #[test]
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn new_from_bytes() {
        let b = Bytes::from_owned(vec![1, 2, 3]);
        let strm = MemoryInputStream::new_from_bytes(&b);
        let buf = Vec::with_capacity(10);
        let ret = strm.read_all(buf, None).unwrap();
        assert_eq!(ret.0, vec![1, 2, 3]);
        assert!(ret.1.is_none());

        let ret = strm.skip(10, None).unwrap();
        assert_eq!(ret, 0);
    }

    #[test]
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn add_bytes() {
        let strm = MemoryInputStream::new();
        let b = Bytes::from_owned(vec![1, 2, 3]);
        strm.add_bytes(&b);
        let buf = Vec::with_capacity(10);
        let ret = strm.read_all(buf, None).unwrap();
        assert_eq!(ret.0, vec![1, 2, 3]);
        assert!(ret.1.is_none());

        let ret = strm.skip(10, None).unwrap();
        assert_eq!(ret, 0);
    }
}
