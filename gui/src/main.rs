use iced::advanced::graphics::color;
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Widget};
use iced::widget::center;
use iced::widget::horizontal_space;
use iced::widget::{column, container, row, scrollable};
use iced::Alignment;
use iced::{mouse, Font};
use iced::{Element, Length, Rectangle, Renderer, Size, Theme, Transformation, Vector};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    info!("start gui");

    iced::application("Hexencer", Hexencer::update, Hexencer::view)
        .theme(Hexencer::theme)
        .font(include_bytes!("../../assets/fonts/5squared-pixel.ttf"))
        .default_font(Font::with_name("5squared pixel"))
        .run();
}

/// initialize the logging system
pub fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("hexencer started");
}

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default, Debug, Clone)]
struct Hexencer {
    theme: Theme,
}

impl Hexencer {
    fn update(&mut self, message: Message) {}

    fn view(&self) -> Element<Message> {
        let header = container(
            row![horizontal_space(), "Header!", horizontal_space(),]
                .padding(10)
                .align_items(Alignment::Center),
        );

        let bottom = container(
            row![horizontal_space(), "Bottom", horizontal_space()]
                .padding(10)
                .align_items(Alignment::Center),
        );

        let arranger = container(
            scrollable(
                column!["Track list"]
                    .spacing(40)
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
            )
            .height(Length::Fill),
        )
        .padding(10);

        let content = column![header, arranger, bottom];
        center(content).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
