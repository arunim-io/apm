use gtk::{self, gdk, glib, prelude::*};
use gtk_layer_shell::{Edge, LayerShell};

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("dev.github.arunim-io.apm")
        .build();

    app.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);

        window.set_child(Some(&gtk::Label::new(Some("Hello, World!"))));

        window.init_layer_shell();
        window.set_layer(gtk_layer_shell::Layer::Overlay);
        window.set_exclusive_zone(-1);

        let controller = gtk::EventControllerKey::new();
        controller.connect_key_pressed(|_, key, _, _| {
            if let gdk::Key::Escape = key {
                std::process::exit(0);
            }
            glib::Propagation::Proceed
        });
        window.add_controller(controller);
        window.set_fullscreened(true);
        window.set_keyboard_mode(gtk_layer_shell::KeyboardMode::Exclusive);
        for edge in vec![Edge::Top, Edge::Bottom, Edge::Left, Edge::Right] {
            window.set_anchor(edge, true);
        }

        window.present();
    });

    app.run()
}

