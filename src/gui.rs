use gtk::{
    gdk::{Display, Key},
    glib::{ExitCode, Propagation},
    prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt, WidgetExt},
    style_context_add_provider_for_display, Application, ApplicationWindow, CssProvider,
    EventControllerKey, Label, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use gtk_layer_shell::{Edge, KeyboardMode, LayerShell};

use crate::config::Config;

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
        window.set_layer(gtk_layer_shell::Layer::Overlay);
        window.set_exclusive_zone(-1);
        window.set_fullscreened(true);
        window.set_keyboard_mode(KeyboardMode::Exclusive);
        for edge in vec![Edge::Top, Edge::Bottom, Edge::Left, Edge::Right] {
            window.set_anchor(edge, true);
        }

        let controller = EventControllerKey::new();
        controller.connect_key_pressed(move |_, key, _, _| {
            if let Key::Escape = key {
                std::process::exit(0);
            }
            Propagation::Proceed
        });
        window.add_controller(controller);

        window.set_child(Some(&Label::new(Some("Hello!"))));

        window.present();
    }
}

