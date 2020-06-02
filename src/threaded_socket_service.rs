use glib::translate::FromGlibPtrFull;
use glib::Cast;

use SocketService;
use ThreadedSocketService;

impl ThreadedSocketService {
    pub fn new(max_threads: Option<u32>) -> ThreadedSocketService {
        let max_threads = max_threads.map(|x| x as i32).unwrap_or(-1);
        unsafe {
            SocketService::from_glib_full(gio_sys::g_threaded_socket_service_new(max_threads))
                .unsafe_cast()
        }
    }
}
