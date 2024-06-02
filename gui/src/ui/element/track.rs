use egui::{
    epaint, layers::ShapeIdx, pos2, Color32, Id, InnerResponse, LayerId, Order, Pos2, Rect,
    Response, Rounding, Sense, Shape, Stroke, Ui, Vec2,
};

use crate::ui::{common::TRACK_HEIGHT, quantize};

/// Track bar onto which clip elements can be placed and moved
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[must_use = "You should call .show()"]
pub struct TrackWidget {
    /// height of this track
    height: f32,
    /// color used to fill te background of the track
    fill: Color32,
}

/// ui state of the track
#[derive(Clone, Default)]
pub struct State {
    /// last known mouse position, used for painting in clips
    last_mouse_position: Pos2,
    /// true if currently drag painting
    started_drag_paint: bool,
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
    pub fn new() -> Self {
        Self::default()
    }

    /// prepares the layout and allocate space for interaction
    fn begin(self, ui: &mut Ui) -> Prepared {
        let where_to_put_background = ui.painter().add(Shape::Noop);
        let outer_rect_bounds = ui.available_rect_before_wrap();

        let available_width = ui.available_width();
        let height = TRACK_HEIGHT;

        let rect = Rect::from_min_size(outer_rect_bounds.min, Vec2::new(available_width, height));

        let content_ui = ui.child_ui(rect, *ui.layout());

        let track_response = self.allocate_space(ui, rect);

        Prepared {
            track: self,
            where_to_put_background,
            content_ui,
            rect,
            track_response,
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
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        self.show_dyn(ui, Box::new(add_contents))
    }

    /// setup and rendering of track
    fn show_dyn<'c, R>(
        self,
        ui: &mut Ui,
        add_contents: Box<dyn FnOnce(&mut Ui) -> R + 'c>,
    ) -> InnerResponse<R> {
        let mut prepared = self.begin(ui);
        let inner = add_contents(&mut prepared.content_ui);
        let response = prepared.end(ui);
        InnerResponse::new(inner, response)
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
}

/// intermediate struct to prepare a track element
pub struct Prepared {
    /// reference to the track widget being prepared
    pub track: TrackWidget,
    /// placeholder where to put the background shape
    where_to_put_background: ShapeIdx,
    /// inner ui
    pub content_ui: Ui,
    /// rect of the entire track
    rect: Rect,
    /// track ui response
    track_response: Response,
}

impl Prepared {
    /// process interaction and paint the element
    pub fn end(self, ui: &mut Ui) -> Response {
        self.paint(ui);
        let mut state = State::load(ui, ui.id());
        if self.track_response.hovered() {
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

        if self.track_response.dragged() {
            self.store_mouse_position(ui, &mut state);
            self.paint_new_clip(ui, &state);
        }
        if self.track_response.drag_stopped() {
            tracing::info!("store clip");
            state.started_drag_paint = false;
        }
        state.store(ui, ui.id());
        self.track_response
    }

    /// sets the current mouse position to given state
    fn store_mouse_position(&self, ui: &mut Ui, state: &mut State) {
        if !state.started_drag_paint {
            if let Some(position) = ui.input(|i| i.pointer.hover_pos()) {
                let quantized_mous_pos_x = quantize(position.x, 24.0, self.rect.min.x);
                state.last_mouse_position = pos2(quantized_mous_pos_x, position.y);
                state.started_drag_paint = true;
            }
        }
    }

    /// paints a new clip based on mouse position
    fn paint_new_clip(&self, ui: &mut Ui, state: &State) {
        let fill_color = Color32::YELLOW;
        if let Some(current_mouse_position) = ui.input(|i| i.pointer.hover_pos()) {
            let quantized_current_mouse_pos_x =
                quantize(current_mouse_position.x, 24.0, self.rect.min.x);
            let rect = if current_mouse_position.x > state.last_mouse_position.x {
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
    fn get_clip_rect_right(&self, state: &State, drag_pos: f32) -> Rect {
        let min = pos2(state.last_mouse_position.x, self.rect.min.y);
        let max = pos2(drag_pos + 24.0, self.rect.max.y);
        Rect::from_two_pos(min, max)
    }

    /// gets the clip rect when painting a new clip
    fn get_clip_rect_left(&self, state: &State, drag_pos: f32) -> Rect {
        let max = pos2(state.last_mouse_position.x + 24.0, self.rect.min.y);
        let min = pos2(drag_pos, self.rect.max.y);
        Rect::from_two_pos(min, max)
    }
}
