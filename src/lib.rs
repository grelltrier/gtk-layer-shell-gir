extern crate gdk;
extern crate glib;
extern crate glib_sys;
extern crate gtk;
extern crate gtk_layer_shell_sys;

extern crate bitflags;
extern crate libc;

/// Asserts that this is the main thread and either `gdk::init` or `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    };
}

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

pub use self::auto::functions::*;
pub use auto::*;

mod auto;
