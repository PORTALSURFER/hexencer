use crate::gui::HexencerContext;
use crate::memory::GuiState;
use crate::ui::common::TRACK_HEIGHT;
use crate::WidgetState;
use egui::layers::ShapeIdx;
use egui::{
    emath::{self, *},
    epaint, Color32, DragAndDrop, InnerResponse, LayerId, Order, Response, Rounding, Sense, Shape,
    Stroke,
};
use egui::{Context, Id, Pos2, Rect, Ui, Vec2};
use hexencer_core::data::{Clip, ClipId};
use hexencer_core::Tick;

/// default clip length for painting
pub const DEFAULT_CLIP_WIDTH: f32 = 96.0;

/// beat width in the viewport, used for creating the lines in the arranger
pub const BEAT_WIDTH: f32 = 24.0;

/// create a new 'clip' and returns it's 'Response'
pub fn clip(ctx: &HexencerContext, ui: &mut Ui, id: ClipId, tick: Tick, length: Tick) -> Response {
    let egui_id = egui::Id::new(id.as_bytes());
    let clip = DragWidget::new(id, egui_id, tick, length);
    clip.show(&ctx, ui)
}

#[derive(Clone, Copy, Debug, Default)]
struct State {
    //     /// current position of the clip, used for movement interaction
    pub drag_position: Pos2,
    pub is_dragged: bool,
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
    pub fn show(
        self,
        ctx: &HexencerContext,
        ui: &mut Ui,
        // add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> Response {
        let mut prepared = self.begin(ctx, ui);
        // let inner = add_contents(&mut prepared.content_ui);
        let response = prepared.end(ui, ctx);
        response
    }

    /// begin building the clip widget
    fn begin(self, ctx: &HexencerContext, ui: &mut Ui) -> Prepared {
        let where_to_put_background = ui.painter().add(Shape::Noop);
        let size = self.get_size(ui);

        // get the current position of the clip
        let clip = self.get_clip(ctx);
        let rect = clip_rect(ui, clip);

        let mut response = ui.interact(rect, self.id, Sense::drag());
        if ctx.egui_ctx.is_being_dragged(self.id) {
            response = self.drag_clip(ui, rect, ctx, response);
        }

        let id = self.id;

        Prepared {
            id,
            clip: self.clone(),
            active: self.active,
            temporarily_invisible: false,
            response,
            rect,
            where_to_put_background,
        }
    }

    fn get_clip(&self, ctx: &HexencerContext) -> Clip {
        let clip_id = self.clip_id;
        let data = ctx.data.read().unwrap();
        let project_manager = &data.project_manager;

        project_manager
            .find_clip(clip_id)
            .expect("trying to build a non existent clip")
    }

    fn get_size(&self, ui: &mut Ui) -> Vec2 {
        let height = ui.available_height();
        let width = (self.width as f32 / 480.0) * 24.0;
        let size = Vec2::new(width, height);
        size
    }

    /// handle dragging around of clip on track
    // fn handle_dragging(
    //     &self,
    //     ui: &mut Ui,
    //     size: Vec2,
    //     ctx: &Context,
    //     start_pos: Pos2,
    // ) -> (Rect, Response) {
    // let mut state = match State::load(self.id, ui) {
    //     Some(state) => state,
    //     _ => State {
    //         drag_position: start_pos,
    //     },
    // };

    // let mut rect = Rect::from_min_size(start_pos, size);
    // let move_response = ui.interact(rect, self.id, Sense::drag());

    // if move_response.dragged() {
    //     DragAndDrop::set_payload(ctx, self.clip_id);
    //     let delta = move_response.drag_delta();

    //     state.drag_position.x += delta.x;
    //     state.drag_position.y += delta.y;

    //     let quantized_x = quantize(state.drag_position.x, 24.0, start_pos.x);
    //     let quantized_y = quantize(state.drag_position.y, TRACK_HEIGHT, start_pos.y);
    //     let new_pos = pos2(quantized_x, quantized_y);
    //     rect = Rect::from_min_size(new_pos, size);

    //     let mut global_state = GuiState::load(ui);
    //     global_state.last_dragged_clip_pos = Some(new_pos);
    //     global_state.store(ui);

    //     state.store(self.id, ui);
    // }

    // if move_response.drag_stopped() {
    //     state.store(self.id, ui);
    // }

    // update response with drag movement
    // move_response.rect = rect;
    // move_response.interact_rect = rect;
    // (rect, move_response)
    // }

    /// paint this clip widget
    fn paint(&self, paint_rect: Rect, fill_color: Color32) -> Shape {
        Shape::Rect(epaint::RectShape::new(
            paint_rect,
            Rounding::ZERO,
            fill_color,
            Stroke::new(1.0, egui::Color32::BLACK),
        ))
    }

