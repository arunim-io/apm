use std::{path::Path, process::Command};

use gtk::{self, gdk, glib, prelude::*};
use gtk_layer_shell::{self, Edge, LayerShell};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct Config {
    buttons: Vec<Button>,
    spacing: Option<i32>,
}

impl Config {
    fn read_from_path(path: &str) -> Self {
        let file = std::fs::read_to_string(path).expect("Unable to read config file");
        return toml::from_str::<Self>(&file).expect("Unable to parse config file");
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Button {
    label: String,
    icon: String,
    cmd: String,
    keybind: String,
    icon_size: Option<i32>,
    icon_height: Option<i32>,
    icon_width: Option<i32>,
    padding: Option<i32>,
    padding_x: Option<i32>,
    padding_y: Option<i32>,
    padding_left: Option<i32>,
    padding_right: Option<i32>,
    padding_top: Option<i32>,
    padding_bottom: Option<i32>,
}

impl Button {
    fn get_key(self) -> gdk::Key {
        return gdk::Key::from_name(self.keybind).expect("Invalid Keybind!");
    }
    fn exec_cmd(self) {
        Command::new("sh")
            .args(["-c", &self.cmd])
            .output()
            .expect("Unable to run command");
        std::process::exit(0);
    }
    fn get_padding(self) -> (i32, i32, i32, i32) {
        let padding = self.padding.unwrap_or_else(|| 25);
        let [mut left, mut right, mut top, mut bottom] = [padding; 4];

        if let Some(padding) = self.padding_x {
            left = padding;
            right = padding;
        }
        if let Some(padding) = self.padding_y {
            top = padding;
            bottom = padding;
        }
        if let Some(padding) = self.padding_left {
            left = padding;
        }
        if let Some(padding) = self.padding_right {
            right = padding;
        }
        if let Some(padding) = self.padding_top {
            top = padding;
        }
        if let Some(padding) = self.padding_bottom {
            bottom = padding;
        }

        return (left, right, top, bottom);
    }
    fn get_icon_size(self) -> (i32, i32) {
        let size = self.icon_size.unwrap_or(50);
        let [mut height, mut width] = [size; 2];
        if let Some(size) = self.icon_height {
            height = size;
        }
        if let Some(size) = self.icon_width {
            width = size;
        }

        return (height, width);
    }
}

fn main() -> glib::ExitCode {
    let config = Config::read_from_path("examples/config.toml");

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
                if button.clone().get_key() == key {
                    button.exec_cmd();
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
            .spacing(config.spacing.unwrap_or(25))
            .halign(gtk::Align::Center)
            .build();

        config.buttons.clone().into_iter().for_each(|data| {
            let label = gtk::Label::new(Some(&data.label));
            let (height, width) = data.clone().get_icon_size();
            let icon = gtk::Image::builder()
                .file(&data.icon)
                .width_request(height)
                .height_request(width)
                .build();

            let (left, right, top, bottom) = data.clone().get_padding();
            let content = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(25)
                .valign(gtk::Align::Center)
                .margin_top(top)
                .margin_end(right)
                .margin_start(left)
                .margin_bottom(bottom)
                .build();
            content.append(&icon);
            content.append(&label);

            let button = gtk::Button::builder()
                .valign(gtk::Align::Center)
                .child(&content)
                .build();
            button.connect_clicked(move |_| data.clone().exec_cmd());

            container.append(&button);
        });

        window.set_child(Some(&container));
        window.present();
    });

    app.run()
}
