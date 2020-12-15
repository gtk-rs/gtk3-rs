// Take a look at the license at the top of the repository in the LICENSE file.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChangeData<'a> {
    UChars(&'a [u8]),
    UShorts(&'a [u16]),
    ULongs(&'a [libc::c_ulong]),
    UChar(u8),
    UShort(u16),
    ULong(libc::c_ulong),
}

#[doc(hidden)]
impl<'a> ChangeData<'a> {
    pub fn to_glib(&self) -> *const u8 {
        match *self {
            ChangeData::UChars(d) => d.as_ptr() as *const _,
            ChangeData::UShorts(d) => d.as_ptr() as *const _,
            ChangeData::ULongs(d) => d.as_ptr() as *const _,
            ChangeData::UChar(d) => &d as *const _ as *const _,
            ChangeData::UShort(d) => &d as *const _ as *const _,
            ChangeData::ULong(d) => &d as *const _ as *const _,
        }
    }

    pub fn len(&self) -> usize {
        match *self {
            ChangeData::UChars(d) => d.len(),
            ChangeData::UShorts(d) => d.len(),
            ChangeData::ULongs(d) => d.len(),
            ChangeData::UChar(_) | ChangeData::UShort(_) | ChangeData::ULong(_) => 1,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() != 0
    }
}
