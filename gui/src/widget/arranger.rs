mod cursor;

use iced::{
    advanced::{
        layout, mouse, renderer,
        widget::{tree, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    event, keyboard,
    widget::container,
    Background, Border, Color, Element, Event, Length, Point, Rectangle, Size, Theme, Vector,
};
mod internals;

/// Alignment of the scrollable's content relative to it's [`Viewport`] in one direction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Alignment {
    /// Content is aligned to the start of the [`Viewport`].
    #[default]
    Start,
    /// Content is aligned to the end of the [`Viewport`]
    _End,
}

/// offset of the [`Viewport`].
#[derive(Debug, Clone, Copy)]
enum Offset {
    Absolute(f32),
    Relative(f32),
}
impl Offset {
    fn absolute(self, viewport: f32, content: f32) -> f32 {
        match self {
            Offset::Absolute(absolute) => absolute.min((content - viewport).max(0.0)),
            Offset::Relative(percentage) => ((content - viewport) * percentage).max(0.0),
        }
    }

    fn translation(self, viewport: f32, content: f32, alignment: Alignment) -> f32 {
        let offset = self.absolute(viewport, content);

        match alignment {
            Alignment::Start => offset,
            Alignment::_End => ((content - viewport).max(0.0) - offset).max(0.0),
        }
    }
}
/// The current [`Viewport`] of the [`Scrollable`].
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    _bounds: Rectangle,
    _content_bounds: Rectangle,
    _offset_x: Offset,
    _offset_y: Offset,
}

/// Arranger widget type, houses tracks and clips
pub struct Arranger<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    width: Length,
    height: Length,
    content: Element<'a, Message, Theme, Renderer>,
    on_scroll: Option<Box<dyn Fn(Viewport) -> Message + 'a>>,
    class: Theme::Class<'a>,
}

impl<'a, Message, Theme, Renderer> Arranger<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    /// Creates a new [`Arranger`] with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        let content = content.into();
        Self {
            width: Length::Fill,
            height: Length::Fill,
            content,
            on_scroll: None,
            class: Theme::default(),
        }
    }

    /// Sets the width of the [`Arranger`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Arranger`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the function that will be called when the [`Viewport`] of the
    pub fn on_scroll<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(Viewport) -> Message,
    {
        self.on_scroll = Some(Box::new(f));
        self
    }

    /// Sets the class of the [`Arranger`].
    pub fn class(mut self, class: Theme::Class<'a>) -> Self {
        self.class = class;
        self
    }
}

/// The state of a [`Arranger`].
#[derive(Debug, Clone, Copy)]
struct State {
    /// The position where the scroll area was last touched.
    scroll_area_touched_at: Option<Point>,
    /// The current offset of the [`Viewport`].
    offset: Offset,
    /// The position where the scroller was last grabbed.
    scroller_grabbed_at: Option<f32>,
    /// The current keyboard modifiers.
    keyboard_modifiers: keyboard::Modifiers,
    /// The last [`Viewport`] that was notified.
    _last_notified: Option<Viewport>,
}

/// The state of a [`Arranger`].
impl State {
    fn new() -> Self {
        State::default()
    }

    fn translation(&self, bounds: Rectangle, content_bounds: Rectangle) -> iced::Vector {
        Vector::new(
            self.offset
                .translation(bounds.width, content_bounds.width, Alignment::Start),
            0.0,
        )
    }

    /// Scrolls the [`Scrollable`] to a relative amount along the x axis.
    ///
    /// `0` represents scrollbar at the beginning, while `1` represents scrollbar at
    /// the end.
    pub fn scroll_x_to(&mut self, percentage: f32, bounds: Rectangle, content_bounds: Rectangle) {
        self.offset = Offset::Relative(percentage.clamp(0.0, 1.0));
        self.unsnap(bounds, content_bounds);
    }

