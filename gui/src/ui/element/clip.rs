use crate::memory::GuiState;
use egui::layers::ShapeIdx;
use egui::{emath::*, epaint, Color32, Response, Rounding, Sense, Shape, Stroke};
use egui::{Context, Id, Pos2, Rect, Ui, Vec2};
use hexencer_core::{DataId, Tick};

/// default clip length for painting
pub const DEFAULT_CLIP_WIDTH: f32 = 96.0;
/// beat width in the viewport, used for creating the lines in the arranger
pub const BEAT_WIDTH: f32 = 24.0;

/// create a new 'clip' and returns it's 'Response'
pub fn clip(ctx: &Context, ui: &mut Ui, id: crate::DataId, tick: Tick) -> Response {
    let egui_id = egui::Id::new(id.as_bytes());
    let clip = ClipWidget::new(id, egui_id, tick);
    clip.show(ctx, ui)
}

/// state of the 'ClipWidget'
#[derive(Clone, Copy, Debug, Default)]
struct State {
    /// current position of the clip, used for movement interaction
    pub pivot_pos: Pos2,
}

impl State {
    /// load this state from memory, or create a default one
    pub fn load_or_default(id: Id, ui: &Ui) -> Self {
        ui.memory(|mem| mem.data.get_temp(id).unwrap_or_default())
    }

    /// load this state from memory, or create a default one
    pub fn load(id: Id, ui: &Ui) -> Option<Self> {
        ui.memory(|mem| mem.data.get_temp(id))
    }

    /// store this state to memory
    pub fn store(self, id: Id, ui: &mut Ui) {
        ui.memory_mut(|mem| mem.data.insert_temp(id, self))
    }
}

/// widget used to represent 'Clips' on a 'Track'
#[must_use = "You should call .show()"]
#[derive(Clone, Copy, Debug)]
pub struct ClipWidget {
    /// data id of the clip, used as id by datalayer
    data_id: DataId,
    /// egui id of this clip widget
    id: Id,
    /// if this clip is active
    active: bool,
    /// current clip offset on the track
    offset: f32,
}

/// quantize a value to a step size
fn quantize(x: f32, initial: f32, step_size: u32) -> f32 {
    initial + ((x - initial) / step_size as f32).floor() * step_size as f32
}

impl ClipWidget {
    /// creates a new 'Clip'
    /// 'tick' will set the position of the 'Clip' on the 'Track'
    pub fn new(data_id: DataId, id: Id, tick: Tick) -> Self {
        let offset = tick.as_f32() / 480.0 * DEFAULT_CLIP_WIDTH;

        Self {
            data_id,
            id,
            active: true,
            offset,
        }
    }

    /// renders this element and returns the 'Response'
    pub fn show(self, ctx: &Context, ui: &mut Ui) -> Response {
        let prepared = self.begin(ctx, ui);
        prepared.end(ui)
    }

    /// begin building the clip widget
    fn begin(self, ctx: &Context, ui: &mut Ui) -> Prepared {
        let where_to_put_background = ui.painter().add(Shape::Noop);

        let height = ui.available_height();
        let width = DEFAULT_CLIP_WIDTH;
        let size = Vec2::new(width, height);

        let mut start_pos = ui.max_rect().min;
        start_pos.x += self.offset;
        let mut state = match State::load(self.id, ui) {
            Some(state) => state,
            _ => State {
                pivot_pos: start_pos,
            },
        };
        let quantized = quantize(state.pivot_pos.x, start_pos.x, 24);
        let new_pos = pos2(quantized, state.pivot_pos.y);

        let rect = Rect::from_min_size(new_pos, size);
        let mut move_response = {
            let move_response = ui.interact(rect, self.id, Sense::drag());

            if move_response.dragged() {
                let delta = move_response.drag_delta();
                state.pivot_pos.x += delta.x;
                tracing::info!("Clip pos {:?}", state.pivot_pos.x);
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

        state.store(self.id, ui);

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
    pub clip: ClipWidget,
    /// whether the clip is active or not
    active: bool,
    /// used to prevent a glicht in egui causing the first frame to flicker, not actively used atm i think
    temporarily_invisible: bool,
    /// used to constraint movement
    constrain_rect: Rect,
    /// response from the clip widget
    move_response: Response,
    /// rect of this clip
    rect: Rect,
    /// inner ui
    content_ui: Ui,
    /// state of this clip
    state: State,
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
            Some(s) if s == self.clip.data_id => SELECTED_COLOR,
            _ => DEFAULT_COLOR,
        };
        self.paint(ui, clip_color);
        self.move_response
    }

    /// paints this clip widget
    fn paint(&self, ui: &Ui, fill_color: Color32) {
        let paint_rect = self.rect;
        if ui.is_rect_visible(paint_rect) {
            let shape = self.clip.paint(paint_rect, fill_color);
            ui.painter().set(self.where_to_put_background, shape);
        }
    }
}
