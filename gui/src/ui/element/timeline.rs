use egui::{epaint, pos2, vec2, Color32, Rect, Response, Rounding, Sense, Shape, Stroke, Ui};

use crate::ui::BEAT_WIDTH;

/// visually shows the tick/time
pub struct TimelineWidget {
    /// height of the timeline widget
    height: f32,
}

impl TimelineWidget {
    /// create a new timeline with the given height
    pub fn new(height: f32) -> Self {
        Self { height }
    }

    /// renders the 'Timeline' and returns the 'Response'
    pub fn show(&self, ui: &mut Ui) -> Response {
        let rect = Rect::from_min_size(ui.max_rect().min, vec2(ui.available_width(), self.height));
        self.paint(ui, rect);
        ui.allocate_rect(rect, Sense::hover())
    }

    /// paints the timeline widget elements
    fn paint(&self, ui: &mut Ui, rect: Rect) {
        ui.set_clip_rect(ui.min_rect());
        let fill = Color32::from_rgb(40, 40, 40);
        let rect_shape = Shape::Rect(epaint::RectShape::new(
            rect,
            Rounding::ZERO,
            fill,
            Stroke::NONE,
        ));
        ui.painter().add(rect_shape);

        let mut shapes = Vec::new();
        for i in 0..100 {
            let fill = Color32::from_rgb(60, 69, 69);
            let rect2 = Rect::from_min_size(rect.min, vec2(BEAT_WIDTH, self.height));
            let rect2 = rect2.translate(vec2(i as f32 * BEAT_WIDTH, 0.0));

            let rect_shape = Shape::Rect(epaint::RectShape::new(
                rect2,
                Rounding::ZERO,
                fill,
                Stroke::new(1.0, Color32::from_rgb(100, 100, 100)),
            ));
            shapes.push(rect_shape);

            let line = Shape::line_segment(
                [
                    rect2.min,
                    pos2(rect2.min.x, rect2.min.y + ui.available_height()),
                ],
                Stroke::new(1.0, Color32::from_rgb(100, 100, 100)),
            );
            shapes.push(line);
        }
        ui.painter().extend(shapes);
    }
}
