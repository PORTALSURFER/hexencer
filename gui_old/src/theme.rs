use iced::{color, Color};

/// The theme of the application.
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Theme {
    /// Lupin Theme
    #[default]
    Lupin,
    /// dark theme
    Dark,
    /// light theme
    Light,
}

/// The color palette of the application.
#[derive(Debug, Clone, Copy)]
pub struct BaseColors {
    /// The background color of the application.
    pub background: Color,
    /// The foreground color of the application.
    pub foreground: Color,
}

/// normal color
#[derive(Debug, Clone, Copy)]
pub struct NormalColors {
    /// primary color
    pub primary: Color,
    /// secondary color
    pub secondary: Color,
    /// surface color
    pub surface: Color,
    /// error color
    pub error: Color,
}

/// bright color
#[derive(Debug, Clone, Copy)]
pub struct BrightColors {
    /// primary color
    pub primary: Color,

    /// secondary color
    pub secondary: Color,
    /// surface color
    pub surface: Color,
    /// error color
    pub error: Color,
}

/// The color palette of the application.
#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    /// The base colors of the application.
    pub base: BaseColors,
    /// The normal colors of the application.
    pub normal: NormalColors,
    /// The bright colors of the application.
    pub bright: BrightColors,
}

/// The color palette of the application.
impl Theme {
    /// All available themes.
    pub const ALL: [Self; 3] = [Self::Lupin, Self::Dark, Self::Light];

    /// Returns the color palette of the theme.
    pub fn palette(self) -> ColorPalette {
        match self {
            Self::Dark => ColorPalette {
                base: BaseColors {
                    background: color!(0x111111),
                    foreground: color!(0x1C1C1C),
                },
                normal: NormalColors {
                    primary: color!(0x5E4266),
                    secondary: color!(0x386e50),
                    surface: color!(0x828282),
                    error: color!(0x992B2B),
                },
                bright: BrightColors {
                    primary: color!(0xBA84FC),
                    secondary: color!(0x49eb7a),
                    surface: color!(0xE0E0E0),
                    error: color!(0xC13047),
                },
            },
            Self::Light => ColorPalette {
                base: BaseColors {
                    background: color!(0xEEEEEE),
                    foreground: color!(0xE0E0E0),
                },
                normal: NormalColors {
                    primary: color!(0x230F08),
                    secondary: color!(0xF9D659),
                    surface: color!(0x818181),
                    error: color!(0x992B2B),
                },
                bright: BrightColors {
                    primary: color!(0x673AB7),
                    secondary: color!(0x3797A4),
                    surface: color!(0x000000),
                    error: color!(0xC13047),
                },
            },
            Self::Lupin => ColorPalette {
                base: BaseColors {
                    background: color!(0x282a36),
                    foreground: color!(0x353746),
                },
                normal: NormalColors {
                    primary: color!(0x58406F),
                    secondary: color!(0x386e50),
                    surface: color!(0xa2a4a3),
                    error: color!(0xA13034),
                },
                bright: BrightColors {
                    primary: color!(0xbd94f9),
                    secondary: color!(0x49eb7a),
                    surface: color!(0xf4f8f3),
                    error: color!(0xE63E6D),
                },
            },
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dark => "Dark",
                Self::Light => "Light",
                Self::Lupin => "Lupin",
            }
        )
    }
}
