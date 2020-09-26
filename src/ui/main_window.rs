extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, Button, MessageDialog};

pub struct MainWindow {
    has_gl: bool,
}

impl MainWindow {
    pub fn create_application_ui(has_gl: bool) {
        let glade_src = include_str!("main_window.glade");
        //let builder = Builder::
    }
}
