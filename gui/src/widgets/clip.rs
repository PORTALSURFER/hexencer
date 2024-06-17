use hexencer_core::data::{ClipId, StorageInterface};
use iced::{
    advanced::{
        graphics::core::event,
        layout::{self, Node},
        mouse,
        renderer::{self, Quad},
        widget::tree::{self, Tree},
        Widget,
    },
    Background, Border, Color, Element, Event, Length, Point, Rectangle, Shadow, Size,
};
use tracing::info;

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
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    is_pressed: bool,
    is_hovered: bool,
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
        tree::State::new(State::default())
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
        let is_mouse_over = cursor.is_over(bounds);
        renderer.fill_quad(quad, Background::Color(Color::from_rgb(1.00, 0.74, 0.98)));
        if let Ok(storage) = self.storage.read() {
            if let Some(clip) = storage.project_manager.find_clip(self.clip_id) {
                if is_mouse_over {
                    let state = tree.state.downcast_ref::<State>();

                    if state.is_pressed {
                        renderer
                            .fill_quad(quad, Background::Color(Color::from_rgb(0.52, 0.84, 1.0)));
                    } else {
                        renderer
                            .fill_quad(quad, Background::Color(Color::from_rgb(0.42, 0.74, 0.98)));
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
        _shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let bounds = layout.bounds();

                if cursor.is_over(bounds) {
                    let state = tree.state.downcast_mut::<State>();
                    state.is_pressed = true;
                    return event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                let state = tree.state.downcast_mut::<State>();
                state.is_pressed = false;
                return event::Status::Captured;
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
