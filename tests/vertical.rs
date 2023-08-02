use itertools::Itertools;
use test_case::test_case;
use tui::{
    backend::TestBackend,
    buffer::Buffer,
    layout::Direction,
    style::{Color, Style},
    widgets::{Block, Borders},
    Terminal,
};
use tui_bars::ValueBar;

const TERMINAL_HEIGHT: u16 = 10;
const RANGE: f32 = 5.;

fn vertical_value_bar<'a>() -> ValueBar<'a> {
    ValueBar::default().direction(Direction::Vertical)
}

fn assert_renders(widget: ValueBar, expected: Buffer) {
    let backend = TestBackend::new(5, TERMINAL_HEIGHT);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            f.render_widget(widget, f.size());
        })
        .unwrap();
    terminal.backend().assert_buffer(&expected)
}

#[test]
fn vertical_renders_zero_value() {
    assert_renders(
        vertical_value_bar().value(0.).range(RANGE),
        Buffer::with_lines(vec![
            "     ",
            "     ",
            "     ",
            "     ",
            "▁▁▁▁▁",
            "     ",
            "     ",
            "     ",
            "     ",
            "     ",
        ]),
    )
}

#[test]
fn vertical_renders_zero_value_with_label() {
    assert_renders(
        vertical_value_bar()
            .value(0.)
            .range(RANGE)
            .label("0.0".to_owned()),
        Buffer::with_lines(vec![
            "     ",
            "     ",
            "     ",
            "     ",
            "▁0.0▁",
            "     ",
            "     ",
            "     ",
            "     ",
            "     ",
        ]),
    )
}

#[test]
fn vertical_renders_zero_value_but_skips_label_if_label_too_large() {
    assert_renders(
        vertical_value_bar().value(0.).range(RANGE).label("FooBar"),
        Buffer::with_lines(vec![
            "     ",
            "     ",
            "     ",
            "     ",
            "▁▁▁▁▁",
            "     ",
            "     ",
            "     ",
            "     ",
            "     ",
        ]),
    )
}

#[test]
fn vertical_renders_zero_value_with_block() {
    assert_renders(
        vertical_value_bar()
            .value(0.)
            .range(RANGE)
            .block(Block::default().title("X").borders(Borders::ALL)),
        Buffer::with_lines(vec![
            "┌X──┐",
            "│   │",
            "│   │",
            "│   │",
            "│▁▁▁│",
            "│   │",
            "│   │",
            "│   │",
            "│   │",
            "└───┘",
        ]),
    )
}

#[test]
fn vertical_renders_zero_value_with_block_and_label() {
    assert_renders(
        vertical_value_bar()
            .value(0.)
            .range(RANGE)
            .label("0")
            .block(Block::default().title("X").borders(Borders::ALL)),
        Buffer::with_lines(vec![
            "┌X──┐",
            "│   │",
            "│   │",
            "│   │",
            "│▁0▁│",
            "│   │",
            "│   │",
            "│   │",
            "│   │",
            "└───┘",
        ]),
    )
}

#[test_case(1. / 8.,    "    ▁     " ; "positive_one_eighths")]
#[test_case(2. / 8.,    "    ▂     " ; "positive_two_eighths")]
#[test_case(3. / 8.,    "    ▃     " ; "positive_three_eighths")]
#[test_case(4. / 8.,    "    ▄     " ; "positive_four_eighths")]
#[test_case(5. / 8.,    "    ▅     " ; "positive_five_eighths")]
#[test_case(6. / 8.,    "    ▆     " ; "positive_six_eighths")]
#[test_case(7. / 8.,    "    ▇     " ; "positive_seven_eighths")]
#[test_case(8. / 8.,    "   ▁█     " ; "positive_eight_eighths")]
#[test_case(RANGE*0.3,  "   ▄█     " ; "positive_range_30_percent")]
#[test_case(RANGE*0.5,  "  ▄██     " ; "positive_range_50_percent")]
#[test_case(RANGE*0.6,  " ▁███     " ; "positive_range_60_percent")]
#[test_case(RANGE*0.8,  "▁████     " ; "positive_range_80_percent")]
#[test_case(RANGE*1.0,  "█████     " ; "positive_range_full")]
#[test_case(-1. / 8.,   "     ▔    " ; "negative_one_eighths")]
#[test_case(-2. / 8.,   "     🮂    " ; "negative_two_eighths")]
#[test_case(-3. / 8.,   "     🮃    " ; "negative_three_eighths")]
#[test_case(-4. / 8.,   "     ▀    " ; "negative_four_eighths")]
#[test_case(-5. / 8.,   "     🮄    " ; "negative_five_eighths")]
#[test_case(-6. / 8.,   "     🮅    " ; "negative_six_eighths")]
#[test_case(-7. / 8.,   "     🮆    " ; "negative_seven_eighths")]
#[test_case(-8. / 8.,   "     █    " ; "negative_eight_eighths")]
#[test_case(-RANGE*0.3, "     █▀   " ; "negative_range_30_percent")]
#[test_case(-RANGE*0.5, "     ██▀  " ; "negative_range_50_percent")]
#[test_case(-RANGE*0.6, "     ███  " ; "negative_range_60_percent")]
#[test_case(-RANGE*0.8, "     ████ " ; "negative_range_80_percent")]
#[test_case(-RANGE*1.0, "     █████" ; "negative_range_full")]
fn vertical_renders_value(value: f32, col: &str) {
    assert_renders(
        vertical_value_bar().value(value).range(RANGE),
        Buffer::with_lines(col.chars().map(|c| c.to_string().repeat(5)).collect()),
    )
}

