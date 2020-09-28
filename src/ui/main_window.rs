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
    pub fn create_application_ui(&self, has_gl: bool) {
        let application = gtk::Application::new(Some("Pillars"), Default::default())
            .expect("Unable to load Application");

        application.connect_activate(|app| {
            self.build_ui(app);
        });
    }

    fn build_ui(&self, application: &gtk::Application) {
        let glade_src = include_str!("main_window.glade");
        let builder = Builder::from_string(glade_src);
        let window = builder
            .get_object("Pillars")
            .expect("Could not get Pillars Main Window");
    }
}
