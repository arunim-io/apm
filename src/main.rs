use gtk::{glib::ExitCode, prelude::*, Application, ApplicationWindow, Text};

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("dev.github.arunim-io.apm")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);

        window.set_child(Some(&Text::builder().text("Hello, World!").build()));
        window.present();
    });

    app.run()
}