    fn drag_clip(
        &self,
        ui: &mut Ui,
        rect: Rect,
        ctx: &HexencerContext,
        response: Response,
    ) -> Response {
        let mut state = State::load_or_default(self.id, ui);
        let drag_rect = rect;
        DragAndDrop::set_payload(&ctx.egui_ctx, self.clip_id);

        let delta = response.drag_delta();
        state.drag_position.x += delta.x;
        state.drag_position.y += delta.y;

        let start_pos = drag_rect.min;

        let quantized_x = quantize(state.drag_position.x, 24.0, start_pos.x);
        let quantized_y = quantize(state.drag_position.y, TRACK_HEIGHT, start_pos.y);
        let new_pos = pos2(quantized_x, quantized_y);

        let mut global_state = GuiState::load(ui);
        global_state.last_dragged_clip_pos = Some(new_pos);
        global_state.store(ui);
        println!("is being dragged");
        state.is_dragged = true;
        state.store(self.id, ui);

        /////

        let layer_id = LayerId::new(Order::Tooltip, self.id);

        let shape =
            epaint::RectShape::new(drag_rect, Rounding::ZERO, Color32::LIGHT_BLUE, Stroke::NONE);
        ui.with_layer_id(layer_id, |ui| ui.painter().add(shape));
        if let Some(pointer_pos) = ctx.egui_ctx.pointer_interact_pos() {
            let delta = pointer_pos - response.rect.center();
            ctx.egui_ctx
                .transform_layer_shapes(layer_id, emath::TSTransform::from_translation(delta));
        }
        response
    }
}

fn clip_rect(ui: &mut Ui, clip: Clip) -> Rect {
    let pos1 = pos2(
        ui.max_rect().min.x + tick_to_track_point_x(clip.start),
        ui.max_rect().min.y,
    );
    let x = tick_to_track_point_x(clip.length);
    let vec2 = vec2(x, 18.0);
    let rect = Rect::from_min_size(pos1, vec2);

    debug_paint_point(pos1, ui, Color32::RED);
    debug_paint_point(vec2.to_pos2(), ui, Color32::RED);
    rect
}

fn debug_paint_point(pos1: Pos2, ui: &mut Ui, color: Color32) {
    let center = pos1;
    let layer_id = LayerId::new(Order::Foreground, Id::new("debug dot"));
    ui.with_layer_id(layer_id, |ui| {
        ui.painter().add(Shape::circle_filled(center, 2.0, color))
    });
}

fn tick_to_track_point_x(tick: Tick) -> f32 {
    (tick.as_f32() / 480.0) * BEAT_WIDTH * 4.0
}

/// intermediate struct used to build the 'ClipWidget'
pub struct Prepared {
    id: Id,
    /// clip widget to be built
    pub clip: DragWidget,
    /// whether the clip is active or not
    active: bool,
    /// used to prevent a glicht in egui causing the first frame to flicker, not actively used atm i think
    temporarily_invisible: bool,
    /// response from the clip widget
    response: Response,
    /// rect of this clip
    rect: Rect,
    // / inner ui
    // content_ui: Ui,
    /// placeholder for painting in the background color
    where_to_put_background: ShapeIdx,
}

impl Prepared {
    /// ends building the widget
    fn end(self, ui: &mut Ui, ctx: &HexencerContext) -> egui::Response {
        /// color for selected clips
        const SELECTED_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 0, 0);
        /// default clip color
        const DEFAULT_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 255, 0);

        let clip_color = match GuiState::load(ui).selected_clip {
            Some(s) if s == self.clip.clip_id => SELECTED_COLOR,
            _ => DEFAULT_COLOR,
        };

        let is_clip_being_dragged = DragAndDrop::has_any_payload(&ctx.egui_ctx);
        if !is_clip_being_dragged {
            self.paint(ui, clip_color, &ctx.egui_ctx);
        }

        self.response
    }

    /// paints this clip widget
    fn paint(&self, ui: &mut Ui, fill_color: Color32, ctx: &Context) {
        let layer_id = LayerId::new(Order::Foreground, Id::new("drag clip"));

        let paint_rect = self.rect;

        // if ui.is_rect_visible(paint_rect) {
        let shape = self.clip.paint(paint_rect, fill_color);
        ui.painter().add(shape); //TODO this bit paints the clip? something about this seems to put the old position in cache and for one frame it will still paint after moving
                                 // look at frame code, it has some work around it seems for preventing a paint the first frame?
                                 // ui.with_layer_id(layer_id, |ui| );
                                 // }
    }
}
