/*
 * Stygian Sift - A Terminal-based File Manager
 * Copyright (c) 2024 Maui The Magnificent (Charon)
 *
 * This software is released under the Stygian Sift License.
 * See LICENSE file in the project root or visit:
 * https://github.com/Mauitron/StygianSift.git
 *
 * Created by: Maui The Magnificent (Charon)
 * Contact: Maui_The_Magnificent@proton.me
 *
 * When using, modifying, or distributing this software,
 * please maintain this attribution notice and provide a link
 * to the original project.
 */

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileType {
    Directory,
    Text,
    Log,
    Document,
    Image,
    Binary,
    Config,
    Executable,
    Unknown,
    Rust,
    Nix = 10,
    Zig = 11,
}

pub struct DimmingConfig {
    max_distance: i32,
    dim_step: u8,
    max_dim: u8,
}

impl DimmingConfig {
    pub fn new(visible_lines: usize) -> Self {
        let (width, height) = size().unwrap();
        let end_y = height - 16;
        let _start_y = 5;
        let nav_width = width / 2;
        let _preview_width = width - nav_width - 2;
        // you can choose the maximum distance based on visible lines.
        // if you are afraid of the dark. make it brighter!
        // use about 1/3 of visible lines as max distance for dimming
        let max_distance = (visible_lines as i32 * 4).max(1).min(end_y.into());

        let dim_step = (110_u8).saturating_div(max_distance as u8);

        DimmingConfig {
            max_distance,
            dim_step,
            max_dim: 100,
        }
    }

    pub fn calculate_dimming(&self, distance_from_selected: i32) -> u8 {
        let abs_distance = distance_from_selected.abs();
        if abs_distance > self.max_distance {
            self.max_dim
        } else {
            (abs_distance as u8 * self.dim_step).min(self.max_dim)
        }
    }

#[rustfmt::skip]
    pub fn dim_color(color: Color, dim_factor: u8) -> Color {
        match color {
            Color::Rgb { r, g, b } => {
                let brightness_factor = (100 - dim_factor) as f32 / 100.0;
                let saturation_factor = (100 - dim_factor as u16).min(100) as f32 / 100.0;

                let max = r.max(g).max(b) as f32;
                let min = r.min(g).min(b) as f32;
                let diff = max - min;

                if diff == 0.0 {
                    let new_value = (r as f32 * brightness_factor) as u8;
                    Color::Rgb {
                        r: new_value,
                        g: new_value,
                        b: new_value,
                    }
                } else {
                    // For adjusting saturation, moving colors closer to grayscale. play with it
                    let gray_value =
                        (r as f32 * 0.299 + g as f32 * 0.587 + b as f32 * 0.114) as f32;

                    let new_r = ((r as f32 - gray_value) * saturation_factor + gray_value)
                        * brightness_factor;
                    let new_g = ((g as f32 - gray_value) * saturation_factor + gray_value)
                        * brightness_factor;
                    let new_b = ((b as f32 - gray_value) * saturation_factor + gray_value)
                        * brightness_factor;

                    Color::Rgb {
                        r: new_r.clamp(0.0, 255.0) as u8,
                        g: new_g.clamp(0.0, 255.0) as u8,
                        b: new_b.clamp(0.0, 255.0) as u8,
                    }
                }
            }
            Color::Black => Color::Rgb { r: 0, g: 0, b: 0 },
            Color::Red => Self::dim_color(Color::Rgb { r: 255, g: 0, b: 0 }, dim_factor),
            Color::Yellow => Self::dim_color(
                Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 0,
                },
                dim_factor,
            ),
            Color::Green => Self::dim_color(Color::Rgb { r: 0, g: 255, b: 0 }, dim_factor),
            Color::Blue => Self::dim_color(
                Color::Rgb {
                    r: 0,
                    g: 100,
                    b: 255,
                },
                dim_factor,
            ),
            Color::Magenta => Self::dim_color(
                Color::Rgb {
                    r: 255,
                    g: 0,
                    b: 255,
                },
                dim_factor,
            ),
            Color::Cyan => Self::dim_color(
                Color::Rgb {
                    r: 0,
                    g: 255,
                    b: 255,
                },
                dim_factor,
            ),
            Color::White => Self::dim_color(
                Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                dim_factor,
            ),
            Color::Reset => Self::dim_color(
                Color::Rgb {
                    r: 200,
                    g: 200,
                    b: 200,
                },
                dim_factor,
            ),
            other => other,
        }
    }
}
// Did not have the effect i wanted.
// needs heavy calibration to make sense.
fn add_green_tint(color: Color) -> Color {
    match color {
        Color::Rgb { r, g, b } => {
            let new_g = (g as u16 + 25).min(255) as u8;
            Color::Rgb { r, g: new_g, b }
        }
        Color::Reset => Color::Rgb {
            r: 200,
            g: 225,
            b: 200,
        },
        other => {
            let rgb = match other {
                Color::Black => (0, 25, 0),
                Color::Red => (255, 25, 0),
                Color::Green => (0, 255, 0),
                Color::Yellow => (255, 255, 0),
                Color::Blue => (0, 25, 255),
                Color::Magenta => (255, 25, 255),
                Color::Cyan => (0, 255, 255),
                Color::White => (255, 255, 255),
                _ => (200, 225, 200),
            };
            Color::Rgb {
                r: rgb.0,
                g: rgb.1,
                b: rgb.2,
            }
        }
    }
}
pub fn cleanup_terminal() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, MoveTo(0, 0))?;
    execute!(stdout, Show)?;

    disable_raw_mode()?;

    stdout.flush()?;
    Ok(())
}

