use crate::metadata::{ModelSpec, Weights};
use serde_json::Value;
use std::vec;
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

impl InfoTable for Weights {
    fn format_table(&self) -> Table {
        let mut builder = self.create_builder();

        builder.push_record(vec![
            "Parameter Name".to_string(),
            "DType".to_string(),
            "Shape".to_string(),
        ]);

        for (name, weight) in self.iter() {
            // TODO: prettify the parameter names?

            builder.push_record(vec![
                name.to_string(),
                weight.dtype.to_string(),
                format!("{:?}", weight.shape),
            ]);
        }

        let table = self.build_table(builder);

        table
    }
}

impl InfoTable for ModelSpec {
    fn format_table(&self) -> Table {
        let mut builder = self.create_builder();

        builder.push_record(vec!["Key".to_string(), "Value".to_string()]);

        let value = serde_json::to_value(&self).unwrap();

        if let Value::Object(map) = value {
            for (key, value) in map {
                if value.is_null() {
                    continue;
                }

                builder.push_record(vec![key.to_string(), value.to_string()]);
            }
        }

        let table = self.build_table(builder);

        table
    }
}
