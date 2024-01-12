use gtk::gdk::{self, Key};
use gtk::glib;
use gtk::prelude::*;
use gtk::{Align, Orientation};
use gtk_layer_shell::{Edge, LayerShell};

use crate::config::{self, Config};

pub fn run(config: Config) -> glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("dev.github.arunim-io.apm")
        .build();

    app.connect_startup(startup());
    app.connect_activate(activate(config));

    app.run()
}

fn startup() -> impl Fn(&gtk::Application) {
    |_| {
        let provider = gtk::CssProvider::new();
        provider.load_from_path(Config::get_styles_path());

        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn activate(config: Config) -> impl Fn(&gtk::Application) {
    move |app| {
        let window = gtk::ApplicationWindow::new(app);

        window.init_layer_shell();
        window.set_layer(gtk_layer_shell::Layer::Overlay);
        window.set_exclusive_zone(-1);
        window.set_keyboard_mode(gtk_layer_shell::KeyboardMode::Exclusive);
        for edge in vec![Edge::Top, Edge::Bottom, Edge::Left, Edge::Right] {
            window.set_anchor(edge, true);
        }

        let controller = gtk::EventControllerKey::new();
        controller.connect_key_pressed(move |_, key, _, _| {
            if let Key::Q | Key::q | Key::Escape = key {
                std::process::exit(0);
            }
            glib::Propagation::Proceed
        });
        window.add_controller(controller);

        window.set_child(Some(&get_container(&config)));
        window.present();
    }
}

fn get_container(config: &Config) -> gtk::Box {
    let container = gtk::Box::builder()
        .name("container")
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .valign(Align::Center)
        .spacing(25)
        .build();

    config
        .to_owned()
        .buttons
        .into_iter()
        .for_each(|button| container.append(&button.get_widget()));

    return container;
}

impl config::Button {
    fn get_widget(self) -> gtk::Box {
        let label = self.label.as_str();
        let container = gtk::Box::builder()
            .name(label)
            .orientation(Orientation::Vertical)
            .spacing(10)
            .build();
        let icon = gtk::Image::builder()
            .file(Config::get_file_path(&self.icon).to_string_lossy())
            .margin_end(10)
            .margin_top(10)
            .margin_start(10)
            .margin_bottom(10)
            .pixel_size(50)
            .build();

        let button = gtk::Button::builder().child(&icon).build();
        button.add_css_class("circular");

        container.append(&button);
        container.append(&gtk::Label::new(Some(label)));

        return container;
    }
}
