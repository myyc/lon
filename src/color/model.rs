use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PantoneColor {
    pub name: String,
    pub hex: String,
    pub rgb: Rgb,
    pub hsl: Hsl,
    pub family: ColorFamily,
    pub library: ColorLibrary,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Hsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

impl fmt::Display for Hsl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hsl({:.0}, {:.0}%, {:.0}%)", self.h, self.s, self.l)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ColorFamily {
    #[default]
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
    Pink,
    Brown,
    Neutral,
}

impl ColorFamily {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Red => "Red",
            Self::Orange => "Orange",
            Self::Yellow => "Yellow",
            Self::Green => "Green",
            Self::Cyan => "Cyan",
            Self::Blue => "Blue",
            Self::Purple => "Purple",
            Self::Pink => "Pink",
            Self::Brown => "Brown",
            Self::Neutral => "Neutral",
        }
    }

    pub fn all() -> &'static [ColorFamily] {
        &[
            Self::Red,
            Self::Orange,
            Self::Yellow,
            Self::Green,
            Self::Cyan,
            Self::Blue,
            Self::Purple,
            Self::Pink,
            Self::Brown,
            Self::Neutral,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ColorLibrary {
    #[default]
    FashionHomeTcx,
    SolidCoated,
}

impl ColorLibrary {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::FashionHomeTcx => "Fashion, Home + Interiors (TCX)",
            Self::SolidCoated => "Solid Coated",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::FashionHomeTcx => "TCX",
            Self::SolidCoated => "Solid Coated",
        }
    }

    pub fn all() -> &'static [ColorLibrary] {
        &[Self::FashionHomeTcx, Self::SolidCoated]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortOrder {
    #[default]
    Name,
    Hue,
    Saturation,
    Lightness,
}

impl SortOrder {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Name => "Name",
            Self::Hue => "Hue",
            Self::Saturation => "Saturation",
            Self::Lightness => "Lightness",
        }
    }

    pub fn all() -> &'static [SortOrder] {
        &[Self::Name, Self::Hue, Self::Saturation, Self::Lightness]
    }
}
