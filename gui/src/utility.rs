use egui::IconData;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

/// load icon from png image
pub fn load_icon(path: &str) -> IconData {
    let image = image::open(path).expect("failed to open icon image");
    if let Some(image) = image.as_rgba8() {
        let (width, height) = image.dimensions();
        let rgba = image.clone().into_vec();
        IconData {
            rgba,
            width,
            height,
        }
    } else {
        IconData::default()
    }
}

/// initialize the logging system
pub fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("hexencer started");
}
