use egui::{
    epaint, layers::ShapeIdx, Color32, InnerResponse, Margin, Rect, Response, Rounding, Sense,
    Shape, Stroke, Ui, Vec2,
};

use crate::ui::common::TRACK_HEIGHT;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[must_use = "You should call .show()"]
pub struct Track {
    height: f32,
    fill: Color32,
}

impl Track {
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

        Prepared {
            track: self,
            where_to_put_background,
            content_ui,
            rect,
        }
    }

    pub fn fill(mut self, fill: Color32) -> Self {
        self.fill = fill;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        self.show_dyn(ui, Box::new(add_contents))
    }

    pub fn show_dyn<'c, R>(
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
}

impl Prepared {
    pub fn end(self, ui: &mut Ui) -> Response {
        self.paint(ui);
        self.allocate_space(ui)
    }

    pub fn paint(&self, ui: &Ui) {
        let paint_rect = self.rect;
        if ui.is_rect_visible(paint_rect) {
            let shape = self.track.paint(paint_rect);
            ui.painter().set(self.where_to_put_background, shape);
        }
    }

    fn allocate_space(&self, ui: &mut Ui) -> Response {
        ui.allocate_rect(self.rect, Sense::hover())
    }
}
