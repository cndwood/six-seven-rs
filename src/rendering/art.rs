pub struct Art {
    text: String,
    pub width: u16,
    pub height: u16,
}

impl Art {
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();

        let width = text
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0) as u16;

        let height = text.lines().count() as u16;

        Self {
            text,
            width,
            height,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
