//extern crate gio;
//extern crate glib;
//extern crate gtk;

//use gio::prelude::*;
//use glib::clone;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, Button, MessageDialog};

pub struct MainWindow {}

impl MainWindow {
    pub fn build_ui(application: &gtk::Application, has_gl: bool) {
        let glade_src = include_str!("main_window.glade");
        let builder = Builder::from_string(glade_src);

        let window: ApplicationWindow =
            builder.get_object("window1").expect("Couldn't get window1");
        window.set_application(Some(application));
        //let bigbutton: Button = builder.get_object("button1").expect("Couldn't get button1");
        //bigbutton.connect_clicked(clone!(@weak dialog => move |_| dialog.show_all()));
        window.show_all();
    }
}
