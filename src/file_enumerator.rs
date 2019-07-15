use std::iter::Iterator;
use Cancellable;
use Error;
use FileEnumerator;
use FileEnumeratorExt;
use FileInfo;

impl Iterator for FileEnumerator {
    type Item = Result<FileInfo, Error>;

    fn next(&mut self) -> Option<Result<FileInfo, Error>> {
        match self.next_file(None::<&Cancellable>) {
            Err(err) => Some(Err(err)),
            Ok(file_info) => file_info.map(Ok),
        }
    }
}
