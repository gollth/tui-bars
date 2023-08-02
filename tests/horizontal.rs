use itertools::Itertools;
use test_case::test_case;
use tui::{
    backend::TestBackend,
    buffer::Buffer,
    style::{Color, Style},
    widgets::{Block, Borders},
    Terminal,
};
use tui_bars::ValueBar;

const TERMINAL_WIDTH: u16 = 10;
const RANGE: f32 = 5.;

fn assert_renders(widget: ValueBar, expected: Buffer) {
    let backend = TestBackend::new(TERMINAL_WIDTH, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            f.render_widget(widget, f.size());
        })
        .unwrap();
    terminal.backend().assert_buffer(&expected)
}

#[test]
fn horizontal_renders_zero_value() {
    assert_renders(
        ValueBar::default().value(0.).range(RANGE),
        Buffer::with_lines(vec![
            "     ▏    ",
            "     ▏    ",
            "     ▏    ",
            "     ▏    ",
            "     ▏    ",
        ]),
    )
}

#[test]
fn horizontal_renders_zero_value_with_label() {
    assert_renders(
        ValueBar::default()
            .value(0.)
            .range(RANGE)
            .label("0.0".to_owned()),
        Buffer::with_lines(vec![
            "     ▏    ",
            "     ▏    ",
            "    0.0   ",
            "     ▏    ",
            "     ▏    ",
        ]),
    )
}

#[test]
fn horizontal_renders_zero_value_but_skips_label_if_label_too_large() {
    assert_renders(
        ValueBar::default()
            .value(0.)
            .range(RANGE)
            .label("FooBarBaz!!"),
        Buffer::with_lines(vec![
            "     ▏    ",
            "     ▏    ",
            "     ▏    ",
            "     ▏    ",
            "     ▏    ",
        ]),
    )
}

#[test]
fn horizontal_renders_zero_value_with_block() {
    assert_renders(
        ValueBar::default()
            .value(0.)
            .range(RANGE)
            .block(Block::default().title("Value").borders(Borders::ALL)),
        Buffer::with_lines(vec![
            "┌Value───┐",
            "│    ▏   │",
            "│    ▏   │",
            "│    ▏   │",
            "└────────┘",
        ]),
    )
}

#[test]
fn horizontal_renders_zero_value_with_block_and_label() {
    assert_renders(
        ValueBar::default()
            .value(0.)
            .range(RANGE)
            .label("0.0")
            .block(Block::default().title("Value").borders(Borders::ALL)),
        Buffer::with_lines(vec![
            "┌Value───┐",
            "│    ▏   │",
            "│   0.0  │",
            "│    ▏   │",
            "└────────┘",
        ]),
    )
}

#[test_case(1. / 8.,    "     ▏    " ; "positive_one_eighths")]
#[test_case(2. / 8.,    "     ▎    " ; "positive_two_eighths")]
#[test_case(3. / 8.,    "     ▍    " ; "positive_three_eighths")]
#[test_case(4. / 8.,    "     ▌    " ; "positive_four_eighths")]
#[test_case(5. / 8.,    "     ▋    " ; "positive_five_eighths")]
#[test_case(6. / 8.,    "     ▊    " ; "positive_six_eighths")]
#[test_case(7. / 8.,    "     ▉    " ; "positive_seven_eighths")]
#[test_case(8. / 8.,    "     █▏   " ; "positive_eight_eighths")]
#[test_case(RANGE*0.3,  "     █▌   " ; "positive_range_30_percent")]
#[test_case(RANGE*0.5,  "     ██▌  " ; "positive_range_50_percent")]
#[test_case(RANGE*0.6,  "     ███▏ " ; "positive_range_60_percent")]
#[test_case(RANGE*0.8,  "     ████▏" ; "positive_range_80_percent")]
#[test_case(RANGE*1.0,  "     █████" ; "positive_range_full")]
#[test_case(-1. / 8.,   "    ▕     " ; "negative_one_eighths")]
#[test_case(-2. / 8.,   "    🮇     " ; "negative_two_eighths")]
#[test_case(-3. / 8.,   "    🮈     " ; "negative_three_eighths")]
#[test_case(-4. / 8.,   "    ▐     " ; "negative_four_eighths")]
#[test_case(-5. / 8.,   "    🮉     " ; "negative_five_eighths")]
#[test_case(-6. / 8.,   "    🮊     " ; "negative_six_eighths")]
#[test_case(-7. / 8.,   "    🮋     " ; "negative_seven_eighths")]
#[test_case(-8. / 8.,   "    █     " ; "negative_eight_eighths")]
#[test_case(-RANGE*0.3, "   ▐█     " ; "negative_range_30_percent")]
#[test_case(-RANGE*0.5, "  ▐██     " ; "negative_range_50_percent")]
#[test_case(-RANGE*0.6, "  ███     " ; "negative_range_60_percent")]
#[test_case(-RANGE*0.8, " ████     " ; "negative_range_80_percent")]
#[test_case(-RANGE*1.0, "█████     " ; "negative_range_full")]
fn horizontal_renders_value(value: f32, line: &str) {
    assert_renders(
        ValueBar::default().value(value).range(RANGE),
        Buffer::with_lines(vec![line, line, line, line, line]),
    )
}

