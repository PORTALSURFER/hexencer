use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{self, Widget};
use iced::{mouse, Background};
use iced::{Border, Color, Element, Length, Rectangle, Size};

pub struct Track<Theme = crate::Theme>
where
    Theme: StyleSheet,
{
    height: f32,
    style: Theme::Style,
}

impl<Theme> Track<Theme>
where
    Theme: StyleSheet,
{
    pub fn new() -> Self {
        Self {
            height: 18.0,
            style: Default::default(),
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Track<Theme>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(_limits.max().width, self.height))
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let style = theme.appearance(&self.style);
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border::default(),
                ..renderer::Quad::default()
            },
            style.background.unwrap(),
        );

        // paint a clip

        let size = Size {
            width: 50.0,
            height: 18.0,
        };
        let rect = Rectangle::new(layout.bounds().position(), size);
        renderer.fill_quad(
            renderer::Quad {
                bounds: rect,
                border: Border::default(),
                ..renderer::Quad::default()
            },
            style.clip_color,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Track<Theme>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + StyleSheet,
    Renderer: 'a + renderer::Renderer,
{
    fn from(track: Track<Theme>) -> Self {
        Self::new(track)
    }
}

// The appearance of a button.
#[derive(Debug, Clone, Copy, Default)]
pub struct Appearance {
    /// The [`Background`] of the button.
    pub background: Option<Background>,
    /// The text [`Color`] of the button.
    pub text_color: Color,
    /// The [`Background`] of the button.
    pub clip_color: Color,
}
/// A set of rules that dictate the [`Appearance`] of a track.
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the [`Appearance`] of a track.
    fn appearance(&self, style: &Self::Style) -> Appearance;
}
