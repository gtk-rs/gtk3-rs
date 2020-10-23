use std::iter::Iterator;
use FileEnumerator;
use FileEnumeratorExt;
use FileInfo;

impl Iterator for FileEnumerator {
    type Item = Result<FileInfo, glib::Error>;

    fn next(&mut self) -> Option<Result<FileInfo, glib::Error>> {
        match self.next_file(::NONE_CANCELLABLE) {
            Err(err) => Some(Err(err)),
            Ok(file_info) => file_info.map(Ok),
        }
    }
}
