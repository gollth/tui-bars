use std::borrow::Cow;

use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
};

#[derive(Debug)]
pub struct ValueBar<'a> {
    value: f32,
    label: Cow<'a, str>,
    style: Style,
    block: Option<Block<'a>>,
    range: f32,
}

impl<'a> Default for ValueBar<'a> {
    fn default() -> Self {
        Self {
            value: 0.,
            range: 1.,
            label: "".into(),
            style: Style::default(),
            block: None,
        }
    }
}

impl<'a> ValueBar<'a> {
    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn range(mut self, range: f32) -> Self {
        self.range = range;
        self
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.label = label.into();
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
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
        if area.height < 1 {
            // Not enough space to render widget
            return;
        }
        let units_per_px = 2. * self.range / area.width as f32;
        let center_row = area.top() + area.height / 2;
        let center_col = area.left() + area.width / 2;
        let label_start = area.left() + area.width / 2 - self.label.len() as u16 / 2;
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let px = (x as f32 - center_col as f32) * units_per_px;
                let mut symbol = " ";
                if px < 0. && self.value < 0. {
                    symbol = match ((self.value - px) / units_per_px * 8. - 8.).round() as i32 {
                        ..=-8 => "█",
                        -7 => "🮋",
                        -6 => "🮊",
                        -5 => "🮉",
                        -4 => "▐",
                        -3 => "🮈",
                        -2 => "🮇",
                        -1 => "▕",
                        0.. => " ",
                    };
                }
                if px >= 0. && self.value >= 0. {
                    symbol = match ((self.value - px) / units_per_px * 8.).round() as i32 {
                        ..=-1 => " ",
                        0 | 1 => "▏",
                        2 => "▎",
                        3 => "▍",
                        4 => "▌",
                        5 => "▋",
                        6 => "▊",
                        7 => "▉",
                        8.. => "█",
                    };
                }

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
