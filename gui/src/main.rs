//! the main entry point for the application

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// contains custom widgets for hexencer
mod widget;

use std::time::Instant;

use hexencer_core::data::{ClipId, StorageInterface};
use hexencer_core::{Tick, TrackId};
use iced::advanced::graphics::color;
use iced::advanced::widget::Tree;
use iced::advanced::{layout, mouse, renderer, Layout, Widget};
use iced::widget::canvas::Program;
use iced::widget::canvas::{stroke, Path, Stroke};
use iced::widget::scrollable::Properties;
use iced::widget::{canvas, horizontal_space};
use iced::widget::{center, text};
use iced::widget::{column, container, row};
use iced::{window, Alignment, Color, Point, Renderer, Size, Subscription, Transformation, Vector};
use iced::{Element, Length, Theme};
use iced::{Font, Rectangle};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use widget::{Arranger, Clip, DragEvent, Track};

fn main() {
    info!("start gui");

    init_logger();

    let _ = iced::application("Hexencer", Hexencer::update, Hexencer::view)
        .theme(Hexencer::theme)
        .font(include_bytes!("../../assets/fonts/5squared-pixel.ttf"))
        // .subscription(Hexencer::subscription)
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

/// test for sequencer line widget
pub struct SequencerLine {
    /// width of the line
    width: f32,
    /// height of the line
    height: f32,
}

impl<Message> Widget<Message, Theme, Renderer> for SequencerLine {
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fixed(50.0),
            height: Length::Fixed(50.0),
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = limits.resolve(self.width, self.height, Size::ZERO);

        layout::Node::new(size)
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        use iced::advanced::graphics::mesh::{self, Mesh, Renderer as _, SolidVertex2D};
        use iced::advanced::Renderer as _;

        let bounds = layout.bounds();

        // R O Y G B I V
        let color_r = [1.0, 0.0, 0.0, 1.0];
        let color_o = [1.0, 0.5, 0.0, 1.0];
        let color_y = [1.0, 1.0, 0.0, 1.0];
        let color_g = [0.0, 1.0, 0.0, 1.0];
        let color_gb = [0.0, 1.0, 0.5, 1.0];
        let color_b = [0.0, 0.2, 1.0, 1.0];
        let color_i = [0.5, 0.0, 1.0, 1.0];
        let color_v = [0.75, 0.0, 0.5, 1.0];

        let posn_center = {
            if let Some(cursor_position) = cursor.position_in(bounds) {
                [cursor_position.x, cursor_position.y]
            } else {
                [bounds.width / 2.0, bounds.height / 2.0]
            }
        };

        let posn_tl = [0.0, 0.0];
        let posn_t = [bounds.width / 2.0, 0.0];
        let posn_tr = [bounds.width, 0.0];
        let posn_r = [bounds.width, bounds.height / 2.0];
        let posn_br = [bounds.width, bounds.height];
        let posn_b = [(bounds.width / 2.0), bounds.height];
        let posn_bl = [0.0, bounds.height];
        let posn_l = [0.0, bounds.height / 2.0];

        let mesh = Mesh::Solid {
            buffers: mesh::Indexed {
                vertices: vec![
                    SolidVertex2D {
                        position: posn_center,
                        color: color::pack([1.0, 1.0, 1.0, 1.0]),
                    },
                    SolidVertex2D {
                        position: posn_tl,
                        color: color::pack(color_r),
                    },
                    SolidVertex2D {
                        position: posn_t,
                        color: color::pack(color_o),
                    },
                    SolidVertex2D {
                        position: posn_tr,
                        color: color::pack(color_y),
                    },
                    SolidVertex2D {
                        position: posn_r,
                        color: color::pack(color_g),
                    },
                    SolidVertex2D {
                        position: posn_br,
                        color: color::pack(color_gb),
                    },
                    SolidVertex2D {
                        position: posn_b,
                        color: color::pack(color_b),
                    },
                    SolidVertex2D {
                        position: posn_bl,
                        color: color::pack(color_i),
                    },
                    SolidVertex2D {
                        position: posn_l,
                        color: color::pack(color_v),
                    },
                ],
                indices: vec![
                    0, 1, 2, // TL
                    0, 2, 3, // T
                    0, 3, 4, // TR
                    0, 4, 5, // R
                    0, 5, 6, // BR
                    0, 6, 7, // B
                    0, 7, 8, // BL
                    0, 8, 1, // L
                ],
            },
            transformation: Transformation::IDENTITY,
            clip_bounds: Rectangle::INFINITE,
        };

        renderer.with_translation(Vector::new(bounds.x, bounds.y), |renderer| {
            renderer.draw_mesh(mesh);
        });
    }
}
impl<'a, Message> From<SequencerLine> for Element<'a, Message> {
    fn from(line: SequencerLine) -> Self {
        Self::new(line)
    }
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