    /// Apply a scrolling offset to the current [`State`], given the bounds of
    /// the [`Scrollable`] and its contents.
    pub fn scroll(&mut self, _delta: Vector<f32>, _bounds: Rectangle, _content_bounds: Rectangle) {
        //     let horizontal_alignment = direction
        //         .horizontal()
        //         .map(|p| p.alignment)
        //         .unwrap_or_default();

        //     let align = |alignment: Alignment, delta: f32| match alignment {
        //         Alignment::Start => delta,
        //         Alignment::End => -delta,
        //     };

        //     let delta = Vector::new(
        //         align(horizontal_alignment, delta.x),
        //         align(vertical_alignment, delta.y),
        //     );

        //     if bounds.height < content_bounds.height {
        //         self.offset_y = Offset::Absolute(
        //             (self.offset_y.absolute(bounds.height, content_bounds.height) - delta.y)
        //                 .clamp(0.0, content_bounds.height - bounds.height),
        //         );
        //     }

        //     if bounds.width < content_bounds.width {
        //         self.offset_x = Offset::Absolute(
        //             (self.offset_x.absolute(bounds.width, content_bounds.width) - delta.x)
        //                 .clamp(0.0, content_bounds.width - bounds.width),
        //         );
    }

    fn unsnap(&self, _bounds: Rectangle, _content_bounds: Rectangle) {}
}

impl Default for State {
    fn default() -> Self {
        Self {
            scroll_area_touched_at: None,
            offset: Offset::Absolute(0.0),
            scroller_grabbed_at: None,
            keyboard_modifiers: keyboard::Modifiers::default(),
            _last_notified: None,
        }
    }
}

/// Properties of a scrollbar within a [`Scrollable`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Properties {
    width: f32,
    margin: f32,
    scroller_width: f32,
    alignment: Alignment,
}

#[derive(Debug)]
/// State of both [`Scrollbar`]s.
struct ScrollbarWrapper {
    inner: Option<internals::Scrollbar>,
}
impl ScrollbarWrapper {
    /// Creates a new [`ScrollbarWrapper`] with the given [`State`] and bounds.
    fn new(state: &State, bounds: Rectangle, content_bounds: Rectangle) -> Self {
        let translation = state.translation(bounds, content_bounds);

        let show_scrollbar = content_bounds.width > bounds.width;

        let test_width = 10.0;

        // create a scrollbar
        let scrollbar = if show_scrollbar {
            // width of the scrollbar
            let width: f32 = 10.0;
            // width of the scrollbar handl
            let scrollbar_handle_width = 10.0;
            let margin = 50.0;

            let total_scrollbar_width = width.max(scrollbar_handle_width) + 2.0 * margin;

            // Total bounds of the scrollbar + margin + scroller width
            let total_scrollbar_bounds = Rectangle {
                x: bounds.x,
                y: bounds.y + bounds.height - total_scrollbar_width,
                width: (bounds.width - test_width).max(0.0),
                height: total_scrollbar_width,
            };

            // Bounds of just the scrollbar
            let scrollbar_bounds = Rectangle {
                x: bounds.x,
                y: bounds.y + bounds.height - total_scrollbar_width / 2.0 - width / 2.0,
                width: (bounds.width - test_width).max(0.0),
                height: width,
            };

            // sizing scalar used to size the scrollbar handle based on the content size
            let ratio = bounds.width / content_bounds.width;
            let mid_width = 2.0;

            // min width for easier grabbing with extra wide content
            let scroller_length = (scrollbar_bounds.width * ratio).max(mid_width);
            let scroller_offset = translation.x * ratio * scrollbar_bounds.width / bounds.width;

            // rebuild scroller bounds with
            let scroll_handle_bounds = Rectangle {
                x: (scrollbar_bounds.x + scroller_offset).max(0.0),
                y: bounds.y + bounds.height
                    - total_scrollbar_width / 2.0
                    - scrollbar_handle_width / 2.0,
                width: scroller_length,
                height: scrollbar_handle_width,
            };

            // create a scrollbar
            Some(internals::Scrollbar {
                total_bounds: total_scrollbar_bounds,
                bounds: scrollbar_bounds,
                scroll_handle: internals::ScrollHandle {
                    bounds: scroll_handle_bounds,
                },
                alignment: Alignment::Start,
            })
        } else {
            None
        };

        Self { inner: scrollbar }
    }

