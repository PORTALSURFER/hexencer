use egui::{epaint, pos2, Color32, Pos2, Rect, Response, Rounding, Sense, Shape, Stroke, Ui, Vec2};

use crate::EDGE_COLOR;
use hexencer_core::data::Clip;

const EDITOR_NOTE_HEIGHT: f32 = 8.0;

pub struct NoteEditor<'c> {
    clip: &'c Clip,
}

impl<'c> NoteEditor<'c> {
    pub fn new(clip: &'c Clip) -> Self {
        Self { clip }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let fill_color = Color32::from_rgb(50, 50, 50);
        let rect = ui.available_rect_before_wrap();
        ui.set_clip_rect(rect);
        let shape = epaint::RectShape::new(
            rect,
            Rounding::ZERO,
            fill_color,
            Stroke::new(1.0, EDGE_COLOR),
        );
        ui.painter().add(shape);
        let response = ui.allocate_rect(rect, Sense::drag());

        for (tick, event) in self.clip.events.iter() {
            let tick = tick.as_f32();
            let key = event.get_key();
            let editor_rect = ui.available_rect_before_wrap();
            let pos = Pos2::new(
                editor_rect.min.x + tick,
                editor_rect.min.y - (key as f32 * EDITOR_NOTE_HEIGHT),
            );

            clip_note(pos, ui);
        }

        draw_lanes(ui);

        if response.drag_started() {
            tracing::info!("add note");
            let mouse_pos = if let Some(mouse_pos) = ui.input(|input| input.pointer.hover_pos()) {
                mouse_pos
            } else {
                Pos2::new(0.0, 0.0)
            };
            clip_note(mouse_pos, ui);
        }

        response
    }
}

#[derive(Clone, Debug)]
struct GridUnit {
    pub offset: f64,
}

fn draw_lanes(ui: &mut Ui) {
    let rect = ui.available_rect_before_wrap();
    let mut shapes = Vec::new();

    let grid_step = GridUnit {
        offset: EDITOR_NOTE_HEIGHT as f64,
    };

    let first = 0;
    let last = 40;

    let mut steps = Vec::new();
    let step_size = 10.0;
    for i in first..last {
        let value = (i as f64) * step_size;
        steps.push(GridUnit { offset: value });
    }

    for step in steps {
        let line = Shape::line_segment(
            [
                Pos2::new(rect.min.x, rect.min.y - step.offset as f32),
                Pos2::new(
                    rect.min.x + ui.available_width(),
                    rect.min.y - step.offset as f32,
                ),
            ],
            Stroke::new(1.0, Color32::RED),
        );
        shapes.push(line);
    }
    ui.painter().extend(shapes);
}

fn clip_note(mouse_pos: Pos2, ui: &mut Ui) {
    let rect = Rect::from_min_size(mouse_pos, Vec2::new(100.0, EDITOR_NOTE_HEIGHT));
    let fill_color = Color32::from_rgb(97, 255, 219);
    let new_note_shape = epaint::RectShape::new(rect, Rounding::ZERO, fill_color, Stroke::NONE);
    ui.painter().add(new_note_shape);
}
