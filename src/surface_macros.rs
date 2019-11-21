// e.g. declare_surface(ImageSurface, SurfaceType::Image)
macro_rules! declare_surface {
    ($surf_name:ident, $surf_type:expr) => {
        #[derive(Debug)]
        pub struct $surf_name(Surface);

        impl TryFrom<Surface> for $surf_name {
            type Error = Surface;

            fn try_from(surface: Surface) -> Result<$surf_name, Surface> {
                if surface.get_type() == $surf_type {
                    Ok($surf_name(surface))
                } else {
                    Err(surface)
                }
            }
        }

        impl $surf_name {
            pub unsafe fn from_raw_full(
                ptr: *mut ffi::cairo_surface_t,
            ) -> Result<$surf_name, Status> {
                let surface = Surface::from_raw_full(ptr)?;
                Self::try_from(surface).map_err(|_| Status::SurfaceTypeMismatch)
            }
        }

        #[cfg(feature = "use_glib")]
        impl<'a> ToGlibPtr<'a, *mut ffi::cairo_surface_t> for $surf_name {
            type Storage = &'a Surface;

            #[inline]
            fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_surface_t, Self> {
                let stash = self.0.to_glib_none();
                Stash(stash.0, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *mut ffi::cairo_surface_t {
                unsafe { ffi::cairo_surface_reference(self.to_glib_none().0) }
            }
        }

        #[cfg(feature = "use_glib")]
        impl FromGlibPtrNone<*mut ffi::cairo_surface_t> for $surf_name {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut ffi::cairo_surface_t) -> $surf_name {
                Self::try_from(from_glib_none::<_, Surface>(ptr)).unwrap()
            }
        }

        #[cfg(feature = "use_glib")]
        impl FromGlibPtrBorrow<*mut ffi::cairo_surface_t> for $surf_name {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_surface_t) -> $surf_name {
                Self::try_from(from_glib_borrow::<_, Surface>(ptr)).unwrap()
            }
        }

        #[cfg(feature = "use_glib")]
        impl FromGlibPtrFull<*mut ffi::cairo_surface_t> for $surf_name {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut ffi::cairo_surface_t) -> $surf_name {
                Self::from_raw_full(ptr).unwrap()
            }
        }

        #[cfg(feature = "use_glib")]
        gvalue_impl!(
            $surf_name,
            ffi::cairo_surface_t,
            ffi::gobject::cairo_gobject_surface_get_type
        );

        impl Deref for $surf_name {
            type Target = Surface;

            fn deref(&self) -> &Surface {
                &self.0
            }
        }

        impl Clone for $surf_name {
            fn clone(&self) -> $surf_name {
                $surf_name(self.0.clone())
            }
        }

        impl fmt::Display for $surf_name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", stringify!($surf_name))
            }
        }
    };
}
