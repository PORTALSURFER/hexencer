use hexencer_core::data::StorageInterface;
use iced::advanced::graphics::core::{event, Element};
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{self, Widget};
use iced::{mouse, Background, Event, Point, Shadow};
use iced::{Border, Color, Element, Length, Rectangle, Size};

pub struct Track<'s, Theme = crate::Theme>
where
    Theme: StyleSheet,
{
    height: f32,
    style: Theme::Style,
    hovered: bool,
    storage: &'s StorageInterface,
    track_index: usize,
}

impl<'s, Theme> Track<'s, Theme>
where
    Theme: StyleSheet,
{
    pub fn new(storage: &'s StorageInterface, track_index: usize) -> Self {
        Self {
            height: 18.0,
            style: Default::default(),
            hovered: false,
            storage,
            track_index,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    is_dragging: bool,
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Track<'_, Theme>
where
    Theme: StyleSheet,
    Renderer: renderer::Renderer,
{
    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State { is_dragging: false })
    }
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
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let storage = self.storage.read().unwrap();
        tracing::info!(
            "First track id: {}",
            storage
                .project_manager
                .tracks
                .iter()
                .next()
                .expect("no first track in storage")
                .id
        );

        let state = tree.state.downcast_ref::<State>();
        let appearance = theme.appearance(&self.style);
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border::default(),
                ..renderer::Quad::default()
            },
            appearance.background.unwrap(),
        );

        // paint a clip
        let size = Size {
            width: 50.0,
            height: 18.0,
        };

        let bounds = layout.bounds();
        // let top_left = match state.is_dragging {
        //     false => Point::new(0.0, bounds_y),
        //     true => Point::new(cursor.position().x, bounds_y),
        // };
        let top_left = Point::new(bounds.x, bounds.y);
        let rect = Rectangle::new(top_left, size);

        let bounds = Rectangle {
            x: layout.bounds().x,
            y: layout.bounds().y,
            width: size.width,
            height: size.height,
        };

        if state.is_dragging {
            if let Some(cursor_position) = cursor.position() {
                let translation = Point::new(cursor_position.x, top_left.y) - top_left;

                let quad = Quad {
                    bounds: bounds,
                    border: Border::default(),
                    shadow: Shadow::default(),
                };
                renderer.with_translation(translation, |renderer| {
                    renderer.fill_quad(quad, Background::Color(Color::from_rgb(0.42, 0.74, 0.98)));
                });
            }
        }

        let clips = storage
            .project_manager
            .tracks
            .get_clips(self.track_index)
            .unwrap();
        for (clip_id, clip) in clips.iter() {
            let clip_widget = crate::widgets::clip::Clip::new(*clip_id, &self.storage);
            clip_widget.draw(tree, renderer, theme, style, layout, cursor, _viewport);
        }
    }

    fn on_event(
        &mut self,
        tree: &mut widget::Tree,
        event: iced::Event,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        _shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        let state = tree.state.downcast_mut::<State>();
        let is_dragging = state.is_dragging;

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {}
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {}
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if cursor.is_over(layout.bounds()) {
                    tracing::info!("Cursor moved over the track");
                    state.is_dragging = true;
                } else {
                    state.is_dragging = false;
                }
                return event::Status::Captured;
            }
            _ => {}
        }
        event::Status::Ignored
    }
}

impl<'a, Message, Theme, Renderer> From<Track<'a, Theme>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + StyleSheet,
    Renderer: 'a + renderer::Renderer,
{
    fn from(track: Track<'a, Theme>) -> Self {
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
