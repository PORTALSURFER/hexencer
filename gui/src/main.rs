#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![allow(dead_code)]
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

//! the main entry point for the application

use std::borrow::Cow;

/// contains styling for the application
mod style;

/// contains the theme for the application
mod theme;

/// custom widgets for the application
mod widgets;

use hexencer_core::{Tick, TrackId};
use iced::advanced::graphics::core::Element;
use theme::Theme;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use widgets::clip::{Clip, DragEvent};
use widgets::track::Track;

use hexencer_core::data::{ClipId, StorageInterface};
use hexencer_engine::{midi_engine::start_midi_engine, start_sequencer_engine};
use iced::widget::{column, container, horizontal_space, row, scrollable, text};
use iced::{Alignment, Application, Font, Length, Renderer};

pub use hexencer_core::DataId;

/// the coolest message
static INSTA_MESSAGE: &str = "drag drop system in place...ya";

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

/// Message enum for the application
#[derive(Debug, Clone, Copy)]
pub enum Message {
    /// exit this application
    Exit,
    /// global message for dragging a clip
    DragClip {
        /// the clip id of the clip being dragged
        clip_id: ClipId,
        /// the point where the mouse click started the drag
        origin: f32,
    },
    /// a clip was dropped
    DroppedClip {
        /// the clip id of the clip being dragged
        clip_id: ClipId,
    },
    /// request to move clip between tracks
    MoveClipRequest {
        /// the id of the clip to move
        clip_id: ClipId,
        /// the new position of the clip
        track_id: TrackId,
        /// tick where the clip should be placed
        cursor_position: f32,
    },
}

/// initialize the settings for the application
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

/// contains the main application state
struct Hexencer {
    /// the storage interface for the application
    storage: StorageInterface,
    /// the clip id of the clip that was dropped
    dropped_clip: Option<ClipId>,
    /// relative origin of the latest clip drag
    drag_origin: f32, //TODO store this in the clip state instead?
}
impl Hexencer {
    /// initialize the application
    fn init() -> Self {
        let storage = StorageInterface::new();
        let midi_engine_sender = start_midi_engine();
        let _sequencer_sender = start_sequencer_engine(midi_engine_sender, storage.clone());
        Hexencer {
            storage,
            dropped_clip: None,
            drag_origin: 0.0,
        }
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
        if let Some(dropped_clip) = self.dropped_clip {
            tracing::info!("dropped clip: {:?}", dropped_clip);
            self.dropped_clip = None;
        }
        match message {
            Message::Exit => std::process::exit(0),
            Message::DragClip { origin, .. } => {
                self.drag_origin = origin;
                self.dropped_clip = None;
                iced::Command::none()
            }
            Message::DroppedClip { clip_id } => {
                self.dropped_clip = Some(clip_id);
                iced::Command::none()
            }
            Message::MoveClipRequest {
                clip_id,
                track_id,
                cursor_position,
            } => {
                info!("move clip request: {:?} to {:?}", clip_id, track_id);
                let tick = Tick::from(cursor_position - self.drag_origin);
                self.storage
                    .write()
                    .unwrap()
                    .project_manager
                    .move_clip(clip_id, track_id, tick);
                iced::Command::none()
            }
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

        let mut elements = Vec::new();
        let data = self.storage.read().unwrap();
        let tracks = &data.project_manager.tracks;

        for (index, track) in tracks.iter().enumerate() {
            let clips = &track.clips;
            let mut clip_elements = Vec::new();

            for (clip_id, _clip) in clips.iter() {
                let clip_key = clip_id.clone();
                let clip_element = Clip::new(
                    clip_key.id,
                    &self.storage,
                    text("drag drop system in place...yay"),
                )
                .on_drag(move |drag_event| {
                    let mut origin = 0.0;
                    if let DragEvent::DragStarted { grab_position } = drag_event {
                        println!("start drag at {:?}", grab_position);
                        origin = grab_position;
                    }
                    Message::DragClip {
                        clip_id: clip_key.id,
                        origin,
                    }
                })
                .on_drop(move |_| Message::DroppedClip {
                    clip_id: clip_key.id,
                });
                clip_elements.push(clip_element.into());
            }
            let track_id = track.id;

            let track = Track::new(
                &self.storage,
                index,
                track_id,
                clip_elements,
                self.dropped_clip,
            )
            .on_drop(move |clip_id, track_id, cursor_position| {
                info!("dropped clip: {:?} onto track {}", clip_id, index);
                Message::MoveClipRequest {
                    clip_id,
                    track_id,
                    cursor_position,
                }
            });
            elements.push(track.into());
        }

        // let tracks = load_tracks(&self.storage);
        let tracks_column = column(elements).spacing(1);
        let test = column![text(INSTA_MESSAGE)];

        let content = container(
            scrollable(
                column!["Some tracks", tracks_column, test, "The end"]
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
