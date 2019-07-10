// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib::translate::*;
use std::fmt;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::mem;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::ptr;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use Display;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use Error;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use Window;

glib_wrapper! {
    pub struct GLContext(Object<gdk_sys::GdkGLContext, GLContextClass>);

    match fn {
        get_type => || gdk_sys::gdk_gl_context_get_type(),
    }
}

impl GLContext {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_debug_enabled(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_gl_context_get_debug_enabled(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_display(&self) -> Option<Display> {
        unsafe { from_glib_none(gdk_sys::gdk_gl_context_get_display(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_forward_compatible(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_gl_context_get_forward_compatible(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_required_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::MaybeUninit::uninit();
            let mut minor = mem::MaybeUninit::uninit();
            gdk_sys::gdk_gl_context_get_required_version(
                self.to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            );
            let major = major.assume_init();
            let minor = minor.assume_init();
            (major, minor)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_shared_context(&self) -> Option<GLContext> {
        unsafe {
            from_glib_none(gdk_sys::gdk_gl_context_get_shared_context(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn get_use_es(&self) -> bool {
        unsafe { from_glib(gdk_sys::gdk_gl_context_get_use_es(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::MaybeUninit::uninit();
            let mut minor = mem::MaybeUninit::uninit();
            gdk_sys::gdk_gl_context_get_version(
                self.to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            );
            let major = major.assume_init();
            let minor = minor.assume_init();
            (major, minor)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_window(&self) -> Option<Window> {
        unsafe { from_glib_none(gdk_sys::gdk_gl_context_get_window(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn is_legacy(&self) -> bool {
        unsafe { from_glib(gdk_sys::gdk_gl_context_is_legacy(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn make_current(&self) {
        unsafe {
            gdk_sys::gdk_gl_context_make_current(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn realize(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gdk_sys::gdk_gl_context_realize(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn set_debug_enabled(&self, enabled: bool) {
        unsafe {
            gdk_sys::gdk_gl_context_set_debug_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn set_forward_compatible(&self, compatible: bool) {
        unsafe {
            gdk_sys::gdk_gl_context_set_forward_compatible(
                self.to_glib_none().0,
                compatible.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn set_required_version(&self, major: i32, minor: i32) {
        unsafe {
            gdk_sys::gdk_gl_context_set_required_version(self.to_glib_none().0, major, minor);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn set_use_es(&self, use_es: i32) {
        unsafe {
            gdk_sys::gdk_gl_context_set_use_es(self.to_glib_none().0, use_es);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn clear_current() {
        assert_initialized_main_thread!();
        unsafe {
            gdk_sys::gdk_gl_context_clear_current();
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_current() -> Option<GLContext> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(gdk_sys::gdk_gl_context_get_current()) }
    }
}

impl fmt::Display for GLContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GLContext")
    }
}
