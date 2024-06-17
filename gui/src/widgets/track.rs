use hexencer_core::data::StorageInterface;
use iced::advanced::graphics::core::event;
use iced::advanced::layout::{self, Layout};
use iced::advanced::overlay::from_children;
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::Text;
use iced::widget::text;
use iced::{
    alignment, mouse, overlay, Alignment, Background, Event, Padding, Point, Shadow, Vector,
};
use iced::{Border, Color, Element, Length, Rectangle, Size};
use tracing::info;

/// The identifier of a [`Container`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(widget::Id);
pub struct Track<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    id: Option<Id>,
    padding: Padding,
    width: Length,
    height: Length,
    max_width: f32,
    max_height: f32,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    style: Theme::Style,
    hovered: bool,
    storage: &'a StorageInterface,
    track_index: usize,
    contents: Vec<Element<'a, Message, Theme, Renderer>>,
}

impl<'s, Message, Theme, Renderer> Track<'s, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    pub fn new(
        storage: &'s StorageInterface,
        track_index: usize,
        contents: Vec<Element<'s, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fixed(18.0),
            style: Default::default(),
            hovered: false,
            storage,
            track_index,
            contents,
            id: None,
            padding: Padding::ZERO,
            max_width: f32::INFINITY,
            max_height: f32::INFINITY,
            horizontal_alignment: alignment::Horizontal::Center,
            vertical_alignment: alignment::Vertical::Top,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    is_dragging: bool,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Track<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        self.contents.iter().map(Tree::new).collect()
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
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        info!("track layout");
        let size = limits.resolve(self.width, self.height, Size::ZERO);

        let children = self
            .contents
            .iter()
            .zip(&mut tree.children)
            .map(|(child, child_tree)| child.as_widget().layout(child_tree, renderer, limits))
            .collect();

        layout::Node::with_children(size, children)
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let storage = self.storage.read().unwrap();
        self.draw_background(storage, tree, theme, renderer, layout, cursor);
        // let theme_style = theme.style(&self.style);

        // self.contents.as_widget().draw(
        //     tree,
        //     renderer,
        //     theme,
        //     &renderer::Style {
        //         text_color: style.text_color,
        //     },
        //     layout.children().next().expect("no children"),
        //     cursor,
        //     viewport,
        // );
        info!("drawing track");
        for (child, child_layout) in self.contents.iter().zip(layout.children()) {
            info!("drawing child");
            child
                .as_widget()
                .draw(tree, renderer, theme, style, child_layout, cursor, viewport);
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
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {}
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {}
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if cursor.is_over(layout.bounds()) {
                    self.hovered = true;
                } else {
                    self.hovered = false;
                }
                return event::Status::Captured;
            }
            _ => {}
        }
        event::Status::Ignored
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        from_children(&mut self.contents, tree, layout, renderer, translation)
    }
}

impl<'a, Message, Theme, Renderer> Track<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    fn draw_background(
        &self,
        storage: std::sync::RwLockReadGuard<hexencer_core::data::DataLayer>,
        tree: &widget::Tree,
        theme: &Theme,
        renderer: &mut Renderer,
        layout: Layout,
        cursor: mouse::Cursor,
    ) {
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

        if self.hovered {
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
    }
}

impl<'a, Message, Theme, Renderer> From<Track<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + Catalog,
    Renderer: 'a + renderer::Renderer,
{
    fn from(track: Track<'a, Message, Theme, Renderer>) -> Self {
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

/// Theme catalog of a ['Track'].
pub trait Catalog {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the [`Appearance`] of a track.
    fn appearance(&self, style: &Self::Style) -> Appearance;
}
