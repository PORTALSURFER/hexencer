use hexencer_core::data::{ClipId, StorageInterface};
use iced::{
    advanced::{
        layout,
        renderer::{self, Quad},
        Widget,
    },
    Background, Border, Color, Element, Length, Rectangle, Shadow, Size,
};

pub struct Clip<'s, Theme>
where
    Theme: StyleSheet,
{
    storage: &'s StorageInterface,
    clip_id: ClipId,
    style: Theme::Style,
}

impl<'s, Theme> Clip<'s, Theme>
where
    Theme: StyleSheet,
{
    pub fn new(clip_id: ClipId, storage: &'s StorageInterface) -> Self {
        Self {
            clip_id,
            storage,
            style: Default::default(),
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Clip<'_, Theme>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> iced::Size<iced::Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        // todo: move to outside of the function when reading the clip, just store in the clip struct on creation
        let storage = self.storage.read().unwrap();
        let clip = storage.project_manager.find_clip(self.clip_id).unwrap();
        layout::Node::new(Size {
            width: clip.length.as_f32(),
            height: 18.0,
        })
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let storage = self.storage.read().unwrap();
        let clip = storage.project_manager.find_clip(self.clip_id).unwrap();

        let quad = Quad {
            bounds: layout.bounds(),
            border: Border::default(),
            shadow: Shadow::default(),
        };
        renderer.fill_quad(quad, Background::Color(Color::from_rgb(0.42, 0.74, 0.98)));
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

impl<'a, Message, Theme, Renderer> From<Clip<'a, Theme>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + StyleSheet,
    Renderer: 'a + renderer::Renderer,
{
    fn from(clip: Clip<'a, Theme>) -> Self {
        Self::new(clip)
    }
}
