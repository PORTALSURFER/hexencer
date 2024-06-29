use hexencer_core::{data::StorageInterface, DataId};
use iced::{
    advanced::{
        graphics::core::Element,
        layout::{self, Node},
        mouse,
        renderer::{self, Quad},
        widget::{self, Tree},
        Layout, Widget,
    },
    event,
    theme::palette,
    Background, Border, Color, Event, Length, Rectangle, Shadow, Size, Theme,
};
use tracing::info;

pub struct EventTrack<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    id: DataId,
    height: Length,
    width: Length,
    class: Theme::Class<'a>,
    storage: StorageInterface,
    children: Vec<Element<'a, Message, Theme, Renderer>>,
    on_drop: DropHandler<'a, Message>,
    hovered: bool,
    dropped_event: Option<DataId>,
}

impl<'a, Message, Theme, Renderer> EventTrack<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    pub fn new(
        id: DataId,
        storage: StorageInterface,
        index: usize,
        children: Vec<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            id,
            height: Length::Fixed(10.0),
            width: Length::Fill,
            class: Theme::default(),
            storage,
            children,
            on_drop: None,
            hovered: false,
            dropped_event: None,
        }
    }

    /// draws the track background    
    fn draw_background(
        &self,
        _storage: std::sync::RwLockReadGuard<hexencer_core::data::DataLayer>,
        _tree: &widget::Tree,
        theme: &Theme,
        renderer: &mut Renderer,
        layout: Layout,
        _cursor: mouse::Cursor,
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

        let appearance = theme.style(&self.class, Status::Active);

        if self.hovered {
            renderer.fill_quad(quad, Background::Color(appearance.background_hovered));
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

/// handler for on track drop events
type DropHandler<'a, Message> = Option<Box<dyn Fn(DataId, DataId, f32) -> Message + 'a>>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The theme catalog of a [`Button`].
pub trait Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

impl<'a, Message, Theme, Renderer> From<EventTrack<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + Catalog,
    Renderer: 'a + renderer::Renderer,
{
    fn from(track: EventTrack<'a, Message, Theme, Renderer>) -> Self {
        Self::new(track)
    }
}

/// A styling function for a [`Button`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

/// The appearance of a button.
#[derive(Debug, Clone, Copy, Default)]
pub struct Style {
    /// The [`Background`] of the button.
    pub background: Option<Background>,
    /// The text [`Color`] of the button.
    pub text_color: Color,
    /// The [`Background`] of the button.
    pub clip_color: Color,
    /// The hovered color
    pub background_hovered: Color,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for EventTrack<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + Catalog,
    Renderer: 'a + renderer::Renderer,
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
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
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
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
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
        let bounds = layout.bounds();
        if let Some(cursor_position) = cursor.position_in(bounds) {
            if let Some(on_drop) = &self.on_drop {
                if let Some(event_id) = self.dropped_event {
                    info!("event {} was dropped on {:?}", event_id, self.id);
                    self.dropped_event = None;
                    let pos = bounds.position();
                    info!("track position: {:?}", pos);
                    info!("cursor position: {:?}", cursor_position.x);
                    let test = cursor_position.x - pos.x;
                    info!("test: {:?}", test);
                    shell.publish(on_drop(event_id, self.id, cursor_position.x));
                    return event::Status::Captured;
                }
            }
        }

        let child_events = self
            .children
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
            .fold(event::Status::Ignored, event::Status::merge);
        let track_event = match event {
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let bounds = layout.bounds();
                if let Some(_position) = cursor.position_in(bounds) {
                    if !self.hovered {
                        self.hovered = true;
                        return event::Status::Captured;
                    }
                } else if self.hovered {
                    self.hovered = false;
                    return event::Status::Captured;
                }
                event::Status::Ignored
            }
            _ => event::Status::Ignored,
        };
        child_events.merge(track_event)
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
    /// The [`Button`] cannot be pressed.
    Disabled,
}

/// A primary button; denoting a main action.
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.primary.strong);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.primary.weak.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

fn styled(pair: palette::Pair) -> Style {
    Style {
        background: Some(Background::Color(pair.color)),
        text_color: pair.text,
        ..Style::default()
    }
}

/// disabled style
fn disabled(style: Style) -> Style {
    Style {
        background: style
            .background
            .map(|background| background.scale_alpha(0.5)),
        text_color: style.text_color.scale_alpha(0.5),
        ..style
    }
}
