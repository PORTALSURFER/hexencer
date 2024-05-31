use egui::Rect;
use egui::{pos2, Color32, Pos2, Ui, Vec2};

use super::{debug_dot, debug_line};

/// transform used for zooming the editor
#[derive(Clone, Copy)]
pub struct Transform {
    /// current scale/zoom of the editor
    pub(crate) scale: f32,
    /// rect/frame of this editor
    pub(crate) frame: Rect,
    /// current translation, used for panning, and used for centering the zoom onto the mouse position
    pub(crate) translation: Vec2,
}

impl Transform {
    /// process zooming of the editor
    pub fn zoom(
        &mut self,
        zoom_delta: Vec2,
        screen_hover_pos: Pos2,
        ui: &mut Ui,
        rect: Rect,
        step_size: f32,
    ) {
        let min_height = rect.min.y;
        let max_height = rect.max.y;

        let zoom_rate = 0.002;
        let zoom_delta_scaled_by_rate = zoom_delta * zoom_rate;
        let clamped_scale = (self.scale + zoom_delta_scaled_by_rate.x).clamp(0.8, 4.0);

        let pointer_in_editor_space = (screen_hover_pos - self.translation) / self.scale;
        self.scale = clamped_scale;

        let mut new_translation = screen_hover_pos - (pointer_in_editor_space * self.scale);
        new_translation.y += zoom_delta.y * self.scale;

        let total_height = step_size * 128.0;
        if new_translation.y >= min_height {
            new_translation.y = min_height;
        } else if new_translation.y + (total_height * self.scale) < max_height {
            new_translation.y = max_height - (total_height * self.scale);
        }
        self.translation = new_translation;

        if cfg!(feature = "debug") {
            self.debug_lines(
                pointer_in_editor_space,
                new_translation,
                ui,
                screen_hover_pos,
                total_height,
            );
        }
    }

    /// draw debug lines, used for zooming code
    fn debug_lines(
        &mut self,
        pointer_in_editor_space: Pos2,
        new_translation: Vec2,
        ui: &mut Ui,
        screen_hover_pos: Pos2,
        total_height: f32,
    ) {
        let pointer_origin_scaled = pos2(125.0, 0.0);
        let pointer_translation_scaled =
            pos2(125.0, new_translation.y + (total_height * self.scale));
        debug_line(
            ui,
            pointer_origin_scaled,
            pointer_translation_scaled,
            Color32::LIGHT_YELLOW,
        );
        let pointer_in_editor_space_clamped = pos2(1.0, pointer_in_editor_space.y);
        let scaled = pointer_in_editor_space * self.scale;
        let scaled_clamped = pos2(1.0, scaled.y);
        let new_center_clamped = pos2(10.0, new_translation.y);
        debug_dot(ui, pointer_in_editor_space_clamped, Color32::RED);
        debug_dot(ui, scaled_clamped, Color32::LIGHT_BLUE);
        debug_dot(ui, new_center_clamped, Color32::GREEN);
        let editor_origin = pos2(150.0, 0.0);
        let editor_translation = pos2(150.0, self.translation.y);
        debug_line(ui, editor_origin, editor_translation, Color32::RED);
        let pointer_origin_before_scaling = pos2(200.0, 0.0);
        let pointer_translation_before_scaling = pos2(200.0, pointer_in_editor_space.y);
        debug_line(
            ui,
            pointer_origin_before_scaling,
            pointer_translation_before_scaling,
            Color32::GREEN,
        );
        let pointer_origin_scaled = pos2(125.0, 0.0);
        let pointer_translation_scaled = pos2(125.0, pointer_in_editor_space.y * self.scale);
        debug_line(
            ui,
            pointer_origin_scaled,
            pointer_translation_scaled,
            Color32::LIGHT_YELLOW,
        );
        let transform_offset_origin = pos2(175.0, self.translation.y);
        let transform_offset = pos2(
            175.0,
            screen_hover_pos.y + (pointer_in_editor_space.y * self.scale),
        );
        debug_line(
            ui,
            transform_offset_origin,
            transform_offset,
            Color32::from_rgb(255, 0, 255),
        );
    }

    /// create a new transform
    pub fn new(frame: Rect, scale: f32) -> Self {
        Self {
            frame,
            scale,
            translation: frame.min.to_vec2(),
        }
    }

    /// apply the translation offset to the given point
    pub fn apply(&self, point: Pos2) -> Pos2 {
        point + self.translation
    }

    /// remote the anslation offset from the given point
    fn inverse_apply(&self, point: Pos2) -> Pos2 {
        point - self.translation
    }
}
