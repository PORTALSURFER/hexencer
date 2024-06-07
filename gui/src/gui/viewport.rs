use crate::{
    arranger::track,
    memory::GuiState,
    ui::{
        common::{TRACK_HEADER_COLOR, TRACK_HEADER_WIDTH, TRACK_HEIGHT},
        NoteEditorWidget, TimelineWidget, BEAT_WIDTH,
    },
};
use egui::{
    epaint, vec2, Color32, FontId, Frame, Id, LayerId, Margin, Order, Pos2, Stroke, Ui, Vec2,
};
use hexencer_core::{
    data::{DataInterface, DataLayer},
    TrackId,
};
use hexencer_engine::{SequencerCommand, SequencerSender};
use std::{
    ops::Deref,
    sync::{Arc, Mutex, MutexGuard},
};

/// main hexencer viewport/ui
#[derive(Default)]
pub struct HexencerGui {
    /// a reference to the data layer, this the the main way to interact with the data
    data: DataInterface,
    /// use this to send commands to the sequencer
    sequencer_sender: Option<SequencerSender>,
}

impl HexencerGui {
    /// create a new instance of the hexencer gui, the main gui
    pub fn new(data_layer: DataInterface, sender: SequencerSender) -> Self {
        Self {
            data: data_layer,
            sequencer_sender: Some(sender),
        }
    }

    /// builds the track manager control ui
    fn track_manager_controls(&mut self, ui: &mut Ui) {
        if ui.button("add track").clicked() {
            self.data.get().project_manager.push_track();
        }
        if ui.button("remove track").clicked() {
            self.data.get().project_manager.remove_track();
        }
    }

    /// builds the sequencer control ui
    fn sequencer_controls(&mut self, ui: &mut Ui) {
        if ui.button("play").clicked() {
            if let Some(sender) = self.sequencer_sender.as_mut() {
                let _ = sender.send(SequencerCommand::Play);
            }
        }
        if ui.button("stop").clicked() {
            if let Some(sender) = self.sequencer_sender.as_mut() {
                let _ = sender.send(SequencerCommand::Stop);
            }
        }
        if ui.button("reset").clicked() {
            if let Some(sender) = self.sequencer_sender.as_mut() {
                let _ = sender.send(SequencerCommand::Reset);
            }
        }
    }

    /// builds the note editor ui
    fn editor_ui(&mut self, ui: &mut Ui) {
        let state = GuiState::load(ui);
        if let Some(selected_clip_id) = state.selected_clip {
            if let Some(selected_clip) =
                self.data.get().project_manager.find_clip(&selected_clip_id)
            {
                NoteEditorWidget::new(selected_clip).show(ui);
            }
        };
    }
}

impl eframe::App for HexencerGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.style_mut(|style| {
            style.spacing.item_spacing = vec2(0.0, 0.0);
            style.spacing.window_margin = Margin::ZERO;
            style.spacing.indent = 0.0
        });

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.centered_and_justified(|ui| ui.label("toolbar menu"));
        });
        egui::TopBottomPanel::bottom("statusbar").show(ctx, |ui| {
            let current_tick = self.data.get().get_tick();
            ui.label(&current_tick.as_time().to_string());
        });

        egui::SidePanel::left("tracks")
            .exact_width(TRACK_HEADER_WIDTH)
            .resizable(false)
            .show(ctx, |ui| {
                let track_ids: Vec<TrackId> = self
                    .data
                    .get()
                    .project_manager
                    .tracks
                    .iter()
                    .map(|track| track.id)
                    .collect();

                track_header_list(ui, track_ids);
            });

        egui::SidePanel::right("info")
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("info");
                self.track_manager_controls(ui);
                ui.add_space(20.0);
                self.sequencer_controls(ui);
            });

        let editor_height = 200.0;
        let mut editor_frame = Frame::none();
        editor_frame.outer_margin = Margin::ZERO;
        editor_frame.inner_margin = Margin::ZERO;
        let fill = egui::Color32::from_rgb(20, 20, 20);
        editor_frame = editor_frame.fill(fill);
        egui::TopBottomPanel::bottom("editor")
            .frame(editor_frame)
            .min_height(editor_height)
            .default_height(editor_height)
            .show(ctx, |ui| {
                self.editor_ui(ui);
            });

        let mut frame = Frame::none();
        frame.outer_margin = Margin::ZERO;
        frame.inner_margin = Margin::ZERO;
        let fill = egui::Color32::from_rgb(20, 20, 20);
        frame = frame.fill(fill);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            {
                let layer_id = LayerId::new(Order::Foreground, "debug_cursor_time".into());
                let id = Id::new("debug__cursor_time");
                let rect = ctx.screen_rect();
                let debug_cursor_time = Ui::new(ctx.clone(), layer_id, id, rect, rect);
                let ctx_pos = match ctx.input(|i| i.pointer.latest_pos()) {
                    Some(mouse_pos) => {
                        let mut pos = mouse_pos;
                        pos.y += 20.0;
                        pos
                    }
                    _ => Pos2::new(0.0, 0.0),
                };
                let font_id = FontId::monospace(12.0);
                let text_color = Color32::RED;
                let galley = ctx.fonts(|f| {
                    f.layout(
                        {
                            let available_width = ui.available_width();
                            let size_of_beat = available_width / BEAT_WIDTH;
                            let ui_position = ui.max_rect().min;
                            let ui_size = ui.max_rect().size();
                            let ui_mouse_pos =
                                (ctx_pos.x - ui_position.x, ctx_pos.y - ui_position.y);
                            let test = available_width / size_of_beat;
                            if ui_mouse_pos.0 >= 0.0
                                && ui_mouse_pos.0 <= ui_size.x
                                && ui_mouse_pos.1 >= 0.0
                                && ui_mouse_pos.1 <= ui_size.y
                            {
                                format!("{:?} - {:?}", ui_mouse_pos.0, ui_mouse_pos.0 / test)
                            } else {
                                String::from("out of bounds")
                            }
                        },
                        font_id,
                        text_color,
                        10000.0,
                    )
                });
                let underline = Stroke::NONE;
                let fallback_color = Color32::BLUE;
                debug_cursor_time.painter().add(epaint::TextShape {
                    pos: ctx_pos,
                    galley,
                    underline,
                    fallback_color,
                    override_text_color: None,
                    opacity_factor: 1.0,
                    angle: 0.0,
                });
            }
            TimelineWidget::new(10.0).show(ui);
            ui.vertical(|ui| {
                let track_ids: Vec<TrackId> = self
                    .data
                    .get()
                    .project_manager
                    .tracks
                    .iter()
                    .map(|track| track.id)
                    .collect();

                for id in track_ids {
                    track(self.data.clone(), ctx, id, ui);
                }
            });
        });

        // Request a new frame
        ctx.request_repaint();
    }
}

