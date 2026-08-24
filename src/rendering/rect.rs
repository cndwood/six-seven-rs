use crate::rendering::{art::Art, position::Position};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub position: Position,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            position: Position { x, y },
            width,
            height,
        }
    }

    pub fn moved_y(&self, amount: i16) -> Option<Self> {
        let y = self.position.y.checked_add_signed(amount)?;

        Some(Self::new(self.position.x, y, self.width, self.height))
    }

    pub fn contains(&self, other: &Rect) -> bool {
        other.left() >= self.left()
            && other.top() >= self.top()
            && other.right() <= self.right()
            && other.bottom() <= self.bottom()
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn left(&self) -> u16 {
        self.position.x
    }

    pub fn right(&self) -> u16 {
        self.left().saturating_add(self.width.saturating_sub(1))
    }

    pub fn top(&self) -> u16 {
        self.position.y
    }

    pub fn bottom(&self) -> u16 {
        self.top().saturating_add(self.height.saturating_sub(1))
    }

    pub fn split_left_right(&self, left_width: u16) -> (Self, Self) {
        let left_width = left_width.min(self.width);

        let left = Rect::new(self.left(), self.top(), left_width, self.height);

        let right = Rect::new(
            self.left() + left_width,
            self.top(),
            self.width - left_width,
            self.height,
        );

        (left, right)
    }

    pub fn split_top_bottom(&self, top_height: u16) -> (Self, Self) {
        let top_height = top_height.min(self.height);

        let top = Rect::new(self.left(), self.top(), self.width, top_height);

        let bottom = Rect::new(
            self.left(),
            self.top() + top_height,
            self.width,
            self.height - top_height,
        );

        (top, bottom)
    }

    pub fn center_rect(&self, art: &Art) -> Rect {
        Rect::new(
            self.left() + self.width.saturating_sub(art.width) / 2,
            self.top() + self.height.saturating_sub(art.height) / 2,
            art.width,
            art.height,
        )
    }
}
