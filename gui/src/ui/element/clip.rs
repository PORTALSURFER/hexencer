use egui::{Context, Margin, Ui};

pub fn clip(ctx: &Context, ui: &mut Ui, track_name: &str) {
    let id = egui::Id::from(format!("{} clip", track_name));
    let area = egui::Area::new(id)
        .movable(true)
        .constrain_to(ui.max_rect());
    area.show(ctx, |ui| {
        let mut frame = egui::Frame::none().fill(egui::Color32::RED);
        frame.outer_margin = Margin::ZERO;
        frame.inner_margin = Margin::ZERO;
        frame.show(ui, |ui| {
            ui.allocate_space(egui::vec2(50.0, 20.0));
            //ui.add(egui::Label::new("Clip").selectable(false));
        });
    });
}
