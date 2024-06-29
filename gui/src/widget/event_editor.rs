//! event editor widget

use hexencer_core::data::StorageInterface;
use iced::advanced::layout::{self, Node};
use iced::advanced::renderer::Quad;
use iced::advanced::widget::{tree, Tree, Widget};
use iced::{advanced::renderer, Border, Color, Element, Shadow};
use iced::{Background, Length, Point, Rectangle, Size, Theme, Vector};
use tracing::info;

/// EventEditor widget
pub struct EventEditor<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    /// width of the editor area
    width: Length,
    /// height of the editor area
    height: Length,
    /// inner content of the editor
    content: Element<'a, Message, Theme, Renderer>,
    /// link to the storage
    storage: StorageInterface,
    /// class of the editor
    class: Theme::Class<'a>,
}

impl<'a, Message, Theme, Renderer> EventEditor<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    /// creates a new ['EventEditor'] widget
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        storage: StorageInterface,
    ) -> Self {
        let content = content.into();
        Self {
            width: Length::Fill,
            height: Length::Fill,
            content,
            storage,
            class: Theme::default(),
        }
    }
}

/// state of the ['EventEditor']
#[derive(Debug, Clone, Copy)]
struct State {
    scrolling: bool,
    offset_x: Offset,
    offset_y: Offset,
    scroll_origin: Point,
}

impl State {
    /// creates a new ['State']
    fn new() -> Self {
        Self {
            scrolling: false,
            offset_x: Offset::Relative(0.0),
            offset_y: Offset::Relative(0.0),
            scroll_origin: Point::ORIGIN,
        }
    }

    fn translation(&self, bounds: Rectangle, content_bounds: Rectangle) -> Vector {
        Vector::new(
            self.offset_x
                .translation(bounds.width, content_bounds.width),
            self.offset_y
                .translation(bounds.width, content_bounds.width),
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum Offset {
    Relative(f32),
}
impl Offset {
    fn translation(&self, viewport: f32, content: f32) -> f32 {
        let offset = self.absolute(viewport, content);

        offset
    }

    fn absolute(&self, viewport: f32, content: f32) -> f32 {
        match self {
            Offset::Relative(percentage) => ((content - viewport) * percentage).max(0.0),
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for EventEditor<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> iced::Size<iced::Length> {
        Size::new(self.width, self.height)
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        layout::contained(limits, self.width, self.height, |limits| {
            let child_limits = layout::Limits::new(
                Size::new(limits.min().width, limits.min().height),
                Size::new(f32::INFINITY, f32::INFINITY),
            );

            self.content
                .as_widget()
                .layout(&mut tree.children[0], renderer, &child_limits)
        })
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();

        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap();
        let content_bounds = content_layout.bounds();

        let Some(visible_bounds) = bounds.intersection(viewport) else {
            return;
        };
        info!("bounds: {:?}", bounds);
        info!("content_bounds: {:?}", content_bounds);
        let translation = state.translation(bounds, content_bounds);
        info!("translation: {:?}", translation);

        let quad = Quad {
            bounds: Rectangle::new(layout.bounds().position(), layout.bounds().size()),
            border: Border::default(),
            shadow: Shadow::default(),
        };
        renderer.fill_quad(quad, Background::Color(Color::BLACK));

        renderer.with_layer(visible_bounds, |renderer| {
            renderer.with_translation(Vector::new(-translation.x, -translation.y), |renderer| {
                self.content.as_widget().draw(
                    &tree.children[0],
                    renderer,
                    theme,
                    defaults,
                    content_layout,
                    cursor,
                    &Rectangle {
                        y: bounds.y + translation.y,
                        x: bounds.x + translation.x,
                        ..bounds
                    },
                );
            });
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: iced::Event,
        layout: layout::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        _shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();

        match event {
            iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Right)) => {
                if cursor.is_over(bounds) {
                    state.scrolling = true;
                    if let Some(point) = cursor.position_in(bounds) {
                        state.scroll_origin = point;
                    }
                    info!("scrolling initiated");
                    return iced::event::Status::Captured;
                }
            }
            iced::Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Right)) => {
                if state.scrolling {
                    state.scrolling = false;
                    info!("scrolling exited");
                    return iced::event::Status::Captured;
                }
            }
            iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
                if state.scrolling {
                    // info!("scrolling editor");
                    state.offset_x = Offset::Relative(scroll_percentage_x(
                        state.scroll_origin,
                        position,
                        bounds.size().width,
                    ));
                    state.offset_y = Offset::Relative(scroll_percentage_y(
                        state.scroll_origin,
                        position,
                        bounds.size().width,
                    ));
                    return iced::event::Status::Captured;
                }
            }
            _ => {}
        }

        iced::event::Status::Ignored
    }
}

/// calculate the desired scrolling percentage
fn scroll_percentage_y(scroll_origin: Point, cursor_position: Point, width: f32) -> f32 {
    info!("so{scroll_origin}, cp{cursor_position}, width{width}");
    let percentage = (cursor_position.y - scroll_origin.y) / width;
    info!("scroll percentage: {}", percentage);
    percentage
}

/// calculate the desired scrolling percentage
fn scroll_percentage_x(scroll_origin: Point, cursor_position: Point, width: f32) -> f32 {
    info!("so{scroll_origin}, cp{cursor_position}, width{width}");
    let percentage = (cursor_position.x - scroll_origin.x) / width;
    info!("scroll percentage: {}", percentage);
    percentage
}

/// Catalog of the editor
pub trait Catalog {
    /// class of the catalog
    type Class<'a>;

    /// default class of the catalog
    fn default<'a>() -> Self::Class<'a>;

    /// style of the catalog
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// Status of the widget
#[derive(Debug, Clone, Copy)]
pub enum Status {
    /// idle status, nothing happening
    Idle,
}

/// Style of a widget
#[derive(Debug, Clone, Copy)]
pub struct Style {
    /// The ['Background'] of the widget
    pub background: Option<Color>,
    /// The text color of the widget
    pub text_color: Color,
    /// border color of the widget
    pub border: Border,
    /// shadow color of the widget
    pub shadow: Shadow,
}

/// function type of the style function, used to the style of the widget
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl<'a, Message, Theme, Renderer> From<EventEditor<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + Catalog,
    Renderer: 'a + renderer::Renderer,
{
    fn from(event_editor: EventEditor<'a, Message, Theme, Renderer>) -> Self {
        Self::new(event_editor)
    }
}

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// Default style of the editor
pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    match status {
        Status::Idle => Style {
            background: Some(palette.background.weak.color),
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
            border: Border::default(),
            shadow: Shadow::default(),
        },
    }
}
