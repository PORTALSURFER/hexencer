use hexencer_core::data::StorageInterface;
use iced::advanced::graphics::core::event;
use iced::advanced::overlay::from_children;
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{self, Operation, Tree, Widget};
use iced::advanced::{layout, Layout, Text};
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
    children: Vec<Element<'a, Message, Theme, Renderer>>,
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
            children: contents,
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
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
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
        let size = limits.resolve(self.width, self.height, Size::ZERO);

        let children = self
            .children
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

        for ((child, tree), child_layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
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
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget_mut().on_event(
                    state,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            })
            .fold(event::Status::Ignored, event::Status::merge)
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        from_children(&mut self.children, tree, layout, renderer, translation)
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child
                    .as_widget()
                    .mouse_interaction(state, layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.children
                .iter()
                .zip(&mut tree.children)
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget()
                        .operate(state, layout, renderer, operation);
                });
        });
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
        let size = layout.bounds().size();

        let bounds = layout.bounds();
        let quad = Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: bounds.y,
                width: size.width,
                height: size.height,
            },
            border: Border::default(),
            shadow: Shadow::default(),
        };

        let appearance = theme.appearance(&self.style);

        if self.hovered {
            renderer.fill_quad(quad, Background::Color(appearance.background_hoovered));
        } else {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border: Border::default(),
                    ..renderer::Quad::default()
                },
                appearance.background.unwrap(),
            );
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
    /// The hovered color
    pub background_hoovered: Color,
}

/// Theme catalog of a ['Track'].
pub trait Catalog {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the [`Appearance`] of a track.
    fn appearance(&self, style: &Self::Style) -> Appearance;
}
