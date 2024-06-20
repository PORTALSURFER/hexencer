use iced::{
    application,
    widget::{button, container, scrollable, text},
    Background, Border, Color, Shadow,
};

use crate::{
    theme::Theme,
    widgets::{self, clip, track},
};

#[derive(Default, Debug, Clone, Copy)]
pub enum Container {
    #[default]
    Header,
    Sidebar,
    Content,
    Bottom,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Application {
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

#[derive(Default, Clone, Copy)]
pub enum Text {
    #[default]
    Default,
    Ok,
    Danger,
    Commentary,
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

#[derive(Default, Debug, Clone, Copy)]
pub enum Scrollable {
    #[default]
    Description,
    Packages,
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
            Scrollable::Description => from_appearance(Color::from_rgb(0.07, 0.51, 0.67)),
            Scrollable::Packages => from_appearance(self.palette().base.foreground),
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

#[derive(Default, Debug, Clone, Copy)]
pub enum Track {
    #[default]
    Description,
    Packages,
}

impl widgets::track::Catalog for Theme {
    type Style = Track;

    fn appearance(&self, _style: &Self::Style) -> widgets::track::Appearance {
        track::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.04, 0.27, 0.47))),
            text_color: Color::WHITE,
            clip_color: Color::from_rgb(0.34, 0.87, 0.97),
            background_hoovered: Color::from_rgb(0.07, 0.30, 0.50),
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Clip {
    #[default]
    Active,
    Inactive,
    Hovered,
    Dragging,
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    #[default]
    Primary,
    Unavailable,
    SelfUpdate,
    Refresh,
    UninstallPackage,
    RestorePackage,
    NormalPackage,
    SelectedPackage,
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
            Button::Primary | Button::SelfUpdate | Button::Refresh => {
                active_appearance(None, p.bright.primary)
            }
            Button::RestorePackage => active_appearance(None, p.bright.secondary),
            Button::NormalPackage => button::Appearance {
                background: Some(Background::Color(p.base.foreground)),
                text_color: p.bright.surface,
                ..appearance
            },
            Button::SelectedPackage => button::Appearance { ..appearance },
            Button::Unavailable | Button::UninstallPackage => {
                active_appearance(None, p.bright.error)
            }
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
            Button::Primary | Button::SelfUpdate | Button::Refresh => {
                hover_appearance(p.bright.primary, None)
            }
            Button::NormalPackage => hover_appearance(p.normal.primary, Some(p.bright.surface)),
            Button::SelectedPackage => hover_appearance(p.normal.primary, None),
            Button::RestorePackage => hover_appearance(p.bright.secondary, None),
            Button::Unavailable | Button::UninstallPackage => {
                hover_appearance(p.bright.error, None)
            }
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
            Button::RestorePackage => disabled_appearance(p.normal.primary, Some(p.bright.primary)),
            Button::UninstallPackage => disabled_appearance(p.bright.error, None),
            Button::Primary => disabled_appearance(p.bright.primary, Some(p.bright.primary)),
            _ => active,
        }
    }
}