#[test_case(1. / 8.,    "    ▁     " ; "positive_one_eighths")]
#[test_case(2. / 8.,    "    ▂     " ; "positive_two_eighths")]
#[test_case(3. / 8.,    "    ▃     " ; "positive_three_eighths")]
#[test_case(4. / 8.,    "    ▄     " ; "positive_four_eighths")]
#[test_case(5. / 8.,    "    ▅     " ; "positive_five_eighths")]
#[test_case(6. / 8.,    "    ▆     " ; "positive_six_eighths")]
#[test_case(7. / 8.,    "    ▇     " ; "positive_seven_eighths")]
#[test_case(8. / 8.,    "   ▁█     " ; "positive_eight_eighths")]
#[test_case(RANGE*0.3,  "   ▄█     " ; "positive_range_30_percent")]
#[test_case(RANGE*0.5,  "  ▄██     " ; "positive_range_50_percent")]
#[test_case(RANGE*0.6,  " ▁███     " ; "positive_range_60_percent")]
#[test_case(RANGE*0.8,  "▁████     " ; "positive_range_80_percent")]
#[test_case(RANGE*1.0,  "█████     " ; "positive_range_full")]
#[test_case(-1. / 8.,   "     ▔    " ; "negative_one_eighths")]
#[test_case(-2. / 8.,   "     🮂    " ; "negative_two_eighths")]
#[test_case(-3. / 8.,   "     🮃    " ; "negative_three_eighths")]
#[test_case(-4. / 8.,   "     ▀    " ; "negative_four_eighths")]
#[test_case(-5. / 8.,   "     🮄    " ; "negative_five_eighths")]
#[test_case(-6. / 8.,   "     🮅    " ; "negative_six_eighths")]
#[test_case(-7. / 8.,   "     🮆    " ; "negative_seven_eighths")]
#[test_case(-8. / 8.,   "     █    " ; "negative_eight_eighths")]
#[test_case(-RANGE*0.3, "     █▀   " ; "negative_range_30_percent")]
#[test_case(-RANGE*0.5, "     ██▀  " ; "negative_range_50_percent")]
#[test_case(-RANGE*0.6, "     ███  " ; "negative_range_60_percent")]
#[test_case(-RANGE*0.8, "     ████ " ; "negative_range_80_percent")]
#[test_case(-RANGE*1.0, "     █████" ; "negative_range_full")]
fn vertical_renders_value_with_label(value: f32, col: &str) {
    let label = "abcdefghij";
    assert_renders(
        vertical_value_bar().value(value).range(RANGE).label(label),
        Buffer::with_lines(col.chars().map(|c| c.to_string().repeat(5)).collect()),
    )
}

#[test_case(Color::Red ; "red")]
#[test_case(Color::Blue ; "blue")]
#[test_case(Color::Yellow ; "yellow")]
#[test_case(Color::Green ; "green")]
fn vertical_renders_with_style_fg(color: Color) {
    let mut expected = Buffer::with_lines(vec![
        "     ",
        "     ",
        "     ",
        "▄▄▄▄▄",
        "█████",
        "     ",
        "     ",
        "     ",
        "     ",
        "     ",
    ]);
    let area = expected.area();
    for (x, y) in (area.left()..area.right()).cartesian_product(area.top()..area.bottom()) {
        expected.get_mut(x, y).set_fg(color);
    }
    assert_renders(
        vertical_value_bar()
            .value(1.5)
            .range(RANGE)
            .style(Style::default().fg(color)),
        expected,
    )
}

#[test_case(Color::Red ; "red")]
#[test_case(Color::Blue ; "blue")]
#[test_case(Color::Yellow ; "yellow")]
#[test_case(Color::Green ; "green")]
fn vertical_renders_with_style_bg(color: Color) {
    let mut expected = Buffer::with_lines(vec![
        "     ",
        "     ",
        "     ",
        "▄▄▄▄▄",
        "█████",
        "     ",
        "     ",
        "     ",
        "     ",
        "     ",
    ]);
    let area = expected.area();
    for (x, y) in (area.left()..area.right()).cartesian_product(area.top()..area.bottom()) {
        expected.get_mut(x, y).set_bg(color);
    }
    assert_renders(
        vertical_value_bar()
            .value(1.5)
            .range(RANGE)
            .style(Style::default().bg(color)),
        expected,
    )
}

#[test_case(Color::Red ; "red")]
#[test_case(Color::Blue ; "blue")]
#[test_case(Color::Yellow ; "yellow")]
#[test_case(Color::Green ; "green")]
fn vertical_renders_with_style_and_label(color: Color) {
    let mut expected = Buffer::with_lines(vec![
        "     ",
        "     ",
        "     ",
        "▄▄▄▄▄",
        "█ABC█",
        "     ",
        "     ",
        "     ",
        "     ",
        "     ",
    ]);
    let area = expected.area();
    for (x, y) in (area.left()..area.right()).cartesian_product(area.top()..area.bottom()) {
        let cell = expected.get_mut(x, y);
        if !cell.symbol.chars().all(char::is_alphabetic) {
            // bar
            cell.set_fg(color);
        } else {
            // label
            if cell.symbol.chars().all(char::is_uppercase) {
                cell.set_bg(color);
            } else {
                cell.set_fg(color);
            };
        }
    }

    assert_renders(
        vertical_value_bar()
            .value(1.5)
            .range(RANGE)
            .label("ABC")
            .style(Style::default().fg(color)),
        expected,
    )
}
