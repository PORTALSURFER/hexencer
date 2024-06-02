use egui::{
    epaint, layers::ShapeIdx, pos2, Color32, Id, InnerResponse, Pos2, Rect, Response, Rounding,
    Sense, Shape, Stroke, Ui, Vec2,
};

use crate::ui::common::TRACK_HEIGHT;

/// Track bar onto which clip elements can be placed and moved
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[must_use = "You should call .show()"]
pub struct TrackWidget {
    /// height of this track
    height: f32,
    /// color used to fill te background of the track
    fill: Color32,
}

#[derive(Clone, Default)]
pub struct State {
    last_mouse_position: Pos2,
}

impl State {
    pub fn load(ui: &Ui, id: Id) -> Self {
        ui.memory(|mem| mem.data.get_temp(id)).unwrap_or_default()
    }

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
        ui.allocate_rect(rect, Sense::click_and_drag())
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
        if self.track_response.drag_started() {
            tracing::info!("drag started");
            if let Some(position) = ui.input(|i| i.pointer.hover_pos()) {
                state.last_mouse_position = position;
            }
        }
        tracing::info!("mouse position {:?}", state.last_mouse_position);

        if self.track_response.dragged() {
            let fill_color = Color32::YELLOW;
            if let Some(current_mouse_position) = ui.input(|i| i.pointer.hover_pos()) {
                tracing::info!("painting clip");
                let min = pos2(state.last_mouse_position.x, self.rect.min.y);
                let max = pos2(current_mouse_position.x, self.rect.max.y);
                let rect = Rect::from_two_pos(min, max);
                let shape = epaint::RectShape::new(rect, Rounding::ZERO, fill_color, Stroke::NONE);
                ui.painter().add(shape);
            }
        } else {
            tracing::info!("no mouse pose");
        }

        if self.track_response.drag_stopped() {
            tracing::info!("store clip")
        }
        state.store(ui, ui.id());
        self.track_response
    }

    /// paint the track widget
    pub fn paint(&self, ui: &Ui) {
        let paint_rect = self.rect;
        if ui.is_rect_visible(paint_rect) {
            let shape = self.track.paint(paint_rect);
            ui.painter().set(self.where_to_put_background, shape);
        }
    }
}
