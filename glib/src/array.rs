// Take a look at the license at the top of the repository in the LICENSE file.

wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Array(Shared<ffi::GArray>);

    match fn {
        ref => |ptr| ffi::g_array_ref(ptr),
        unref => |ptr| ffi::g_array_unref(ptr),
        get_type => || ffi::g_array_get_type(),
    }
}
