use crate::memory::GuiState;
use crate::ui::common::TRACK_HEIGHT;
use crate::WidgetState;
use egui::layers::ShapeIdx;
use egui::{
    emath::*, epaint, Color32, DragAndDrop, InnerResponse, LayerId, Order, Response, Rounding,
    Sense, Shape, Stroke,
};
use egui::{Context, Id, Pos2, Rect, Ui, Vec2};
use hexencer_core::data::ClipId;
use hexencer_core::Tick;

/// default clip length for painting
pub const DEFAULT_CLIP_WIDTH: f32 = 96.0;
/// beat width in the viewport, used for creating the lines in the arranger
pub const BEAT_WIDTH: f32 = 24.0;

/// create a new 'clip' and returns it's 'Response'
pub fn clip(ctx: &Context, ui: &mut Ui, id: ClipId, tick: Tick, length: Tick) -> Response {
    let egui_id = egui::Id::new(id.as_bytes());
    let clip = DragWidget::new(id, egui_id, tick, length);
    clip.show(ctx, ui, |_| {}).response
}

#[derive(Clone, Copy, Debug, Default)]
struct State {
    //     /// current position of the clip, used for movement interaction
    pub drag_position: Pos2,
}

impl WidgetState for State {}

/// widget used to represent 'Clips' on a 'Track'
#[must_use = "You should call .show()"]
#[derive(Clone, Debug)]
pub struct DragWidget {
    /// data id of the clip, used as id by datalayer
    clip_id: ClipId,
    /// egui id of this clip widget
    id: Id,
    /// if this clip is active
    active: bool,
    /// current clip offset on the track
    clip_position: f32,
    /// width of the clip in ticks?
    width: u64,
}

/// quantize a value to a step size
pub fn quantize(value: f32, step_size: f32, offset: f32) -> f32 {
    // offset + ((value - offset) / step_size).floor() * step_size
    offset + ((value - offset) / step_size).round() * step_size
}

impl DragWidget {
    /// creates a new 'Clip'
    /// 'tick' will set the position of the 'Clip' on the 'Track'
    pub fn new(clip_id: ClipId, id: Id, tick: Tick, length: Tick) -> Self {
        let offset = tick.as_f32() / 480.0 * DEFAULT_CLIP_WIDTH;

        Self {
            clip_id,
            id,
            active: true,
            clip_position: offset,
            width: length.as_f32() as u64, // TODO, convert tick to pixel width
        }
    }

    /// renders this element and returns the 'Response'
    pub fn show<R>(
        self,
        ctx: &Context,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let mut prepared = self.begin(ctx, ui);
        let inner = add_contents(&mut prepared.content_ui);
        let response = prepared.end(ui);
        InnerResponse::new(inner, response)
    }

    /// begin building the clip widget
    fn begin(self, ctx: &Context, ui: &mut Ui) -> Prepared {
        let where_to_put_background = ui.painter().add(Shape::Noop);
        let size = self.get_size(ui);

        let mut start_pos = ui.max_rect().min;
        start_pos.x += self.clip_position;

        // let state = State::load(self.id, ui);

        // let is_new = state.is_none();
        // if is_new {
        //     ctx.request_repaint();
        // }

        // let mut state = state.unwrap_or(State {
        //     drag_position: start_pos,
        // });

        let (rect, move_response) = self.handle_dragging(ui, size, ctx, start_pos);

        let content_ui = ui.child_ui(rect, *ui.layout());

        Prepared {
            clip: self.clone(),
            active: self.active,
            temporarily_invisible: false,
            move_response,
            rect,
            where_to_put_background,
            content_ui,
        }
    }

    fn get_size(&self, ui: &mut Ui) -> Vec2 {
        let height = ui.available_height();
        let width = (self.width as f32 / 480.0) * 24.0;
        let size = Vec2::new(width, height);
        size
    }

    /// handle dragging around of clip on track
    fn handle_dragging(
        &self,
        ui: &mut Ui,
        size: Vec2,
        ctx: &Context,
        start_pos: Pos2,
    ) -> (Rect, Response) {
        // load state
        // if was loaded,set drag_pos to the start pos
        let mut state = match State::load(self.id, ui) {
            Some(state) => state,
            _ => State {
                drag_position: start_pos,
            },
        };

        // let quantized = quantize(state.pivot_pos.x, 24.0, start_pos.x);
        // let quantized_y = quantize(state.pivot_pos.y, TRACK_HEIGHT, start_pos.y);

        let mut rect = Rect::from_min_size(start_pos, size);
        let mut move_response = ui.interact(rect, self.id, Sense::drag());

        if move_response.dragged() {
            tracing::info!("drag pos {}", state.drag_position.x);
            DragAndDrop::set_payload(ctx, self.clip_id);
            let delta = move_response.drag_delta();

            state.drag_position.x += delta.x;
            state.drag_position.y += delta.y;

            let quantized = quantize(state.drag_position.x, 24.0, start_pos.x);
            let quantized_y = quantize(state.drag_position.y, TRACK_HEIGHT, start_pos.y);
            let new_pos = pos2(quantized, quantized_y);
            rect = Rect::from_min_size(new_pos, size);

            let mut global_state = GuiState::load(ui);
            global_state.last_dragged_clip_pos = Some(new_pos);
            global_state.store(ui);

            state.store(self.id, ui);
        }

        if move_response.drag_stopped() {
            tracing::info!("stopped dragging at {}", state.drag_position.x);
            state.drag_position = start_pos;
            state.store(self.id, ui);
        }

        // update response with drag movement
        move_response.rect = rect;
        move_response.interact_rect = rect;
        (rect, move_response)
    }

    /// paint this clip widget
    fn paint(&self, paint_rect: Rect, fill_color: Color32) -> Shape {
        Shape::Rect(epaint::RectShape::new(
            paint_rect,
            Rounding::ZERO,
            fill_color,
            Stroke::new(1.0, egui::Color32::BLACK),
        ))
    }
}

/// intermediate struct used to build the 'ClipWidget'
pub struct Prepared {
    /// clip widget to be built
    pub clip: DragWidget,
    /// whether the clip is active or not
    active: bool,
    /// used to prevent a glicht in egui causing the first frame to flicker, not actively used atm i think
    temporarily_invisible: bool,
    /// response from the clip widget
    move_response: Response,
    /// rect of this clip
    rect: Rect,
    /// inner ui
    content_ui: Ui,
    /// placeholder for painting in the background color
    where_to_put_background: ShapeIdx,
}

impl Prepared {
    /// ends building the widget
    fn end(self, ui: &mut Ui) -> egui::Response {
        /// color for selected clips
        const SELECTED_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 0, 0);
        /// default clip color
        const DEFAULT_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 255, 0);

        let clip_color = match GuiState::load(ui).selected_clip {
            Some(s) if s == self.clip.clip_id => SELECTED_COLOR,
            _ => DEFAULT_COLOR,
        };
        self.paint(ui, clip_color);
        self.move_response
    }

    /// paints this clip widget
    fn paint(&self, ui: &mut Ui, fill_color: Color32) {
        let layer_id = LayerId::new(Order::Foreground, Id::new("drag clip"));

        let paint_rect = self.rect;

        if ui.is_rect_visible(paint_rect) {
            let shape = self.clip.paint(paint_rect, fill_color);
            ui.with_layer_id(layer_id, |ui| ui.painter().add(shape));
        }
    }
}
