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

        window.add_controller(get_controller(config.clone().buttons));
        window.set_child(get_container(&config).as_ref());

        window.present();
    }
}

fn get_container(config: &Config) -> Option<gtk::Box> {
    let container = gtk::Box::builder()
        .name("container")
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .valign(Align::Center)
        .spacing(config.spacing.unwrap_or_else(|| 25))
        .build();

    config.to_owned().buttons.into_iter().for_each(|button| {
        container.append(&button.get_widget(config.icon_size, config.icon_margin));
    });

    return Some(container);
}

fn get_controller(buttons: Vec<config::Button>) -> gtk::EventControllerKey {
    let controller = gtk::EventControllerKey::new();

    controller.connect_key_pressed(move |_, key, _, _| {
        if let Key::Escape = key {
            std::process::exit(0);
        }

        buttons.to_owned().into_iter().for_each(|button| {
            if button.clone().get_key() == key {
                button.clone().exec_cmd();
            }
        });

        glib::Propagation::Proceed
    });

    return controller;
}

impl config::Button {
    fn get_key(self) -> Key {
        let err_msg = format!("Invalid key for {} button.", self.label);
        let key = self.key.expect(&err_msg);

        return Key::from_name(key).expect(&err_msg);
    }
    fn get_widget(self, icon_size: Option<i32>, icon_margin: Option<i32>) -> gtk::Box {
        let label = self.label.as_str();
        let container = gtk::Box::builder()
            .name(label)
            .orientation(Orientation::Vertical)
            .spacing(10)
            .build();

        let margin = icon_margin.unwrap_or_else(|| 10);
        let icon = gtk::Image::builder()
            .file(Config::get_file_path(&self.icon).to_string_lossy())
            .margin_end(margin)
            .margin_top(margin)
            .margin_start(margin)
            .margin_bottom(margin)
            .pixel_size(icon_size.unwrap_or_else(|| 50))
            .build();

        let button = gtk::Button::builder().child(&icon).build();
        button.add_css_class("circular");

        container.append(&button);
        container.append(&gtk::Label::new(Some(label)));

        return container;
    }
}
