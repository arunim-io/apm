use iced::{executor, Application, Command, Element, Renderer, Settings};

#[derive(Debug)]
pub struct App;

impl Application for App {
    type Executor = executor::Default;
    type Message = ();
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        "Hello, world!".into()
    }
}

pub fn run_gui() -> iced::Result {
    App::run(Settings::default())
}
