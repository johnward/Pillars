pub mod main_window;
use main_window::MainWindow;

pub fn run() {
    MainWindow::create_application_ui(true);
}
