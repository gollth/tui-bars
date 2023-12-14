use std::borrow::Cow;

use ratatui::{
    buffer::Buffer,
    layout::{Direction, Rect},
    style::{Color, Style},
    widgets::{Block, Widget},
};

/// A symmetrical gauge for a value
#[derive(Debug, Clone)]
pub struct ValueBar<'a> {
    value: f32,
    label: Cow<'a, str>,
    direction: Direction,
    style: Style,
    block: Option<Block<'a>>,
    range: f32,
}

impl<'a> Default for ValueBar<'a> {
    fn default() -> Self {
        Self {
            value: 0.,
            range: 1.,
            direction: Direction::Horizontal,
            label: "".into(),
            style: Style::default(),
            block: None,
        }
    }
}

impl<'a> ValueBar<'a> {
    /// Set the value how much this bar should be filled. Should be between [`-range`..`range`]
    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    /// The upper and lower bound of the gauge.
    pub fn range(mut self, range: f32) -> Self {
        self.range = range;
        self
    }

    /// Show a label at the zero position of the bar. By default no label is shown.
    /// If width of bar is too small, the label won't be rendered.
    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.label = label.into();
        self
    }

    /// Set that this bar is filling horizontally (default) or vertically
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Surround this bar by a [Block]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Apply a custom style to this bar
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    fn symbol(&self, p: i32) -> &str {
        use Direction::*;
        let negative = self.value < 0.;
        match (p, negative, &self.direction) {
            (..=-8, true, Horizontal) => "█",
            (-7, true, Horizontal) => "🮋",
            (-6, true, Horizontal) => "🮊",
            (-5, true, Horizontal) => "🮉",
            (-4, true, Horizontal) => "▐",
            (-3, true, Horizontal) => "🮈",
            (-2, true, Horizontal) => "🮇",
            (-1, true, Horizontal) => "▕",
            (0 | 1, false, Horizontal) => "▏",
            (2, false, Horizontal) => "▎",
            (3, false, Horizontal) => "▍",
            (4, false, Horizontal) => "▌",
            (5, false, Horizontal) => "▋",
            (6, false, Horizontal) => "▊",
            (7, false, Horizontal) => "▉",
            (8.., false, Horizontal) => "█",
            (..=-8, true, Vertical) => "█",
            (-7, true, Vertical) => "🮆",
            (-6, true, Vertical) => "🮅",
            (-5, true, Vertical) => "🮄",
            (-4, true, Vertical) => "▀",
            (-3, true, Vertical) => "🮃",
            (-2, true, Vertical) => "🮂",
            (-1, true, Vertical) => "▔",
            (0 | 1, false, Vertical) => "▁",
            (2, false, Vertical) => "▂",
            (3, false, Vertical) => "▃",
            (4, false, Vertical) => "▄",
            (5, false, Vertical) => "▅",
            (6, false, Vertical) => "▆",
            (7, false, Vertical) => "▇",
            (8.., false, Vertical) => "█",
            _ => " ",
        }
    }
}

impl<'a> Widget for ValueBar<'a> {
    fn render(mut self, area: Rect, buffer: &mut Buffer) {
        let area = match self.block.take() {
            Some(block) => {
                let inner = block.inner(area);
                block.render(area, buffer);
                inner
            }
            None => area,
        };
        let (length, width, start) = match self.direction {
            Direction::Horizontal => (area.width, area.height, area.left()),
            Direction::Vertical => (area.height, area.width, area.top()),
        };
        if width < 1 {
            // Not enough space to render?
            return;
        }

        let units_per_px = 2. * self.range / length as f32;
        let center_row = area.top() + area.height.saturating_sub(1) / 2;
        let center_col = start + length / 2;
        let label_start =
            (area.left() + area.width / 2).saturating_sub(self.label.len() as u16 / 2);
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let px = units_per_px
                    * match self.direction {
                        Direction::Horizontal => x as f32 - center_col as f32,
                        Direction::Vertical => center_row as f32 - y as f32,
                    };
                // println!("{center_row} - {y} * {units_per_px}");
                let symbol = if px < 0. && self.value < 0. {
                    self.symbol(((self.value - px) / units_per_px * 8. - 8.).round() as i32)
                } else if px >= 0. && self.value >= 0. {
                    self.symbol(((self.value - px) / units_per_px * 8.).round() as i32)
                } else {
                    " "
                };

                let cell = buffer.get_mut(x, y);
                cell.set_style(self.style);
                cell.set_symbol(symbol);

                if y != center_row {
                    continue;
                }
                if area.width < self.label.len() as u16 {
                    // Not enough space to render label
                    continue;
                }
                let idx = x
                    .checked_sub(label_start)
                    .and_then(|x| self.label.chars().nth(x as usize));
                if let Some(c) = idx {
                    cell.set_char(c);
                    cell.set_style(if symbol == "█" {
                        Style::default()
                            .fg(Color::Reset)
                            .bg(self.style.fg.unwrap_or(Color::Reset))
                    } else {
                        self.style
                    });
                }
            }
        }
    }
}
