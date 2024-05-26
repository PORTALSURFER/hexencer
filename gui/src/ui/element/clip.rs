use egui::layers::ShapeIdx;
use egui::{emath::*, epaint, Order, Response, Rounding, Sense, Shape, Stroke};
use egui::{Context, Id, InnerResponse, LayerId, Margin, Pos2, Rect, Ui, Vec2};

pub fn clip<R>(
    ctx: &Context,
    ui: &mut Ui,
    track_name: &str,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let id = egui::Id::from(format!("{} clip", track_name));
    let clip = Clip::new(id);
    clip.show(ctx, ui, add_contents)
}

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub pivot_pos: Pos2,
}

#[must_use = "You should call .show()"]
#[derive(Clone, Copy, Debug)]
pub struct Clip {
    pub id: Id,
    active: bool,
    new_pos: Option<Pos2>,
    order: Order,
}

impl Clip {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            active: true,
            new_pos: None,
            order: Order::Middle,
        }
    }

    pub fn show<R>(
        self,
        ctx: &Context,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let mut prepared = self.begin(ctx, ui);
        let inner = add_contents(&mut prepared.content_ui);
        let response = prepared.end(ctx, ui);
        InnerResponse { inner, response }
    }

    fn begin(self, ctx: &Context, ui: &mut Ui) -> Prepared {
        let where_to_put_background = ui.painter().add(Shape::Noop);

        let outer_rect_bounds = ui.available_rect_before_wrap();

        let height = ui.available_height();
        let width = 100.0;
        let size = Vec2::new(width, height);

        let start_pos = ui.max_rect().min;

        let mut state = ctx.memory(|mem| {
            mem.data.get_temp::<State>(self.id).unwrap_or(State {
                pivot_pos: start_pos,
            })
        });

        let rect = Rect::from_min_size(state.pivot_pos, size);
        let mut move_response = {
            let move_response = ui.interact(rect, self.id, Sense::drag());

            if move_response.dragged() {
                state.pivot_pos.x += move_response.drag_delta().x;
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

    fn paint(&self, paint_rect: Rect) -> Shape {
        let shape = Shape::Rect(epaint::RectShape::new(
            paint_rect,
            Rounding::ZERO,
            egui::Color32::from_rgb(120, 140, 50),
            Stroke::NONE,
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
        ctx.memory_mut(|memory| memory.data.insert_temp(self.clip.id, self.state));
        self.paint(ui);
        self.move_response
    }

    fn paint(&self, ui: &Ui) {
        let paint_rect = self.rect;
        if ui.is_rect_visible(paint_rect) {
            let shape = self.clip.paint(paint_rect);
            ui.painter().set(self.where_to_put_background, shape);
        }
    }
}
