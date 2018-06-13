extern crate gtk;
extern crate glib;
extern crate gobject_sys as gobject_ffi;

use gtk::prelude::*;
use glib::translate::*;
use std::ptr;

fn main() {
    gtk::init().expect("GTK Initialization failed.");
    let button = gtk::Button::new();
    button.connect_clicked(|_| {
        println!("Hello!");
    });

    unsafe {
        let num_signals_found = gobject_ffi::g_signal_handlers_disconnect_matched(
            button.to_glib_none().0, //instance
            // I am using `G_SIGNAL_MATCH_DETAIL` here because I thought this had something to do with the signal
            // name.
            // What I'm trying to achieve is to delete the signal handler connected above using
            // `connect_clicked`. Or if that is not possible, delete all signal handlers connected
            // to `button` without destroying the object.
            gobject_ffi::G_SIGNAL_MATCH_DETAIL,
            0,
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut()
        );
        assert_eq!(num_signals_found, 1);
    }
}
