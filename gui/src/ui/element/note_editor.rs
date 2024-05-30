use egui::{
    epaint, vec2, Color32, Id, Pos2, Rect, Response, Rounding, Sense, Shape, Stroke, Ui, Vec2,
};
use tracing::Instrument;

use crate::EDGE_COLOR;
use hexencer_core::data::Clip;

const EDITOR_NOTE_HEIGHT: f32 = 8.0;

#[derive(Clone)]
pub struct State {
    pub note_height: f32,
    pub last_click_pos_for_zoom: Option<Pos2>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            note_height: 10.0,
            last_click_pos_for_zoom: None,
        }
    }
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
        // ui.set_clip_rect(editor_rect);

        let id = Id::new("note_editor");

        let mut state = State::load(ui, id).unwrap_or_else(|| State::default());
        let shape = epaint::RectShape::new(
            editor_rect,
            Rounding::ZERO,
            fill_color,
            Stroke::new(1.0, EDGE_COLOR),
        );
        ui.painter().add(shape);
        let response = ui.allocate_rect(editor_rect, Sense::drag());

        let note_height = state.note_height;
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
            tracing::info!("add note");
            let mouse_pos = if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                mouse_pos
            } else {
                Pos2::new(0.0, 0.0)
            };
            clip_note(mouse_pos, ui, note_height);
        }

        let scale_factor = 0.1;

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
                    state.note_height = scaled_value;
                    // ui.painter().add(zoom_box_shape);
                })
            }
        }

        let step_size = note_height;
        self.draw_lanes(ui, step_size, editor_rect);
        if let Some(zoom_box_shape) = zoom_box_shape {
            tracing::info!("paint zoom box");
            ui.painter().add(zoom_box_shape);
        }
        // ui.painter().add(bounds_shape);
        state.store(ui, id);
        response
    }

    fn draw_lanes(&self, ui: &mut Ui, step_size: f32, rect: Rect) {
        let mut shapes = Vec::new();

        let first = 0;
        let last = {
            let available_space = rect.height();
            let count = available_space / step_size;
            let count = count.clamp(2.0, 50.0) + 1.0;
            count as u32
        };

        let mut steps = Vec::new();
        for i in first..last {
            let value = (i as f64) * step_size as f64;
            steps.push(GridUnit { offset: value });
        }

        for step in steps {
            let line = Shape::line_segment(
                [
                    Pos2::new(rect.min.x, rect.min.y + step.offset as f32),
                    Pos2::new(rect.min.x + rect.width(), rect.min.y + step.offset as f32),
                ],
                Stroke::new(1.0, EDGE_COLOR),
            );
            shapes.push(line);
        }
        ui.painter().extend(shapes);
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
