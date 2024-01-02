use gtk::{glib::ExitCode, prelude::*, Application, ApplicationWindow, Text};
use gtk_layer_shell::{Layer, LayerShell};

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("dev.github.arunim-io.apm")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);

        window.set_child(Some(&Text::builder().text("Hello, World!").build()));

        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.set_exclusive_zone(-1);

        window.present();
    });

    app.run()
}

