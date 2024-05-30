use std::ops::RangeInclusive;

use egui::{
    emath::Real, epaint, lerp, pos2, Color32, Id, PointerButton, Pos2, Rect, Response, Rounding,
    Sense, Shape, Stroke, Ui, Vec2,
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

    fn zoom(&mut self, zoom_factor: Vec2, center: EditorPoint) {
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
    fn zoom(&mut self, zoom_delta: f32, screen_hover_pos: Pos2) {
        let zoom_rate = 0.001;
        let zoom_delta = zoom_delta * zoom_rate;
        let new_scale = (self.scale + zoom_delta).clamp(0.1, 10.0);

        let pointer_in_editor = (screen_hover_pos - self.translation) / self.scale;

        self.scale = new_scale;

        tracing::info!("pointer in editor {:?}", pointer_in_editor);
        tracing::info!("new_scale {:?}:", new_scale);

        let new_translation = screen_hover_pos - pointer_in_editor * self.scale;
        self.translation = new_translation;

        // let center = self.value_from_position(center);
        // tracing::info!("center {:?}", center);
        // let mut new_bounds = self.bounds;
        // new_bounds.zoom(zoom_factor, center);

        // if new_bounds.is_valid() {
        //     self.bounds = new_bounds;
        // }
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
        point * self.scale + self.translation
    }
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
    pub note_height: f32,
    pub last_click_pos_for_zoom: Option<Pos2>,
    pub transform: Transform,
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

        let min_auto_bounds = EditorBounds::NONE;
        let mut state = State::load(ui, id).unwrap_or_else(|| State {
            note_height: 10.0,
            last_click_pos_for_zoom: None,
            transform: Transform::new(editor_rect, min_auto_bounds, 1.0),
        });
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

        let zoom_button = PointerButton::Secondary;
        // box_zooming(&response, &mut state, ui, note_height, scale_factor);

        // test if pointer is inside of the response area
        // and is able to grab hover position
        if let (true, Some(hover_pos)) = (
            response.contains_pointer,
            ui.input(|i| i.pointer.hover_pos()),
        ) {
            // test if dragging with right mouse button
            if response.dragged_by(zoom_button) {
                // store delta of the drag
                let zoom_delta = ui.input(|input| input.pointer.delta());
                tracing::info!("hover {:?}", hover_pos);
                let zoom_factor = zoom_delta.x;
                // zoom the transform
                state.transform.zoom(zoom_factor, hover_pos);

                // debug of transform bounds
                // let zoom_rect = state.transform.bounds.to_rect();
                // ui.painter().add(epaint::RectShape::new(
                //     zoom_rect,
                //     Rounding::ZERO,
                //     Color32::from_rgb(255, 0, 0),
                //     Stroke::new(1.0, Color32::from_rgb(255, 0, 0)),
                // ));
                // state.auto_bounds = state.auto_bounds.and
            }
        }

        let step_size = note_height;
        let step_size = step_size / state.transform.scale;

        self.draw_lanes(ui, step_size, editor_rect, state.transform, 10);

        state.store(ui, id);
        response
    }

    fn draw_lanes(
        &self,
        ui: &mut Ui,
        step_size: f32,
        rect: Rect,
        transform: Transform,
        step_count: i32,
    ) {
        let mut current_step = 0;
        while current_step < step_count {
            let step_pos = current_step as f32 * step_size;
            let screen_pos = transform.apply(Pos2::new(step_pos, step_pos));

            if rect.contains(screen_pos) {
                ui.painter().line_segment(
                    [
                        pos2(screen_pos.x, rect.min.y),
                        pos2(screen_pos.x, rect.max.y),
                    ],
                    Stroke::new(1.0, Color32::RED),
                );
                ui.painter().line_segment(
                    [
                        pos2(rect.min.x, screen_pos.y),
                        pos2(rect.max.y, screen_pos.y),
                    ],
                    Stroke::new(1.0, Color32::RED),
                );
            }
            current_step += 1;
        }

        // let mut shapes = Vec::new();

        // let first = 0;
        // let last = {
        //     let available_space = rect.height();
        //     let count = available_space / step_size;
        //     let count = count.clamp(2.0, 50.0) + 1.0;
        //     count as u32
        // };

        // let mut steps = Vec::new();
        // for i in first..last {
        //     let value = (i as f64) * step_size as f64;
        //     steps.push(GridUnit { offset: value });
        // }

        // for step in steps {
        //     let line = Shape::line_segment(
        //         [
        //             Pos2::new(rect.min.x, rect.min.y + step.offset as f32),
        //             Pos2::new(rect.min.x + rect.width(), rect.min.y + step.offset as f32),
        //         ],
        //         Stroke::new(1.0, EDGE_COLOR),
        //     );
        //     shapes.push(line);
        // }
        // ui.painter().extend(shapes);
    }
}

fn box_zooming(
    response: &Response,
    state: &mut State,
    ui: &mut Ui,
    note_height: f32,
    scale_factor: f32,
) {
    // this is box zooming
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
