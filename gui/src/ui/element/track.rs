use egui::{
    epaint, layers::ShapeIdx, Color32, InnerResponse, Rect, Response, Rounding, Sense, Shape,
    Stroke, Ui, Vec2,
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

impl TrackWidget {
    /// creates a new 'Track' element
    pub fn new() -> Self {
        Self::default()
    }

    /// begin building the widget
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
    /// finish building the track element
    pub fn end(self, ui: &mut Ui) -> Response {
        self.paint(ui);

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
