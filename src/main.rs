use iced::widget::{button, column, Column};
use iced::{window, Application, Command, Element, Settings, Size, Theme};

#[derive(Copy, Clone, Debug)]
enum Data {
    Sus,
}

struct App;

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Data;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("RGBcraft")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Data> {
        column!(button("sus").on_press(Data::Sus)).into()
    }
}

fn main() {
    App::run(Settings {
        window: window::Settings {
            size: Size::new(800.0, 500.0),
            ..window::Settings::default()
        },
        ..Default::default()
    })
    .unwrap()
}
