use tabled::{
    builder::Builder,
    settings::{peaker::PriorityMax, Settings, Style, Width},
    Table,
};
use terminal_size::{terminal_size, Height as TerminalHeight, Width as TerminalWidth};

fn get_terminal_size() -> (usize, usize) {
    let (TerminalWidth(width), TerminalHeight(height)) =
        terminal_size().expect("failed to obtain a terminal size");

    (width as usize, height as usize)
}

pub trait InfoTable {
    fn format_table(&self) -> Table;

    fn create_builder(&self) -> Builder {
        Builder::default()
    }

    fn build_table(&self, builder: Builder) -> Table {
        let mut table = builder.build();

        let (width, _height) = get_terminal_size();

        table.with(Style::rounded());
        table.with(
            Settings::empty()
                .with(Width::wrap(width).priority(PriorityMax))
                .with(Width::increase(width)),
        );

        table.clone()
    }
}
