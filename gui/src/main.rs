use egui::Vec2;
use hexencer_core::ProjectManager;

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
    project_manager: ProjectManager,
}

impl Hexencer {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

const TRACK_HEIGHT: f32 = 25.0;

impl eframe::App for Hexencer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.label("some toolbar");
        });
        egui::SidePanel::left("info").show(ctx, |ui| {
            ui.label("info");
            if ui.button("add track").clicked() {
                self.project_manager.add_track();
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                let track_count = self.project_manager.track_count();
                for i in 0..=track_count {
                    track(self, ctx, i, ui);
                }
            });
        });
    }
}

fn track(app: &mut Hexencer, ctx: &egui::Context, index: usize, ui: &mut egui::Ui) {
    egui::Frame::none().fill(egui::Color32::RED).show(ui, |ui| {
        ui.horizontal( |ui| {
                    ui.set_min_size(egui::vec2(ui.available_width(), TRACK_HEIGHT));
            egui::Frame::none()
                .fill(egui::Color32::BLUE)
                .show(ui, |ui| {
                    ui.set_min_width(100.0);
                     ui.label(format!("Track {}", index));
                });
            ui.horizontal(|ui| {
                ui.button("Add Clip")
                    .on_hover_text("Add a new clip to the track");
                ui.button("Remove Track").on_hover_text("Remove the track");
            });
        });;
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
