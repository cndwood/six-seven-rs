use crossterm::style::Color;

use crate::{
    rendering::{Art, Damage, Layout, Rect},
    strings,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Number {
    Six,
    Seven,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtStyle {
    Blocky,
    Compact,
    Ascii,
}

impl ArtStyle {
    fn next(self) -> Self {
        match self {
            Self::Blocky => Self::Compact,
            Self::Compact => Self::Ascii,
            Self::Ascii => Self::Blocky,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Blocky => "Blocky",
            Self::Compact => "Compact",
            Self::Ascii => "ASCII",
        }
    }

    fn text(self, number: Number) -> &'static str {
        match (number, self) {
            (Number::Six, Self::Blocky) => strings::SIX_BLOCKY,
            (Number::Six, Self::Compact) => strings::SIX_COMPACT,
            (Number::Six, Self::Ascii) => strings::SIX_ASCII,

            (Number::Seven, Self::Blocky) => strings::SEVEN_BLOCKY,
            (Number::Seven, Self::Compact) => strings::SEVEN_COMPACT,
            (Number::Seven, Self::Ascii) => strings::SEVEN_ASCII,
        }
    }
}

const COLORS: &[Color] = &[
    Color::White,
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::Cyan,
    Color::Blue,
    Color::Magenta,
];

pub struct PlacedArt {
    pub art: Art,
    pub rect: Rect,
    pub style: ArtStyle,
    pub color: Color,
    color_index: usize,
    powered_up: bool,
}

impl PlacedArt {
    pub fn new(art: Art, rect: Rect) -> Self {
        Self {
            art,
            rect,
            style: ArtStyle::Blocky,
            color: COLORS[0],
            color_index: 0,
            powered_up: false,
        }
    }

    fn cycle_color(&mut self) {
        self.color_index = (self.color_index + 1) % COLORS.len();
        self.color = COLORS[self.color_index];
    }
}

pub struct Scene {
    pub six: PlacedArt,
    pub seven: PlacedArt,
    pub layout: Layout,
}

impl Scene {
    pub fn new(layout: Layout) -> Self {
        let six_art = Art::new(ArtStyle::Blocky.text(Number::Six));

        let seven_art = Art::new(ArtStyle::Blocky.text(Number::Seven));

        let six_rect = layout.left.center_rect(&six_art);
        let seven_rect = layout.right.center_rect(&seven_art);

        Self {
            six: PlacedArt::new(six_art, six_rect),
            seven: PlacedArt::new(seven_art, seven_rect),
            layout,
        }
    }

    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;

        self.six.rect = self.layout.left.center_rect(&self.six.art);

        self.seven.rect = self.layout.right.center_rect(&self.seven.art);
    }

    pub fn start_pow(&mut self, number: Number) -> Damage {
        let style = {
            let placed = self.get_mut(number);

            if placed.powered_up {
                return Damage::default();
            }

            placed.style
        };

        let art = pow_art(style, number);

        let area = self.area(number);

        if art.width > area.width || art.height > area.height {
            return Damage::default();
        }

        let old_rect = self.replace_art(number, art);
        self.get_mut(number).powered_up = true;

        match number {
            Number::Six => Damage::six(old_rect),
            Number::Seven => Damage::seven(old_rect),
        }
    }

    pub fn end_pow(&mut self, number: Number) -> Damage {
        let style = {
            let placed = self.get_mut(number);

            if !placed.powered_up {
                return Damage::default();
            }

            placed.style
        };

        let old_rect = self.replace_art(number, Art::new(style.text(number)));

        self.get_mut(number).powered_up = false;

        match number {
            Number::Six => Damage::six(old_rect),
            Number::Seven => Damage::seven(old_rect),
        }
    }

    fn area(&self, number: Number) -> Rect {
        match number {
            Number::Six => self.layout.left,
            Number::Seven => self.layout.right,
        }
    }

    fn get_mut(&mut self, number: Number) -> &mut PlacedArt {
        match number {
            Number::Six => &mut self.six,
            Number::Seven => &mut self.seven,
        }
    }

    fn cycle_style(&mut self, number: Number) -> Rect {
        let art = {
            let placed = self.get_mut(number);

            placed.style = placed.style.next();

            if placed.powered_up {
                pow_art(placed.style, number)
            } else {
                Art::new(placed.style.text(number))
            }
        };

        self.replace_art(number, art)
    }

    fn replace_art(&mut self, number: Number, art: Art) -> Rect {
        let area = self.area(number);
        let placed = self.get_mut(number);
        let old_rect = placed.rect;

        let mut new_rect = area.center_rect(&art);
        let old_center_y = old_rect.top().saturating_add(old_rect.height / 2);

        new_rect.position.y = old_center_y.saturating_sub(new_rect.height / 2);

        let maximum_y = area
            .bottom()
            .saturating_sub(new_rect.height.saturating_sub(1));

        new_rect.position.y = if maximum_y < area.top() {
            area.top()
        } else {
            new_rect.position.y.clamp(area.top(), maximum_y)
        };

        placed.art = art;
        placed.rect = new_rect;

        old_rect
    }

    pub fn cycle_colors(&mut self) -> Damage {
        let old_six = self.six.rect;
        let old_seven = self.seven.rect;

        self.six.cycle_color();
        self.seven.cycle_color();

        Damage::both(old_six, old_seven)
    }

    pub fn cycle_styles(&mut self) -> Damage {
        let old_six = self.six.rect;
        let old_seven = self.seven.rect;

        self.cycle_style(Number::Six);
        self.cycle_style(Number::Seven);

        Damage::both(old_six, old_seven)
    }
}

fn scale_2x(text: &str) -> String {
    let mut scaled = String::new();

    for line in text.lines() {
        let expanded: String = line
            .chars()
            .flat_map(|character| [character, character])
            .collect();

        scaled.push_str(&expanded);
        scaled.push('\n');
        scaled.push_str(&expanded);
        scaled.push('\n');
    }

    scaled
}
fn pow_art(style: ArtStyle, number: Number) -> Art {
    match (number, style) {
        (Number::Six, ArtStyle::Ascii) => Art::new(strings::SIX_ASCII_POW),
        (Number::Seven, ArtStyle::Ascii) => Art::new(strings::SEVEN_ASCII_POW),
        _ => Art::new(scale_2x(style.text(number))),
    }
}
