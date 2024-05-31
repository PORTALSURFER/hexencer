use super::point::EditorPoint;

use egui::Vec2;

use egui::pos2;

use egui::Rect;

#[derive(Clone, Copy)]
pub(crate) struct EditorBounds {
    pub min: [f64; 2],
    pub max: [f64; 2],
}

impl EditorBounds {
    pub(crate) fn to_rect(&self) -> Rect {
        Rect::from_min_max(
            pos2(self.min[0] as f32, self.min[1] as f32),
            pos2(self.max[0] as f32, self.max[1] as f32),
        )
    }

    pub const NONE: Self = Self {
        min: [f64::INFINITY; 2],
        max: [f64::INFINITY; 2],
    };

    pub(crate) fn box_zoom(&mut self, zoom_factor: Vec2, center: EditorPoint) {
        self.min[0] = center.x + (self.min[0] - center.x) / (zoom_factor.x as f64);
        self.max[0] = center.x + (self.max[0] - center.x) / (zoom_factor.x as f64);
        self.min[1] = center.y + (self.min[1] - center.y) / (zoom_factor.y as f64);
        self.max[1] = center.y + (self.max[1] - center.y) / (zoom_factor.y as f64);
    }

    pub fn is_finite(&self) -> bool {
        self.min[0].is_finite()
            && self.min[1].is_finite()
            && self.max[0].is_finite()
            && self.max[1].is_finite()
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.is_finite() && self.width() > 0.0 && self.height() > 0.0
    }

    pub(crate) fn width(&self) -> f64 {
        self.max[0] - self.min[0]
    }

    pub(crate) fn height(&self) -> f64 {
        self.max[1] - self.min[1]
    }
}