/// builds gui for the track header
fn track_header(ui: &mut Ui, id: TrackId) {
    let mut frame = egui::Frame::none().fill(TRACK_HEADER_COLOR);
    frame.inner_margin = Margin::ZERO;
    frame.outer_margin = Margin::ZERO;
    frame.show(ui, |ui| {
        let label = egui::Label::new(format!("Track {:?}", id));
        ui.add_sized(vec2(BEAT_WIDTH, TRACK_HEIGHT), label);

        // let port = self
        //     .data_layer
        //     .lock()
        //     .unwrap()
        //     .project_manager
        //     .tracks
        //     .get_track_port(id)
        //     .to_string();
        // // ui.add_space(16.0);
        // let text_input_rect = egui::vec2(8.0, 24.0);
        // let clone = Arc::clone(&self.data_layer);
        // port_selector(ui, text_input_rect, port, &self.data_layer, id);
        // channel_selector(clone, id, ui, text_input_rect);
    });
}

/// builds the port selector ui widget
fn port_selector(
    ui: &mut Ui,
    text_input_rect: Vec2,
    mut port: String,
    data_layer: &Arc<Mutex<DataLayer>>,
    index: TrackId,
) {
    let port_selector = ui.add_sized(text_input_rect, egui::TextEdit::singleline(&mut port));
    if port_selector.lost_focus() || port_selector.changed() {
        match port.parse::<u8>() {
            Ok(value) => {
                data_layer
                    .lock()
                    .unwrap()
                    .project_manager
                    .tracks
                    .get_mut(index)
                    .unwrap()
                    .set_port(value);
            }
            Err(_) => tracing::warn!("port must be a number"),
        }
    }
}

/// builds the channel selector ui widget
fn channel_selector(
    data_layer: Arc<Mutex<DataLayer>>,
    index: TrackId,
    ui: &mut Ui,
    text_input_rect: Vec2,
) {
    let mut channel = data_layer
        .lock()
        .unwrap()
        .project_manager
        .tracks
        .get(index)
        .unwrap()
        .instrument
        .channel
        .to_string();

    let channel_selector = egui::TextEdit::singleline(&mut channel);
    let response = ui.add_sized(text_input_rect, channel_selector);
    if response.lost_focus() || response.changed() {
        match channel.parse::<u8>() {
            Ok(value) => {
                data_layer
                    .lock()
                    .unwrap()
                    .project_manager
                    .tracks
                    .get_mut(index)
                    .unwrap()
                    .set_channel(value);
            }
            Err(_) => tracing::warn!("port must be a number"),
        }
    }
}

/// builds the track headers list
fn track_header_list(ui: &mut Ui, track_ids: Vec<TrackId>) {
    ui.vertical(|ui| {
        for id in track_ids {
            track_header(ui, id);
        }
    });
}