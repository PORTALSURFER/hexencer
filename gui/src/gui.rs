/// the main viewport
mod core;

use eframe::NativeOptions;
use egui::Color32;
use hexencer_core::data::DataInterface;
use hexencer_engine::SequencerSender;

pub use self::core::HexencerApp;
pub use self::core::HexencerContext;
pub use self::core::SystemCommand;

/// color used for all regular edges in the ui
pub const EDGE_COLOR: Color32 = Color32::from_rgb(20, 20, 20);

/// run the gui
pub fn run(options: NativeOptions, data_layer: DataInterface, sequencer_sender: SequencerSender) {
    eframe::run_native(
        "Hexencer",
        options,
        Box::new(|cc| {
            Box::new(HexencerApp::new(
                cc,
                data_layer,
                sequencer_sender,
                cc.egui_ctx.clone(),
            ))
        }),
    )
    .expect("failed to start eframe app");
}

/// creates options for the application
pub fn options() -> NativeOptions {
    NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // .with_icon(icon)
            .with_inner_size(egui::vec2(800.0, 600.0)),

        ..Default::default()
    }
}
