/// This module defines the style for the GUI components in the application.
/// It provides style implementations for containers, text widgets, scrollables, tracks, clips, and buttons.
/// The styles are defined based on the `Theme` struct, which contains the color palette for the application.
/// Each GUI component has its own style enum, which defines different appearances for the component.
/// The `StyleSheet` trait is implemented for each style enum, providing methods to customize the appearance of the component based on the current theme.
/// The `Theme` struct implements the `application::StyleSheet` trait, which defines the overall appearance of the application.
/// It sets the background color and text color based on the palette defined in the theme.
/// The `container::StyleSheet`, `text::StyleSheet`, `scrollable::StyleSheet`, `track::Catalog`, and `clip::StyleSheet` traits are implemented for the `Theme` struct, providing methods to customize the appearance of containers, text widgets, scrollables, tracks, and clips respectively.
/// The `button::StyleSheet` trait is also implemented for the `Theme` struct, providing methods to customize the appearance of buttons.
/// Each style enum has variants that represent different appearances for the component.
/// The `Default` variant is used as the default appearance for the component.
/// The other variants define specific appearances based on the theme's color palette.
/// The `appearance` method is implemented for each style enum, which returns the appearance of the component based on the current theme and style variant.
use iced::{
    application,
    widget::{button, container, scrollable, text},
    Background, Border, Color, Shadow,
};

use crate::{
    theme::Theme,
    widgets::{self, clip, track},
};

/// The appearance of a container.
#[derive(Default, Debug, Clone, Copy)]
pub enum Container {
    /// The header container
    #[default]
    Header,
    /// The sidebar container
    Sidebar,
    /// The main content container
    Content,
    /// The bottom container
    Bottom,
}
/// The appearance of of the application
#[derive(Default, Debug, Clone, Copy)]
pub enum Application {
    /// The default appearance
    #[default]
    Default,
}

impl application::StyleSheet for Theme {
    type Style = Application;

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.palette().base.background,
            text_color: self.palette().bright.surface,
        }
    }
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Header => container::Appearance {
                background: Some(iced::Background::Color(Color::from_rgb(0.07, 0.51, 0.87))),
                text_color: Some(Color::from_rgb(0.69, 1.0, 0.97)),
                border: Border::default(),
                ..container::Appearance::default()
            },
            Container::Bottom => container::Appearance {
                background: Some(iced::Background::Color(Color::from_rgb(0.07, 0.51, 0.87))),
                text_color: Some(Color::from_rgb(0.69, 1.0, 0.97)),
                border: Border::default(),
                ..container::Appearance::default()
            },
            _ => container::Appearance::default(),
        }
    }
}

/// The appearance of a text widets
#[derive(Default, Clone, Copy)]
pub enum Text {
    /// defalut text appearance
    #[default]
    Default,
    /// ok menus
    Ok,
    /// danger
    Danger,
    /// for any commentary
    Commentary,
    /// Represents a color.
    ///
    /// This struct is used to store and manipulate colors in the application.
    /// It can be used to set the color of various UI elements.
    Color(Color),
}

impl From<Color> for Text {
    fn from(color: Color) -> Self {
        Self::Color(color)
    }
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::Default => text::Appearance::default(),
            Text::Ok => text::Appearance {
                color: Some(self.palette().bright.secondary),
            },
            Text::Danger => text::Appearance {
                color: Some(self.palette().bright.error),
            },
            Text::Commentary => text::Appearance {
                color: Some(self.palette().normal.surface),
            },
            Text::Color(c) => text::Appearance { color: Some(c) },
        }
    }
}

