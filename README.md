# Cheatsheet-RS

Cheatsheet-RS is a terminal-based application built with Rust and the cursive
library to display a customizable cheatsheet for various environments and their
associated commands or inputs.

This was and is primarily used to get some knowledge about Rust.


## Features
- **Dynamic Layout**: Automatically organizes commands into columns that fit your terminal size.
- **Customizable Input**: Reads from a `cheatsheet.json` file, allowing users to define environments, command prefixes, and inputs.
- **Responsive Design**: Adjusts column heights to prevent overflow, ensuring readability.

## Installation
1. **Prerequisites**:
   - Rust and Cargo (install via [rustup](https://rustup.rs/)).
   - A terminal emulator supporting UTF-8.

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/mfutselaar/cheatsheet-rs.git
   cd cheatsheet-rs
   ```

3. **Build and Run**:
   ```bash
   cargo build --release
   cargo run --release
   ```

## Usage
1. Create a `cheatsheet.json` file in one of the following locations:
  
  - Current working directory (`./cheatsheet.json`)
  - `$HOME/.local/share/cheatsheet-rs/cheatsheet.json`
  - `$HOME/cheatsheet.json`

2. Run the application:
   ```bash
   cargo run --release
   ```
3. Press any key to quit the application.

### Integrate into Sway or i3

You can add this cheatsheet to your Sway or i3 (or hyprland) by simply adding a new keybinding:

Example for Sway using ghostty:

```
bindsym $mod+Shift+c exec --no-startup-id ghostty --class=sway.ghostty.cheatsheet -e /path/to/cheatsheet
for_window [app_id="sway.ghostty.cheatsheet"] floating enable, resize set 50 ppt 50 ppt
```

This will open a 50% x 50% terminal window with the cheatsheet in the center of your screen.

![Screenshot](screenshot.png?raw=true "Cheatsheet as a floating window in Sway")

### Example `cheatsheet.json`
```json
{
  "environments": [
    {
        "environment": "SwayWM",
        "prefix": "󰘳",
        "inputs": [
            {
                "input": "** + 󰌑",
                "description": "Open terminal"
            },
            {
                "input": "---"
            },
            {
                "input": "** + h",
                "description": "Toggle horizontal split"
            },
            {
                "input": "** + v",
                "description": "Toggle vertical split"
            },
            {
                "input": "---"
            },
            {
                "input": "** + q",
                "description": "Close application"
            }
        ]
    },
    {
        "environment": "Tmux",
        "prefix": "󰘴 + b,",
        "inputs": [
            {
                "input" : "** \"",
                "description": "Open horizontal panel"
            },
            {
                "input": "** %",
                "description": "Open vertical panel"
            },
            {
                "input": "** D",
                "description": "Detach session"
            }
        ]
    },
    {
      "environment": "Git",
      "prefix": "git ",
      "inputs": [
        {
          "input": "**commit -m \"message\"",
          "description": "Commit changes with a message"
        },
        {
          "input": "**push origin main",
          "description": "Push changes to the main branch"
        }
      ]
    },
    {
      "environment": "Docker",
      "prefix": "docker ",
      "inputs": [
        {
          "input": "**run -it ubuntu",
          "description": "Run an Ubuntu container interactively"
        }
      ]
    }
  ]
}
```

- **Structure**:
  - `environments`: Array of environment objects.
  - `environment`: Name of the environment (e.g., "Git").
  - `prefix`: Optional command prefix (e.g., "git "). Replaced in `input` where `**` appears.
  - `inputs`: Array of input objects with `input` (command) and `description` (explanation).

_Note: You can define the `input` as `---` to add an empty line as a separator._

## License
This project is licensed under the GNU General Public License v3.0 with an additional restriction:
you may not sell this software or any derivative works for monetary compensation.
See the [LICENSE](LICENSE) file for details.

Copyright (c) Mathijs Futselaar 2025.

## Contact
For questions or suggestions, open an issue on GitHub (https://github.com/mfutselaar/cheatsheet-rs).
