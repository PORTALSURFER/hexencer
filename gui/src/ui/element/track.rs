use crate::{
    memory::GuiState,
    ui::{
        common::{TRACK_HEADER_WIDTH, TRACK_HEIGHT},
        quantize,
    },
};
use egui::{
    epaint, layers::ShapeIdx, pos2, Color32, Context, DragAndDrop, Id, LayerId, Layout, Order,
    Rect, Response, Rounding, Sense, Shape, Stroke, Ui, Vec2,
};
use hexencer_core::{
    data::{Clip, ClipId, DataLayer},
    DataId, Tick, TrackId,
};
use std::sync::{Arc, Mutex};

/// Track bar onto which clip elements can be placed and moved
#[derive(Clone, Debug, Default)]
#[must_use = "You should call .show()"]
pub struct TrackWidget {
    /// height of this track
    height: f32,
    /// color used to fill te background of the track
    fill: Color32,
    /// identifier of the track
    track_id: TrackId,
    /// referene to the data_layer
    data_layer: Arc<Mutex<DataLayer>>,
}

/// ui state of the track
#[derive(Clone, Default)]
pub struct State {
    /// last known mouse position, used for painting in clips
    drag_start_position: f32,
    /// true if currently drag painting
    started_drag_paint: bool,
    /// last known mouse position when drag ended
    drag_end_position: f32,
}

impl State {
    /// load state from memory
    pub fn load(ui: &Ui, id: Id) -> Self {
        ui.memory(|mem| mem.data.get_temp(id)).unwrap_or_default()
    }

    /// store state to memory
    pub fn store(self, ui: &mut Ui, id: Id) {
        ui.memory_mut(|mem| mem.data.insert_temp(id, self));
    }
}

impl TrackWidget {
    /// creates a new 'Track' element
    pub fn new(data_layer: Arc<Mutex<DataLayer>>, index: TrackId) -> Self {
        Self {
            data_layer,
            track_id: index,
            ..Default::default()
        }
    }

    /// prepares the layout and allocate space for interaction
    fn begin(self, ui: &mut Ui, ctx: &Context) -> Prepared {
        let where_to_put_background = ui.painter().add(Shape::Noop);
        let outer_rect_bounds = ui.available_rect_before_wrap();
        let available_width = ui.available_width();
        let height = TRACK_HEIGHT;
        let rect = Rect::from_min_size(outer_rect_bounds.min, Vec2::new(available_width, height));
        let response = self.allocate_space(ui, rect);

        let is_anything_being_dragged = DragAndDrop::has_any_payload(ctx);
        let can_accept_what_is_being_dragged = DragAndDrop::has_payload_of_type::<DataId>(ctx);

        if let Some(payload) = response.dnd_release_payload::<(Id, ClipId)>() {
            let (id, clip_id) = payload.as_ref();
            tracing::info!(
                "clip with id:{:?} and clip_id:{:?} was released",
                id,
                clip_id
            );

            // reassign clip to this track instead
            let mut data = self.data_layer.lock().unwrap();
            // let clip_id = data
            //     .project_manager
            //     .find_clip(payload.as_ref())
            //     .map(|clip| clip.get_id());

            let gui_state = GuiState::load(ui);
            if let Some(pos) = gui_state.last_dragged_clip_pos {
                let clip = data.project_manager.move_clip(clip_id, &self.track_id);
                if let Some(clip) = clip {
                    if let Some(track) = data.project_manager.tracks.get_mut(self.track_id) {
                        let tick = (pos.x - rect.min.x) / 24.0 * 120.0;
                        tracing::info!("pos {}", pos.x);
                        track.add_clip(Tick::from(tick), clip);
                    }
                }
            }
        }

        self.layout_clips(ui, self.track_id, ctx, rect);

        Prepared {
            track: self,
            where_to_put_background,
            rect,
            response,
        }
    }

    /// allocate response space for track element
    fn allocate_space(&self, ui: &mut Ui, rect: Rect) -> Response {
        ui.allocate_rect(rect, Sense::drag())
    }

    /// assign the color to be used for element background fill
    pub fn fill(mut self, fill: Color32) -> Self {
        self.fill = fill;
        self
    }

    /// use this to process the element so it is painted and returns a response
    pub fn show(self, ui: &mut Ui, ctx: &Context) -> Response {
        let prepared = self.begin(ui, ctx);
        prepared.end(ui)
    }

    /// paints the track elements
    fn paint(&self, paint_rect: Rect) -> Shape {
        let Self { fill, .. } = *self;

        Shape::Rect(epaint::RectShape::new(
            paint_rect,
            Rounding::ZERO,
            fill,
            Stroke::new(0.0, Color32::from_rgb(10, 10, 10)),
        ))
    }

    /// layout clips on the track
    fn layout_clips(&self, ui: &mut Ui, index: TrackId, ctx: &Context, rect: Rect) {
        let mut child_ui = ui.child_ui(rect, Layout::default());
        child_ui.horizontal(|ui| {
            let data = self.data_layer.lock().unwrap();
            let track = data.project_manager.tracks.get(index);
            if let Some(track) = track {
                for (tick, clip) in &track.clips {
                    if crate::ui::clip(ctx, ui, clip.get_id(), *tick, clip.end).drag_started() {
                        tracing::info!("clip clicked");
                        let mut gui_state = GuiState::load(ui);
                        gui_state.selected_clip = Some(*clip.get_id());
                        gui_state.store(ui);
                    };
                }
            }
        });
    }
}

