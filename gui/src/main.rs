//! the main entry point for the application

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// contains custom widgets for hexencer
mod widget;

use std::sync::Arc;
use std::time::Instant;

use hexencer_core::data::{ClipId, StorageInterface};
use hexencer_core::{Tick, TrackId};
use hexencer_engine::{midi_engine, Sequencer, SequencerCommand, SequencerHandle};
use iced::advanced::graphics::color;
use iced::advanced::widget::Tree;
use iced::advanced::{layout, mouse, renderer, Layout, Widget};
use iced::mouse::Cursor;
use iced::widget::canvas::{stroke, Path, Stroke};
use iced::widget::scrollable::Properties;
use iced::widget::{button, canvas, horizontal_space, stack};
use iced::widget::{center, text};
use iced::widget::{column, container, row};
use iced::{window, Alignment, Color, Point, Renderer, Size, Subscription, Transformation, Vector};
use iced::{Element, Length, Theme};
use iced::{Font, Rectangle};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use widget::{Arranger, Clip, DragEvent, EventEditor, Track};

#[tokio::main]
async fn main() {
    info!("start gui");

    init_logger();

    let _ = iced::application("Hexencer", Hexencer::update, Hexencer::view)
        .theme(Hexencer::theme)
        .font(include_bytes!("../../assets/fonts/5squared-pixel.ttf"))
        // .subscription(Hexencer::subscription)
        .default_font(Font::with_name("5squared pixel"))
        .antialiasing(true)
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
    /// play the sequencer
    PlaySequencer,
    /// resets the sequencer
    ResetSequencer,
    /// pauses the sequencer
    PauseSequencer,
    /// set clip to selected
    SelectClip {
        /// id of the recently selected clip
        clip_id: ClipId,
    },
}

#[derive(Debug)]
struct Hexencer {
    /// the theme for the application
    theme: Theme,
    /// the storage interface for the application
    storage: StorageInterface,
    /// sequencer
    sequencer_handle: SequencerHandle,
    /// a clip that was dropped
    dropped_clip: Option<ClipId>, // TODO #53 move this elsewhere
    /// the origin of the drag for the clip that was dropped
    drag_origin: f32,
    /// state used for drawing a canvas, used for the transport line drawing
    line_state: LineState,
    /// selected clip
    selected_clip: Option<ClipId>,
    /// available notes
    notes: Vec<String>,
}

/// state type used for canvas drawing of the transport line
#[derive(Debug)]
struct LineState {
    /// unused clock taken from example
    now: Instant,
    /// cache which stores the canvas drawing elements
    system_cache: canvas::Cache,
    /// the current tick for the sequencer
    tick: f64,
}
impl LineState {
    /// create a new state
    fn new() -> LineState {
        let now = Instant::now();
        Self {
            now,
            system_cache: canvas::Cache::default(),
            tick: 0.0,
        }
    }

    /// update the canvas state
    pub fn update2(&mut self, now: Instant, tick: f64) {
        self.now = now;
        self.system_cache.clear();
        self.tick = tick;
    }
}

impl<Message> canvas::Program<Message> for LineState {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let start_point = Point::new(self.tick as f32 * 240.0, 0.0);
        let line_length = 500.0;
        let target = Point::new(start_point.x, start_point.y + line_length);
        let line_cache = self.system_cache.draw(renderer, bounds.size(), |frame| {
            let line = Path::line(start_point, target);
            frame.stroke(
                &line,
                Stroke {
                    style: stroke::Style::Solid(Color::from_rgba8(200, 240, 255, 0.5)),
                    width: 1.0,
                    ..Stroke::default()
                },
            );
        });

        vec![line_cache]
    }
}

