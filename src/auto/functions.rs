// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib::object::IsA;
use glib::translate::*;
#[cfg(any(feature = "v0_5", feature = "dox"))]
use glib::GString;
use gtk;
use gtk_layer_shell_sys;
use Edge;
use Layer;


pub fn auto_exclusive_zone_enable<P: IsA<gtk::Window>>(window: &P) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_auto_exclusive_zone_enable(window.as_ref().to_glib_none().0);
    }
}

#[cfg(any(feature = "v0_5", feature = "dox"))]
pub fn auto_exclusive_zone_is_enabled<P: IsA<gtk::Window>>(window: &P) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gtk_layer_shell_sys::gtk_layer_auto_exclusive_zone_is_enabled(window.as_ref().to_glib_none().0))
    }
}

#[cfg(any(feature = "v0_5", feature = "dox"))]
pub fn get_anchor<P: IsA<gtk::Window>>(window: &P, edge: Edge) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gtk_layer_shell_sys::gtk_layer_get_anchor(window.as_ref().to_glib_none().0, edge.to_glib()))
    }
}

#[cfg(any(feature = "v0_5", feature = "dox"))]
pub fn get_exclusive_zone<P: IsA<gtk::Window>>(window: &P) -> i32 {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_get_exclusive_zone(window.as_ref().to_glib_none().0)
    }
}

#[cfg(any(feature = "v0_5", feature = "dox"))]
pub fn get_keyboard_interactivity<P: IsA<gtk::Window>>(window: &P) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gtk_layer_shell_sys::gtk_layer_get_keyboard_interactivity(window.as_ref().to_glib_none().0))
    }
}

#[cfg(any(feature = "v0_5", feature = "dox"))]
pub fn get_layer<P: IsA<gtk::Window>>(window: &P) -> Layer {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gtk_layer_shell_sys::gtk_layer_get_layer(window.as_ref().to_glib_none().0))
    }
}

#[cfg(any(feature = "v0_4", feature = "dox"))]
pub fn get_major_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_get_major_version()
    }
}

#[cfg(any(feature = "v0_5", feature = "dox"))]
pub fn get_margin<P: IsA<gtk::Window>>(window: &P, edge: Edge) -> i32 {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_get_margin(window.as_ref().to_glib_none().0, edge.to_glib())
    }
}

#[cfg(any(feature = "v0_4", feature = "dox"))]
pub fn get_micro_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_get_micro_version()
    }
}

#[cfg(any(feature = "v0_4", feature = "dox"))]
pub fn get_minor_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_get_minor_version()
    }
}

#[cfg(any(feature = "v0_5", feature = "dox"))]
pub fn get_monitor<P: IsA<gtk::Window>>(window: &P) -> Option<gdk::Monitor> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(gtk_layer_shell_sys::gtk_layer_get_monitor(window.as_ref().to_glib_none().0))
    }
}

#[cfg(any(feature = "v0_5", feature = "dox"))]
pub fn get_namespace<P: IsA<gtk::Window>>(window: &P) -> Option<GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(gtk_layer_shell_sys::gtk_layer_get_namespace(window.as_ref().to_glib_none().0))
    }
}

//#[cfg(any(feature = "v0_4", feature = "dox"))]
//pub fn get_zwlr_layer_surface_v1<P: IsA<gtk::Window>>(window: &P) -> /*Unimplemented*/Option<Fundamental: Pointer> {
//    unsafe { TODO: call gtk_layer_shell_sys:gtk_layer_get_zwlr_layer_surface_v1() }
//}

pub fn init_for_window<P: IsA<gtk::Window>>(window: &P) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_init_for_window(window.as_ref().to_glib_none().0);
    }
}

#[cfg(any(feature = "v0_5", feature = "dox"))]
pub fn is_layer_window<P: IsA<gtk::Window>>(window: &P) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gtk_layer_shell_sys::gtk_layer_is_layer_window(window.as_ref().to_glib_none().0))
    }
}

#[cfg(any(feature = "v0_5", feature = "dox"))]
pub fn is_supported() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(gtk_layer_shell_sys::gtk_layer_is_supported())
    }
}

pub fn set_anchor<P: IsA<gtk::Window>>(window: &P, edge: Edge, anchor_to_edge: bool) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_set_anchor(window.as_ref().to_glib_none().0, edge.to_glib(), anchor_to_edge.to_glib());
    }
}

pub fn set_exclusive_zone<P: IsA<gtk::Window>>(window: &P, exclusive_zone: i32) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_set_exclusive_zone(window.as_ref().to_glib_none().0, exclusive_zone);
    }
}

pub fn set_keyboard_interactivity<P: IsA<gtk::Window>>(window: &P, interacitvity: bool) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_set_keyboard_interactivity(window.as_ref().to_glib_none().0, interacitvity.to_glib());
    }
}

pub fn set_layer<P: IsA<gtk::Window>>(window: &P, layer: Layer) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_set_layer(window.as_ref().to_glib_none().0, layer.to_glib());
    }
}

pub fn set_margin<P: IsA<gtk::Window>>(window: &P, edge: Edge, margin_size: i32) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_set_margin(window.as_ref().to_glib_none().0, edge.to_glib(), margin_size);
    }
}

pub fn set_monitor<P: IsA<gtk::Window>>(window: &P, monitor: &gdk::Monitor) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_set_monitor(window.as_ref().to_glib_none().0, monitor.to_glib_none().0);
    }
}

pub fn set_namespace<P: IsA<gtk::Window>>(window: &P, name_space: &str) {
    assert_initialized_main_thread!();
    unsafe {
        gtk_layer_shell_sys::gtk_layer_set_namespace(window.as_ref().to_glib_none().0, name_space.to_glib_none().0);
    }
}
