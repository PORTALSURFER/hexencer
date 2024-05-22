fn main() {
    println!("Hello, world!");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    let mut name = "Arthur".to_owned();
    let mut age = 42;

    eframe::run_native(
        "Hexencer",
        options,
        Box::new(|cc| Box::new(Hexencer::new(cc))),
    );
}

#[derive(Default)]
struct Hexencer {}

impl Hexencer {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Hexencer {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.label("some toolbar");
        });
        egui::SidePanel::left("info").show(ctx, |ui| {
            ui.label("info");
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("sequencer");
                })
            });
        });
    }
}
