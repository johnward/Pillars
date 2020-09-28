pub mod main_window;
pub use self::main_window::MainWindow;

use gio::prelude::*;
//use gtk::prelude::*;

pub fn run() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        MainWindow::build_ui(app, true);
    });

    application.run(&[]);
}
