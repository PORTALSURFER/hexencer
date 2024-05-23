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

struct TrackManager {
    tracks: Vec<Track>,
}

struct InstrumentManager {
    struments: Vec<Instrument>,
}
struct ProjectManager {
    track_manager: TrackManager,
    instrument_manager: InstrumentManager,
}
#[derive(Default)]
struct Hexencer {}

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
        });
        egui::CentralPanel::default().show(ctx, |_ui| {
            egui::CentralPanel::default().show(ctx, |_ui| {
                for i in 0..10 {
                    track(ctx, i);
                }
                egui::CentralPanel::default().show(ctx, |_ui| {});
            });
        });
    }
}

fn track(ctx: &egui::Context, index: u32) {
    egui::TopBottomPanel::top(format!("tracklane_{}", index))
        .min_height(TRACK_HEIGHT)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(format!("track {}", index)).selectable(false));
                ui.button("X");
            });
        });
}
