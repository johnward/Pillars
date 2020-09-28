//extern crate gio;
//extern crate glib;
//extern crate gtk;

//use gio::prelude::*;
//use glib::clone;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, Button, Image, MessageDialog};

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

        let background_image: Image = builder
            .get_object("gamebackground")
            .expect("Couldn't get Background Image");

        background_image.set_from_file("ui/gamebackground.png");

        let next_object_image: Image = builder
            .get_object("gamenextobject")
            .expect("gamenextobject not found");

        next_object_image.set_from_file("gamenextobject.png");

        window.show_all();
    }
}
