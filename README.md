# tui-bars

![CI](https://github.com/gollth/tui-bars/actions/workflows/rust.yml/badge.svg)
[![crate.io](https://img.shields.io/crates/v/tui-bars.svg)](https://crates.io/crates/tui-bars)
[![Docs](https://docs.rs/tui-bars/badge.svg)](https://docs.rs/crate/tui-bars/)

Provides additional bar widgets for the great [ratatui](https://github.com/ratatui-org/ratatui) crate.

## Demo

![](https://raw.githubusercontent.com/gollth/tui-bars/main/media/demo.gif)

```rust,no_run
use ratatui::{widgets::Block, layout::Direction};
use tui_bars::ValueBar;

let x = 1.234;

ValueBar::default()
    .direction(Direction::Vertical)
    .value(x)
    .label(format!("{x:.2}"))
    .range(5.)
    .block(Block::default().title("My Value X"));
```
