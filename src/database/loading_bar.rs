use std::io::Write;

use crate::database::TABLE_GENERATION_LOADING_BAR_LENGTH;

pub struct LoadingBar {
    percent: f32,
    previous_print_percent: f32,
    item_count: usize,
    current_item: usize,
}

impl LoadingBar {
    pub fn new(item_count: usize) -> Self {
        let loading_bar = Self {
            percent: 0.0,
            previous_print_percent: 0.0,
            item_count,
            current_item: 0,
        };

        eprint!(
            "\x1b[30m[{}]\x1b[39m",
            " ".repeat(TABLE_GENERATION_LOADING_BAR_LENGTH)
        );
        eprint!("\x1B[2G");
        std::io::stderr().flush().unwrap();

        loading_bar
    }

    pub fn update(&mut self) {
        self.current_item += 1;

        self.percent = self.current_item as f32 * 100.0 / self.item_count as f32;
        let normalized_percent = self.percent.ceil();

        if normalized_percent - self.previous_print_percent
            == (100 / TABLE_GENERATION_LOADING_BAR_LENGTH) as f32
            && self.percent != 100.0
        {
            self.previous_print_percent = normalized_percent;
            eprint!("\x1b[32m=\x1b[39m");
            std::io::stderr().flush().unwrap();
        }

        if self.current_item == self.item_count {
            eprintln!();
        }
    }
}
