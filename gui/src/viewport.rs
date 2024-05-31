use std::sync::{Arc, Mutex};

use egui::{
    epaint, vec2, Color32, FontId, Frame, Id, LayerId, Margin, Order, Pos2, Stroke, Ui, Vec2,
};
use hexencer_core::{data::DataLayer, DataId};
use hexencer_engine::{SequencerCommand, SequencerSender};

use crate::{
    arranger::{track, SELECTED_CLIP},
    ui::{
        common::{TRACK_HEADER_COLOR, TRACK_HEIGHT},
        NoteEditor, Timeline, BEAT_WIDTH,
    },
};

#[derive(Default)]
pub struct MainViewport {
    data_layer: Arc<std::sync::Mutex<DataLayer>>,
    sequencer_sender: Option<SequencerSender>,
}

impl MainViewport {
    pub fn new(data_layer: Arc<Mutex<DataLayer>>, sender: SequencerSender) -> Self {
        Self {
            data_layer,
            sequencer_sender: Some(sender),
        }
    }

    fn track_header_list(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            let track_ids: Vec<usize> = self
                .data_layer
                .lock()
                .unwrap()
                .project_manager
                .tracks
                .iter()
                .map(|track| track.id)
                .collect();

            for id in track_ids {
                track_header(ui, id);
            }
        });
    }

    fn track_manager_controls(&mut self, ui: &mut Ui) {
        if ui.button("add track").clicked() {
            self.data_layer.lock().unwrap().project_manager.add_track();
        }
        if ui.button("remove track").clicked() {
            self.data_layer
                .lock()
                .unwrap()
                .project_manager
                .remove_track();
        }
    }

    fn sequencer_controls(&mut self, ui: &mut Ui) {
        if ui.button("play").clicked() {
            self.sequencer_sender.as_mut().map(|sender| {
                let _ = sender.send(SequencerCommand::Play);
            });
        }
        if ui.button("stop").clicked() {
            self.sequencer_sender.as_mut().map(|sender| {
                let _ = sender.send(SequencerCommand::Stop);
            });
        }
        if ui.button("reset").clicked() {
            self.sequencer_sender.as_mut().map(|sender| {
                let _ = sender.send(SequencerCommand::Reset);
            });
        }
    }

    fn editor_ui(&mut self, ui: &mut Ui) {
        if let Some(selected_clip_id) =
            ui.memory(|mem| mem.data.get_temp::<DataId>(SELECTED_CLIP.into()))
        {
            if let Some(selected_clip) = self
                .data_layer
                .lock()
                .unwrap()
                .project_manager
                .find_clip(selected_clip_id)
            {
                NoteEditor::new(selected_clip).show(ui);
            }
        };
    }
}

impl eframe::App for MainViewport {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.style_mut(|style| {
            style.spacing.item_spacing = vec2(0.0, 0.0);
            style.spacing.window_margin = Margin::ZERO;
            style.spacing.indent = 0.0
        });

        // let gui_state = GuiState::default();
        // ctx.memory_mut(|memory| memory.data.insert_temp(Id::NULL, gui_state));

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.centered_and_justified(|ui| ui.label("toolbar menu"));
        });
        egui::TopBottomPanel::bottom("statusbar").show(ctx, |ui| {
            let current_tick = self.data_layer.lock().unwrap().get_tick();
            ui.label(&format!("{}", &current_tick.as_time()));
        });

        egui::SidePanel::left("tracks")
            .resizable(false)
            .show(ctx, |ui| {
                self.track_header_list(ui);
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
                        String::from({
                            let available_width = ui.available_width();
                            let size_of_beat = available_width / BEAT_WIDTH;

                            let ui_position = ui.max_rect().min;
                            let ui_size = ui.max_rect().size();

                            let ui_mouse_pos =
                                (ctx_pos.x - ui_position.x, ctx_pos.y - ui_position.y);

                            let test = available_width / size_of_beat;

                            let output_text = if ui_mouse_pos.0 >= 0.0
                                && ui_mouse_pos.0 <= ui_size.x
                                && ui_mouse_pos.1 >= 0.0
                                && ui_mouse_pos.1 <= ui_size.y
                            {
                                format!("{:?} - {:?}", ui_mouse_pos.0, ui_mouse_pos.0 / test)
                            } else {
                                String::from("out of bounds")
                            };

                            output_text
                        }),
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
            Timeline::new(10.0).show(ui);
            ui.vertical(|ui| {
                let track_ids: Vec<usize> = self
                    .data_layer
                    .lock()
                    .unwrap()
                    .project_manager
                    .tracks
                    .iter()
                    .map(|track| track.id)
                    .collect();

                for id in track_ids {
                    let clone = Arc::clone(&self.data_layer);
                    track(clone, ctx, id, ui);
                }
            });
        });

        // Request a new frame
        ctx.request_repaint();
    }
}

fn track_header(ui: &mut Ui, id: usize) {
    let mut frame = egui::Frame::none().fill(TRACK_HEADER_COLOR);
    frame.inner_margin = Margin::ZERO;
    frame.outer_margin = Margin::ZERO;
    frame.show(ui, |ui| {
        let label = egui::Label::new(format!("Track {}", id));
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

fn port_selector(
    ui: &mut Ui,
    text_input_rect: Vec2,
    mut port: String,
    data_layer: &Arc<Mutex<DataLayer>>,
    index: usize,
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

fn channel_selector(
    data_layer: Arc<Mutex<DataLayer>>,
    index: usize,
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
