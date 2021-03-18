// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::MemoryOutputStream;

    #[test]
    fn steal_empty() {
        let strm = MemoryOutputStream::new_resizable();
        assert_eq!(strm.get_data_size(), 0);

        assert!(strm.close(crate::NONE_CANCELLABLE).is_ok());
        assert_eq!(strm.steal_as_bytes(), [].as_ref());
    }

    #[test]
    fn steal() {
        let strm = MemoryOutputStream::new_resizable();

        assert!(strm.write(&[1, 2, 3], crate::NONE_CANCELLABLE).is_ok());
        assert_eq!(strm.get_data_size(), 3);

        assert!(strm.write(&[4, 5], crate::NONE_CANCELLABLE).is_ok());
        assert_eq!(strm.get_data_size(), 5);

        assert!(strm.close(crate::NONE_CANCELLABLE).is_ok());
        assert_eq!(strm.steal_as_bytes(), [1, 2, 3, 4, 5].as_ref());
    }
}
