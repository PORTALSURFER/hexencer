//! clip widget

use hexencer_core::data::{ClipId, StorageInterface};
use iced::{
    advanced::{
        layout, mouse,
        renderer::{self, Quad},
        widget::{tree, Tree},
        Widget,
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
        clip_id: ClipId,
    },
    /// drag was canceled
    Canceled {
        /// clip id of the clip which was dragged but got cancelled
        clip_id: ClipId,
    },
}

/// A widget that represents a clip in the gui
pub struct Clip<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    /// inner element of the clip
    content: Element<'a, Message, Theme, Renderer>,
    /// link to the hexencer storage interface
    storage: StorageInterface,
    /// should be selected
    selected: bool,
    /// id of the clip, identifier in the storage
    clip_id: ClipId,
    /// style of the clip
    _class: Theme::Class<'a>,
    /// is the clip hovered
    _hovered: bool,
    /// on drag event
    on_drag: Option<Box<dyn Fn(DragEvent) -> Message + 'a>>,
    /// on drop event
    on_drop: Option<Box<dyn Fn(ClipId) -> Message + 'a>>,
    /// on selected event
    on_selected: Option<Box<dyn Fn(ClipId) -> Message + 'a>>,
}

impl<'a, Message, Theme, Renderer> Clip<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    /// create a new clip
    pub(crate) fn new(
        clip_id: ClipId,
        selected: bool,
        storage: StorageInterface,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        let content = content.into();
        Self {
            clip_id,
            storage,
            _class: Theme::default(),
            _hovered: false, //TODO #55 is this used at all?
            content,
            on_drag: None,
            on_drop: None,
            on_selected: None,
            selected,
        }
    }

    /// bind an on drag event
    pub fn on_drag<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(DragEvent) -> Message,
    {
        self.on_drag = Some(Box::new(f));
        self
    }

    /// bind an on drop event
    pub fn on_drop<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(ClipId) -> Message,
    {
        self.on_drop = Some(Box::new(f));
        self
    }

    /// bind an on selected event
    pub fn on_selected<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(ClipId) -> Message,
    {
        self.on_selected = Some(Box::new(f));
        self
    }
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
        Status::Hovered => Style {
            background: Some(Background::Color(palette.primary.base.color)),
            ..base
        },
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
        clip_id: ClipId,
    },
    /// clip is selected
    Selected,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Clip<'a, Message, Theme, Renderer>
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
        Size {
            width: Length::Fixed(120.0),
            height: Length::Fixed(18.0),
        }
    }

    fn children(&self) -> Vec<Tree> {
        vec![tree::Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn layout(
        &self,
        _tree: &mut iced::advanced::widget::Tree,
        _renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let mut out_node =
            layout::Node::new(limits.resolve(self.size().width, self.size().height, Size::ZERO));
        // todo: move to outside of the function when reading the clip, just store in the clip struct on creation
        if let Ok(storage) = self.storage.read() {
            if let Some(clip) = storage.project_manager.find_clip(self.clip_id) {
                let width = Length::Fixed(clip.duration.as_f32());
                let height = Length::Fixed(18.0);
                let size = limits.resolve(width, height, Size::ZERO);
                let node = layout::Node::new(size);
                let position = Point::new(clip.start.as_f32(), 0.0);
                out_node = node.move_to(position);
            }
        }
        out_node
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let bounds = layout.bounds();
        let quad = Quad {
            bounds,
            border: Border {
                color: Color::from_rgb(0.0, 0.0, 0.0),
                width: 1.0,
                radius: 2.into(),
            },
            shadow: Shadow::default(),
        };

        let state = tree.state.downcast_ref::<State>();
        match state {
            State::Dragged { origin, clip_id: _ } => {
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

        renderer.with_layer(bounds, |_renderer| {});
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
                    State::Dragged {
                        origin: _,
                        clip_id: _,
                    } => {
                        if let Some(on_drop) = &self.on_drop {
                            info!("dropped clip {}", self.clip_id);
                            if let Some(_cursor_position) = cursor.position() {
                                info!("dropped clip {}", self.clip_id);
                                shell.publish(on_drop(self.clip_id));
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
                            println!("selected clip {}", self.clip_id);
                            shell.publish(on_selected(self.clip_id));
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
            Event::Mouse(mouse::Event::CursorMoved {
                position: _position,
            }) => {
                let state = tree.state.downcast_mut::<State>();
                if *state == State::Pressed {
                    if let Some(cursor_position) = cursor.position_over(bounds) {
                        let relative_mouse = cursor_position.x - bounds.position().x;
                        if *state == State::Pressed {
                            *state = State::Dragged {
                                origin: cursor_position,
                                clip_id: self.clip_id,
                            };
                            if let Some(on_drag) = &self.on_drag {
                                shell.publish(on_drag(DragEvent::DragStarted {
                                    grab_position: relative_mouse,
                                    clip_id: self.clip_id,
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

impl<'a, Message, Theme, Renderer> From<Clip<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + Catalog,
    Renderer: 'a + renderer::Renderer,
{
    fn from(clip: Clip<'a, Message, Theme, Renderer>) -> Self {
        Self::new(clip)
    }
}
