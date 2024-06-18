use crate::Message::DragClip;
use hexencer_core::data::{ClipId, StorageInterface};

use iced::{
    advanced::{
        graphics::core::event,
        layout::{self},
        mouse,
        renderer::{self, Quad},
        widget::tree::{self, Tree},
        Widget,
    },
    Background, Border, Color, Element, Event, Length, Point, Rectangle, Shadow, Size,
};
#[derive(Debug, Clone, Copy)]
pub enum DragEvent {
    DragStarted {},
    Dropped { clip_id: ClipId },
    Canceled {},
}
pub struct Clip<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    storage: &'a StorageInterface,
    clip_id: ClipId,
    style: Theme::Style,
    hovered: bool,
    on_drag: Option<Box<dyn Fn(DragEvent) -> Message + 'a>>,
    on_drop: Option<Box<dyn Fn(DragEvent) -> Message + 'a>>,
}

impl<'a, Message, Theme, Renderer> Clip<'a, Message, Theme, Renderer>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
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

    pub fn on_drag<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(DragEvent) -> Message,
    {
        self.on_drag = Some(Box::new(f));
        self
    }
    pub fn on_drop<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(DragEvent) -> Message,
    {
        self.on_drop = Some(Box::new(f));
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Idle,
    Pressed,
    Hovered,
    Dragged { origin: Point, clip_id: ClipId },
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
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let mut out_node =
            layout::Node::new(limits.resolve(self.size().width, self.size().height, Size::ZERO));
        // todo: move to outside of the function when reading the clip, just store in the clip struct on creation
        if let Ok(storage) = self.storage.read() {
            if let Some(clip) = storage.project_manager.find_clip(self.clip_id) {
                let width = Length::Fixed(clip.length.as_f32());
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
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let bounds = layout.bounds();
        let quad = Quad {
            bounds,
            border: Border::default(),
            shadow: Shadow::default(),
        };

        let state = tree.state.downcast_ref::<State>();
        if let Ok(storage) = self.storage.read() {
            if let Some(clip) = storage.project_manager.find_clip(self.clip_id) {
                match state {
                    State::Dragged { origin, clip_id } => {
                        if let Some(cursor_position) = cursor.position() {
                            let bounds = layout.bounds();

                            let translation = cursor_position - Point::new(origin.x, origin.y);
                            renderer.with_translation(translation, |renderer| {
                                renderer.with_layer(bounds, |renderer| {
                                    renderer.fill_quad(
                                        quad,
                                        Background::Color(Color::from_rgb(0.92, 0.24, 0.24)),
                                    );
                                });
                            });
                        }
                    }
                    State::Hovered => {
                        renderer
                            .fill_quad(quad, Background::Color(Color::from_rgb(0.42, 0.74, 0.98)));
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

                if let State::Dragged { origin, clip_id } = *state {
                    let state = tree.state.downcast_mut::<State>();
                    if let Some(on_drop) = &self.on_drop {
                        shell.publish(on_drop(DragEvent::Dropped {
                            clip_id: self.clip_id,
                        }));
                    }
                    *state = State::Idle;
                }

                return event::Status::Captured;
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                let state = tree.state.downcast_mut::<State>();
                if let Some(cursor_position) = cursor.position_over(bounds) {
                    if *state == State::Pressed {
                        *state = State::Dragged {
                            origin: cursor_position,
                            clip_id: self.clip_id,
                        };
                        if let Some(on_drag) = &self.on_drag {
                            shell.publish(on_drag(DragEvent::DragStarted {}));
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

#[derive(Debug, Clone, Copy, Default)]
pub struct Appearance {
    pub background_color: Color,
    pub border_radius: f32,
    pub color: Color,
}

pub trait StyleSheet {
    type Style: Default;

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
