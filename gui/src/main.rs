use eframe::wgpu::core::device::DeviceLostInvocation;
use egui::Vec2;
use hexencer_core::{MidiEvent, MidiMessage, ProjectManager};
use hexencer_engine::Sequencer;

fn main() {
    println!("Hello, world!");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Hexencer",
        options,
        Box::new(|cc| Box::new(Hexencer::new(cc))),
    )
    .expect("failed to start eframe app");
}

#[derive(Default)]
struct Hexencer {
    sequencer: Sequencer,
    sequencer_sender: Option<tokio::sync::mpsc::UnboundedReceiver<MidiMessage>>,
}

impl Hexencer {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

const TRACK_HEIGHT: f32 = 25.0;
const TRACK_HEADER_WIDTH: f32 = 100.0;

impl eframe::App for Hexencer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.label("some toolbar");
        });
        egui::SidePanel::left("info").show(ctx, |ui| {
            ui.label("info");
            if ui.button("add track").clicked() {
                self.sequencer.project_manager.add_track();
            }
            if ui.button("play").clicked() {
                println!("play");
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                let track_ids: Vec<usize> = self
                    .sequencer
                    .project_manager
                    .track_manager
                    .tracks
                    .iter()
                    .map(|track| track.id)
                    .collect();

                for id in track_ids {
                    new_track(self, ctx, id, ui);
                }
            });
        });
    }
}

fn new_track(app: &mut Hexencer, ctx: &egui::Context, index: usize, ui: &mut egui::Ui) {
    egui::Frame::none().fill(egui::Color32::RED).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.set_min_size(egui::vec2(ui.available_width(), TRACK_HEIGHT));
            egui::Frame::none()
                .fill(egui::Color32::BLUE)
                .show(ui, |ui| {
                    ui.set_min_width(TRACK_HEADER_WIDTH);
                    ui.label(format!("Track {}", index));
                });
            ui.horizontal(|ui| {
                // let events: Vec<bool> = app
                //     .project_manager
                //     .track_manager
                //     .tracks
                //     .iter()
                //     .flat_map(|track| track.events.iter().map(|events| return events.on))
                //     .collect();

                for event in &mut app.sequencer.project_manager.track_manager.tracks[index].events {
                    ui.checkbox(&mut event.on, "Beat");
                }
            });
        });
    });
}

fn clip(ctx: &egui::Context, ui: &mut egui::Ui) {
    let id = egui::Id::from("new clip");
    // ui.button("Clip");
    egui::Area::new(id)
        .movable(true)
        .constrain_to(ui.max_rect())
        .show(ctx, |ui| {
            egui::Frame::none().fill(egui::Color32::RED).show(ui, |ui| {
                ui.allocate_space(egui::vec2(10.0, ui.available_height() - 15.0));
                //ui.add(egui::Label::new("Clip").selectable(false));
            });
        });
}