    /// Returns whether the mouse is over the scrollbar or not.
    fn is_mouse_over(&self, cursor: mouse::Cursor) -> bool {
        if let Some(cursor_position) = cursor.position() {
            self.inner
                .as_ref()
                .map(|scrollbar| scrollbar.is_mouse_over(cursor_position))
                .unwrap_or(false)
        } else {
            false
        }
    }

    /// Returns true if the scrollbar is active.
    fn active(&self) -> bool {
        self.inner.is_some()
    }

    /// set the scroll handle to grabbed?
    fn grab_scroll_handle(&self, cursor_position: Point) -> Option<f32> {
        self.inner.and_then(|scrollbar| {
            if scrollbar.total_bounds.contains(cursor_position) {
                Some(
                    if scrollbar.scroll_handle.bounds.contains(cursor_position) {
                        (cursor_position.x - scrollbar.scroll_handle.bounds.x)
                            / scrollbar.scroll_handle.bounds.width
                    } else {
                        0.5
                    },
                )
            } else {
                None
            }
        })
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Arranger<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
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

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
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
                Size::new(f32::INFINITY, limits.max().height),
            );

            self.content
                .as_widget()
                .layout(&mut tree.children[0], renderer, &child_limits)
        })
    }

    // TODO #57 figure out how this works
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
        let state = tree.state.downcast_ref::<State>();

        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap();
        let content_bounds = content_layout.bounds();

        let Some(visible_bounds) = bounds.intersection(viewport) else {
            return;
        };

        let scrollbar = ScrollbarWrapper::new(state, bounds, content_bounds);

        let cursor_over_scrollable = cursor.position_over(bounds);
        let mouse_over_scrollbar = scrollbar.is_mouse_over(cursor);

        let translation = state.translation(bounds, content_bounds);

        let cursor = match cursor_over_scrollable {
            Some(cursor_position) if !(mouse_over_scrollbar) => {
                mouse::Cursor::Available(cursor_position + translation)
            }
            _ => mouse::Cursor::Unavailable,
        };

        let status = if state.scroller_grabbed_at.is_some() {
            Status::Dragged
        } else if cursor_over_scrollable.is_some() {
            Status::Hovered
        } else {
            Status::Normal
        };

        let style = theme.style(&self.class, status);

        container::draw_background(renderer, &style.container, layout.bounds());

        // Draw inner content
        if scrollbar.active() {
            renderer.with_layer(visible_bounds, |renderer| {
                renderer.with_translation(
                    Vector::new(-translation.x, -translation.y),
                    |renderer| {
                        self.content.as_widget().draw(
                            &tree.children[0],
                            renderer,
                            theme,
                            defaults,
                            content_layout,
                            cursor,
                            &Rectangle {
                                y: bounds.y + translation.y,
                                x: bounds.x + translation.x,
                                ..bounds
                            },
                        );
                    },
                );
            });

            let draw_scrollbar =
                |renderer: &mut Renderer, style: Scrollbar, scrollbar: &internals::Scrollbar| {
                    if scrollbar.bounds.width > 0.0
                        && scrollbar.bounds.height > 0.0
                        && (style.background.is_some()
                            || (style.border.color != Color::TRANSPARENT
                                && style.border.width > 0.0))
                    {
                        renderer.fill_quad(
                            renderer::Quad {
                                bounds: scrollbar.bounds,
                                border: style.border,
                                ..renderer::Quad::default()
                            },
                            style
                                .background
                                .unwrap_or(Background::Color(Color::TRANSPARENT)),
                        );
                    }

                    if scrollbar.scroll_handle.bounds.width > 0.0
                        && scrollbar.scroll_handle.bounds.height > 0.0
                        && (style.scroller.color != Color::TRANSPARENT
                            || (style.scroller.border.color != Color::TRANSPARENT
                                && style.scroller.border.width > 0.0))
                    {
                        renderer.fill_quad(
                            renderer::Quad {
                                bounds: scrollbar.scroll_handle.bounds,
                                border: style.scroller.border,
                                ..renderer::Quad::default()
                            },
                            style.scroller.color,
                        );
                    }
                };

            renderer.with_layer(
                Rectangle {
                    width: (visible_bounds.width + 2.0).min(viewport.width),
                    height: (visible_bounds.height + 2.0).min(viewport.height),
                    ..visible_bounds
                },
                |renderer| {
                    if let Some(scrollbar) = scrollbar.inner {
                        draw_scrollbar(renderer, style.scrollbar, &scrollbar);
                    }

                    if let Some(scrollbar) = scrollbar.inner {
                        let background = style.gap.or(style.container.background);

                        if let Some(background) = background {
                            renderer.fill_quad(
                                renderer::Quad {
                                    bounds: Rectangle {
                                        x: 5.0,
                                        y: scrollbar.bounds.y,
                                        width: 5.0,
                                        height: scrollbar.bounds.height,
                                    },
                                    ..renderer::Quad::default()
                                },
                                background,
                            );
                        }
                    }
                },
            );
        } else {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                defaults,
                content_layout,
                cursor,
                &Rectangle {
                    x: bounds.x + translation.x,
                    y: bounds.y + translation.y,
                    ..bounds
                },
            );
        }
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        // get the scrollers state
        let state = tree.state.downcast_mut::<State>();

        // get the bounds of this widget
        let bounds = layout.bounds();

        // create a area for detecting scrolling without click
        let block_width = 100.0;
        let scroll_block_x = bounds.x + bounds.width - block_width;
        let scroll_block_y = bounds.y;
        let right_scroll_block = Rectangle {
            x: scroll_block_x,
            y: scroll_block_y,
            width: block_width,
            height: bounds.height,
        };
        if let Some(_cursor_over_right_scroll_block) = cursor.position_over(right_scroll_block) {
            // info!("mouse over scroll block");
            state.scroller_grabbed_at = Some(0.2);
        }

        // detect if the cursor is insize of the scrollable area
        let cursor_over_scrollable = cursor.position_over(bounds);

        // get the first child in the layout, in this care the root elements.
        let content = layout.children().next().unwrap();
        // collect bounds of the inner content
        let content_bounds = content.bounds();

        // create a scrollbar
        let scrollbars = ScrollbarWrapper::new(state, bounds, content_bounds);

        // check if the mouse is over the scrollbar
        let mouse_over_scrollbar = scrollbars.is_mouse_over(cursor);

        // if the scroller was grabbed
        if let Some(scroller_grabbed_at) = state.scroller_grabbed_at {
            // info!("scroller grabbed at: {:?}", scroller_grabbed_at);
            // and mouse it moved
            if let Event::Mouse(mouse::Event::CursorMoved { .. }) = event {
                // get the cursor position
                let Some(cursor_position) = cursor.position() else {
                    return event::Status::Ignored;
                };

                // if there is a scrollbar
                if let Some(scrollbar) = scrollbars.inner {
                    // info!("scrollbar found");
                    // scroll the scrollbar
                    state.scroll_x_to(
                        scrollbar.scroll_percentage(scroller_grabbed_at, cursor_position),
                        bounds,
                        content_bounds,
                    );
                    let _ = notify_on_scroll(state, &self.on_scroll, bounds, content_bounds, shell);
                }

                return event::Status::Captured;
            }
        } else if mouse_over_scrollbar {
            if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
                let Some(cursor_position) = cursor.position() else {
                    return event::Status::Ignored;
                };

                if let (Some(scroller_grabbed_at), Some(scrollbar)) = (
                    scrollbars.grab_scroll_handle(cursor_position),
                    scrollbars.inner,
                ) {
                    state.scroll_x_to(
                        scrollbar.scroll_percentage(scroller_grabbed_at, cursor_position),
                        bounds,
                        content_bounds,
                    );

                    state.scroller_grabbed_at = Some(scroller_grabbed_at);

                    let _ = notify_on_scroll(state, &self.on_scroll, bounds, content_bounds, shell);

                    return event::Status::Captured;
                }
            }
        }

        let mut event_status = {
            let cursor = match cursor_over_scrollable {
                Some(cursor_position) if !mouse_over_scrollbar => mouse::Cursor::Available(
                    cursor_position + state.translation(bounds, content_bounds),
                ),
                _ => mouse::Cursor::Unavailable,
            };

            let translation = state.translation(bounds, content_bounds);

            self.content.as_widget_mut().on_event(
                &mut tree.children[0],
                event.clone(),
                content,
                cursor,
                renderer,
                clipboard,
                shell,
                &Rectangle {
                    y: bounds.y + translation.y,
                    x: bounds.x + translation.x,
                    ..bounds
                },
            )
        };

        if matches!(
            event,
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
        ) {
            state.scroll_area_touched_at = None;
            state.scroller_grabbed_at = None;

            return event_status;
        }

        if let event::Status::Captured = event_status {
            return event::Status::Captured;
        }

        if let Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) = event {
            state.keyboard_modifiers = modifiers;

            return event::Status::Ignored;
        }

        if let Event::Mouse(mouse::Event::WheelScrolled { delta }) = event {
            if cursor_over_scrollable.is_none() {
                return event::Status::Ignored;
            }

            let delta = match delta {
                mouse::ScrollDelta::Lines { x, y } => {
                    // TODO: #58 Configurable speed/friction (?)
                    let movement = if !cfg!(target_os = "macos") // macOS automatically inverts the axes when Shift is pressed
                        && state.keyboard_modifiers.shift()
                    {
                        Vector::new(y, x)
                    } else {
                        Vector::new(x, y)
                    };

                    movement * 60.0
                }
                mouse::ScrollDelta::Pixels { x, y } => Vector::new(x, y),
            };

            state.scroll(delta, bounds, content_bounds);

            event_status =
                if notify_on_scroll(state, &self.on_scroll, bounds, content_bounds, shell) {
                    event::Status::Captured
                } else {
                    event::Status::Ignored
                };
        }

        event_status
    }
}

