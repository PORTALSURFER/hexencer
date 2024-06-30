use hexencer_core::{data::StorageInterface, DataId};
use iced::{
    advanced::{
        layout, mouse,
        renderer::{self, Quad},
        widget::{tree, Tree},
        Renderer, Widget,
    },
    event,
    theme::palette,
    Background, Border, Color, Element, Event, Length, Point, Rectangle, Shadow, Size, Theme,
};
use tracing::info;

/// drag events
#[derive(Debug, Clone, Copy)]
pub enum DragEvent {
    /// drag has started
    DragStarted {
        /// position of the grab, relative to the clip
        grab_position: f32,
        /// clip id of the clip being dragged
        id: DataId,
    },
    /// drag was canceled
    Canceled {
        /// clip id of the clip which was dragged but got cancelled
        id: DataId,
    },
}

pub struct EventSegment<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    /// inner element of the clip
    content: Element<'a, Message, Theme, Renderer>,
    storage: StorageInterface,
    position: f32,
    id: DataId,
    class: Theme::Class<'a>,
    selected: bool,
    /// on drag event
    on_drag: Option<Box<dyn Fn(DragEvent) -> Message + 'a>>,
    /// on drop event
    on_drop: Option<Box<dyn Fn(DataId) -> Message + 'a>>,
    /// on selected event
    on_selected: Option<Box<dyn Fn(DataId) -> Message + 'a>>,
}

impl<'a, Message, Theme, Renderer> EventSegment<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    pub fn new(
        storage: StorageInterface,
        id: DataId,
        position: f32,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        let content = content.into();
        Self {
            storage,
            id,
            position,
            class: Theme::default(),
            content,
            selected: false,
            on_drag: None,
            on_drop: None,
            on_selected: None,
        }
    }
}

/// stylesheet of the clip
pub trait Catalog {
    /// default style of the clip
    type Class<'a>;

    /// default class of the clip
    fn default<'a>() -> Self::Class<'a>;

    /// style the clip
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`Button`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}
/// primary colors
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.primary.strong);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => {
            Style { background: Some(Background::Color(palette.primary.base.color)), ..base }
        }
    }
}
/// The state of the [`Clip`].
#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    /// idle
    Idle,
    /// pressed
    Pressed,
    /// hovered
    _Hovered,
    /// dragged
    Dragged {
        /// origin of the drag
        origin: Point,
        /// id of the clip being dragged
        id: DataId,
    },
    /// clip is selected
    Selected,
}
/// The possible status of a [`Button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`Button`] can be pressed.
    Active,
    /// The [`Button`] can be pressed and it is being hovered.
    Hovered,
    /// The [`Button`] is being pressed.
    Pressed,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for EventSegment<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::Idle)
    }

    fn size(&self) -> iced::Size<iced::Length> {
        Size { width: Length::Fixed(120.0), height: Length::Fixed(18.0) }
    }

    fn children(&self) -> Vec<Tree> {
        vec![tree::Tree::new(&self.content)]
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
        let mut out_node =
            layout::Node::new(limits.resolve(self.size().width, self.size().height, Size::ZERO));
        out_node = out_node.move_to(Point::new(self.position, 0.0));
        out_node
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let bounds = layout.bounds();
        let quad = Quad {
            bounds,
            border: Border { color: Color::from_rgb(0.0, 0.0, 0.0), width: 1.0, radius: 2.into() },
            shadow: Shadow::default(),
        };

        let state = tree.state.downcast_ref::<State>();
        match state {
            State::Dragged { origin, id: _ } => {
                if let Some(cursor_position) = cursor.position() {
                    let bounds = layout.bounds();

                    let translation = cursor_position - Point::new(origin.x, origin.y);
                    renderer.with_translation(translation, |renderer| {
                        renderer.with_layer(bounds, |renderer| {
                            renderer.fill_quad(
                                quad,
                                Background::Color(Color::from_rgb(0.82, 0.44, 0.92)),
                            );
                        });
                    });
                }
            }
            State::_Hovered => {
                renderer.fill_quad(quad, Background::Color(Color::from_rgb(0.92, 0.74, 0.98)));
            }
            State::Idle => {
                match self.selected {
                    true => renderer
                        .fill_quad(quad, Background::Color(Color::from_rgb(1.00, 0.24, 0.28))),
                    false => renderer
                        .fill_quad(quad, Background::Color(Color::from_rgb(1.00, 0.74, 0.98))),
                };
            }
            State::Pressed => {
                renderer.fill_quad(quad, Background::Color(Color::from_rgb(0.52, 0.84, 1.0)));
            }
            State::Selected => {
                renderer.fill_quad(quad, Background::Color(Color::from_rgb(0.52, 0.84, 1.0)));
            }
        }
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: layout::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        let bounds = layout.bounds();
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if cursor.is_over(bounds) {
                    let state = tree.state.downcast_mut::<State>();
                    *state = State::Pressed;
                    return event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                let state = tree.state.downcast_mut::<State>();
                match *state {
                    State::Dragged { origin: _, id: _ } => {
                        if let Some(on_drop) = &self.on_drop {
                            info!("dropped clip {}", self.id);
                            if let Some(_cursor_position) = cursor.position() {
                                info!("dropped clip {}", self.id);
                                shell.publish(on_drop(self.id));
                            }
                        }
                        *state = State::Idle;
                        return event::Status::Captured;
                    }
                    State::Pressed => {
                        // if self.selected {
                        *state = State::Idle;
                        // }
                        if let Some(on_selected) = &self.on_selected {
                            println!("selected clip {}", self.id);
                            shell.publish(on_selected(self.id));
                        }
                        return event::Status::Captured;
                    }
                    _ => {
                        return event::Status::Ignored;
                    }
                }
                // if let State::Dragged { .. } = *state {
            }
            // }
            Event::Mouse(mouse::Event::CursorMoved { position: _position }) => {
                let state = tree.state.downcast_mut::<State>();
                if *state == State::Pressed {
                    if let Some(cursor_position) = cursor.position_over(bounds) {
                        let relative_mouse = cursor_position.x - bounds.position().x;
                        if *state == State::Pressed {
                            *state = State::Dragged { origin: cursor_position, id: self.id };
                            if let Some(on_drag) = &self.on_drag {
                                shell.publish(on_drag(DragEvent::DragStarted {
                                    grab_position: relative_mouse,
                                    id: self.id,
                                }));
                            }
                            return event::Status::Captured;
                        }
                    }
                }
            }
            _ => {}
        }
        iced::advanced::graphics::core::event::Status::Ignored
    }
}
/// The style of a track.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The [`Background`] of the button.
    pub background: Option<Background>,
    /// The text [`Color`] of the button.
    pub text_color: Color,
    /// The [`Border`] of the buton.
    pub border: Border,
    /// The [`Shadow`] of the butoon.
    pub shadow: Shadow,
}
// TODO #54 make my own palette
fn styled(pair: palette::Pair) -> Style {
    Style {
        background: Some(Background::Color(pair.color)),
        text_color: pair.text,
        border: Border::rounded(2),
        ..Style::default()
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: None,
            text_color: Color::BLACK,
            border: Border::default(),
            shadow: Shadow::default(),
        }
    }
}

impl<'a, Message, Theme, Renderer> From<EventSegment<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + Catalog,
    Renderer: 'a + renderer::Renderer,
{
    fn from(segment: EventSegment<'a, Message, Theme, Renderer>) -> Self {
        Self::new(segment)
    }
}
