use std::path::Path;

use gtk::{self, gdk, glib, prelude::*};
use gtk_layer_shell::{Edge, LayerShell};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct Config {
    buttons: Vec<Button>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Button {
    label: String,
    icon: String,
}

fn main() -> glib::ExitCode {
    let path = std::fs::read_to_string("examples/config.toml").unwrap();
    let config: Config = toml::from_str(&path).unwrap();

    let app = gtk::Application::builder()
        .application_id("dev.github.arunim-io.apm")
        .build();

    app.connect_activate(activate(config));
    app.connect_startup(startup);
    app.run()
}

fn startup(_app: &gtk::Application) {
    let provider = gtk::CssProvider::new();
    provider.load_from_path(Path::new("examples/styles.css"));

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn activate(config: Config) -> impl Fn(&gtk::Application) {
    move |app| {
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
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 25);

        container.set_halign(gtk::Align::Center);
        config
            .clone()
            .buttons
            .into_iter()
            .for_each(|button| container.append(&get_button(button)));

        window.set_child(Some(&container));
        window.present();
    }
}

fn get_button(button: Button) -> gtk::Button {
    let label = gtk::Label::new(Some(&button.label));
    let icon = gtk::Image::from_file(Path::new(&button.icon));
    icon.set_width_request(100);
    icon.set_height_request(100);

    let container = gtk::Box::new(gtk::Orientation::Vertical, 25);
    container.set_valign(gtk::Align::Center);
    container.set_margin_top(50);
    container.set_margin_end(50);
    container.set_margin_start(50);
    container.set_margin_bottom(50);
    container.append(&icon);
    container.append(&label);

    let button = gtk::Button::new();
    button.set_valign(gtk::Align::Center);
    button.set_child(Some(&container));

    button.connect_clicked(|_| println!("Button clicked."));

    return button;
}
