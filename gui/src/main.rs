//! the main entry point for the application

#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]

mod widget;

use hexencer_core::data::{ClipId, StorageInterface};
use hexencer_core::{Tick, TrackId};
use iced::widget::horizontal_space;
use iced::widget::{center, text};
use iced::widget::{column, container, row, scrollable};
use iced::Alignment;
use iced::Font;
use iced::{Element, Length, Theme};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use widget::{Clip, DragEvent, Track};

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

#[derive(Debug, Clone)]
struct Hexencer {
    theme: Theme,
    storage: StorageInterface,
    dropped_clip: Option<ClipId>, // TODO move this elsewhere
    drag_origin: f32,
}

impl Default for Hexencer {
    fn default() -> Self {
        Self {
            theme: Theme::KanagawaDragon,
            storage: StorageInterface::default(),
            dropped_clip: None,
            drag_origin: 0.0,
        }
    }
}

impl Hexencer {
    fn update(&mut self, message: Message) {
        if let Some(dropped_clip) = self.dropped_clip {
            tracing::info!("dropped clip: {:?}", dropped_clip);
            self.dropped_clip = None;
        }
        match message {
            Message::Exit => std::process::exit(0),
            Message::DragClip { origin, .. } => {
                self.drag_origin = origin;
                self.dropped_clip = None;
            }
            Message::DroppedClip { clip_id } => {
                self.dropped_clip = Some(clip_id);
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
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let header = container(
            row![horizontal_space(), "Header!", horizontal_space(),]
                .padding(10)
                .align_items(Alignment::Center),
        );

        let bpm = self.storage.read().unwrap().bpm();
        let bpm_widget = text(bpm.to_string()).size(60);

        let bottom = container(
            row![horizontal_space(), bpm_widget, horizontal_space()]
                .padding(10)
                .align_items(Alignment::Center),
        );

        let mut elements = Vec::new();
        let data = self.storage.read().unwrap();
        let track_list = &data.project_manager.tracks;

        for (index, track) in track_list.iter().enumerate() {
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
                        info!("clip drag started at {:?}", grab_position);
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

        let arranger = container(
            scrollable(
                column!["Track list", tracks_column]
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
