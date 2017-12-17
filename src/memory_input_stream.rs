// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>


#[cfg(test)]
mod tests {
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    use glib::Bytes;
    use *;

    #[test]
    fn new() {
        let strm = MemoryInputStream::new();
        let ret = strm.skip(1, None);
        assert!(!ret.is_err());
        assert_eq!(ret.unwrap(), 0);

        let mut buf = vec![0;10];
        let ret = strm.read(&mut buf, None).unwrap();
        assert_eq!(ret, 0);
    }

    #[test]
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn new_from_bytes() {
        let b = Bytes::from_owned(vec![1, 2, 3]);
        let strm = MemoryInputStream::new_from_bytes(&b);
        let mut buf = vec![0;10];
        let ret = strm.read(&mut buf, None).unwrap();
        assert_eq!(ret, 3);
        assert_eq!(buf[0], 1);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2], 3);

        let ret = strm.skip(10, None).unwrap();
        assert_eq!(ret, 0);
    }

    #[test]
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn add_bytes() {
        let strm = MemoryInputStream::new();
        let b = Bytes::from_owned(vec![1, 2, 3]);
        strm.add_bytes(&b);
        let mut buf = vec![0;10];
        let ret = strm.read(&mut buf, None).unwrap();
        assert_eq!(ret, 3);
        assert_eq!(buf[0], 1);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2], 3);

        let ret = strm.skip(10, None).unwrap();
        assert_eq!(ret, 0);
    }
}
