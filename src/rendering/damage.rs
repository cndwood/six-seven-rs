use crate::rendering::Rect;

#[derive(Default)]
pub struct Damage {
    pub old_six: Option<Rect>,
    pub old_seven: Option<Rect>,
}

impl Damage {
    pub fn six(rect: Rect) -> Self {
        Self {
            old_six: Some(rect),
            old_seven: None,
        }
    }

    pub fn seven(rect: Rect) -> Self {
        Self {
            old_six: None,
            old_seven: Some(rect),
        }
    }

    pub fn both(six: Rect, seven: Rect) -> Self {
        Self {
            old_six: Some(six),
            old_seven: Some(seven),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.old_six.is_none() && self.old_seven.is_none()
    }

    pub fn merge(&mut self, other: Self) {
        self.old_six = self.old_six.or(other.old_six);
        self.old_seven = self.old_seven.or(other.old_seven);
    }
}
