use hexencer_core::{data::StorageInterface, DataId};
use iced::{
    advanced::{graphics::core::Element, renderer, Widget},
    theme::palette,
    Background, Color, Length, Theme,
};

pub struct EventTrack<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    height: Length,
    width: Length,
    class: Theme::Class<'a>,
    storage: StorageInterface,
    children: Vec<Element<'a, Message, Theme, Renderer>>,
    on_drop: DropHandler<'a, Message>,
}

impl<'a, Message, Theme, Renderer> EventTrack<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    pub fn new(
        storage: StorageInterface,
        index: usize,
        children: Vec<impl Into<Element<'a, Message, Theme, Renderer>>>,
    ) -> Self {
        Self {
            height: todo!(),
            width: todo!(),
            class: todo!(),
            storage,
            children: todo!(),
            on_drop: todo!(),
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
    fn size(&self) -> iced::Size<Length> {
        todo!()
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        todo!()
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
        todo!()
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
