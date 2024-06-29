use hexencer_core::{
    data::{ClipId, StorageInterface},
    TrackId,
};
use iced::{
    advanced::{
        layout, mouse,
        overlay::{self, from_children},
        renderer::{self, Quad},
        widget::{self, Tree},
        Layout, Widget,
    },
    alignment, event,
    theme::palette,
    Background, Border, Color, Element, Event, Length, Padding, Rectangle, Shadow, Size, Theme,
    Vector,
};
use tracing::info;

/// handler for on track drop events
type DropHandler<'a, Message> = Option<Box<dyn Fn(ClipId, TrackId, f32) -> Message + 'a>>;

/// A track widget
pub struct Track<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    /// track id this widget represents
    track_id: TrackId,
    /// The padding of the track.
    _padding: Padding,
    /// The width of the track.
    width: Length,
    /// The height of the track.
    height: Length,
    /// The maximum width of the track.
    _max_width: f32,
    /// The maximum height of the track.
    _max_height: f32,
    /// The horizontal alignment of the track.
    _horizontal_alignment: alignment::Horizontal,
    /// The vertical alignment of the track.
    _vertical_alignment: alignment::Vertical,
    /// The style of the track.
    class: Theme::Class<'a>,
    /// Is the track hovered?
    hovered: bool,
    /// The storage interface for the track.
    storage: StorageInterface,
    /// The index of the track.
    _track_index: usize,
    /// The children of the track.
    children: Vec<Element<'a, Message, Theme, Renderer>>,
    /// The dropped clip.
    dropped_clip: Option<ClipId>,
    /// if something was dropped on this track
    on_drop: DropHandler<'a, Message>,
}

impl<'a, Message, Theme, Renderer> Track<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    /// Creates a new [`Track`] with the given [`StorageInterface`], index, track id, children, and dropped clip.
    pub(crate) fn new(
        storage: hexencer_core::data::StorageInterface,
        index: usize,
        track_id: TrackId,
        children: Vec<Element<'a, Message, Theme, Renderer>>,
        dropped_clip: Option<ClipId>,
    ) -> Self {
        Self {
            storage,
            _track_index: index,
            track_id,
            dropped_clip,
            on_drop: None,
            _padding: Padding::ZERO,
            width: Length::Fill,
            height: Length::Fixed(18.0),
            _max_width: f32::INFINITY,
            _max_height: f32::INFINITY,
            _horizontal_alignment: alignment::Horizontal::Center,
            _vertical_alignment: alignment::Vertical::Top,
            class: Theme::default(),
            hovered: false,
            children,
        }
    }

    /// takes a closure for when something is dropped on this track
    pub fn on_drop<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(ClipId, TrackId, f32) -> Message,
    {
        self.on_drop = Some(Box::new(f));
        self
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

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Track<'a, Message, Theme, Renderer>
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
        let bounds = layout.bounds();
        if let Some(cursor_position) = cursor.position_in(bounds) {
            if let Some(on_drop) = &self.on_drop {
                if let Some(clip_id) = self.dropped_clip {
                    info!("clip {} was dropped on {:?}", clip_id, self.track_id);
                    self.dropped_clip = None;

                    let pos = bounds.position();
                    info!("track position: {:?}", pos);
                    info!("cursor position: {:?}", cursor_position.x);
                    let test = cursor_position.x - pos.x;
                    info!("test: {:?}", test);
                    shell.publish(on_drop(clip_id, self.track_id, cursor_position.x));
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
