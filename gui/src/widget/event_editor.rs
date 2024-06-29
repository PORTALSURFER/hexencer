use hexencer_core::data::StorageInterface;
use iced::advanced::layout::{self, Node};
use iced::advanced::renderer::Quad;
use iced::advanced::widget::{tree, Tree, Widget};
use iced::{advanced::renderer, Border, Color, Element, Shadow};
use iced::{Background, Length, Rectangle, Size, Theme};

pub struct EventEditor<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    /// width of the editor area
    width: Length,
    /// height of the editor area
    height: Length,
    /// inner content of the editor
    content: Element<'a, Message, Theme, Renderer>,
    /// link to the storage
    storage: StorageInterface,
    /// class of the editor
    class: Theme::Class<'a>,
}

impl<'a, Message, Theme, Renderer> EventEditor<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        storage: StorageInterface,
    ) -> Self {
        let content = content.into();
        Self {
            width: Length::Fill,
            height: Length::Fill,
            content,
            storage,
            class: Theme::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {}
impl State {
    fn new() -> Self {
        Self {}
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for EventEditor<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> iced::Size<iced::Length> {
        Size::new(self.width, self.height)
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
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
        layout::contained(limits, self.width, self.height, |limits| {
            let child_limits = layout::Limits::new(
                Size::new(limits.min().width, limits.min().height),
                Size::new(limits.min().height, limits.max().height),
            );

            self.content
                .as_widget()
                .layout(&mut tree.children[0], renderer, &child_limits)
        })
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        defaults: &renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap();

        let quad = Quad {
            bounds: Rectangle::new(layout.bounds().position(), layout.bounds().size()),
            border: Border::default(),
            shadow: Shadow::default(),
        };
        renderer.fill_quad(quad, Background::Color(Color::BLACK));

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            defaults,
            content_layout,
            cursor,
            &Rectangle {
                x: bounds.x,
                y: bounds.y,
                ..bounds
            },
        );
    }
}

/// Catalog of the editor
pub trait Catalog {
    /// class of the catalog
    type Class<'a>;

    /// default class of the catalog
    fn default<'a>() -> Self::Class<'a>;

    /// style of the catalog
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// Status of the widget
#[derive(Debug, Clone, Copy)]
pub enum Status {
    /// idle status, nothing happening
    Idle,
}

/// Style of a widget
#[derive(Debug, Clone, Copy)]
pub struct Style {
    /// The ['Background'] of the widget
    pub background: Option<Color>,
    /// The text color of the widget
    pub text_color: Color,
    /// border color of the widget
    pub border: Border,
    /// shadow color of the widget
    pub shadow: Shadow,
}

/// function type of the style function, used to the style of the widget
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl<'a, Message, Theme, Renderer> From<EventEditor<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + Catalog,
    Renderer: 'a + renderer::Renderer,
{
    fn from(event_editor: EventEditor<'a, Message, Theme, Renderer>) -> Self {
        Self::new(event_editor)
    }
}

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    match status {
        Status::Idle => Style {
            background: Some(palette.background.weak.color.into()),
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
            border: Border::default(),
            shadow: Shadow::default(),
        },
    }
}
