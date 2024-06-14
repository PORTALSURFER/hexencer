#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![allow(dead_code)]
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

//! the main entry point for the application
use std::borrow::Cow;

mod style;
mod theme;
mod widgets;

use theme::Theme;
use widgets::track::Track;

use hexencer_core::data::DataInterface;
use hexencer_engine::{midi_engine::start_midi_engine, start_sequencer_engine};
use iced::widget::{canvas, column, container, horizontal_space, row, scrollable, Canvas};
use iced::{
    mouse, Alignment, Application, Color, Element, Font, Length, Point, Rectangle, Renderer,
};

pub use hexencer_core::DataId;

#[tokio::main]
async fn main() -> iced::Result {
    let data_layer = DataInterface::new();
    let midi_engine_sender = start_midi_engine();
    let sequencer_sender = start_sequencer_engine(midi_engine_sender, data_layer.clone());

    // let options = options();
    // run(options, data_layer, sequencer_sender);

    let fonts = vec![Cow::from(include_bytes!(
        "../../assets/fonts/5squared-pixel.ttf"
    ))];

    let settings = iced::Settings {
        fonts,
        default_font: Font::with_name("5squared pixel"),
        ..Default::default()
    };
    Hexencer::run(settings)
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Exit,
}

struct Hexencer {}

impl iced::Application for Hexencer {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Hexencer, iced::Command<Message>) {
        (Hexencer {}, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("Hexencer")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Exit => std::process::exit(0),
        }
    }

    fn view(&self) -> Element<Self::Message, Self::Theme> {
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

        // let sidebar = container(
        //     column!["Sidebar!"]
        //         .spacing(40)
        //         .padding(10)
        //         .width(200)
        //         .align_items(Alignment::Center),
        // );
        // .height(Length::Fill);

        let mut tracks = Vec::new();
        for i in 0..5 {
            let track = Track::new();
            tracks.push(track.into());
        }

        let tracks_column = column(tracks).spacing(2);

        let content = container(
            scrollable(
                column!["Some tracks", tracks_column, "The end"]
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