fn notify_on_scroll<Message>(
    _state: &mut State,
    _on_scroll: &Option<Box<dyn Fn(Viewport) -> Message + '_>>,
    _bounds: Rectangle,
    _content_bounds: Rectangle,
    _shell: &mut Shell<Message>,
) -> bool {
    false
}

pub trait Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}
pub enum Status {
    Normal,
    Hovered,
    Dragged,
}
/// The appearance of an arrangere
#[derive(Debug, Clone, Copy)]
pub struct Style {
    /// The [`container::Style`] of a scrollable.
    pub container: container::Style,
    /// The horizontal [`Scrollbar`] appearance.
    pub scrollbar: Scrollbar,
    /// The [`Background`] of the gap between a horizontal and vertical scrollbar.
    pub gap: Option<Background>,
}

/// The appearance of the scrollbar of a scrollable.
#[derive(Debug, Clone, Copy)]
pub struct Scrollbar {
    /// The [`Background`] of a scrollbar.
    pub background: Option<Background>,
    /// The [`Border`] of a scrollbar.
    pub border: Border,
    /// The appearance of the [`Scroller`] of a scrollbar.
    pub scroller: Scroller,
}

/// The appearance of the scroller of a scrollable.
#[derive(Debug, Clone, Copy)]
pub struct Scroller {
    /// The [`Color`] of the scroller.
    pub color: Color,
    /// The [`Border`] of the scroller.
    pub border: Border,
}

/// The default style of a [`Scrollable`].
pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let scrollbar = Scrollbar {
        background: Some(palette.background.weak.color.into()),
        border: Border::rounded(2),
        scroller: Scroller {
            color: palette.background.strong.color,
            border: Border::rounded(2),
        },
    };

    match status {
        Status::Hovered => {
            let hovered_scrollbar = Scrollbar {
                scroller: Scroller {
                    color: palette.primary.strong.color,
                    ..scrollbar.scroller
                },
                ..scrollbar
            };

            Style {
                container: container::Style::default(),
                scrollbar: hovered_scrollbar,
                gap: None,
            }
        }
        Status::Normal => Style {
            container: container::Style::default(),
            scrollbar,
            gap: None,
        },
        Status::Dragged => {
            let pressed_scrollbar = Scrollbar {
                scroller: Scroller {
                    color: palette.primary.strong.color,
                    ..scrollbar.scroller
                },
                ..scrollbar
            };

            Style {
                container: container::Style::default(),
                scrollbar: pressed_scrollbar,
                gap: None,
            }
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Arranger<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a + Catalog,
    Renderer: 'a + renderer::Renderer,
{
    fn from(
        text_input: Arranger<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(text_input)
    }
}
