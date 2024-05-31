use std::ops::RangeInclusive;

use egui::{
    emath::Real,
    epaint::{self, CircleShape},
    lerp, pos2, Color32, FontId, Id, LayerId, Order, PointerButton, Pos2, Rect, Response, Rounding,
    Sense, Stroke, Ui, Vec2,
};

use crate::EDGE_COLOR;
use hexencer_core::data::Clip;

const EDITOR_NOTE_HEIGHT: f32 = 8.0;

#[derive(Clone, Copy)]
struct EditorBounds {
    pub min: [f64; 2],
    pub max: [f64; 2],
}
impl EditorBounds {
    fn to_rect(&self) -> Rect {
        Rect::from_min_max(
            pos2(self.min[0] as f32, self.min[1] as f32),
            pos2(self.max[0] as f32, self.max[1] as f32),
        )
    }

    pub const NONE: Self = Self {
        min: [f64::INFINITY; 2],
        max: [f64::INFINITY; 2],
    };

    fn box_zoom(&mut self, zoom_factor: Vec2, center: EditorPoint) {
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

    fn is_valid(&self) -> bool {
        self.is_finite() && self.width() > 0.0 && self.height() > 0.0
    }

    fn width(&self) -> f64 {
        self.max[0] - self.min[0]
    }

    fn height(&self) -> f64 {
        self.max[1] - self.min[1]
    }
}

#[derive(Clone, Copy)]
pub struct Transform {
    bounds: EditorBounds,
    scale: f32,
    frame: Rect,
    translation: Vec2,
}

#[derive(Clone, Debug)]
struct EditorPoint {
    pub x: f64,
    pub y: f64,
}

impl EditorPoint {
    fn new(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl Transform {
    fn zoom(
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

    /// Convert a position in the frame to a value in the bounds.
    fn value_from_position(&self, pos: Pos2) -> EditorPoint {
        tracing::info!("frame {:?}", self.frame);
        let x = remap(
            pos.x as f64,
            (self.frame.left() as f64)..=(self.frame.right() as f64),
            self.bounds.min[0]..=self.bounds.max[0],
        );
        let y = remap(
            pos.y as f64,
            (self.frame.bottom() as f64)..=(self.frame.top() as f64),
            self.bounds.min[1]..=self.bounds.max[1],
        );
        EditorPoint::new(x, y)
    }

    fn new(frame: Rect, bounds: EditorBounds, scale: f32) -> Self {
        Self {
            frame,
            bounds,
            scale,
            translation: frame.min.to_vec2(),
        }
    }

    fn apply(&self, point: Pos2) -> Pos2 {
        point + self.translation
    }

    fn inverse_apply(&self, point: Pos2) -> Pos2 {
        point - self.translation
    }
}

fn debug_line(ui: &mut Ui, origin: Pos2, target: Pos2, color: Color32) {
    let top_layer = LayerId::new(Order::Foreground, Id::new("debug_line"));
    ui.with_layer_id(top_layer, |ui| {
        ui.painter()
            .line_segment([origin, target], Stroke::new(1.0, color));
    });
}

fn debug_dot(ui: &Ui, point: Pos2, color: Color32) {
    ui.painter().add(CircleShape {
        center: point,
        radius: 2.0,
        fill: color,
        stroke: Stroke::NONE,
    });
}

/// Remap a value from one range to another.
fn remap<T>(x: T, from: impl Into<RangeInclusive<T>>, to: impl Into<RangeInclusive<T>>) -> T
where
    T: Real,
{
    let from = from.into();
    let to = to.into();

    let t = (x - *from.start()) / (*from.end() - *from.start());
    lerp(to, t)
}

#[derive(Clone)]
pub struct State {
    pub last_click_pos_for_zoom: Option<Pos2>,
    pub transform: Transform,
    pub step_size: f32,
}

impl State {
    pub fn load(ui: &Ui, id: Id) -> Option<Self> {
        ui.memory(|memory| memory.data.get_temp(id))
    }

    pub fn store(self, ui: &Ui, id: Id) {
        ui.memory_mut(|memory| memory.data.insert_temp(id, self));
    }
}

pub struct NoteEditor<'c> {
    clip: &'c Clip,
}

impl<'c> NoteEditor<'c> {
    pub fn new(clip: &'c Clip) -> Self {
        Self { clip }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let fill_color = Color32::from_rgb(50, 50, 50);
        let editor_rect = ui.available_rect_before_wrap();

        ui.set_clip_rect(editor_rect);
        let id = Id::new("note_editor");

        let min_auto_bounds = EditorBounds::NONE;
        let mut state = State::load(ui, id).unwrap_or_else(|| State {
            last_click_pos_for_zoom: None,
            transform: Transform::new(editor_rect, min_auto_bounds, 1.0),
            step_size: 10.0,
        });
        let shape = epaint::RectShape::new(
            editor_rect,
            Rounding::ZERO,
            fill_color,
            Stroke::new(1.0, EDGE_COLOR),
        );
        ui.painter().add(shape);
        let response = ui.allocate_rect(editor_rect, Sense::drag());

        let note_height = state.step_size;
        for (tick, event) in self.clip.events.iter() {
            let tick = tick.as_f32();
            let key = event.get_key();
            let editor_rect = ui.available_rect_before_wrap();
            let pos = Pos2::new(
                editor_rect.min.x + tick,
                editor_rect.min.y - (key as f32 * EDITOR_NOTE_HEIGHT),
            );

            clip_note(pos, ui, note_height);
        }

        if response.drag_started_by(egui::PointerButton::Primary) {
            let mouse_pos = if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                mouse_pos
            } else {
                Pos2::new(0.0, 0.0)
            };
            clip_note(mouse_pos, ui, note_height);
        }

        let zoom_button = PointerButton::Secondary;
        if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
            if response.dragged_by(zoom_button) {
                let zoom_delta = ui.input(|input| input.pointer.delta());
                state
                    .transform
                    .zoom(zoom_delta, hover_pos, ui, editor_rect, state.step_size);
            }
        }

        self.draw_lanes(ui, state.step_size, editor_rect, state.transform);

        state.store(ui, id);
        response
    }

