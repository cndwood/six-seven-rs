use crate::{rendering::Damage, scene::Scene};

pub struct InvertedVerticalAnimation {
    direction: i16,
    offset: i16,
    max_offset: i16,
}

impl InvertedVerticalAnimation {
    pub fn new(max_offset: u16) -> Self {
        Self {
            direction: 1,
            offset: 0,
            max_offset: max_offset.min(i16::MAX as u16) as i16,
        }
    }

    pub fn update(&mut self, scene: &mut Scene) -> Option<Damage> {
        if let Some(damage) = self.try_step(scene) {
            return Some(damage);
        }

        self.direction = -self.direction;
        self.try_step(scene)
    }

    fn try_step(&mut self, scene: &mut Scene) -> Option<Damage> {
        let next_offset = self.offset + self.direction;

        if next_offset.abs() > self.max_offset {
            return None;
        }

        let movement = next_offset - self.offset;

        let new_six = scene.six.rect.moved_y(-movement)?;
        let new_seven = scene.seven.rect.moved_y(movement)?;

        if !scene.layout.left.contains(&new_six) || !scene.layout.right.contains(&new_seven) {
            return None;
        }

        let damage = Damage::both(scene.six.rect, scene.seven.rect);

        scene.six.rect = new_six;
        scene.seven.rect = new_seven;
        self.offset = next_offset;

        Some(damage)
    }

    pub fn reset(&mut self) {
        self.direction = 1;
        self.offset = 0;
    }
}
