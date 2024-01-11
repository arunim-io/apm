use crate::config::{self, Config};
use gtk::gdk::{Display, Key};
use gtk::glib::{ExitCode, Propagation};
use gtk::{prelude::*, Image};
use gtk::{
    style_context_add_provider_for_display, Align, Application, ApplicationWindow, Box, Button,
    CssProvider, EventControllerKey, Orientation, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use gtk_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

pub fn run(config: Config) -> ExitCode {
    let app = Application::builder()
        .application_id("dev.github.arunim-io.apm")
        .build();

    app.connect_startup(startup());
    app.connect_activate(activate(config));

    app.run()
}

fn startup() -> impl Fn(&Application) {
    |_| {
        let provider = CssProvider::new();
        provider.load_from_path(Config::get_styles_path());

        style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn activate(config: Config) -> impl Fn(&Application) {
    move |app| {
        let window = ApplicationWindow::new(app);

        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.set_exclusive_zone(-1);
        window.set_keyboard_mode(KeyboardMode::Exclusive);
        for edge in vec![Edge::Top, Edge::Bottom, Edge::Left, Edge::Right] {
            window.set_anchor(edge, true);
        }

        let controller = EventControllerKey::new();
        controller.connect_key_pressed(move |_, key, _, _| {
            if let Key::Q | Key::q | Key::Escape = key {
                std::process::exit(0);
            }
            Propagation::Proceed
        });
        window.add_controller(controller);

        window.set_child(Some(&get_container(&config)));
        window.present();
    }
}

fn get_container(config: &Config) -> Box {
    let container = Box::builder()
        .name("container")
        .orientation(Orientation::Vertical)
        .halign(Align::BaselineCenter)
        .valign(Align::Center)
        .spacing(25)
        .build();

    let list = Box::new(Orientation::Horizontal, 25);
    config.to_owned().buttons.into_iter().for_each(|item| {
        list.append(&get_button(item));
    });
    container.append(&list);

    return container;
}

fn get_button(data: config::Button) -> Button {
    let icon = Image::builder()
        .file(Config::get_file_path(&data.icon).to_string_lossy())
        .margin_end(10)
        .margin_top(10)
        .margin_start(10)
        .margin_bottom(10)
        .pixel_size(50)
        .build();

    let button = Button::builder().child(&icon).build();
    button.add_css_class("circular");

    return button;
}
