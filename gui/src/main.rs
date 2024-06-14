#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![allow(dead_code)]
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

//! the main entry point for the application

/// arranger part of the gui
mod arranger;
/// gui code
mod gui;
/// utilities for loading and storing data to egui memory
mod memory;
/// ui elements
mod ui;
/// utility code
mod utility;

pub use gui::{options, run};
use hexencer_core::data::DataInterface;
use hexencer_engine::{midi_engine::start_midi_engine, start_sequencer_engine};
use iced::widget::{
    button, canvas, checkbox, column, container, horizontal_space, pick_list, row, scrollable, text,
};
use iced::{
    color, mouse, Alignment, Application, Element, Font, Length, Point, Rectangle, Renderer,
    Subscription, Theme,
};
pub use memory::WidgetState;
use tracing::instrument::WithSubscriber;
use utility::init_logger;

pub use hexencer_core::DataId;

#[tokio::main]
async fn main() -> iced::Result {
    init_logger();

    let data_layer = DataInterface::new();
    let midi_engine_sender = start_midi_engine();
    let sequencer_sender = start_sequencer_engine(midi_engine_sender, data_layer.clone());

    // let options = options();
    // run(options, data_layer, sequencer_sender);

    Hexencer::run(iced::Settings::default())
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Exit,
}

struct Hexencer {}

impl iced::Application for Hexencer {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
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

    fn view(&self) -> iced::Element<Message> {
        let header = container(
            row![horizontal_space(), "Header!", horizontal_space(),]
                .padding(10)
                .align_items(Alignment::Center),
        );
        let bottom = container(
            row![horizontal_space(), "bottom!", horizontal_space(),]
                .padding(10)
                .align_items(Alignment::Center),
        );

        let sidebar = container(
            column!["Sidebar!", square(40)]
                .spacing(40)
                .padding(10)
                .width(200)
                .align_items(Alignment::Center),
        )
        .height(Length::Fill);

        let content = container(
            scrollable(
                column!["Content!", "The end"]
                    .spacing(40)
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
            )
            .height(Length::Fill),
        )
        .padding(10);

        column![header, row![sidebar, content], bottom].into()
    }

    fn theme(&self) -> Theme {
        iced::Theme::Dark
    }
}

fn square<'a>(size: impl Into<Length> + Copy) -> iced::Element<'a, Message> {
    struct Square;

    impl canvas::Program<Message> for Square {
        type State = ();

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let mut frame = canvas::Frame::new(renderer, bounds.size());

            let palette = theme.extended_palette();

            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                palette.background.strong.color,
            );

            vec![frame.into_geometry()]
        }
    }

    canvas(Square).width(size).height(size).into()
}
