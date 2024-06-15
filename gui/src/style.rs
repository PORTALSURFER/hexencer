use iced::{
    application,
    widget::{
        container,
        scrollable::{self},
        text,
    },
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

impl widgets::track::StyleSheet for Theme {
    type Style = Track;

    fn appearance(&self, _style: &Self::Style) -> widgets::track::Appearance {
        track::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.04, 0.27, 0.47))),
            text_color: Color::WHITE,
            clip_color: Color::from_rgb(0.34, 0.87, 0.97),
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

impl widgets::clip::StyleSheet for Theme {
    type Style = Clip;

    fn appearance(&self, _style: &Self::Style) -> widgets::clip::Appearance {
        clip::Appearance {
            background_color: Color::from_rgb(0.04, 0.27, 0.47),
            color: Color::from_rgb(0.34, 0.87, 0.97),
            border_radius: 0.0,
        }
    }
}