#[rustfmt::skip]
pub fn display_help_screen(
    stdout: &mut impl Write,
    config: &Config,
    app_state: &AppState,
    full_redraw: bool,
) -> io::Result<()> {
let mut current_page = 1;
    let total_pages = 2;
    
    let _ = full_redraw;
    let (width, height) = size()?;
    let nav_width = width / 2;
    let _preview_width = width - nav_width - 2;

    let _ = clear_nav();
    let _ = clear_preview();

    let title = "File Browser Help Menu\r";
    let separator = "=".repeat(title.len());
    execute!(stdout, MoveTo(nav_width / 3, 3))?;
    writeln!(stdout, "{}\r", title.bold().green())?;
    execute!(stdout, MoveTo(nav_width / 3, 4))?;
    writeln!(stdout, "{}\r", separator.green())?;

    let get_key_for_action = |action: &Action| -> String {
        config
            .keybindings
            .as_ref()
            .and_then(|kb| {
                kb.iter().find_map(|(k, v)| {
                    if v == action {
                        Some(key_event_to_string(k))
                    } else {
                        None
                    }
                })
            })
            .unwrap_or_else(|| "Unbound".to_string())
    };
    let page1_sections = [
        (
            "Navigation",
            vec![
                (format!("{} / {}", get_key_for_action(&Action::MoveUp).trim_matches('"'), "k"), "Move up in the file list"),
                (format!("{} / {}", get_key_for_action(&Action::MoveDown).trim_matches('"'), "j"), "Move down in the file list"),
                (format!("{} / {}", get_key_for_action(&Action::MoveLeft).trim_matches('"'), "h"), "Go to parent directory"),
                (format!("{} / {}", get_key_for_action(&Action::MoveRight).trim_matches('"'), "l"), "Enter selected directory"),
                (get_key_for_action(&Action::GoToTop).trim_matches('"').to_string(), "Go to top of list"),
                (get_key_for_action(&Action::GoToBottom).trim_matches('"').to_string(), "Go to bottom of list"),
                (get_key_for_action(&Action::SearchFiles).trim_matches('"').to_string(), "Search the file system"),
                ("0-9".to_string(), "Navigate to shortcut directory"),
            ],
        ),
        (
            "File Operations",
            vec![
                (get_key_for_action(&Action::ExecuteFile).trim_matches('"').to_string(), "Execute a file"),
                (get_key_for_action(&Action::GiveBirthDir).trim_matches('"').to_string(), "Create a directory"),
                (get_key_for_action(&Action::GiveBirthFile).trim_matches('"').to_string(), "Create a file"),
                (get_key_for_action(&Action::MoveItem).trim_matches('"').to_string(), "Move selected file or directory"),
                (get_key_for_action(&Action::Rename).trim_matches('"').to_string(), "Rename (keep extension)"),
                (get_key_for_action(&Action::RenameWithoutExtension).trim_matches('"').to_string(), "Rename (allow extension change)"),
                (get_key_for_action(&Action::Duplicate).trim_matches('"').to_string(), "Duplicate selected file/folder"),
                (get_key_for_action(&Action::Murder).trim_matches('"').to_string(), "Delete selected file(s)/folder(s)"),
                (get_key_for_action(&Action::Copy).trim_matches('"').to_string(), "Copy to clipboard"),
                (get_key_for_action(&Action::Paste).trim_matches('"').to_string(), "Paste from clipboard"),
                (get_key_for_action(&Action::OpenInEditor).trim_matches('"').to_string(), "Open in text editor"),
            ],
        ),
        (
            "View and Display",
            vec![
                (get_key_for_action(&Action::TogglePreview).trim_matches('"').to_string(), "Toggle preview pane (Default: [SPACE])"),
                (get_key_for_action(&Action::ToggleCount).trim_matches('"').to_string(), "Toggle item count display"),
                (get_key_for_action(&Action::SortCycleForward).trim_matches('"').to_string(), "Change sort order (forward)"),
                (get_key_for_action(&Action::SetLineAmount).trim_matches('"').to_string(), "Set number of lines in preview"),
                (get_key_for_action(&Action::CycleItemColor).trim_matches('"').to_string(), "Cycle item color"),
                (get_key_for_action(&Action::RemoveItemColor).trim_matches('"').to_string(), "Remove item color"),
                (get_key_for_action(&Action::SetColorRules).trim_matches('"').to_string(), "Set color rules"),
            ],
        ),
        (
            "Search and Filter",
            vec![
                (get_key_for_action(&Action::Search).trim_matches('"').to_string(), "Search the selected file"),
                (get_key_for_action(&Action::SearchFiles).trim_matches('"').to_string(), "Search for files"),
            ],
        ),
        (
            "Multi-Select",
            vec![
                (get_key_for_action(&Action::SelectAll).trim_matches('"').to_string(), "Select all files"),
                (get_key_for_action(&Action::ToggleSelect).trim_matches('"').to_string(), "Toggle select mode"),
                (get_key_for_action(&Action::MultiSelectUp).trim_matches('"').to_string(), "Select multiple files (moving up)"),
                (get_key_for_action(&Action::MultiSelectDown).trim_matches('"').to_string(), "Select multiple files (moving down)"),
            ],
        ),
        (
            "System and Tools",
            vec![
                (get_key_for_action(&Action::TerminalCommand).trim_matches('"').to_string(), "Open terminal in current folder"),
                (get_key_for_action(&Action::GitMenu).trim_matches('"').to_string(), "Open Git menu"),
                (get_key_for_action(&Action::Undo).trim_matches('"').to_string(), "Undo last operation"),
            ],
        ),
        (
            "Configuration and Help",
            vec![
                (get_key_for_action(&Action::EditConfig).trim_matches('"').to_string(), "Open configuration menu"),
                (get_key_for_action(&Action::Help).trim_matches('"').to_string(), "Show this help menu"),
                (get_key_for_action(&Action::ShowShortcuts).trim_matches('"').to_string(), "Show your stored shortcuts"),
            ],
        ),
    (
        "Shortcuts",
        vec![
            ("0-9".to_string(), "Use shortcut from current layer"),
            ("Shift + 0-9".to_string(), "Set shortcut in current layer"),
            ("F1-F10".to_string(), "Quick switch between layers"),
        ],
    ),
    ];
    let page2_sections: [(&str, Vec<(String, &str)>); 8] = [
            (
                "Layer Management",
                vec![
                    ("F1-F10".to_string(), "Switch to corresponding layer"),
                    (get_key_for_action(&Action::RenameLayer).trim_matches('"').to_string(), "Rename current layer"),
                    ("!-) (Shift+0-9)".to_string(), "Set shortcut in current layer"),
                    ("0-9".to_string(), "Use shortcut from current layer"),
                    ("F2".to_string(), "Show layer overview and shortcuts"),
                ],
            ),

            (
        "",
                vec![(" ".to_string(), "")],
            ),
            (
        "",
                vec![(" ".to_string(), "")],
            ),
            (
        "",
                vec![(" ".to_string(), "")],
            ),
            (
        "",
                vec![(" ".to_string(), "")],
            ),
            (
        "",
                vec![(" ".to_string(), "")],
            ),
            (
        "",
                vec![(" ".to_string(), "")],
            ),
            (
        "",
                vec![(" ".to_string(), "")],
            ),
            
        ];

         loop {
        let _ = clear_nav();
        let _ = clear_preview();

        let title = "File Browser Help Menu\r";
        let separator = "=".repeat(title.len());
        execute!(stdout, MoveTo(nav_width / 3, 3))?;
        writeln!(stdout, "{}\r", title.bold().green())?;
        execute!(stdout, MoveTo(nav_width / 3, 4))?;
        writeln!(stdout, "{}\r", separator.green())?;

        let sections = if current_page == 1 { &page1_sections } else { &page2_sections };
        let mut current_column = 0;
        let mut current_row = height / 8;
        let column_width = (width / 2) - 4;

        for (i, (section_title, commands)) in sections.iter().enumerate() {
            if i > 0 && i % 4 == 0 {
                current_column += column_width;
                current_row = 6;
            }

            execute!(stdout, MoveTo(current_column + 8, current_row))?;
            writeln!(stdout, "{}\r", section_title.bold().green())?;
            current_row += 1;

            for (key, description) in commands {
                execute!(stdout, MoveTo(current_column + 8, current_row))?;
                writeln!(stdout, "{:<15}  {}\r", key.clone().red(), description.cyan())?;
                current_row += 1;
            }
            current_row += 1;
        }

        execute!(stdout, MoveTo((width / 2) + 4, height - 18))?;
        writeln!(stdout, "{}\r", "Search Depth:".bold().yellow())?;
        execute!(stdout, MoveTo((width / 2) + 4, height - 17))?;
        writeln!(stdout, "Current Depth: {}\r", app_state.search_depth_limit.to_string().red())?;

        execute!(stdout, MoveTo((width / 2) + 4, height - 15))?;
        writeln!(stdout, "{}\r", "Undo System Information:".bold().yellow())?;
        execute!(stdout, MoveTo((width / 2) + 4, height - 14))?;
        writeln!(stdout, "RAM Limit: {} MB\r", app_state.undo_manager.ram_limit / 1_048_576)?;
        execute!(stdout, MoveTo((width / 2) + 4, height - 13))?;
        writeln!(stdout, "Disk Limit: {} GB\r", app_state.undo_manager.disk_limit / 1_073_741_824)?;
        execute!(stdout, MoveTo((width / 2) + 4, height - 12))?;
        writeln!(stdout, "Disk Storage Allowed: {}\r",
            if app_state.undo_manager.allow_disk_storage { "Yes".green() } else { "No".red() }
        )?;

        execute!(stdout, MoveTo((width / 2) + 4, height - 10))?;
        writeln!(stdout, "{}\r", "Navigation:".bold().yellow())?;
        execute!(stdout, MoveTo((width / 2) + 4, height - 9))?;
        writeln!(stdout, "Page {} of {} (Use → and ← to navigate pages)\r", current_page, total_pages)?;
        execute!(stdout, MoveTo((width / 2) + 4, height - 8))?;
        writeln!(stdout, "Press ESC to return to the file browser...\r")?;
        execute!(stdout, MoveTo((width / 2) + 4, height - 7))?;
        writeln!(stdout, "{}: {}\r", "Tip: You can remap these keybindings by pressing".italic().blue(), "F3".italic().red())?;
        execute!(stdout, MoveTo((width / 2) + 4, height - 6))?;
        writeln!(stdout, "{}\r", "Tip: Experiment with different commands to become more proficient!".italic().blue())?;

        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Right | KeyCode::Char('l') if current_page < total_pages => {
                    current_page += 1;
                }
                KeyCode::Left | KeyCode::Char('h') if current_page > 1 => {
                    current_page -= 1;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
#[rustfmt::skip]
pub fn draw_initial_border(stdout: &mut impl Write, page_state: &PageState) -> io::Result<()> {
    let _ = page_state;
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 1;

    // Draw top border
    queue!(stdout, MoveTo(4, 2))?;
    write!(
        stdout,
        "{}{}{}",
        "═".repeat(nav_width as usize - 7),
        "⨅⨅⨅⨅".red(),
        "═".repeat(preview_width as usize - 5),
    )?;

    stdout.flush()?;
    queue!(stdout, MoveTo(4, 3))?;
    write!(stdout, "∥")?;
    queue!(stdout, MoveTo(3, 3))?;
    write!(stdout, "∥")?;

    // Draw side borders
    for y in 3..height - 2 {
        // Left side
        queue!(stdout, MoveTo(0, y + 3))?;
        write!(stdout, "{}", "▒".dark_red())?;
        queue!(stdout, MoveTo(1, y + 3))?;
        write!(stdout, "∥")?;
        queue!(stdout, MoveTo(2, y + 2))?;
        write!(stdout, "∥")?;
        queue!(stdout, MoveTo(3, y + 1))?;
        write!(stdout, "∥")?;
        queue!(stdout, MoveTo(4, y))?;
        write!(stdout, "│")?;

        // Middle spine
        queue!(stdout, SetForegroundColor(Color::DarkYellow))?;
        queue!(stdout, MoveTo(nav_width - 3, y))?;
        write!(stdout, "│")?;
        queue!(stdout, MoveTo(nav_width - 2, y))?;
        write!(stdout, "⎞⎛")?;
        queue!(stdout, MoveTo(nav_width, y))?;
        write!(stdout, "│")?;
        queue!(stdout, SetForegroundColor(Color::Reset))?;

        // Right side
        queue!(stdout, MoveTo(width - 5, y))?;
        write!(stdout, "│")?;
        queue!(stdout, MoveTo(width - 4, y + 1))?;
        write!(stdout, "∥")?;
        queue!(stdout, MoveTo(width - 3, y + 2))?;
        write!(stdout, "∥")?;
        queue!(stdout, MoveTo(width - 2, y + 3))?;
        write!(stdout, "∥")?;
        queue!(stdout, MoveTo(width - 1, y + 3))?;
        write!(stdout, "{}", "▒".dark_red())?;
    }

    // Draw bottom border
    stdout.flush()?;
    queue!(stdout, MoveTo(3, height - 2))?;
    write!(
        stdout,
        "{}",
        "⨌".dark_yellow().to_string().repeat(nav_width as usize - 7),
    )?;
    queue!(stdout, MoveTo(2, height))?;
    write!(
        stdout,
        "{}",
        "⩲".dark_red().to_string().repeat(nav_width as usize - 4)
    )?;

    // Draw corners
    let corners = [
        (4, 2, "⌌"),
        (3, 3, "╱"),
        (2, 4, "╱"),
        (1, 5, "╱"), // Top left
        (3, height - 2, "╱"),
        (2, height - 1, "╱"), // Bottom left
        (width - 5, 2, "⌍"),
        (width - 4, 3, "╲"),
        (width - 3, 4, "╲"),
        (width - 2, 5, "╲"), // Top right
        (width - 4, height - 2, "╲"),
        (width - 3, height - 1, "╲"), // Bottom right
    ];
    for (x, y, ch) in corners.iter() {
        queue!(stdout, MoveTo(*x, *y))?;
        write!(stdout, "{}", ch)?;
    }

    // Draw pages and spine
    queue!(stdout, MoveTo(preview_width - 1, height - 1))?;
    write!(
        stdout,
        "{}",
        "⩲".dark_red().to_string().repeat(nav_width as usize - 1),
    )?;
    queue!(stdout, MoveTo(preview_width + 2 , height - 2))?;
    write!(
        stdout,
        "{}",
        "⨌".dark_yellow().to_string().repeat(nav_width as usize - 6),
    )?;
    queue!(stdout, SetForegroundColor(Color::DarkYellow))?;
    queue!(stdout, MoveTo(nav_width - 3, height - 3))?;
    write!(
        stdout,
        "{}{}{}{}",
        "\\".red(),
        "\\".dark_yellow(),
        "/".dark_yellow(),
        "/".red()
    )?;

    queue!(stdout, MoveTo(preview_width - 2, height - 2))?;
    write!(stdout, "{}", "\\\\/".red())?;

    queue!(stdout, MoveTo(nav_width - 3, height))?;
    write!(stdout, "{}", "⨅⨅⨅⨅".red())?;
    queue!(stdout, MoveTo(nav_width - 4, height - 1))?;
    write!(stdout, "{}", "//".red())?;
    queue!(stdout, MoveTo(nav_width, height - 1))?;
    write!(stdout, "{}", "\\\\".red())?;
    queue!(stdout, MoveTo(nav_width - 3, height - 2))?;
    write!(stdout, "{}", "/".red())?;
    queue!(stdout, MoveTo(nav_width, height - 2))?;
    write!(stdout, "{}", "\\".red())?;
    queue!(stdout, SetForegroundColor(Color::Reset))?;
    stdout.flush()?;

    queue!(stdout, SetBackgroundColor(Color::Reset))?;
    stdout.flush()
}
pub fn update_page_num(page_state: &PageState) -> io::Result<()> {
    let (width, height) = size()?;

    execute!(stdout(), SetForegroundColor(Color::Green))?;
    queue!(stdout(), MoveTo(width / 18, height - 4))?;
    write!(stdout(), "Page")?;
    queue!(stdout(), MoveTo(width / 18 + 5, height - 4))?;
    write!(stdout(), "{}", page_state.left_page)?;
    queue!(stdout(), MoveTo(width - 16, height - 4))?;
    write!(stdout(), "Page")?;
    queue!(stdout(), MoveTo(width - 11, height - 4))?;
    write!(stdout(), "{}", page_state.right_page)?;
    execute!(stdout(), SetForegroundColor(Color::Reset))?;
    stdout().flush()?;
    Ok(())
}

pub fn clear_nav() -> io::Result<()> {
    let (width, height) = size()?;
    let end_y = height - 3;
    let start_y = 3;
    let nav_width = width / 2;
    for a in start_y..end_y {
        execute!(stdout(), MoveTo(5, a))?;
        write!(stdout(), "{}", " ".repeat(nav_width as usize - 8),)?;
    }

    stdout().flush()?;
    Ok(())
}

pub fn clear_preview() -> io::Result<()> {
    let (width, height) = size()?;
    let end_y = height - 2;
    let start_y = 5;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;
    for a in start_y..end_y {
        execute!(stdout(), MoveTo(preview_width + 3, a - 2))?;
        write!(stdout(), "{}", " ".repeat(preview_width as usize - 4),)?;
    }

    stdout().flush()?;
    Ok(())
}

fn restore_terminal(stdout: &mut impl Write) -> io::Result<()> {
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()
}

fn setup_terminal(stdout: &mut impl Write) -> io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        terminal::Clear(ClearType::All),
        cursor::Hide
    )
}

pub fn file_type_order(file_type: &FileType) -> u8 {
    match file_type {
        FileType::Directory => 0,
        FileType::Text => 1,
        FileType::Log => 2,
        FileType::Document => 3,
        FileType::Image => 4,
        FileType::Binary => 5,
        FileType::Config => 6,
        FileType::Executable => 7,
        FileType::Unknown => 8,
        FileType::Rust => 9,
        FileType::Nix => 10,
        FileType::Zig => 11,
    }
}

pub fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

pub fn sort_order_to_string(order: &SortOrder) -> StyledContent<&str> {
    match order {
        SortOrder::NameAsc => "Name (A-Z) ↑".green(),
        SortOrder::NameDesc => "Name (Z-A) ↓".green(),
        SortOrder::TypeAsc => "Type (A-Z) ↑".green(),
        SortOrder::TypeDesc => "Type (Z-A) ↓".green(),
        SortOrder::ColorAsc => "Color ↑".green(),
        SortOrder::ColorDesc => "Color ↓".green(),
        SortOrder::SizeAsc => "Size (Small to Large) ↑".green(),
        SortOrder::SizeDesc => "Size (Large to Small) ↓".green(),
        SortOrder::DateModifiedAsc => "Date Modified (Old to New) ↑".green(),
        SortOrder::DateModifiedDesc => "Date Modified (New to Old) ↓".green(),
    }
}
pub fn write_header(
    stdout: &mut impl Write,
    show_count: bool,
    display_lines: usize,
    current_dir: &Path,
    sort_order: &SortOrder,
) -> io::Result<()> {
    let _ = show_count;
    let (width, height) = size()?;
    let nav_width = width / 2;

    queue!(stdout, MoveTo(0, 1))?;
    writeln!(
        stdout,
        "{} {} for Help | {} {} for shortcuts | {} {} for config | {} {} for color ruleset",
        "Press".green(),
        "F12".red(),
        "Press".green(),
        "F11".red(),
        "Press".green(),
        "~".red(),
        "Press".green(),
        "Shift + F2".red()
    )?;
    let truncated_dir = truncate_path(current_dir, (nav_width / 2) as usize);
    queue!(stdout, MoveTo(nav_width / 12, height / 10))?;
    writeln!(stdout, " 🖥  Current directory: {}", truncated_dir.green())?;

    queue!(stdout, MoveTo(nav_width / 12 + 1, height / 10 + 1))?;
    writeln!(
        stdout,
        "Sort: {} | Lines: {}",
        sort_order_to_string(sort_order),
        // if show_count { "🟢" } else { "🔴" }, // Not in use, yet.
        display_lines.to_string().green()
    )?;

    Ok(())
}

fn display_file_info(
    stdout: &mut impl Write,
    entry: &FileEntry,
    start_x: u16,
    start_y: u16,
) -> io::Result<()> {
    queue!(stdout, MoveTo(start_x, start_y))?;
    write!(
        stdout,
        "File: {} | Type: {:?} | Size: {} | Admin: {} | Read-only: {}",
        truncate_str(&entry.name, 20),
        entry.file_type,
        format_size(entry.size),
        if entry.admin_required {
            "Yes".red()
        } else {
            "No".green()
        },
        if entry.read_only {
            "Yes".red()
        } else {
            "No".green()
        }
    )?;
    Ok(())
}

#[rustfmt::skip]
fn display_file_info_or_preview(
    stdout: &mut impl Write,
    entry: &FileEntry,
    nav_width: u16,
    preview_width: u16,
    start_y: u16,
    end_y: u16,
    is_preview: bool,
    page_state: &mut PageState,
) -> io::Result<()> {
    let _ = preview_width;
    let _ = nav_width;
    let (width, _height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;

    let clear_width = preview_width * 5 / 6;
    let available_width = clear_width as usize;
    
    update_page_num(page_state)?;
    page_state.update_right_page(start_y.into(), end_y.into());
        let animation_duration = Duration::from_millis(0); // total animation time. set to 0 for the time being.
        let current_end_y = start_y + ((end_y - start_y) as f32 * (preview_width as f32)) as u16;
        let dimming_config = DimmingConfig::new((current_end_y  - start_y) as usize);

    if is_preview {
        let start_time = Instant::now();
        
        
        loop {
            let elapsed = start_time.elapsed();
            if elapsed >= animation_duration {
                render_preview_frame(stdout, entry, nav_width, preview_width, start_y, end_y, 1.0)?;
                break;
            }

            let progress = elapsed.as_secs_f32() / animation_duration.as_secs_f32();
            render_preview_frame(stdout, entry, nav_width, preview_width, start_y, end_y, progress)?;
            
        }
    } else {

            let metadata = match fs::metadata(&entry.path) {
                Ok(meta) => meta,
                Err(e) => {
                    queue!(stdout, MoveTo(nav_width + 4, start_y - 3))?;
                    writeln!(stdout, "Error: Unable to read file metadata")?;
                    queue!(stdout, MoveTo(nav_width + 4, start_y - 2))?;
                    writeln!(stdout, "Reason: {}", e)?;
                    return Ok(());
                }
            };

            let format_time = |duration: Option<Duration>| -> String {
                duration.map_or_else(
                    || "Unknown".to_string(),
                    |d| {
                        let secs = d.as_secs();
                        format!(
                            " {}-{:02}-{:02} {:02}:{:02}:{:02}",
                            1970 + secs / 31536000,
                            (secs % 31536000) / 2592000 + 1,
                            (secs % 2592000) / 86400 + 1,
                            (secs % 86400) / 3600,
                            (secs % 3600) / 60,
                            secs % 60
                        )
                    },
                )
            };

 for i in 0..12 {
            queue!(stdout, MoveTo(nav_width + 4, start_y + i - 5))?;
            writeln!(stdout, "{}", " ".repeat(preview_width as usize - 7))?;
        }
            let info = vec![
                ("CHAPTER", entry.name.to_ascii_uppercase()),
                ("Type", format!("{:?}", entry.file_type)),
                ("Size", format_size(entry.size)),
                ("Created", format_time(metadata.created().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()))),
                ("Modified", format_time(metadata.modified().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()))),
                ("Accessed", format_time(metadata.accessed().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()))),
                ("Permissions", format!("{:o}", metadata.mode())),
                ("Owner", format!("{}:{}", metadata.uid(), metadata.gid())),
                ("Inode", metadata.ino().to_string()),
                ("Number of hard links", metadata.nlink().to_string()),
                ("Read-only", if entry.read_only { "Yes" } else { "No" }.to_string()),
                ("Admin required", if entry.admin_required { "Yes" } else { "No" }.to_string()),
            ];

            let current_max_items = ((current_end_y - start_y + 3) as f32) as usize;

            for (i, (label, value)) in info.iter().enumerate() {
                if i >= current_max_items { break; }
                if start_y - 3 + i as u16 >= end_y { break; }
                
                let distance = i  as i32;
                let dim_factor = dimming_config.calculate_dimming(distance);
                
                queue!(stdout, MoveTo(nav_width + 4, start_y - 5 + i as u16))?;

                let formatted_line = match (i, *label) {
                    (0, _) => {
                        let base_color = Color::Green;
                        let dimmed_color = DimmingConfig::dim_color(base_color, dim_factor);
                        queue!(stdout, MoveTo(nav_width + 4, start_y - 5 + i as u16))?;
                        queue!(stdout, SetForegroundColor(dimmed_color))?;
                        format!("{}: {}", label, value).bold().to_string()
                    },
                    (_, "Read-only" | "Admin required") => {
                        let base_color = if value == "Yes" { Color::Red } else { Color::Green };
                        let dimmed_color = DimmingConfig::dim_color(base_color, dim_factor);
                        queue!(stdout, SetForegroundColor(dimmed_color))?;
                        format!("{}: {}", label, value)
                    },
                    _ => {
                        let base_color = Color::Reset;
                        let dimmed_color = DimmingConfig::dim_color(base_color, dim_factor);
                        queue!(stdout, SetForegroundColor(dimmed_color))?;
                        format!("{}: {}", label, value)
                    },
                };
                write!(stdout, "{}", truncate_str(&formatted_line, available_width))?;
            }
        }

        stdout.flush()?;
        // Delay between frames (skip delay on last frame).
        // Leave this as an artifact of the idea. come back when
        // I understand async in Rust better.
        // let a = rayon::ThreadBuilder::run();


    execute!(stdout, SetForegroundColor(Color::Reset), SetBackgroundColor(Color::Reset))?;
    Ok(())
}

