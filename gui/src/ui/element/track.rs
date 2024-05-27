use egui::{
    epaint, layers::ShapeIdx, Color32, InnerResponse, Rect, Response, Rounding, Sense, Shape,
    Stroke, Ui, Vec2,
};

use crate::ui::common::TRACK_HEIGHT;

/// Track bar onto which clip elements can be placed and moved
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[must_use = "You should call .show()"]
pub struct Track {
    height: f32,
    fill: Color32,
}

impl Track {
    /// creates a new 'Track' element
    pub fn new() -> Self {
        Self::default()
    }

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

    fn paint(&self, paint_rect: Rect) -> Shape {
        let Self { fill, .. } = *self;

        let track_shape = Shape::Rect(epaint::RectShape::new(
            paint_rect,
            Rounding::ZERO,
            fill,
            Stroke::new(0.0, Color32::from_rgb(10, 10, 10)),
        ));
        track_shape
    }
}

pub struct Prepared {
    pub track: Track,
    where_to_put_background: ShapeIdx,
    pub content_ui: Ui,
    rect: Rect,
    track_response: Response,
}

impl Prepared {
    pub fn end(self, ui: &mut Ui) -> Response {
        self.paint(ui);

        self.track_response
    }

    pub fn paint(&self, ui: &Ui) {
        let paint_rect = self.rect;
        if ui.is_rect_visible(paint_rect) {
            let shape = self.track.paint(paint_rect);
            ui.painter().set(self.where_to_put_background, shape);
        }
    }
}