impl Default for Hexencer {
    fn default() -> Self {
        let storage = StorageInterface::new();
        let midi_sender = midi_engine::start_midi_engine();
        let (sequencer_sender, sequencer_receiver) = tokio::sync::mpsc::unbounded_channel();
        let sequencer = Sequencer::new(storage.clone(), midi_sender, sequencer_receiver);

        let sequencer_handle = SequencerHandle {
            state: Arc::clone(&sequencer.state),
            command_sender: sequencer_sender,
        };

        tokio::spawn(sequencer.run());

        let note_steps = [
            "c", "c#", "d", "d#", "e", "f", "f#", "g", "g#", "a", "a#", "b",
        ];

        let mut notes = Vec::new();
        for index in 0..120 {
            let step = index % note_steps.len();
            let note_num = index / note_steps.len();
            info!("note: {}{note_num}", note_steps[step]);
            let note = format!("{}{}", note_steps[step], note_num);
            notes.push(note);
        }

        info!("notes added {}", notes.len());

        Self {
            theme: Theme::KanagawaDragon,
            storage,
            dropped_clip: None,
            drag_origin: 0.0,
            line_state: LineState::new(),
            sequencer_handle,
            selected_clip: None,
            notes,
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
                let tick = self
                    .sequencer_handle
                    .state
                    .read()
                    .unwrap()
                    .current_tick
                    .as_f64()
                    / 480.0;
                self.line_state.update2(instant, tick);
            }
            Message::PlaySequencer => {
                self.sequencer_handle
                    .command_sender
                    .send(SequencerCommand::Play)
                    .expect("unable to send sequencer command, perhaps the channel was dropped?");
                info!("play sequencer");
            }
            Message::ResetSequencer => {
                self.sequencer_handle
                    .command_sender
                    .send(SequencerCommand::Reset)
                    .expect("unable to send sequencer command, perhaps the channel was dropped?");
                info!("play sequencer");
            }
            Message::PauseSequencer => {
                self.sequencer_handle
                    .command_sender
                    .send(SequencerCommand::Pause)
                    .expect("unable to send command");
                info!("pause sequencer command sent");
            }
            Message::SelectClip { clip_id } => {
                println!("test");
                info!("selected clip {}", clip_id);
                self.selected_clip = Some(clip_id);
            }
        }
    }

    /// remove a clip from the storage
    pub fn _remove_clip(&mut self, clip_id: ClipId) {
        let mut to_remove = None;

        let mut data = self.storage.write().unwrap();
        let track_collection = &mut data.project_manager.track_collection;
        for track in track_collection.tracks() {
            for (clip_key, clip) in track.clip_collection.iter() {
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
                track.clip_collection.remove(&clip_key).unwrap();
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

        let bottom = status_bar(self.storage.clone(), &self.sequencer_handle);

        let elements = self.create_track_elements();

        let line_canvas = canvas(&self.line_state)
            .width(Length::Fill)
            .height(Length::Fixed(500.0));
        let tracks_column = column(elements).spacing(1);

        let _scroll_properties = Properties::default();

        let arranger_background = container(
            Arranger::new(
                column!["Track list", tracks_column, wgpu_box]
                    .spacing(40)
                    .align_items(Alignment::Center)
                    .width(Length::Fixed(5000.0))
                    .height(Length::Fill),
            )
            .height(Length::Fill),
        )
        .padding(10);

        let arranger = stack![arranger_background, line_canvas];

        let editor = self.create_editor();

        let content = column![header, arranger, editor, bottom];
        center(content).into()
    }

    /// get the theme for the application
    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    /// get the subscription for the application
    fn subscription(&self) -> Subscription<Message> {
        window::frames().map(Message::Tick)
    }

    /// New method to create track elements
    // TODO make this faster, now causing visual glitches
    fn create_track_elements(&self) -> Vec<Element<Message>> {
        let storage = self.storage.read().unwrap();
        let track_collection = &storage.project_manager.track_collection;

        track_collection
            .iter()
            .enumerate()
            .map(|(index, track)| {
                self.create_track_element(index, track, self.storage.clone(), self.dropped_clip)
            })
            .collect()
    }

    /// create a new track widget element
    fn create_track_element(
        &self,
        index: usize,
        track: &hexencer_core::data::Track,
        storage: StorageInterface,
        dropped_clip: Option<ClipId>,
    ) -> Element<Message> {
        let children = self.create_clip_elements(index);
        Track::new(storage, index, track.id, children, dropped_clip)
            .on_drop(move |clip_id, track_id, cursor_position| {
                info!("dropped clip: {:?} onto track {}", clip_id, index);
                Message::MoveClipRequest {
                    clip_id,
                    track_id,
                    cursor_position,
                }
            })
            .into()
    }

    /// create new clip widget element
    fn create_clip_elements(&self, track_index: usize) -> Vec<Element<Message>> {
        let storage = self.storage.read().unwrap();
        let clip_collection = &storage
            .project_manager
            .track_collection
            .get(track_index)
            .unwrap()
            .clip_collection;

        clip_collection
            .iter()
            .map(|(key, clip)| {
                let id = clip.id;
                let selected = self.selected_clip.is_some() && self.selected_clip.unwrap() == id;
                Clip::new(id, selected, self.storage.clone(), text("clip"))
                    .on_drag(|drag_event| {
                        let (origin, clip_id) = match drag_event {
                            DragEvent::DragStarted {
                                grab_position,
                                clip_id,
                            } => (grab_position, clip_id),
                            _ => panic!("invalid drag event"),
                        };
                        Message::DragClip { clip_id, origin }
                    })
                    .on_drop(|clip_id| Message::DroppedClip { clip_id })
                    .on_selected(|clip_id| Message::SelectClip { clip_id })
                    .into()
            })
            .collect()
    }

    /// create the editor ui section
    fn create_editor(&self) -> Element<Message> {
        let header_string = match self.selected_clip {
            Some(id) => format!("editing clip {}", id),
            None => "nothing selected".to_string(),
        };

        let header = text(header_string);

        let height = match self.selected_clip {
            Some(_) => 250.0,
            None => 50.0,
        };

        let track_collection = &self
            .storage
            .read()
            .unwrap()
            .project_manager
            .track_collection;

        let mut label = "null".to_string();

        if let Some(id) = self.selected_clip {
            let mut clip = None;
            for track in track_collection.iter() {
                if let Some(track_clip) = track.clip_collection.find(id) {
                    clip = Some(track_clip)
                };
            }

            if let Some(clip) = clip {
                label = clip.start.to_string();
            }
        }

        let notes = self.draw_notes(label);
        let content = column![header, notes];
        let editor = EventEditor::new(content, self.storage.clone());

        editor.into()
    }

    /// creates the notes for the note editor
    fn draw_notes(&self, label: String) -> Element<Message> {
        // draw lanes for every note
        let mut note_lanes = Vec::new();

        for note in &self.notes {
            // info!("note lane for note {}", note);
            let note_lane = text(note.to_string()).size(10.0).into();
            note_lanes.push(note_lane);
        }

        column(note_lanes)
            .spacing(1)
            .align_items(Alignment::Start)
            .width(Length::Fixed(2000.0))
            .height(Length::Fixed(2000.0))
            .into()
    }
}

/// create the status bar ui
fn status_bar(storage: StorageInterface, sequencer: &SequencerHandle) -> Element<'_, Message> {
    let play_button = button("play").on_press(Message::PlaySequencer);
    let pause_button = button("pause").on_press(Message::PauseSequencer);
    let reset_button = button("reset").on_press(Message::ResetSequencer);

    let state = sequencer.state.read().unwrap();
    let test = state.current_tick;
    let current_tick = test;
    let tick_widget = text(current_tick.to_string());

    let bpm = storage.read().unwrap().bpm();
    let bpm_widget = text(bpm.to_string()).size(60);

    let bottom = container(
        row![
            play_button,
            pause_button,
            reset_button,
            horizontal_space(),
            bpm_widget,
            horizontal_space(),
            tick_widget,
        ]
        .padding(10)
        .align_items(Alignment::Center),
    );
    bottom.into()
}