/// intermediate struct to prepare a track element
pub struct Prepared {
    /// reference to the track widget being prepared
    pub track: TrackWidget,
    /// placeholder where to put the background shape
    where_to_put_background: ShapeIdx,
    /// rect of the entire track
    rect: Rect,
    /// track ui response
    response: Response,
}

impl Prepared {
    /// process interaction and paint the element
    pub fn end(self, ui: &mut Ui) -> Response {
        self.paint(ui);
        let mut state = State::load(ui, ui.id());
        if self.response.hovered() {
            if let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) {
                let mut clip_min = pos2(hover_pos.x, self.rect.min.y);
                let quantized_clip_min_x = quantize(hover_pos.x, 24.0, self.rect.min.x);
                clip_min.x = quantized_clip_min_x;
                let clip_max = pos2(quantized_clip_min_x + 24.0, self.rect.max.y);
                let rect = Rect::from_two_pos(clip_min, clip_max);
                let fill_color = Color32::LIGHT_BLUE;
                let clip_shape =
                    epaint::RectShape::new(rect, Rounding::ZERO, fill_color, Stroke::NONE);
                ui.painter().add(clip_shape);
            }
        }

        self.handle_clip_painting(ui, &mut state);
        state.store(ui, ui.id());
        self.response
    }

    /// handles painting of clips into drags, adding them to the track
    fn handle_clip_painting(&self, ui: &mut Ui, state: &mut State) {
        if self.response.drag_started() {
            self.register_drag_start_position(ui, state);
        }

        if self.response.dragged() {
            self.paint_new_clip(ui, state);
        }
        if self.response.drag_stopped() {
            let pos = state.drag_start_position - self.rect.min.x;
            let tick = pos_to_clip_tick(pos);
            let width = state.drag_end_position - state.drag_start_position;
            let end = pixel_width_to_tick(width);
            tracing::info!("store clip at {} {}", pos, end);
            state.started_drag_paint = false;
            let clip = Clip::new("new clip", end as u64);
            self.track.data_layer.lock().unwrap().add_clip(
                self.track.track_id,
                Tick::from(tick),
                clip,
            );
        }
    }

    /// sets the current mouse position to given state
    fn register_drag_start_position(&self, ui: &mut Ui, state: &mut State) {
        if !state.started_drag_paint {
            if let Some(position) = ui.input(|i| i.pointer.hover_pos()) {
                let quantized_mous_pos_x = quantize(position.x, 24.0, self.rect.min.x);
                state.drag_start_position = quantized_mous_pos_x;
                state.started_drag_paint = true;
            }
        }
    }

    /// paints a new clip based on mouse position
    fn paint_new_clip(&self, ui: &mut Ui, state: &mut State) {
        let fill_color = Color32::YELLOW;
        if let Some(current_mouse_position) = ui.input(|i| i.pointer.hover_pos()) {
            let quantized_current_mouse_pos_x =
                quantize(current_mouse_position.x, 24.0, self.rect.min.x);
            let rect = if current_mouse_position.x > state.drag_start_position {
                self.get_clip_rect_right(state, quantized_current_mouse_pos_x)
            } else {
                self.get_clip_rect_left(state, quantized_current_mouse_pos_x)
            };

            let shape = epaint::RectShape::new(rect, Rounding::ZERO, fill_color, Stroke::NONE);
            let top_layer = LayerId::new(Order::Foreground, ui.id());
            ui.with_layer_id(top_layer, |ui| {
                ui.painter().add(shape);
            });
        }
    }

    /// paint the track widget
    pub fn paint(&self, ui: &Ui) {
        let paint_rect = self.rect;
        if ui.is_rect_visible(paint_rect) {
            let shape = self.track.paint(paint_rect);
            ui.painter().set(self.where_to_put_background, shape);
        }
    }

    /// gets the clip rect when painting a new clip
    fn get_clip_rect_right(&self, state: &mut State, drag_pos: f32) -> Rect {
        let min = pos2(state.drag_start_position, self.rect.min.y);
        let max = pos2(drag_pos + 24.0, self.rect.max.y);
        state.drag_end_position = max.x;
        Rect::from_two_pos(min, max)
    }

    /// gets the clip rect when painting a new clip
    fn get_clip_rect_left(&self, state: &mut State, drag_pos: f32) -> Rect {
        let max = pos2(state.drag_start_position + 24.0, self.rect.min.y);
        let min = pos2(drag_pos, self.rect.max.y);
        state.drag_end_position = max.x;
        Rect::from_two_pos(min, max)
    }
}

/// converts position to clip tick
fn pos_to_clip_tick(pos: f32) -> f32 {
    (pos / 24.0) * 120.0
}

/// converts pixel width to tick
fn pixel_width_to_tick(width: f32) -> f32 {
    (width / 24.0) * 480.0
}
