use cursive::style::{BorderStyle, Effect, Palette, Style};
use cursive::utils::markup::{StyledString};
use cursive::views::{LinearLayout, ResizedView, TextView};
use cursive::Vec2;
use cursive::View;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::ExitCode;
use terminal_size::{Height, Width, terminal_size};

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

fn main() -> ExitCode {
    let cheatsheet_source: String =
        fs::read_to_string("cheatsheet.json").expect("Could not read cheatsheet.json");
    let mut cheatsheet_data: CSCheatSheet =
        serde_json::from_str(&cheatsheet_source).expect("Could not parse json data");

    let mut siv = cursive::default();

    let (max_width, max_height)  = match terminal_size() {
        Some((Width(w), Height(h))) => (w, h),
        None => {
            println!("Could not determine terminal size, aborting.");
            return ExitCode::FAILURE;
        }
    };
    siv.add_global_callback(cursive::event::Event::Char('q'), |s| s.quit());

    siv.set_theme(cursive::theme::Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: Palette::terminal_default(),
    });

    let mut main_layout = LinearLayout::vertical();
    let mut current_row = LinearLayout::horizontal();

    cheatsheet_data.environments.sort_by(|a, b| b.inputs.len().cmp(&a.inputs.len()));
    
    let columns: u16 = max_width / CELL_WIDTH;
    let mut first_row: bool = true;

    let cells : Vec<Vec<LinearLayout>>;

    for (index, env) in cheatsheet_data.environments.iter().enumerate() {
        let mut row_index = if first_row { 2 } else { 3 };

        if index > 0 && index % columns as usize == 0 {
            main_layout.add_child(current_row);
            current_row = LinearLayout::horizontal();
            first_row = false;
        }

        let mut current_cell = LinearLayout::vertical();

        let mut caption = StyledString::new();
        caption.append_styled(&env.environment, Style::from(Effect::Bold));

        if !first_row {
            current_cell.add_child(TextView::new("    "));
        }

        current_cell.add_child(TextView::new(caption));
        current_cell.add_child(TextView::new("    "));

        

        for input in env.inputs.iter() {
//            if row_index > (max_height - 4) {
  //              current_cell = 
    //        }
            let mut row = LinearLayout::horizontal();
            let mut combo = StyledString::new();
            combo.append_styled(format!("{:16}", input.input.replace("**", &env.prefix)), Style::from(Effect::Dim));
            row.add_child(TextView::new(combo));
            row.add_child(TextView::new(&input.description));
            current_cell.add_child(row);
        }

        let cell_height = current_cell.required_size(Vec2::new(CELL_WIDTH as usize, 10000));

        current_cell.add_child(TextView::new(format!("Cell height: {}", cell_height.y)));

        current_row.add_child(ResizedView::with_fixed_width(
            CELL_WIDTH as usize,
            current_cell,
        ));

    }

    main_layout.add_child(current_row);

    siv.add_layer(main_layout);
    siv.run();

    ExitCode::SUCCESS
}
