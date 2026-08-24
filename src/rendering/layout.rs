use crate::rendering::Rect;

pub struct Layout {
    pub info_left: Rect,
    pub info_right: Rect,
    pub left: Rect,
    pub right: Rect,
}

impl Layout {
    pub fn new(scene: &Rect, show_info: bool) -> Self {
        if !show_info {
            let width = scene.width / 2;
            let (left, right) = scene.split_left_right(width);

            return Self {
                info_left: Rect::new(0, 0, 0, 0),
                info_right: Rect::new(0, 0, 0, 0),
                left,
                right,
            };
        }

        const INFO_HEIGHT: u16 = 8;

        let info_height = INFO_HEIGHT.min(scene.height.saturating_sub(1));

        let (info, content) = scene.split_top_bottom(info_height);

        let (info_left, info_right) = info.split_left_right(info.width / 2);

        let (left, right) = content.split_left_right(content.width / 2);

        Self {
            info_left,
            info_right,
            left,
            right,
        }
    }
}
