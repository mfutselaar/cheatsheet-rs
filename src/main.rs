// Copyright (c) Mathijs Futselaar 2025
// Licensed under the GNU General Public License v3.0 with an additional restriction:
// You may not sell this software or derivatives for monetary compensation.
// See the LICENSE file for details.

use cursive::Vec2;
use cursive::View;
use cursive::style::{BorderStyle, Effect, Palette, Style};
use cursive::utils::markup::StyledString;
use cursive::view::Margins;
use cursive::views::{LinearLayout, PaddedView, ResizedView, TextView};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::ExitCode;
use terminal_size::{Height, Width, terminal_size};
use cursive::event::{Event};

#[derive(Deserialize, Debug, Serialize)]
struct CSInput {
    input: String,
    description: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct CSEnvironment {
    environment: String,
    #[serde(default)]
    prefix: String,
    inputs: Vec<CSInput>,
}

#[derive(Deserialize, Serialize, Debug)]
struct CSCheatSheet {
    environments: Vec<CSEnvironment>,
}

const CELL_WIDTH: u16 = 48;

fn read_cheatsheet() -> Result<String, std::io::Error> {
    let locations = [
        "./cheatsheet.json".to_string(),
        std::env::var("HOME")
            .map(|home| format!("{}/.local/share/cheatsheet-rs/cheatsheet.json", home))
            .unwrap_or_default(),
        std::env::var("HOME")
            .map(|home| format!("{}/cheatsheet.json", home))
            .unwrap_or_default(),
    ];

    for location in locations {
        if !location.is_empty() && Path::new(&location).exists() {
            return fs::read_to_string(&location);
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Could not find cheatsheet.json in any specified location",
    ))
}

fn calculate_column_height(column: &mut LinearLayout) -> u16 {
    let cell_height = column.required_size(Vec2::new(CELL_WIDTH as usize, 10000));
    return cell_height.y as u16;
}

fn main() -> ExitCode {
    let cheatsheet_data: CSCheatSheet = match read_cheatsheet() {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
                return ExitCode::FAILURE;
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };

    let mut cheatsheet_data = cheatsheet_data;

    cheatsheet_data
        .environments
        .sort_by(|a, b| b.inputs.len().cmp(&a.inputs.len()));

    let mut siv = cursive::default();

    let max_height = match terminal_size() {
        Some((Width(_), Height(h))) => h,
        None => {
            eprintln!("Could not determine terminal size, aborting.");
            return ExitCode::FAILURE;
        }
    };

    for c in ' '..='~' {
        siv.add_global_callback(Event::Char(c), |s| s.quit());
    }

    siv.set_theme(cursive::theme::Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: Palette::terminal_default(),
    });

    let mut main_layout = LinearLayout::horizontal();

    let mut column = LinearLayout::vertical();

    let mut first_in_column = true;

    let mut it = cheatsheet_data.environments.iter().peekable();
    while let Some(env) = it.next() {
        let mut caption = StyledString::new();
        caption.append_styled(&env.environment, Style::from(Effect::Bold));

        if !first_in_column {
            column.add_child(TextView::new("    "));
        }

        column.add_child(TextView::new(caption));
        column.add_child(TextView::new("    "));

        first_in_column = false;

        let mut it2 = env.inputs.iter().peekable();
        while let Some(input) = it2.next() {
            let column_height = calculate_column_height(&mut column);
            if column_height == max_height - 1 {
                main_layout.add_child(PaddedView::new(
                    Margins::lr(1, 1),
                    ResizedView::with_fixed_width(CELL_WIDTH as usize, column),
                ));
                column = LinearLayout::vertical();
                column.add_child(TextView::new("   "));
                column.add_child(TextView::new("   "));
            }

            let mut row = LinearLayout::horizontal();
            let mut combo = StyledString::new();
            combo.append_styled(
                format!("{:16}", input.input.replace("**", &env.prefix)),
                Style::from(Effect::Dim),
            );
            row.add_child(TextView::new(combo));
            row.add_child(TextView::new(&input.description));
            column.add_child(row);
        }

        let column_height = calculate_column_height(&mut column);
        if column_height >= max_height - 5 || it.peek().is_none() {
            main_layout.add_child(PaddedView::new(
                Margins::lr(1, 1),
                ResizedView::with_fixed_width(CELL_WIDTH as usize, column),
            ));

            column = LinearLayout::vertical();
            first_in_column = true;
        }
    }

    siv.add_layer(main_layout);
    siv.run();

    ExitCode::SUCCESS
}
