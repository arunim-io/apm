use std::path::Path;

use gtk::{self, gdk, glib, prelude::*};
use gtk_layer_shell::{self, Edge, LayerShell};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct Config {
    buttons: Vec<Button>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Button {
    label: String,
    icon: String,
    cmd: String,
    keybind: String,
}

fn main() -> glib::ExitCode {
    let path = std::fs::read_to_string("examples/config.toml").unwrap();
    let config: Config = toml::from_str(&path).unwrap();

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

    app.connect_activate(move |app| {
        let buttons = config.clone().buttons;
        let window = gtk::ApplicationWindow::new(app);

        window.init_layer_shell();
        window.set_layer(gtk_layer_shell::Layer::Overlay);
        window.set_exclusive_zone(-1);

        let controller = gtk::EventControllerKey::new();
        controller.connect_key_pressed(move |_, key, _, _| {
            if let gdk::Key::Escape = key {
                std::process::exit(0);
            }
            buttons.clone().into_iter().for_each(|button| {
                let keybind = gdk::Key::from_name(&button.keybind).unwrap();
                if keybind == key {
                    println!("{}", keybind);
                    std::process::exit(0);
                }
            });
            glib::Propagation::Proceed
        });
        window.add_controller(controller);
        window.set_fullscreened(true);
        window.set_keyboard_mode(gtk_layer_shell::KeyboardMode::Exclusive);
        for edge in vec![Edge::Top, Edge::Bottom, Edge::Left, Edge::Right] {
            window.set_anchor(edge, true);
        }
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(25)
            .halign(gtk::Align::Center)
            .build();

        config.buttons.clone().into_iter().for_each(|data| {
            let label = gtk::Label::new(Some(&data.label));
            let icon = gtk::Image::builder()
                .file(data.icon)
                .width_request(100)
                .height_request(100)
                .build();

            let content = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(25)
                .valign(gtk::Align::Center)
                .margin_top(50)
                .margin_end(50)
                .margin_start(50)
                .margin_bottom(50)
                .build();
            content.append(&icon);
            content.append(&label);

            let button = gtk::Button::builder()
                .valign(gtk::Align::Center)
                .child(&content)
                .build();
            button.connect_clicked(move |_| println!("{}", data.cmd));

            container.append(&button);
        });

        window.set_child(Some(&container));
        window.present();
    });

    app.run()
}