    fn draw_lanes(&self, ui: &mut Ui, step_size: f32, rect: Rect, transform: Transform) {
        let mut lines = Vec::new();
        let scaled_step_size = step_size * transform.scale;

        let start_step = 0;
        let end_step = 127;

        for current_step in start_step..=end_step {
            let step_pos = current_step as f32 * scaled_step_size;
            let screen_pos = transform.apply(Pos2::new(step_pos, step_pos));

            let origin = pos2(rect.min.x, screen_pos.y);
            let target = pos2(rect.max.x, screen_pos.y);
            let line =
                epaint::Shape::line_segment([origin, target], Stroke::new(1.0, Color32::RED));
            lines.push(line);
            line_number(ui, origin, current_step);
        }
        ui.painter().extend(lines);
    }
}

fn line_number(ui: &mut Ui, pos: Pos2, num: i32) {
    let font_id = FontId::monospace(12.0);
    let text_color = Color32::RED;
    let galley = ui.fonts(|f| {
        f.layout(
            String::from({
                let output_text = format!("note {}", num);
                output_text
            }),
            font_id,
            text_color,
            10000.0,
        )
    });
    let underline = Stroke::NONE;
    let fallback_color = Color32::BLUE;
    ui.painter().add(epaint::TextShape {
        pos,
        galley,
        underline,
        fallback_color,
        override_text_color: None,
        opacity_factor: 1.0,
        angle: 0.0,
    });
}

// this is box zooming, draw a rectangle around the area you want to zoom into to focus in on it
fn box_zooming(
    response: &Response,
    state: &mut State,
    ui: &mut Ui,
    note_height: f32,
    scale_factor: f32,
) {
    let mut zoom_box_shape = None;
    if response.drag_started_by(egui::PointerButton::Secondary) {
        state.last_click_pos_for_zoom = response.hover_pos();
    }
    let box_start_pos = state.last_click_pos_for_zoom;
    let box_end_pos = response.hover_pos();
    if let (Some(box_start_pos), Some(box_end_pos)) = (box_start_pos, box_end_pos) {
        if response.dragged_by(egui::PointerButton::Secondary) {
            tracing::info!("zoom box start {:?} end {:?}", box_start_pos, box_end_pos);
            let zoom_box = Rect::from_two_pos(box_start_pos, box_end_pos);
            zoom_box_shape = Some(epaint::RectShape::new(
                zoom_box,
                Rounding::ZERO,
                Color32::from_rgb(0, 255, 0),
                Stroke::new(2.0, Color32::from_rgb(0, 255, 0)),
            ));
            ui.input(|input| {
                let mut scaled_value = note_height;
                scaled_value += input.pointer.delta().x * scale_factor;
                scaled_value = scaled_value.clamp(10.0, 40.0);
                tracing::info!("scaling note editor to {}", scaled_value);
                state.step_size = scaled_value;
            })
        }
    }
    if let Some(zoom_box_shape) = zoom_box_shape {
        tracing::info!("paint zoom box");
        ui.painter().add(zoom_box_shape);
    }
}

#[derive(Clone, Debug)]
struct GridUnit {
    pub offset: f64,
}
fn clip_note(mouse_pos: Pos2, ui: &mut Ui, height: f32) {
    let rect = Rect::from_min_size(mouse_pos, Vec2::new(100.0, height));
    let fill_color = Color32::from_rgb(97, 255, 219);
    let new_note_shape = epaint::RectShape::new(rect, Rounding::ZERO, fill_color, Stroke::NONE);
    ui.painter().add(new_note_shape);
}
