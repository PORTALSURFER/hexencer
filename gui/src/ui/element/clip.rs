use egui::layers::ShapeIdx;
use egui::{emath::*, epaint, Color32, Order, Response, Rounding, Sense, Shape, Stroke};
use egui::{Context, Id, Pos2, Rect, Ui, Vec2};
use hexencer_core::{DataId, Tick};

use crate::arranger::SELECTED_CLIP;

/// create a new 'clip' and returns it's 'Response'
pub fn clip(ctx: &Context, ui: &mut Ui, id: crate::DataId, tick: Tick) -> Response {
    let egui_id = egui::Id::new(id.as_bytes());
    let clip = Clip::new(id, egui_id, tick);
    clip.show(ctx, ui)
}

#[derive(Clone, Copy, Debug)]
struct State {
    pub pivot_pos: Pos2,
}

/// widget used to represent 'Clips' on a 'Track'
#[must_use = "You should call .show()"]
#[derive(Clone, Copy, Debug)]
pub struct Clip {
    data_id: DataId,
    id: Id,
    active: bool,
    _pos: Option<Pos2>,
    _order: Order,
    offset: f32,
}

fn quantize(x: f32, initial: f32, step_size: u32) -> f32 {
    initial + ((x - initial) / step_size as f32).floor() * step_size as f32
}

impl Clip {
    /// creates a new 'Clip'
    /// 'tick' will set the position of the 'Clip' on the 'Track'
    pub fn new(data_id: DataId, id: Id, tick: Tick) -> Self {
        let offset = tick.as_f32() / 480.0 * 96.0;

        Self {
            data_id,
            id,
            active: true,
            _pos: None,
            _order: Order::Middle,
            offset,
        }
    }

    /// renders this element and returns the 'Response'
    pub fn show(self, ctx: &Context, ui: &mut Ui) -> Response {
        let prepared = self.begin(ctx, ui);
        let response = prepared.end(ctx, ui);
        response
    }

    fn begin(self, ctx: &Context, ui: &mut Ui) -> Prepared {
        let where_to_put_background = ui.painter().add(Shape::Noop);

        let height = ui.available_height();
        let width = 96.0;
        let size = Vec2::new(width, height);

        let mut start_pos = ui.max_rect().min;
        start_pos.x += self.offset;
        let mut state = ctx.memory(|mem| {
            mem.data.get_temp::<State>(self.id).unwrap_or(State {
                pivot_pos: start_pos,
            })
        });

        let quantized = quantize(state.pivot_pos.x, start_pos.x, 24);
        let new_pos = pos2(quantized, state.pivot_pos.y);

        let rect = Rect::from_min_size(new_pos, size);
        let mut move_response = {
            let move_response = ui.interact(rect, self.id, Sense::drag());

            if move_response.dragged() {
                let delta = move_response.drag_delta();
                state.pivot_pos.x += delta.x;
            }

            if move_response.dragged() || move_response.clicked() {
                ctx.memory_mut(|memory| memory.areas().visible_last_frame(&move_response.layer_id));
                ctx.request_repaint();
            }
            move_response
        };

        let constrain_rect = ui.available_rect_before_wrap();

        // update response with drag movement
        move_response.rect = rect;
        move_response.interact_rect = rect;

        let content_ui = ui.child_ui(rect, *ui.layout());

        Prepared {
            state,
            clip: self,
            active: self.active,
            temporarily_invisible: false,
            constrain_rect,
            move_response,
            rect,
            where_to_put_background,
            content_ui,
        }
    }

    fn paint(&self, paint_rect: Rect, fill_color: Color32) -> Shape {
        let shape = Shape::Rect(epaint::RectShape::new(
            paint_rect,
            Rounding::ZERO,
            fill_color,
            // egui::Color32::from_rgb(120, 140, 50),
            Stroke::new(1.0, egui::Color32::BLACK),
        ));
        shape
    }
}

pub struct Prepared {
    pub clip: Clip,
    active: bool,
    temporarily_invisible: bool,
    constrain_rect: Rect,
    move_response: Response,
    rect: Rect,
    content_ui: Ui,
    state: State,
    where_to_put_background: ShapeIdx,
}
impl Prepared {
    fn end(self, ctx: &Context, ui: &mut Ui) -> egui::Response {
        const SELECTED_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 0, 0);
        const DEFAULT_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 255, 0);

        let selected_color =
            match ui.memory(|mem| mem.data.get_temp::<DataId>(SELECTED_CLIP.into())) {
                Some(s) if s == self.clip.data_id => SELECTED_COLOR,
                _ => DEFAULT_COLOR,
            };

        ctx.memory_mut(|memory| memory.data.insert_temp(self.clip.id, self.state));
        self.paint(ui, selected_color);
        self.move_response
    }

    fn paint(&self, ui: &Ui, fill_color: Color32) {
        let paint_rect = self.rect;
        if ui.is_rect_visible(paint_rect) {
            let shape = self.clip.paint(paint_rect, fill_color);
            ui.painter().set(self.where_to_put_background, shape);
        }
    }
}
