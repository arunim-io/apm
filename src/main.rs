use std::path::Path;

use gtk::{self, gdk, glib, prelude::*};
use gtk_layer_shell::{Edge, LayerShell};

static GAPS: i32 = 25;

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("dev.github.arunim-io.apm")
        .build();

    app.connect_startup(|_| {
        let provider = gtk::CssProvider::new();
        provider.load_from_path(Path::new("examples/styles.css"));

        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    app.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);

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

        let list = gtk::Box::new(gtk::Orientation::Horizontal, GAPS);
        list.set_halign(gtk::Align::Center);
        for i in vec![1, 2, 3] {
            let label = gtk::Label::new(Some(&i.to_string()));
            let icon = gtk::Image::from_file(Path::new("examples/shutdown.png"));

            let container = gtk::Box::new(gtk::Orientation::Vertical, 10);
            container.set_valign(gtk::Align::Center);

            container.append(&icon);
            container.append(&label);

            list.append(&container);
        }

        window.set_child(Some(&list));

        window.present();
    });

    app.run()
}