#[test_case(1. / 8.,    "     ▏    " ; "positive_one_eighths")]
#[test_case(2. / 8.,    "     ▎    " ; "positive_two_eighths")]
#[test_case(3. / 8.,    "     ▍    " ; "positive_three_eighths")]
#[test_case(4. / 8.,    "     ▌    " ; "positive_four_eighths")]
#[test_case(5. / 8.,    "     ▋    " ; "positive_five_eighths")]
#[test_case(6. / 8.,    "     ▊    " ; "positive_six_eighths")]
#[test_case(7. / 8.,    "     ▉    " ; "positive_seven_eighths")]
#[test_case(8. / 8.,    "     █▏   " ; "positive_eight_eighths")]
#[test_case(RANGE*0.3,  "     █▌   " ; "positive_range_30_percent")]
#[test_case(RANGE*0.5,  "     ██▌  " ; "positive_range_50_percent")]
#[test_case(RANGE*0.6,  "     ███▏ " ; "positive_range_60_percent")]
#[test_case(RANGE*0.8,  "     ████▏" ; "positive_range_80_percent")]
#[test_case(RANGE*1.0,  "     █████" ; "positive_range_full")]
#[test_case(-1. / 8.,   "    ▕     " ; "negative_one_eighths")]
#[test_case(-2. / 8.,   "    🮇     " ; "negative_two_eighths")]
#[test_case(-3. / 8.,   "    🮈     " ; "negative_three_eighths")]
#[test_case(-4. / 8.,   "    ▐     " ; "negative_four_eighths")]
#[test_case(-5. / 8.,   "    🮉     " ; "negative_five_eighths")]
#[test_case(-6. / 8.,   "    🮊     " ; "negative_six_eighths")]
#[test_case(-7. / 8.,   "    🮋     " ; "negative_seven_eighths")]
#[test_case(-8. / 8.,   "    █     " ; "negative_eight_eighths")]
#[test_case(-RANGE*0.3, "   ▐█     " ; "negative_range_30_percent")]
#[test_case(-RANGE*0.5, "  ▐██     " ; "negative_range_50_percent")]
#[test_case(-RANGE*0.6, "  ███     " ; "negative_range_60_percent")]
#[test_case(-RANGE*0.8, " ████     " ; "negative_range_80_percent")]
#[test_case(-RANGE*1.0, "█████     " ; "negative_range_full")]
fn horizontal_renders_value_with_label(value: f32, line: &str) {
    let label = "abcdefghij";
    assert_renders(
        ValueBar::default().value(value).range(RANGE).label(label),
        Buffer::with_lines(vec![line, line, label, line, line]),
    )
}

#[test_case(Color::Red ; "red")]
#[test_case(Color::Blue ; "blue")]
#[test_case(Color::Yellow ; "yellow")]
#[test_case(Color::Green ; "green")]
fn horizontal_renders_with_style_fg(color: Color) {
    let mut expected = Buffer::with_lines(vec![
        "     █▏   ",
        "     █▏   ",
        "     █▏   ",
        "     █▏   ",
        "     █▏   ",
    ]);
    let area = expected.area();
    for (x, y) in (area.left()..area.right()).cartesian_product(area.top()..area.bottom()) {
        expected.get_mut(x, y).set_fg(color);
    }
    assert_renders(
        ValueBar::default()
            .value(1.0)
            .range(RANGE)
            .style(Style::default().fg(color)),
        expected,
    )
}

#[test_case(Color::Red ; "red")]
#[test_case(Color::Blue ; "blue")]
#[test_case(Color::Yellow ; "yellow")]
#[test_case(Color::Green ; "green")]
fn horizontal_renders_with_style_bg(color: Color) {
    let mut expected = Buffer::with_lines(vec![
        "     █▏   ",
        "     █▏   ",
        "     █▏   ",
        "     █▏   ",
        "     █▏   ",
    ]);
    let area = expected.area();
    for (x, y) in (area.left()..area.right()).cartesian_product(area.top()..area.bottom()) {
        expected.get_mut(x, y).set_bg(color);
    }
    assert_renders(
        ValueBar::default()
            .value(1.0)
            .range(RANGE)
            .style(Style::default().bg(color)),
        expected,
    )
}

#[test_case(Color::Red ; "red")]
#[test_case(Color::Blue ; "blue")]
#[test_case(Color::Yellow ; "yellow")]
#[test_case(Color::Green ; "green")]
fn horizontal_renders_with_style_and_label(color: Color) {
    let mut expected = Buffer::with_lines(vec![
        "     ██▏  ",
        "     ██▏  ",
        "abcdeFGhij",
        "     ██▏  ",
        "     ██▏  ",
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
            }
        }
    }

    assert_renders(
        ValueBar::default()
            .value(2.)
            .range(RANGE)
            .label("abcdeFGhij")
            .style(Style::default().fg(color)),
        expected,
    )
}
