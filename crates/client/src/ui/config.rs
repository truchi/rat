use super::*;

#[doc(hidden)]
macro_rules! rgb {
    ($r:literal, $g:literal, $b:literal) => {
        x::Color::Rgb {
            r: $r,
            g: $g,
            b: $b,
        }
    };
}

#[derive(Copy, Clone, Debug)]
pub struct Colors {
    pub background: x::Color,
    pub current:    x::Color,
    pub foreground: x::Color,
    pub comment:    x::Color,
    pub cyan:       x::Color,
    pub green:      x::Color,
    pub orange:     x::Color,
    pub pink:       x::Color,
    pub purple:     x::Color,
    pub red:        x::Color,
    pub yellow:     x::Color,
}

const COLORS: Colors = Colors {
    background: rgb!(40, 42, 54),
    current:    rgb!(68, 71, 90),
    foreground: rgb!(248, 248, 242),
    comment:    rgb!(98, 114, 164),
    cyan:       rgb!(139, 233, 253),
    green:      rgb!(80, 250, 123),
    orange:     rgb!(255, 184, 108),
    pink:       rgb!(255, 121, 198),
    purple:     rgb!(189, 147, 249),
    red:        rgb!(255, 85, 85),
    yellow:     rgb!(241, 250, 140),
};

#[derive(Copy, Clone, Debug)]
pub struct Config {
    pub width:  u16,
    pub height: u16,
    pub colors: Colors,
}

impl Config {
    pub fn new() -> Self {
        let (width, height) = x::size().unwrap();

        Self {
            width,
            height,
            colors: COLORS,
        }
    }

    pub fn rect(&self) -> Rect {
        Rect {
            x: 0,
            y: 0,
            w: self.width,
            h: self.height,
        }
    }
}
