use crate::EDGE_COLOR;
use egui::{
    emath::Real,
    epaint::{self, CircleShape},
    lerp, pos2, Color32, FontId, Id, LayerId, Order, PointerButton, Pos2, Rect, Response, Rounding,
    Sense, Shape, Stroke, Ui,
};

use self::transform::Transform;
use super::BEAT_WIDTH;
use hexencer_core::{data::Clip, Tick};
use std::ops::RangeInclusive;

/// trainsform used for panning and zooming
mod transform;

/// paint a debug line on the top layer
fn debug_line(ui: &mut Ui, origin: Pos2, target: Pos2, color: Color32) {
    let top_layer = LayerId::new(Order::Foreground, Id::new("debug_line"));
    ui.with_layer_id(top_layer, |ui| {
        ui.painter()
            .line_segment([origin, target], Stroke::new(1.0, color));
    });
}

/// paint a debug dot on top layer
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

/// note editor state
#[derive(Clone)]
pub struct State {
    /// the last click position when zooming in the note editor
    pub last_click_pos_for_zoom: Option<Pos2>,
    /// transform state for the note editor, used for zooming and panning
    pub transform: Transform,
    /// step size for the note editor
    pub step_size: f32,
}

impl State {
    /// load the note editor state from memory
    pub fn load(ui: &Ui, id: Id) -> Option<Self> {
        ui.memory(|memory| memory.data.get_temp(id))
    }

    /// store the note editor state to memory
    pub fn store(self, ui: &Ui, id: Id) {
        ui.memory_mut(|memory| memory.data.insert_temp(id, self));
    }
}

/// A note editor widget that can be used to display and edit notes of the currently selected clip
pub struct NoteEditorWidget<'c> {
    /// reference to the clip this note editor is editing
    clip: &'c Clip,
}

impl<'c> NoteEditorWidget<'c> {
    /// Create a new note editor widget
    pub fn new(clip: &'c Clip) -> Self {
        Self { clip }
    }

    /// setup and render the widget, and return a response
    pub fn show(self, ui: &mut Ui) -> Response {
        let fill_color = Color32::from_rgb(50, 50, 50);
        let editor_rect = ui.available_rect_before_wrap();

        ui.set_clip_rect(editor_rect);
        let id = Id::new("note_editor");

        let mut state = State::load(ui, id).unwrap_or_else(|| State {
            last_click_pos_for_zoom: None,
            transform: Transform::new(editor_rect, 1.0),
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

        let zoom_button = PointerButton::Secondary;
        if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
            if response.dragged_by(zoom_button) {
                let zoom_delta = ui.input(|input| input.pointer.delta());
                state
                    .transform
                    .zoom(zoom_delta, hover_pos, ui, editor_rect, state.step_size);
            }
        }

        self.draw_note_lanes(ui, state.step_size, editor_rect, state.transform);
        self.draw_beat_columns(ui, editor_rect);

        state.store(ui, id);
        response
    }

    /// draw the note lanes in the editor
    fn draw_note_lanes(
        &self,
        ui: &mut Ui,
        step_size: f32,
        editor_rect: Rect,
        transform: Transform,
    ) {
        let mut lines = Vec::new();
        let scaled_step_size = step_size * transform.scale;

        let start_step = 0;
        let end_step = 127;

        for current_step in start_step..=end_step {
            let step_pos = current_step as f32 * scaled_step_size;
            let lane_offset = transform.apply(Pos2::new(step_pos, step_pos));

            let origin = pos2(editor_rect.min.x, lane_offset.y);
            let target = pos2(editor_rect.max.x, lane_offset.y);
            let line =
                epaint::Shape::line_segment([origin, target], Stroke::new(1.0, Color32::RED));
            lines.push(line);
            line_number(ui, origin, current_step);
        }

        for (tick, events) in self.clip.events.iter() {
            for event in events {
                let note_lane = event.get_key();
                let note_end_tick = event.get_end();
                clip_note(
                    note_lane,
                    ui,
                    scaled_step_size,
                    transform,
                    *tick,
                    note_end_tick,
                    editor_rect,
                );
            }
        }

        ui.painter().extend(lines);
    }

    /// draws the beat columns in the note editor
    fn draw_beat_columns(&self, ui: &mut Ui, rect: Rect) {
        let mut shapes = Vec::new();
        for i in 0..100 {
            let offset = rect.min.x + i as f32 * BEAT_WIDTH;
            let line = Shape::line_segment(
                [pos2(offset, rect.min.y), pos2(offset, rect.min.y + 1000.0)],
                Stroke::new(1.0, Color32::from_rgb(100, 100, 100)),
            );
            shapes.push(line);
        }
        ui.painter().extend(shapes);
    }
}

/// paints the note editor line numbers
fn line_number(ui: &mut Ui, pos: Pos2, num: i32) {
    let font_id = FontId::monospace(12.0);
    let text_color = Color32::RED;
    let galley = ui.fonts(|f| {
        f.layout(
            {
                let output_text = format!("note {}", num);
                output_text
            },
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

/// this is box zooming, draw a rectangle around the area you want to zoom into to focus in on it
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

/// create a clip note
fn clip_note(
    note_lane: u8,
    ui: &mut Ui,
    height: f32,
    transform: Transform,
    tick: Tick,
    tick_end_tick: Tick,
    editor_rect: Rect,
) {
    let step_pos = note_lane as f32 * height;
    let lane_offset = transform.apply(Pos2::new(step_pos, step_pos));
    let note_offset = editor_rect.min.x + ((tick.as_f32() / 480.0) * BEAT_WIDTH);
    let origin = pos2(note_offset, lane_offset.y);
    let target = pos2(
        note_offset + ((tick_end_tick.as_f32() / 480.0) * BEAT_WIDTH),
        lane_offset.y + height,
    );
    let rect = Rect::from_two_pos(origin, target);

    let fill_color = Color32::from_rgb(97, 255, 219);
    let new_note_shape = epaint::RectShape::new(rect, Rounding::ZERO, fill_color, Stroke::NONE);
    ui.painter().add(new_note_shape);
}