    /// tick message for updating the system
    Tick(Instant),
}

#[derive(Debug)]
struct Hexencer {
    /// the theme for the application
    theme: Theme,

    /// the storage interface for the application
    storage: StorageInterface,
    /// a clip that was dropped
    dropped_clip: Option<ClipId>, // TODO #53 move this elsewhere
    /// the origin of the drag for the clip that was dropped
    drag_origin: f32,
    /// state used for drawing a canvas, used for the transport line drawing
    state: State,
}

/// state type used for canvas drawing of the transport line
#[derive(Debug)]
struct State {
    /// unused clock taken from example
    now: Instant,
    /// cache which stores the canvas drawing elements
    system_cache: canvas::Cache,
}
impl State {
    /// create a new state
    fn new() -> State {
        let now = Instant::now();
        Self {
            now,
            system_cache: canvas::Cache::default(),
        }
    }

    /// update the canvas state
    pub fn update2(&mut self, now: Instant) {
        self.now = now;
        self.system_cache.clear();
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut mouse_point = Point::new(200.0, 200.0);
        if let Some(cursor_position) = cursor.position_in(bounds) {
            mouse_point = cursor_position;
        };
        let line_cache = self.system_cache.draw(renderer, bounds.size(), |frame| {
            let line = Path::line(Point::new(0.0, 0.0), mouse_point);
            frame.stroke(
                &line,
                Stroke {
                    style: stroke::Style::Solid(Color::from_rgba8(0, 153, 255, 0.1)),
                    width: 5.0,
                    ..Stroke::default()
                },
            );
        });

        vec![line_cache]
    }
}

impl Default for Hexencer {
    fn default() -> Self {
        Self {
            theme: Theme::KanagawaDragon,
            storage: StorageInterface::default(),
            dropped_clip: None,
            drag_origin: 0.0,
            state: State::default(),
        }
    }
}

impl Hexencer {
    /// update the application state
    fn update(&mut self, message: Message) {
        if let Some(dropped_clip) = self.dropped_clip {
            tracing::info!("dropped clip: {:?}", dropped_clip);
            self.dropped_clip = None;
        }
        match message {
            Message::Exit => std::process::exit(0),
            Message::DragClip { origin, clip_id: _ } => {
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
            Message::Tick(instant) => {
                self.state.update2(instant);
            }
        }
    }
    /// remove a clip from the storage
    pub fn remove_clip(&mut self, clip_id: ClipId) {
        let mut to_remove = None;

        let mut data = self.storage.write().unwrap();
        let track_collection = &mut data.project_manager.track_collection;
        for track in track_collection.tracks() {
            for (clip_key, clip) in track.clips.iter() {
                if clip.id() == clip_id {
                    info!("clip found: {:?} in track {:?}", clip_id, track.id);
                    to_remove = Some((track.id, *clip_key));
                    break;
                }
            }
            if to_remove.is_some() {
                break;
            }
        }

        if let Some((track_id, clip_key)) = to_remove {
            if let Some(track) = track_collection.get_mut(track_id) {
                track.clips.remove(&clip_key).unwrap();
            }
        }
    }

    /// draw the view
    fn view(&self) -> Element<Message> {
        let wgpu_box = SequencerLine {
            width: 50.0,
            height: 50.0,
        };

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
        let track_list = &data.project_manager.track_collection;

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

        // TODO #52 draw this in an overlay?
        let line_canvas = canvas(&self.state)
            .width(Length::Fill)
            .height(Length::Fixed(200.0));
        // let tracks = load_tracks(&self.storage);
        let tracks_column = column(elements).spacing(1);

        let _scroll_properties = Properties::default();

        let arranger = container(
            Arranger::new(
                column![line_canvas, "Track list", tracks_column, wgpu_box]
                    .spacing(40)
                    .align_items(Alignment::Center)
                    .width(Length::Fixed(5000.0)),
            )
            .height(Length::Fill),
        )
        .padding(10);

        let content = column![header, arranger, bottom];
        center(content).into()
    }

    /// get the theme for the application
    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    /// get the subscription for the application
    fn _subscription(&self) -> Subscription<Message> {
        window::frames().map(Message::Tick)
    }
}
