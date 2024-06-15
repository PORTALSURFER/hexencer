#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![allow(dead_code)]
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

//! the main entry point for the application
use std::borrow::Cow;

mod style;
mod theme;
mod widgets;

use iced::advanced::graphics::core::Element;
use theme::Theme;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use widgets::track::Track;

use hexencer_core::data::StorageInterface;
use hexencer_engine::{midi_engine::start_midi_engine, start_sequencer_engine};
use iced::widget::{column, container, horizontal_space, row, scrollable};
use iced::{Alignment, Application, Font, Length, Renderer};

pub use hexencer_core::DataId;

#[tokio::main]
async fn main() -> iced::Result {
    init_logger();

    let settings = init_settings();
    Hexencer::run(settings)
}
/// initialize the logging system
pub fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("hexencer started");
}
#[derive(Debug, Clone, Copy)]
enum Message {
    Exit,
}

fn init_settings<Flags: Default>() -> iced::Settings<Flags> {
    let fonts = vec![Cow::from(include_bytes!(
        "../../assets/fonts/5squared-pixel.ttf"
    ))];

    iced::Settings {
        fonts,
        default_font: Font::with_name("5squared pixel"),
        ..Default::default()
    }
}

struct Hexencer {
    storage: StorageInterface,
}
impl Hexencer {
    fn init() -> Self {
        let storage = StorageInterface::new();
        let midi_engine_sender = start_midi_engine();
        let sequencer_sender = start_sequencer_engine(midi_engine_sender, storage.clone());
        Hexencer { storage }
    }
}

impl iced::Application for Hexencer {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Hexencer, iced::Command<Message>) {
        (Hexencer::init(), iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("Hexencer")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Exit => std::process::exit(0),
        }
    }

    fn view(&self) -> Element<Self::Message, Self::Theme, Renderer> {
        let header = container(
            row![horizontal_space(), "Header!", horizontal_space(),]
                .padding(10)
                .align_items(Alignment::Center),
        )
        .style(style::Container::Header);

        let bottom = container(
            row![horizontal_space(), "bottom!", horizontal_space(),]
                .padding(10)
                .align_items(Alignment::Center),
        )
        .style(style::Container::Bottom);

        // let tracks = load_tracks();
        // let tracks_column = column(tracks).spacing(1);

        let track = Track::new(&self.storage);

        let content = container(
            scrollable(
                column!["Some tracks", track, "The end"]
                    .spacing(40)
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
            )
            .height(Length::Fill),
        )
        .padding(10);

        column![header, content, bottom].into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }
}

// fn load_tracks() -> Vec<Element<'static, Message, Theme, Renderer>> {
//     let mut tracks = Vec::new();
//     for _ in 0..5 {
//         let track = Track::new();
//         tracks.push(track.into());
//     }
//     tracks
// }

// fn square<'a>(size: impl Into<Length> + Copy) -> Element<'a, Message, Renderer> {
//     struct Square;

//     impl canvas::Program<Message, Theme, Renderer> for Square {
//         type State = ();

//         fn draw(
//             &self,
//             _state: &Self::State,
//             renderer: &Renderer,
//             theme: &Theme,
//             bounds: Rectangle,
//             _cursor: mouse::Cursor,
//         ) -> Vec<canvas::Geometry> {
//             let mut frame = canvas::Frame::new(renderer, bounds.size());

//             frame.fill_rectangle(
//                 Point::ORIGIN,
//                 bounds.size(),
//                 Color::from_rgb(1.0, 0.0, 0.0),
//             );

//             vec![frame.into_geometry()]
//         }
//     }

//     Canvas(Square).width(size).height(size).into()
// }