// Animating the preview is not as simple as i thought. need to go async
// to avoid freezing the whole program to a halt. Is not needed but would
// be a nice addition, for the bling.
fn render_preview_frame(
    stdout: &mut impl Write,
    entry: &FileEntry,
    nav_width: u16,
    preview_width: u16,
    start_y: u16,
    end_y: u16,
    progress: f32,
) -> io::Result<()> {
    let current_end_y = start_y + ((end_y - start_y) as f32 * progress) as u16;
    let dimming_config = DimmingConfig::new((current_end_y - start_y) as usize);

    clear_preview()?;

    if entry.path.is_dir() {
        display_folder_preview(
            &entry.path,
            stdout,
            nav_width,
            preview_width,
            start_y,
            current_end_y,
        )?;
    } else {
        match File::open(&entry.path) {
            Ok(mut file) => {
                let mut buffer = vec![0; PREVIEW_LIMIT];
                let bytes_read = file.read(&mut buffer)?;
                buffer.truncate(bytes_read);

                let mut y = start_y;
                let mut byte_index = 0;

                while y < current_end_y && byte_index < bytes_read {
                    let mut line = String::new();
                    let line_start = byte_index;

                    while byte_index < bytes_read
                        && buffer[byte_index] != b'\n'
                        && byte_index != (end_y - 3).into()
                        && buffer[byte_index] != b'\r'
                    {
                        byte_index += 1;
                    }

                    for &byte in &buffer[line_start..byte_index] {
                        if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
                            line.push(byte as char);
                        } else {
                            line.push_str(&format!("\\x{:02X}", byte));
                        }
                    }

                    let distance = ((y - 3).saturating_sub(start_y)) as i32;
                    let dim_factor = dimming_config.calculate_dimming(distance);
                    let dimmed_color = DimmingConfig::dim_color(Color::Yellow, dim_factor);

                    queue!(
                        stdout,
                        MoveTo(nav_width + 2, y - 3),
                        SetForegroundColor(dimmed_color)
                    )?;

                    write!(
                        stdout,
                        " {}",
                        truncate_str(&line, preview_width as usize - 14)
                    )?;

                    byte_index += 1;
                    y += 1;
                }
            }
            Err(e) => {
                queue!(stdout, MoveTo(nav_width + 1, start_y))?;
                writeln!(stdout, "Error: Unable to read file")?;
                queue!(stdout, MoveTo(nav_width + 1, start_y + 1))?;
                writeln!(stdout, "Reason: {}", e)?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}
#[rustfmt::skip]
pub fn write_entry(
    app_state: &AppState,
    stdout: &mut impl Write,
    entry: &FileEntry,
    is_selected: bool,
    distance_from_selected: i32,
    width: u16,
    dimming_config: &DimmingConfig,
) -> io::Result<()> {
    let (icon, name) = match entry.file_type {
        FileType::Directory => ("📁", format!("{}/", entry.name)),
        _ => ("📄", entry.name.clone()),
    };

    let size_str = if let FileType::Directory = entry.file_type {
        String::new()
    } else {
        format_size(entry.size)
    };
    // Add more in the future. Don't forget.
    let (type_icon, permission_icon) = match entry.file_type {
        FileType::Directory => (" ", if entry.admin_required { "🔐" } else if entry.read_only { "🔏" } else { " " }),
        FileType::Text => ("📄", if entry.admin_required { "🔐" } else if entry.read_only { "🔏" } else { " " }),
        FileType::Log => ("📜", if entry.admin_required { "🔐" } else if entry.read_only { "🔏" } else { " " }),
        FileType::Document => ("📘", if entry.admin_required { "🔐" } else if entry.read_only { "🔏" } else { " " }),
        FileType::Image => ("🌃", if entry.admin_required { "🔐" } else if entry.read_only { "🔏" } else { " " }),
        FileType::Binary => ("💽", if entry.admin_required { "🔐" } else if entry.read_only { "🔏" } else { " " }),
        FileType::Config => ("📑", if entry.admin_required { "🔐" } else if entry.read_only { "🔏" } else { " " }),
        FileType::Executable => ("🎮", if entry.admin_required { "🔐" } else if entry.read_only { "🔏" } else { " " }),
        FileType::Unknown => ("  ", if entry.admin_required { "🔐" } else if entry.read_only { "🔏" } else { " " }),
        FileType::Rust => ("🦀", if entry.admin_required { "🔐" } else if entry.read_only { "🔏" } else { " " }),
        FileType::Nix => ("❄", if entry.admin_required { "🔐" } else if entry.read_only { "🔏" } else { " " }),
        FileType::Zig => ("⚡", if entry.admin_required { "🔐" } else if entry.read_only { "🔏" } else { " " }),
    };

   let available_width = width as usize - 34;
    let truncated_name = truncate_str(&name, available_width);
    let marker_color = app_state.config.get_item_color(&entry.path);
    let is_multi_selected = app_state.multiple_selected_files.as_ref().map_or(false, |selected| selected.contains(&entry.path));
    let dim_factor = dimming_config.calculate_dimming(distance_from_selected);

    fn enhance_color(color: Color) -> Color {
        match color {
            Color::Rgb { r, g, b } => {
                let enhance = 8.0;
                let new_r = ((r as f32 - 128.0) * enhance + 128.0).clamp(0.0, 255.0) as u8;
                let new_g = ((g as f32 - 128.0) * enhance + 128.0 + 25.0).clamp(0.0, 255.0) as u8;
                let new_b = ((b as f32 - 128.0) * enhance + 128.0).clamp(0.0, 255.0) as u8;
                
                Color::Rgb { r: new_r, g: new_g, b: new_b }
            }
            _ => color,
        }
    }

    let base_fg = if is_multi_selected {
        marker_color.map_or(Color::White, |c| c.to_color())
    } else if is_selected {
        if let Some(color) = marker_color {
            enhance_color(color.to_color())
        } else {
            Color::Rgb { r: 220, g: 245, b: 220 }
        }
    } else if let Some(color) = marker_color {
        color.to_color()
    } else {
        Color::Reset
    };

    let fg_color = if !is_selected {
        DimmingConfig::dim_color(base_fg, dim_factor)
    } else if !app_state.changing_color {
        Color::DarkGreen
    } else {
        base_fg
    };

    // Set background color for multi-selected items. 
    // I need to check how it looks on a multitude of terminals.
    let bg_color = if is_multi_selected {
        Color::Rgb { r: 40, g: 44, b: 102 } 
    } else {
        Color::Reset
    };

    queue!(
        stdout,
        SetForegroundColor(fg_color),
        SetBackgroundColor(bg_color)  
    )?;

    if is_selected {
        queue!(stdout, SetAttribute(Attribute::Bold))?;
    }

    write!(
        stdout,
        "{:1} {} {:<width$} {:>10} {} {}",
        if is_selected { " → " } else { "  " },
        icon,
        truncated_name,
        size_str,
        type_icon,
        permission_icon,
        width = available_width
    )?;

    execute!(
        stdout,
        SetAttribute(Attribute::Reset),
        SetForegroundColor(Color::Reset),
        SetBackgroundColor(Color::Reset)
    )?;
    
    Ok(())
}
// Go over more carefully. try making the preview and dir async.
pub fn display_directory(
    app_state: &mut AppState,
    entries: &[FileEntry],
    current_dir: &Path,
    selected_index: usize,
    stdout: &mut impl Write,
    mut scroll_offset: usize,
    visible_lines: usize,
    full_redraw: bool,
) -> io::Result<()> {
    let _ = visible_lines;
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;
    let start_y = 11;
    let end_y = height - 4;
    let total_entries = entries.len();
    let visible_lines = (end_y - start_y) as usize;
    let selected_index = selected_index;
    let adjusted_selected_index = selected_index.min(total_entries.saturating_sub(1));
    let middle_line = visible_lines / 2;

    let dimming_config = DimmingConfig::new(visible_lines);

    let new_scroll_offset = if total_entries <= visible_lines {
        0
    } else if adjusted_selected_index < middle_line {
        0
    } else if adjusted_selected_index >= total_entries - middle_line {
        total_entries - visible_lines
    } else {
        adjusted_selected_index - middle_line
    };

    if new_scroll_offset != scroll_offset {
        scroll_offset = new_scroll_offset;
    }

    if full_redraw {
        execute!(stdout, Clear(ClearType::All))?;
        draw_initial_border(stdout, &app_state.page_state)?;
    }

    write_header(
        stdout,
        app_state.show_count,
        visible_lines,
        current_dir,
        &app_state.config.default_sort,
    )?;
    stdout.flush()?;
    display_git_info(stdout, current_dir, nav_width, start_y - 1)?;
    app_state.scroll_state.offset = scroll_offset;

    let stdout_mutex = Mutex::new(&mut *stdout);

    let output_data: Vec<(u16, String)> = entries
        .par_iter()
        .enumerate()
        .skip(scroll_offset)
        .take(visible_lines)
        .filter_map(|(index, entry)| {
            let y = (index - scroll_offset + 7) as u16;
            if y > end_y {
                return None;
            }

            let distance_from_selected = index as i32 - adjusted_selected_index as i32;
            let is_selected = index == adjusted_selected_index;

            let mut buffer = Vec::new();
            {
                let mut cursor = Cursor::new(&mut buffer);
                if write_entry(
                    app_state,
                    &mut cursor,
                    entry,
                    is_selected,
                    distance_from_selected,
                    nav_width,
                    &dimming_config,
                )
                .is_ok()
                {
                    Some((y, String::from_utf8(buffer).unwrap_or_default()))
                } else {
                    None
                }
            }
        })
        .collect();

    for (y, output) in output_data {
        let mut stdout_guard = stdout_mutex.lock().unwrap();
        queue!(stdout_guard, MoveTo(5, y + 2))?;
        write!(stdout_guard, "{}", output)?;
    }

    execute!(stdout, cursor::Hide)?;

    // Scroll indicators, reflect on if the color should change on distance.
    if scroll_offset > 0 {
        queue!(stdout, MoveTo(nav_width / 2, (height / 5) - 1))?;
        write!(stdout, "{}", "▲".green())?;
    } else {
        queue!(stdout, MoveTo(nav_width / 2, (height / 5) - 1))?;
        write!(stdout, " ")?;
    }

    if scroll_offset + visible_lines < total_entries {
        queue!(stdout, MoveTo(nav_width / 2, height - 6))?;
        write!(stdout, "{}", "▼".green())?;
    } else {
        queue!(stdout, MoveTo(nav_width / 2, height - 6))?;
        write!(stdout, " ")?;
    }

    queue!(stdout, SetForegroundColor(Color::Reset))?;
    stdout.flush()?;

    if let Some(entry) = entries.get(adjusted_selected_index) {
        if app_state.preview_active {
            display_file_info_or_preview(
                stdout,
                entry,
                nav_width + 1,
                preview_width - 9,
                start_y - 3,
                end_y,
                true,
                &mut app_state.page_state,
            )?;
        } else {
            display_file_info_or_preview(
                stdout,
                entry,
                nav_width,
                preview_width,
                start_y,
                end_y,
                false,
                &mut app_state.page_state,
            )?;
        }
    }

    stdout.flush()
}

pub fn display_folder_preview(
    path: &Path,
    stdout: &mut impl Write,
    start_x: u16,
    width: u16,
    start_y: u16,
    end_y: u16,
) -> io::Result<()> {
    match fs::read_dir(path) {
        Ok(entries) => {
            let mut y = start_y;
            for entry in entries
                .filter_map(Result::ok)
                .take((end_y - start_y) as usize)
            {
                let file_name = entry.file_name().to_string_lossy().into_owned();
                let file_type = if entry.file_type()?.is_dir() {
                    "📁"
                } else {
                    "📄"
                };
                let display_string = format!("{} {}", file_type, file_name);
                queue!(stdout, MoveTo(start_x + 2, y))?;
                write!(
                    stdout,
                    " {}",
                    truncate_str(
                        &display_string.as_str().dark_yellow().to_string(),
                        width as usize - 2
                    )
                )?;
                y += 1;
                if y >= end_y {
                    break;
                }
            }
        }
        Err(e) => {
            execute!(stdout, SetForegroundColor(Color::Red))?;
            queue!(stdout, MoveTo(start_x + 14, start_y + 13))?;
            write!(stdout, "{}", "▲".repeat(53))?;
            queue!(stdout, MoveTo(start_x + 30, start_y + 15))?;
            execute!(
                stdout,
                SetForegroundColor(Color::Red),
                SetAttribute(crossterm::style::Attribute::Bold)
            )?;
            writeln!(stdout, "Unable to read directory")?;
            queue!(stdout, MoveTo(start_x + 21, start_y + 16))?;
            writeln!(stdout, "Reason: {}", e)?;
            execute!(
                stdout,
                SetForegroundColor(Color::Red),
                SetAttribute(crossterm::style::Attribute::Bold)
            )?;
            queue!(stdout, MoveTo(start_x + 14, start_y + 18))?;
            write!(stdout, "{}", "▼".repeat(53))?;
        }
    }
    Ok(())
}

// for consistency i should add in the same highlighting as the rest. do this later
pub fn display_shortcuts(app_state: &mut AppState, stdout: &mut impl Write) -> io::Result<()> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;

    loop {
        let _ = clear_nav();
        let _ = clear_preview();

        let (layer_index, layer_name) = app_state.config.get_current_layer_info();
        let title = format!("Shortcuts - {} (Layer {})\r", layer_name, layer_index)
            .bold()
            .green();
        let separator = "=".repeat(title.to_string().len());

        execute!(stdout, MoveTo(nav_width / 3 + 1, 7))?;
        writeln!(stdout, "{}\r", title)?;
        execute!(stdout, MoveTo(nav_width / 3 - 9, 8))?;
        writeln!(stdout, "{}\r", separator.green())?;

        execute!(stdout, MoveTo(nav_width / 3 - 9, 10))?;
        writeln!(stdout, "{}", "Layer Controls:".yellow().bold())?;
        for i in 0..10 {
            let layer_name = &app_state.config.shortcut_layers[i].name;
            execute!(
                stdout,
                MoveTo(nav_width / 3 - 7, (11 + i).try_into().unwrap())
            )?;
            if i == layer_index {
                writeln!(
                    stdout,
                    "{}F{}: {} \r",
                    "→ ".yellow().italic(),
                    (i + 1).to_string().green(),
                    layer_name.clone().green().on_dark_grey(),
                )?;
            } else {
                writeln!(
                    stdout,
                    "F{}: Switch to {}\r",
                    (i + 1).to_string().red(),
                    layer_name.clone().green()
                )?;
            }
        }

        execute!(stdout, MoveTo(preview_width * 11 / 8, 10))?;
        writeln!(stdout, "{}", "Current Layer Shortcuts:".yellow().bold())?;

        if let Some(layer) = app_state.config.shortcut_layers.get(layer_index) {
            if let Some(shortcuts) = &layer.shortcuts {
                let sorted_shortcuts: BTreeMap<_, _> = shortcuts.iter().collect();
                for (i, (key, (path, name, _))) in sorted_shortcuts.iter().enumerate() {
                    let display_name = if name.is_empty() {
                        path.display().to_string()
                    } else {
                        name.clone()
                    };

                    execute!(stdout, MoveTo(preview_width * 11 / 10, 12 + i as u16))?;
                    // execute!(stdout, MoveTo(preview_width - 18, 12 + i as u16))?;
                    if path.is_dir() {
                        writeln!(
                            stdout,
                            "{}: {} [{}]\r",
                            key.green(),
                            display_name.red(),
                            path.to_string_lossy().green()
                        )?;
                    } else {
                        writeln!(stdout, "{}: {}\r", key.green(), display_name.blue())?;
                    }
                }
            } else {
                execute!(stdout, MoveTo(preview_width - 18, 12))?;
                let _ = interaction_field("No shortcuts set in this layer");
            }
        }

        execute!(stdout, SetForegroundColor(Color::Green))?;
        queue!(stdout, MoveTo(nav_width / 5, height - 11))?;
        writeln!(stdout, "Use F1-F10 to switch layers")?;
        queue!(stdout, MoveTo(nav_width / 5, height - 10))?;
        writeln!(stdout, "Press ESC to return to browser\r")?;

        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => break,
                KeyCode::F(n) if n >= 1 && n <= 10 => {
                    let layer_index = (n - 1) as usize;
                    app_state.config.switch_layer(layer_index)?;
                }
                _ => {}
            }
        }
    }
    let _ = clear_nav();
    let _ = clear_preview();
    Ok(())
}
// Does not work correctly. Pull crashes the program.
pub fn display_git_menu(
    git_menu: &GitMenu,
    stdout: &mut impl Write,
    nav_width: u16,
    preview_width: u16,
    start_y: u16,
    end_y: u16,
) -> io::Result<()> {
    let _ = preview_width;
    let available_height = end_y - start_y;

    // let _ = clear_preview();

    execute!(
        stdout,
        MoveTo(nav_width + 2, start_y),
        SetForegroundColor(Color::Green)
    )?;
    write!(stdout, "Git Operations Menu\r")?;
    execute!(stdout, ResetColor)?;

    for (index, item) in git_menu.items.iter().enumerate() {
        if index as u16 >= available_height {
            break;
        }

        execute!(stdout, MoveTo(nav_width + 2, start_y + 2 + index as u16))?;

        if index == git_menu.selected {
            execute!(stdout, SetBackgroundColor(Color::DarkGrey))?;
        }

        write!(stdout, "{} - {} \r", item.label, item.description)?;

        if index == git_menu.selected {
            execute!(stdout, ResetColor)?;
        }
    }

    Ok(())
}
pub fn display_git_info(
    stdout: &mut impl Write,
    current_dir: &Path,
    nav_width: u16,
    start_y: u16,
) -> std::io::Result<()> {
    let _ = start_y;
    let _ = nav_width;
    let (width, height) = size()?;
    let nav_width = width / 2;

    if is_git_repository(current_dir) {
        let branch = get_current_branch(current_dir);
        let status = get_git_status(current_dir);
        queue!(stdout, MoveTo(nav_width / 12 + 1, height / 10 + 3))?;
        writeln!(
            stdout,
            "{}\r",
            "Current directory is a Git repository".green()
        )?;

        queue!(stdout, MoveTo(nav_width / 12 + 1, height / 10 + 2))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;
        write!(stdout, "Git: ")?;
        execute!(stdout, SetForegroundColor(Color::Yellow))?;
        write!(stdout, "{}", branch)?;
        execute!(stdout, SetForegroundColor(Color::White))?;
        write!(stdout, " | ")?;
        execute!(stdout, SetForegroundColor(Color::Cyan))?;
        writeln!(stdout, "{}", status)?;
    }

    Ok(())
}
pub fn clear_interaction_field() -> io::Result<()> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 1;
    let mut stdout = stdout();

    queue!(stdout, MoveTo(preview_width + 2, height - 12))?;
    write!(stdout, "{}", " ".repeat((preview_width - 5).into()))?;
    queue!(stdout, MoveTo(preview_width + 2, height - 11))?;
    write!(stdout, "{}", " ".repeat((preview_width - 5).into()))?;
    queue!(stdout, MoveTo(preview_width + 2, height - 10))?;
    write!(stdout, "{}", " ".repeat((preview_width - 5).into()))?;
    queue!(stdout, MoveTo(preview_width + 2, height - 9))?;
    write!(stdout, "{}", " ".repeat((preview_width - 5).into()))?;
    queue!(stdout, MoveTo(preview_width + 2, height - 8))?;
    write!(stdout, "{}", " ".repeat((preview_width - 5).into()))?;
    stdout.flush()?;
    Ok(())
}
pub fn interaction_field(input: &str) -> io::Result<()> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 1;
    let mut stdout = stdout();

    queue!(stdout, MoveTo(preview_width + 2, height - 12))?;
    write!(stdout, "{}", "-".repeat((preview_width - 5).into()).green())?;
    queue!(stdout, MoveTo(preview_width + 2, height - 11))?;
    write!(stdout, "{}", " ".repeat((preview_width - 5).into()))?;
    queue!(
        stdout,
        MoveTo(preview_width + 2 + input.len() as u16, height - 10)
    )?;
    write!(stdout, "{}", input)?;
    queue!(stdout, MoveTo(preview_width + 2, height - 9))?;
    write!(stdout, "{}", " ".repeat((preview_width - 5).into()))?;
    queue!(stdout, MoveTo(preview_width + 2, height - 8))?;
    write!(stdout, "{}", "-".repeat((preview_width - 5).into()).green())?;
    stdout.flush()?;
    Ok(())
}
