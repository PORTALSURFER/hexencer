use hexencer_core::data::{ClipId, StorageInterface};

use iced::{
    advanced::{
        graphics::core::event,
        layout, mouse,
        renderer::{self, Quad},
        widget::tree::{self, Tree},
        Widget,
    },
    Background, Border, Color, Element, Event, Length, Point, Rectangle, Shadow, Size,
};

/// drag events
#[derive(Debug, Clone, Copy)]
pub enum DragEvent {
    /// drag has started
    DragStarted {
        /// position of the grab, relative to the clip
        grab_position: f32,
    },
    /// clip was dropped
    Dropped {
        /// id of the clip which was dropped
        clip_id: ClipId,
    },

    /// drag was canceled
    Canceled {},
}

/// A widget that represents a clip in the gui
pub struct Clip<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
    /// inner element of the clip
    content: Element<'a, Message, Theme, Renderer>,
    /// link to the hexencer storage interface
    storage: &'a StorageInterface,
    /// id of the clip, identifier in the storage
    clip_id: ClipId,
    /// style of the clip
    style: Theme::Style,
    /// is the clip hovered
    hovered: bool,
    /// on drag event
    on_drag: Option<Box<dyn Fn(DragEvent) -> Message + 'a>>,
    /// on drop event
    on_drop: Option<Box<dyn Fn(DragEvent) -> Message + 'a>>,
}

impl<'a, Message, Theme, Renderer> Clip<'a, Message, Theme, Renderer>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
    /// creates a new clip widget
    pub fn new(
        clip_id: ClipId,
        storage: &'a StorageInterface,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        let content = content.into();
        Self {
            clip_id,
            storage,
            style: Default::default(),
            hovered: false,
            content,
            on_drag: None,
            on_drop: None,
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
        F: 'a + Fn(DragEvent) -> Message,
    {
        self.on_drop = Some(Box::new(f));
        self
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
    Hovered,
    /// dragged
    Dragged {
        /// origin of the drag
        origin: Point,
        /// id of the clip being dragged
        clip_id: ClipId,
    },
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Clip<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + StyleSheet,
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
        if let Ok(storage) = self.storage.read() {
            if let Some(_clip) = storage.project_manager.find_clip(self.clip_id) {
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
                    State::Hovered => {
                        renderer
                            .fill_quad(quad, Background::Color(Color::from_rgb(0.92, 0.74, 0.98)));
                    }
                    State::Idle => {
                        renderer
                            .fill_quad(quad, Background::Color(Color::from_rgb(1.00, 0.74, 0.98)));
                    }
                    State::Pressed => {
                        renderer
                            .fill_quad(quad, Background::Color(Color::from_rgb(0.52, 0.84, 1.0)));
                    }
                }
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
                if let State::Dragged { .. } = *state {
                    if let Some(on_drop) = &self.on_drop {
                        if let Some(_cursor_position) = cursor.position() {
                            shell.publish(on_drop(DragEvent::Dropped {
                                clip_id: self.clip_id,
                            }));
                        }
                    }
                    *state = State::Idle;
                    return event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::CursorMoved {
                position: _position,
            }) => {
                let state = tree.state.downcast_mut::<State>();
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
                            }));
                        }
                        return event::Status::Captured;
                    }
                }
            }
            _ => {}
        }
        iced::advanced::graphics::core::event::Status::Ignored
    }
}

/// The appearance of a [`Clip`].
#[derive(Debug, Clone, Copy, Default)]
pub struct Appearance {
    /// The background color of the [`Clip`].
    pub background_color: Color,
    /// The border of the [`Clip`].
    pub border_radius: f32,
    /// The border of the [`Clip`].
    pub color: Color,
}

/// stylesheet of the clip
pub trait StyleSheet {
    /// default style of the clip
    type Style: Default;

    /// appearance of the clip
    fn appearance(&self, style: &Self::Style) -> Appearance;
}

impl<'a, Message, Theme, Renderer> From<Clip<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + StyleSheet,
    Renderer: 'a + renderer::Renderer,
{
    fn from(clip: Clip<'a, Message, Theme, Renderer>) -> Self {
        Self::new(clip)
    }
}