/// The appearance of a scrollable widget.
#[derive(Default, Debug, Clone, Copy)]
pub enum Scrollable {
    /// The default scrollable
    #[default]
    Default,
}

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, style: &Self::Style) -> iced::widget::scrollable::Appearance {
        let from_appearance = |c: Color| scrollable::Appearance {
            container: container::Appearance {
                background: Some(Background::Color(c)),
                border: Border::default(),
                text_color: Some(Color::WHITE),
                shadow: Shadow::default(),
            },
            scrollbar: scrollable::Scrollbar {
                background: Some(Background::Color(c)),
                border: Border::default(),
                scroller: scrollable::Scroller {
                    color: Color::WHITE,
                    border: Border::default(),
                },
            },
            gap: Some(Background::Color(c)),
        };

        match style {
            Scrollable::Default => from_appearance(Color::from_rgb(0.07, 0.51, 0.67)),
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        _mouse_over_scrollbar: bool,
    ) -> iced::widget::scrollable::Appearance {
        self.active(style)
    }

    fn dragging(&self, style: &Self::Style) -> iced::widget::scrollable::Appearance {
        self.active(style)
    }
}

/// The appearance of a track widget.
#[derive(Default, Debug, Clone, Copy)]
pub enum Track {
    /// The default track appearance
    #[default]
    Description,
}

impl widgets::track::Catalog for Theme {
    type Style = Track;

    fn appearance(&self, _style: &Self::Style) -> widgets::track::Appearance {
        track::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.04, 0.27, 0.47))),
            text_color: Color::WHITE,
            clip_color: Color::from_rgb(0.34, 0.87, 0.97),
            background_hovered: Color::from_rgb(0.07, 0.30, 0.50),
        }
    }
}

/// The appearance of a clip widget.
#[derive(Default, Debug, Clone, Copy)]
pub enum Clip {
    /// Active Clip
    #[default]
    Active,
    /// Inactive Clip
    Inactive,
    /// when the clip is hovered
    Hovered,
    /// when the clip is being dragged
    Dragging,
    /// when the clip is selected
    Selected,
}

impl clip::StyleSheet for Theme {
    type Style = Clip;

    fn appearance(&self, style: &Self::Style) -> clip::Appearance {
        match style {
            Clip::Active => clip::Appearance {
                background_color: Color::from_rgb(0.04, 0.27, 0.47),
                color: Color::from_rgb(0.34, 0.87, 0.97),
                border_radius: 0.0,
            },
            Clip::Hovered => clip::Appearance {
                background_color: Color::from_rgb(0.07, 0.30, 0.50),
                color: Color::from_rgb(0.34, 0.87, 0.97),
                border_radius: 0.0,
            },
            Clip::Dragging => clip::Appearance {
                background_color: Color::from_rgb(0.04, 0.27, 0.47),
                color: Color::from_rgb(0.34, 0.87, 0.97),
                border_radius: 0.0,
            },
            Clip::Selected => clip::Appearance {
                background_color: Color::from_rgb(0.04, 0.27, 0.47),
                color: Color::from_rgb(0.34, 0.87, 0.97),
                border_radius: 0.0,
            },
            Clip::Inactive => clip::Appearance {
                background_color: Color::from_rgb(0.04, 0.27, 0.47),
                color: Color::from_rgb(0.34, 0.87, 0.97),
                border_radius: 0.0,
            },
        }
    }
}

/// The appearance of a button.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    /// The primary button
    #[default]
    Default,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let p = self.palette();

        let appearance = button::Appearance {
            ..Default::default()
        };

        let active_appearance = |bg: Option<Color>, _mc| button::Appearance {
            background: Some(Background::Color(bg.unwrap_or(p.base.foreground))),
            ..appearance
        };

        match style {
            Button::Default => active_appearance(None, p.bright.primary),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        let p = self.palette();

        let hover_appearance = |bg, tc: Option<Color>| button::Appearance {
            background: Some(Background::Color(Color { a: 0.25, ..bg })),
            text_color: tc.unwrap_or(bg),
            ..active
        };

        match style {
            Button::Default => hover_appearance(p.bright.primary, None),
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        let p = self.palette();

        let disabled_appearance = |bg, tc: Option<Color>| button::Appearance {
            background: Some(Background::Color(Color { a: 0.05, ..bg })),
            text_color: Color {
                a: 0.50,
                ..tc.unwrap_or(bg)
            },
            ..active
        };

        match style {
            Button::Default => disabled_appearance(p.bright.primary, Some(p.bright.primary)),
        }
    }
}
