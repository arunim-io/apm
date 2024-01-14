use color_eyre::eyre::{ContextCompat, Result};
use gtk::gdk::{self, Key};
use gtk::glib;
use gtk::prelude::*;
use gtk::{Align, Orientation};
use gtk_layer_shell::{Edge, LayerShell};

use crate::config::{self, Config};

pub fn run(config: Config) -> Result<glib::ExitCode> {
    let app = gtk::Application::builder()
        .application_id("dev.github.arunim-io.apm")
        .build();

    app.connect_startup(move |_| {
        if let Some(display) = gdk::Display::default() {
            let provider = gtk::CssProvider::new();
            provider.load_from_path(Config::get_styles_path());

            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    });
    app.connect_activate(move |app| {
        let window = gtk::ApplicationWindow::new(app);

        window.init_layer_shell();
        window.set_layer(gtk_layer_shell::Layer::Overlay);
        window.set_exclusive_zone(-1);
        window.set_keyboard_mode(gtk_layer_shell::KeyboardMode::Exclusive);
        for edge in vec![Edge::Top, Edge::Bottom, Edge::Left, Edge::Right] {
            window.set_anchor(edge, true);
        }

        window.add_controller(get_controller(&config.buttons));
        window.set_child(Some(&get_container(&config)));

        window.present();
    });

    Ok(app.run())
}

fn get_container(config: &Config) -> gtk::Box {
    let container = gtk::Box::builder()
        .name("container")
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .valign(Align::Center)
        .spacing(config.spacing.unwrap_or_else(|| 25))
        .build();
    let ref buttons = config.buttons;

    buttons.into_iter().for_each(|button| {
        container.append(&button.get_widget(config.icon_size, config.icon_margin));
    });

    return container;
}

fn get_controller(buttons: &Vec<config::Button>) -> gtk::EventControllerKey {
    let controller = gtk::EventControllerKey::new();
    let buttons = buttons.to_owned();

    controller.connect_key_pressed(move |_, key, _, _| {
        if let Key::Escape = key {
            std::process::exit(0);
        }

        buttons.to_owned().into_iter().for_each(|ref button| {
            if let Ok(bkey) = button.get_key() {
                if bkey == key {
                    button.exec_cmd();
                }
            }
        });

        glib::Propagation::Proceed
    });

    return controller;
}

impl config::Button {
    fn get_key(&self) -> Result<Key> {
        let context = || format!("Invalid key for {} button.", self.label);
        let key = self.to_owned().key.with_context(context)?;

        Ok(Key::from_name(key).with_context(context)?)
    }

    fn get_widget(&self, icon_size: Option<i32>, icon_margin: Option<i32>) -> gtk::Box {
        let data = self.to_owned();
        let container = gtk::Box::builder()
            .name(self.label.clone())
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
        button.connect_clicked(move |_| data.exec_cmd());

        container.append(&button);
        container.append(&gtk::Label::new(Some(self.label.as_str())));

        return container;
    }
}
