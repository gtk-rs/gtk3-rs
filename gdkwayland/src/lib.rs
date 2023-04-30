// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use gdk;

mod wayland_device;
pub use wayland_device::WaylandDevice;

mod wayland_display;
pub use wayland_display::WaylandDisplay;

mod wayland_gl_context;
pub use wayland_gl_context::WaylandGLContext;

mod wayland_monitor;
pub use wayland_monitor::WaylandMonitor;

mod wayland_seat;
pub use wayland_seat::WaylandSeat;

mod wayland_window;
pub use wayland_window::WaylandWindow;
