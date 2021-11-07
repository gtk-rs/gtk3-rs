// Take a look at the license at the top of the repository in the LICENSE file.

#![doc = include_str!("../README.md")]

pub use gdk;

mod wayland_device;
pub use wayland_device::WaylandDevice;

mod wayland_display;
pub use wayland_display::WaylandDisplay;

mod wayland_gl_context;
pub use wayland_gl_context::WaylandGLContext;

#[cfg(any(feature = "v3_22", feature = "dox"))]
mod wayland_monitor;
#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
pub use wayland_monitor::WaylandMonitor;

#[cfg(any(feature = "v3_20", feature = "dox"))]
mod wayland_seat;
#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
pub use wayland_seat::WaylandSeat;

mod wayland_window;
pub use wayland_window::WaylandWindow;
