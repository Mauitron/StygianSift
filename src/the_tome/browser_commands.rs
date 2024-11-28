/*
* Stygian Sift - A Terminal-based File Manager
 * Copyright (C) 2024 Maui The Magnificent (Charon)
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 *
 * Contact: Maui_The_Magnificent@proton.me
 * Project repository: https://github.com/Mauitron/StygianSift.git
 */

use super::*;
#[cfg(unix)]
fn kill_process(child: &mut Child) -> io::Result<()> {
    use std::os::unix::process::CommandExt;
    Command::new("kill")
        .arg("-TERM")
        .arg(child.id().to_string())
        .exec();
    Err(io::Error::last_os_error())
}

#[cfg(windows)]
fn kill_process(child: &mut Child) -> io::Result<()> {
    child.kill()
}

pub fn open_file_with_editor(
    file_path: &Path,
    editor: &str,
    stdout: &mut impl Write,
) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    let _ = clear_interaction_field();
    interaction_field!("Opening file in {}.\r", editor)?;
    stdout.flush()?;

    let status = Command::new(editor).arg(file_path).status().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to open editor: {}\r", e),
        )
    })?;

    terminal::enable_raw_mode()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Editor closed with an error\r",
        ));
    }

    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        terminal::EnterAlternateScreen
    )?;
    writeln!(stdout, "Returning to browser.\r")?;
    stdout.flush()?;
    Ok(())
}

pub fn read_new_sort(stdout: &mut impl Write) -> io::Result<SortOrder> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let _preview_width = width - nav_width - 2;
    Ok(loop {
        queue!(stdout, MoveTo(nav_width + 20, 7 + height / 8 + 1 as u16))?;
        writeln!(stdout, "Select new default sort order:\r")?;
        queue!(stdout, MoveTo(nav_width + 20, 7 + height / 8 + 2 as u16))?;
        writeln!(stdout, "1. Name Ascending\r")?;
        queue!(stdout, MoveTo(nav_width + 20, 7 + height / 8 + 3 as u16))?;
        writeln!(stdout, "2. Name Descending\r")?;
        queue!(stdout, MoveTo(nav_width + 20, 7 + height / 8 + 4 as u16))?;
        writeln!(stdout, "3. Size Ascending\r")?;
        queue!(stdout, MoveTo(nav_width + 20, 7 + height / 8 + 5 as u16))?;
        writeln!(stdout, "4. Size Descending\r")?;
        queue!(stdout, MoveTo(nav_width + 20, 7 + height / 8 + 6 as u16))?;
        writeln!(stdout, "5. Type Ascending\r")?;
        queue!(stdout, MoveTo(nav_width + 20, 7 + height / 8 + 7 as u16))?;
        writeln!(stdout, "6. Type Descending\r")?;
        queue!(stdout, MoveTo(nav_width + 20, 7 + height / 8 + 8 as u16))?;
        write!(stdout, "Enter your choice ({}): \r", "1-6".red())?;
        queue!(stdout, MoveTo(nav_width + 20, 7 + height / 8 + 9 as u16))?;
        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('1') => return Ok(SortOrder::NameAsc),
                KeyCode::Char('2') => return Ok(SortOrder::NameDesc),
                KeyCode::Char('3') => return Ok(SortOrder::SizeAsc),
                KeyCode::Char('4') => return Ok(SortOrder::SizeDesc),
                KeyCode::Char('5') => return Ok(SortOrder::TypeAsc),
                KeyCode::Char('6') => return Ok(SortOrder::TypeDesc),
                KeyCode::Esc => break SortOrder::TypeAsc,
                _ => writeln!(
                    stdout,
                    "Invalid input. Please enter a number between 1 and 6.\r"
                )?,
            }
            let _input = read_line()?;
        }
    })
}
pub fn read_new_path(stdout: &mut impl Write) -> io::Result<String> {
    write!(
        stdout,
        "Enter new home folder path (or leave empty to unset): \r"
    )?;
    stdout.flush()?;
    read_line()
}
pub fn read_new_lines(stdout: &mut impl Write) -> io::Result<usize> {
    loop {
        write!(stdout, "Enter new number of lines to show: \r")?;
        stdout.flush()?;
        let input = read_line()?;
        match input.trim().parse() {
            Ok(lines) if lines > 0 => return Ok(lines),
            _ => writeln!(stdout, "Invalid input. Please enter a positive number.\r")?,
        }
    }
}
fn write_config_line(config_path: &PathBuf, key: &str, value: &str) -> io::Result<()> {
    let content = std::fs::read_to_string(config_path)?;
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    let new_line = format!("{} = {}", key, value);
    if let Some(index) = lines.iter().position(|line| line.starts_with(key)) {
        lines[index] = new_line;
    } else {
        lines.push(new_line);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

pub fn remove_config_line(config_path: &PathBuf, key: &str) -> io::Result<()> {
    let content = std::fs::read_to_string(config_path)?;
    let lines: Vec<String> = content
        .lines()
        .filter(|line| !line.starts_with(key))
        .map(String::from)
        .collect();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

pub fn navigate_to_shortcut(app_state: &mut AppState, key: char) -> io::Result<()> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 1;

    if let Some((path, _, _)) = app_state.get_shortcut(key) {
        if path.is_dir() {
            app_state.current_dir = path.clone();
            Ok(())
        } else {
            queue!(stdout(), MoveTo(preview_width + 34, height - 10))?;
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Shortcut does not point to a directory",
            ))
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Shortcut not found",
        ))
    }
}

pub fn edit_config(
    app_state: &mut AppState,
    stdout: &mut impl Write,
    current_dir: &Path,
) -> io::Result<()> {
    let config_path = Config::get_config_path()?;
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;

    if !config_path.exists() {
        create_default_config(&config_path, app_state)?;
    }

    let mut selected_item = 0;
    let menu_items = vec![
        "Set Current Directory as Home",
        "Default Sort",
        "Remap Keys",
        "Set Text Editor",
        "Set Search Depth Limit",
        "Set Dimming Settings",
        "Undo Settings",
        "Return to Browser",
    ];

    loop {
        let _ = clear_nav();
        let _ = clear_preview();

        let title = "Edit Configuration\r";
        let separator = "=".repeat(title.len());
        execute!(stdout, MoveTo(nav_width / 3, 3))?;
        writeln!(stdout, "{}\r", title.bold().green())?;
        execute!(stdout, MoveTo(nav_width / 3, 4))?;
        writeln!(stdout, "{}\r", separator.green())?;

        execute!(stdout, MoveTo(nav_width / 8, 6))?;
        writeln!(stdout, "{}", "Current Settings:".yellow().bold())?;

        execute!(stdout, MoveTo(nav_width / 8, 8))?;
        writeln!(
            stdout,
            "Home Folder: {}\r",
            app_state
                .config
                .home_folder
                .as_ref()
                .unwrap_or(&app_state.current_dir)
                .to_string_lossy()
                .green()
        )?;

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, MoveTo(nav_width / 8, 10 + i as u16))?;

            if i == selected_item {
                write!(stdout, "{} ", "→".green())?;
            } else {
                write!(stdout, "  ")?;
            }

            match i {
                0 => writeln!(stdout, "{}\r", item.cyan())?,
                1 => writeln!(
                    stdout,
                    "{} (current: {})\r",
                    item.cyan(),
                    app_state.config.default_sort.to_string().green()
                )?,
                2 => writeln!(stdout, "{}\r", item.cyan())?,
                3 => writeln!(
                    stdout,
                    "{} (current: {})\r",
                    item.cyan(),
                    app_state.config.text_editor.clone().green()
                )?,
                4 => writeln!(
                    stdout,
                    "{} (current: {})\r",
                    item.cyan(),
                    app_state.search_depth_limit.to_string().green()
                )?,
                5 => writeln!(
                    stdout,
                    "{} (distance: {}, intensity: {})\r",
                    item.cyan(),
                    app_state.config.max_distance.to_string().green(),
                    app_state.config.dim_step.to_string().green()
                )?,
                6 => writeln!(
                    stdout,
                    "{} (RAM: {} MB, Disk: {} GB, Storage: {})\r",
                    item.cyan(),
                    (app_state.undo_manager.ram_limit / 1_048_576)
                        .to_string()
                        .green(),
                    (app_state.undo_manager.disk_limit / 1_073_741_824)
                        .to_string()
                        .green(),
                    if app_state.undo_manager.allow_disk_storage {
                        "Enabled".green()
                    } else {
                        "Disabled".red()
                    }
                )?,
                _ => writeln!(stdout, "{}\r", item.cyan())?,
            }
        }

        execute!(stdout, MoveTo(preview_width + 2, height - 12))?;
        writeln!(
            stdout,
            "{}\r",
            "-".repeat((preview_width - 4) as usize).green()
        )?;

        execute!(stdout, MoveTo(preview_width + 4, height - 11))?;
        writeln!(stdout, "Use ↑↓ (or k/j) to navigate")?;
        execute!(stdout, MoveTo(preview_width + 4, height - 10))?;
        writeln!(stdout, "Press Enter to select, ESC to return")?;

        execute!(stdout, MoveTo(preview_width + 2, height - 9))?;
        writeln!(
            stdout,
            "{}\r",
            "-".repeat((preview_width - 4) as usize).green()
        )?;

        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    let _ = clear_interaction_field();
                    selected_item = selected_item.saturating_sub(1);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let _ = clear_interaction_field();
                    selected_item = (selected_item + 1).min(menu_items.len() - 1);
                }
                KeyCode::Enter | KeyCode::Char('l') => {
                    match selected_item {
                        0 => {
                            app_state.set_home_folder(Some(current_dir.to_path_buf()))?;
                            interaction_field!(
                                "Home folder set to {}",
                                current_dir.to_string_lossy()
                            )?;
                        }
                        1 => {
                            if let Ok(new_sort) = read_new_sort(stdout) {
                                app_state.config.default_sort = new_sort;
                                app_state.config.save_config()?;
                            }
                        }
                        2 => {
                            if let Some(_) = &app_state.config.keybindings {
                                manage_keybindings(app_state, stdout)?;
                            }
                        }
                        3 => {
                            let _ = interaction_field!("Enter a new text editor command:")?;
                            if let Ok(new_editor) = read_line() {
                                if !new_editor.trim().is_empty() {
                                    app_state.config.text_editor = new_editor;
                                    app_state.config.save_config()?;
                                    interaction_field!("Text editor updated")?;
                                }
                            }
                        }
                        4 => {
                            let _ = interaction_field!("Enter search depth (0 for Unlimited):")?;
                            if let Ok(new_depth) = read_line()?.trim().parse() {
                                app_state.search_depth_limit = new_depth;
                                app_state.config.search_depth_limit = new_depth;
                                app_state.config.save_config()?;
                                interaction_field!("Search depth updated to {}", new_depth)?;
                            } else {
                                interaction_field!("Invalid input. Search depth not changed")?;
                            }
                        }
                        5 => {
                            configure_undo_settings(app_state, stdout)?;
                        }
                        6 | _ => break,
                    }
                    let _ = clear_nav();
                    let _ = clear_preview();
                }
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    let _ = clear_nav();
    let _ = clear_preview();
    Ok(())
}
fn configure_undo_settings(app_state: &mut AppState, stdout: &mut impl Write) -> io::Result<()> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 1;
    let _ = clear_nav();
    let _ = clear_preview();

    let menu_items = vec![
        "RAM Undo Limit",
        "Disk Undo Limit",
        "Toggle Disk Storage",
        "Return to Main Menu",
    ];
    let mut selected_item = 0;

    loop {
        execute!(stdout, MoveTo(nav_width / 3, 7))?;
        writeln!(stdout, "Configure Undo Settings\r")?;
        execute!(stdout, MoveTo(nav_width / 3, 8))?;
        writeln!(stdout, "=====================\r")?;

        for (i, item) in menu_items.iter().enumerate() {
            execute!(stdout, MoveTo(nav_width / 8, 10 + i as u16))?;

            if i == selected_item {
                write!(stdout, "{} ", "→".green())?;
            } else {
                write!(stdout, "  ")?;
            }

            match i {
                0 => writeln!(
                    stdout,
                    "{}: {} MB\r",
                    item,
                    app_state.undo_manager.ram_limit / 1_048_576 //mibs to bibs
                )?,
                1 => writeln!(
                    stdout,
                    "{}: {} GB\r",
                    item,
                    app_state.undo_manager.disk_limit / 1_073_741_824 //gigs to bibs
                )?,
                2 => writeln!(
                    stdout,
                    "{}: {}\r",
                    item,
                    if app_state.undo_manager.allow_disk_storage {
                        "Enabled".green()
                    } else {
                        "Disabled".red()
                    }
                )?,
                _ => writeln!(stdout, "{}\r", item)?,
            }
        }

        execute!(stdout, MoveTo(nav_width / 8, 15))?;
        writeln!(stdout, "Use ↑↓ to navigate, Enter to select\r")?;

        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    selected_item = selected_item.saturating_sub(1);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    selected_item = (selected_item + 1).min(menu_items.len() - 1);
                }
                KeyCode::Enter | KeyCode::Char('l') => {
                    match selected_item {
                        0 => {
                            let _ = clear_nav();
                            let _ = clear_interaction_field();
                            interaction_field!("Enter new RAM limit in MB:")?;

                            queue!(stdout, MoveTo((preview_width * 11 / 8) as u16, height - 10))?;
                            if let Ok(input) = read_line() {
                                if let Ok(limit) = input.trim().parse::<usize>() {
                                    app_state.undo_manager.ram_limit = limit * 1_048_576; // Mibs to bibs
                                    app_state.config.ram_undo_limit =
                                        app_state.undo_manager.ram_limit;
                                    app_state.config.save_config()?;
                                    let _ = clear_interaction_field();
                                    interaction_field!("RAM limit update.\r")?;
                                } else {
                                    let _ = clear_interaction_field();
                                    interaction_field!("Invalid input. enter a number.\r")?;
                                }
                            }
                            let _ = clear_interaction_field();
                            let _ = clear_nav();
                        }
                        1 => {
                            let _ = clear_nav();
                            let _ = clear_interaction_field();
                            interaction_field!("Enter new disk limit in GB: ")?;
                            if let Ok(input) = read_line() {
                                if let Ok(limit) = input.trim().parse::<u64>() {
                                    app_state.undo_manager.disk_limit = limit * 1_073_741_824; // Gigs to bibs
                                    app_state.config.disk_undo_limit =
                                        app_state.undo_manager.disk_limit;
                                    app_state.config.save_config()?;
                                    interaction_field!("Disk limit updated successfully.\r")?;
                                } else {
                                    interaction_field!("Invalid input. Please enter a number.\r")?;
                                }
                            }
                            let _ = clear_interaction_field();
                            let _ = clear_nav();
                        }
                        2 => {
                            app_state.undo_manager.allow_disk_storage =
                                !app_state.undo_manager.allow_disk_storage;
                            app_state.config.allow_disk_undo =
                                app_state.undo_manager.allow_disk_storage;
                            app_state.config.save_config()?;
                            execute!(stdout, MoveTo(nav_width / 8, 16))?;
                            writeln!(
                                stdout,
                                "Disk storage {}.\r",
                                if app_state.undo_manager.allow_disk_storage {
                                    "enabled"
                                } else {
                                    "disabled"
                                }
                            )?;
                            let _ = clear_nav();
                        }
                        3 | _ => break,
                    }
                }
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }
    Ok(())
}
pub fn rename_file(
    stdout: &mut impl Write,
    entry: &FileEntry,
    keep_extension: bool,
    app_state: &mut AppState,
) -> io::Result<()> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;
    // let _ = clear_nav();
    let _ = clear_preview();

    if !app_state.check_operation_allowed(&entry.path, "rename") {
        let _ = clear_interaction_field();
        interaction_field!("Rename not allowed for file: {}\r", entry.name)?;
        return Ok({});
    }
    let (width, _height) = size()?;
    let nav_width = width / 2;
    let start_y = 11;
    execute!(stdout, MoveTo(nav_width + 4, start_y - 3))?;
    let _ = clear_interaction_field();

    let _ = interaction_field!("");
    queue!(
        stdout,
        MoveTo((preview_width * 11 / 10) as u16, height - 10)
    )?;
    writeln!(stdout, "Renaming: {}:", entry.name.clone().green())?;
    execute!(stdout, MoveTo(nav_width + 4, start_y - 2))?;
    writeln!(stdout, "Enter new name (or press Esc to cancel):")?;
    stdout.flush()?;

    let mut new_name = String::new();
    execute!(stdout, MoveTo(nav_width + 4, start_y))?;
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    new_name.push(c);
                    queue!(
                        stdout,
                        MoveTo(
                            (preview_width * 11 / 9)
                                + entry.name.len() as u16
                                + new_name.len() as u16,
                            height - 10
                        )
                    )?;
                    print!("{}", c.red());
                    stdout.flush()?;
                }
                KeyCode::Backspace => {
                    if !new_name.is_empty() {
                        new_name.pop();
                        print!("\x08 \x08");
                        stdout.flush()?;
                    }
                }
                KeyCode::Enter => {
                    if !new_name.is_empty() {
                        let old_path = entry.path.clone();
                        let old_name = old_path.file_name().unwrap().to_string_lossy().into_owned();
                        let parent = old_path.parent().unwrap_or(Path::new(""));

                        let mut new_file_name = new_name.clone();
                        if keep_extension {
                            if let Some(ext) = old_path.extension() {
                                new_file_name.push('.');
                                new_file_name.push_str(ext.to_str().unwrap());
                            }
                        }

                        let new_path = parent.join(&new_file_name);

                        queue!(
                            stdout,
                            MoveTo((preview_width * 11 / 8) + 18 as u16, height - 10)
                        )?;
                        match fs::rename(&old_path, &new_path) {
                            Ok(_) => {
                                app_state.undo_manager.add_tome_entry(UndoEntry {
                                    operation: Operation::Rename {
                                        old_name,
                                        new_name: new_file_name,
                                        path: parent.to_path_buf(),
                                        timestamp: SystemTime::now(),
                                    },
                                    storage: UndoStorage::Ram(Vec::new()),
                                    original_path: new_path,
                                    size: 0,
                                })?;

                                execute!(stdout, MoveTo(nav_width + 4, start_y + 2))?;
                                interaction_field!("File renamed successfully.\r")?;
                            }
                            Err(e) => {
                                execute!(stdout, MoveTo(nav_width + 4, start_y + 2))?;
                                interaction_field!("Error renaming file: {}\r", e)?;
                            }
                        }
                    }
                    break;
                }
                KeyCode::Esc => {
                    execute!(stdout, MoveTo(nav_width + 4, start_y + 3))?;
                    interaction_field!("\nRename cancelled.\r")?;
                    break;
                }
                _ => {}
            }
        }
    }
    stdout.flush()?;
    Ok(())
}
pub fn copy_files(app_state: &mut AppState, entries: &[FileEntry], selected_index: usize) {
    let files_to_copy = if let Some(selected) = &app_state.multiple_selected_files {
        selected.iter().cloned().collect::<Vec<_>>()
    } else if app_state
        .check_operation_allowed(entries[selected_index as usize].path.as_path(), "copy")
    {
        vec![entries[selected_index].path.clone()]
    } else {
        let _ = interaction_field!("Not allowed to be copied");
        return;
    };

    let allowed_files: Vec<PathBuf> = files_to_copy
        .into_iter()
        .filter(|path| app_state.check_operation_allowed(path, "copy"))
        .collect();

    if !allowed_files.is_empty() {
        app_state.clipboard = Some(allowed_files);
        let _ = interaction_field!(
            "{} file(s) copied to clipboard",
            app_state.clipboard.as_ref().unwrap().len()
        );
    } else {
        interaction_field!("No files were copied (permission denied or no selection)").unwrap();
    }
}

pub fn paste_files(app_state: &mut AppState, current_dir: &Path) -> io::Result<()> {
    if let Some(files_to_paste) = &app_state.clipboard {
        for source_path in files_to_paste {
            if !app_state.check_operation_allowed(source_path, "copy") {
                let _ = interaction_field!("Not allowed to be pasted");
                continue;
            }
            let file_name = source_path.file_name().unwrap_or_default();
            let destination = current_dir.join(file_name);

            if source_path.is_dir() {
                copy_dir_all(source_path, &destination)?;
            } else {
                fs::copy(source_path, &destination)?;
            }

            let file_content = if destination.is_file() {
                fs::read(&destination)?
            } else {
                Vec::new()
            };

            app_state.undo_manager.add_tome_entry(UndoEntry {
                operation: Operation::Copy {
                    source_path: source_path.clone(),
                    dest_path: destination.clone(),
                    timestamp: SystemTime::now(),
                },
                storage: UndoStorage::Ram(file_content.clone()),
                original_path: destination,
                size: file_content.len(),
            })?;
        }
    }
    let _ = interaction_field!("Content pasted");
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<u64> {
    let mut total_size = 0;
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let typ = entry.file_type()?;
        if typ.is_dir() {
            total_size += copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            total_size += fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(total_size)
}
pub fn duplicate_files(
    stdout: &mut impl Write,
    app_state: &mut AppState,
    entries: &[FileEntry],
    selected_index: Option<usize>,
) -> io::Result<()> {
    let files_to_duplicate = if let Some(selected) = &app_state.multiple_selected_files {
        selected.iter().cloned().collect::<Vec<_>>()
    } else if app_state.check_operation_allowed(
        entries[selected_index.unwrap() as usize].path.as_path(),
        "copy",
    ) {
        vec![entries[selected_index.unwrap()].path.clone()]
    } else {
        println!("No files were duplicated (permission denied or no selection)");
        return Ok(());
    };

    for path in files_to_duplicate {
        let parent = path.parent().unwrap();
        let file_stem = path.file_stem().unwrap().to_str().unwrap();
        let extension = path
            .extension()
            .map(|ext| ext.to_str().unwrap())
            .unwrap_or("");
        let mut counter = 1;
        let mut new_path = parent.join(format!(
            "{} ({}){}.{}",
            file_stem,
            counter,
            if extension.is_empty() { "" } else { "." },
            extension
        ));
        while new_path.exists() {
            counter += 1;
            new_path = parent.join(format!(
                "{} ({}){}.{}",
                file_stem,
                counter,
                if extension.is_empty() { "" } else { "." },
                extension
            ));
        }

        if path.is_dir() {
            fs::create_dir_all(&new_path)?;
            copy_dir_all(&path, &new_path)?;
        } else {
            fs::copy(&path, &new_path)?;
        }

        let file_content = if new_path.is_file() {
            fs::read(&new_path)?
        } else {
            Vec::new()
        };

        app_state.undo_manager.add_tome_entry(UndoEntry {
            operation: Operation::Duplicate {
                original_path: path.clone(),
                new_path: new_path.clone(),
                timestamp: SystemTime::now(),
            },
            storage: UndoStorage::Ram(file_content.clone()),
            original_path: new_path.clone(),
            size: file_content.len(),
        })?;

        writeln!(
            stdout,
            "Duplicated: {} -> {}\r",
            path.display(),
            new_path.display()
        )?;
    }

    Ok(())
}
pub fn prompt_line_amount(current_lines: usize, page_state: &PageState) -> io::Result<usize> {
    let mut stdout = stdout();
    let mut lines = current_lines;
    let mut input_buffer = String::new();
    let mut input_mode = false;

    loop {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
        println!("How many lines do you want to display?\r");
        println!("Current selection: {}\r", lines.to_string().green());
        println!("Enter a number (min 20, max 100k)\r");
        println!(
            "Press Enter to confirm, 'r' to reset to current ({}), or Esc to cancel\r",
            current_lines
        );

        if input_mode {
            println!("Enter number: {}", input_buffer);
        }

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    input_mode = true;
                    input_buffer.push(c);
                }
                KeyCode::Up if !input_mode => lines = lines.saturating_add(1).min(100_000),
                KeyCode::Down if !input_mode => lines = lines.saturating_sub(1).max(20),
                KeyCode::Backspace if input_mode => {
                    input_buffer.pop();
                    if input_buffer.is_empty() {
                        input_mode = false;
                    }
                }
                KeyCode::Enter => {
                    if input_mode {
                        if let Ok(num) = input_buffer.parse::<usize>() {
                            lines = num.clamp(20, 100_000);
                        }
                    }
                    execute!(stdout, Clear(ClearType::All))?;

                    let _ = draw_initial_border(&mut stdout, page_state);
                    return Ok(lines);
                }
                KeyCode::Char('r') => {
                    lines = current_lines;
                    input_mode = false;
                    input_buffer.clear();
                }
                KeyCode::Esc => {
                    if input_mode {
                        input_mode = false;
                        input_buffer.clear();
                    } else {
                        execute!(stdout, Clear(ClearType::All))?;
                        let _ = draw_initial_border(&mut stdout, page_state);
                        return Ok(current_lines);
                    }
                }
                _ => {}
            }
        }

        if input_mode && !input_buffer.is_empty() {
            if let Ok(num) = input_buffer.parse::<usize>() {
                lines = num.clamp(20, 100_000);
            }
        }
    }
}

pub fn parse_key_event(s: &str) -> KeyEvent {
    let parts: Vec<&str> = s.split('+').collect();
    let mut modifiers = KeyModifiers::empty();
    let key_code = parts.last().unwrap_or(&"");

    for part in &parts[..parts.len().saturating_sub(1)] {
        match *part {
            "Shift" => modifiers.insert(KeyModifiers::SHIFT),
            "Ctrl" => modifiers.insert(KeyModifiers::CONTROL),
            "Alt" => modifiers.insert(KeyModifiers::ALT),
            _ => {}
        }
    }

    let code = match *key_code {
        "Up" => KeyCode::Up,
        "Down" => KeyCode::Down,
        "Left" => KeyCode::Left,
        "Right" => KeyCode::Right,
        "Enter" => KeyCode::Enter,
        "Esc" => KeyCode::Esc,
        "F1" => KeyCode::F(1),
        " " => KeyCode::Char(' '),
        c if c.len() == 1 => KeyCode::Char(c.chars().next().unwrap()),
        _ => KeyCode::Null,
    };

    KeyEvent::new(code, modifiers)
}

pub fn key_event_to_string(key: &KeyEvent) -> String {
    let mut parts = Vec::new();

    if key.modifiers.contains(KeyModifiers::SHIFT) {
        parts.push("Shift".to_string());
    }
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        parts.push("Ctrl".to_string());
    }
    if key.modifiers.contains(KeyModifiers::ALT) {
        parts.push("Alt".to_string());
    }

    parts.push(match key.code {
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::Up => "Up".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::PageUp => "PageUp".to_string(),
        KeyCode::PageDown => "PageDown".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::Delete => "Delete".to_string(),
        KeyCode::Insert => "Insert".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Char(' ') => "[SPACE]".to_string(),
        KeyCode::Char(c) => c.to_string(),
        KeyCode::F(n) => format!("F{}", n),
        _ => format!("{:?}", key.code),
    });

    // format!("\"{}\" ({:?}, )", parts.join("+"), key.code)
    format!("\"{}\"", parts.join("+"))
}

fn copy_to_storage(path: &Path) -> io::Result<(UndoStorage, usize)> {
    if path.is_file() {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        let size = file.read_to_end(&mut buffer)?;
        Ok((UndoStorage::Ram(buffer), size))
    } else {
        let temp_dir = std::env::temp_dir().join("file_manager_undo_temp");
        fs::create_dir_all(&temp_dir)?;
        let temp_path = temp_dir.join(path.file_name().unwrap());
        let size = copy_dir_all(path, &temp_path)?;
        Ok((UndoStorage::Disk(temp_path), size as usize))
    }
}

pub fn murder_files(
    app_state: &mut AppState,
    stdout: &mut impl Write,
    entries: &[FileEntry],
    selected_index: usize,
    skip_confirm: bool,
) -> io::Result<()> {
    let files_to_delete = if let Some(selected) = &app_state.multiple_selected_files {
        selected.iter().cloned().collect::<Vec<_>>()
    } else {
        vec![entries[selected_index].path.clone()]
    };

    let _ = clear_interaction_field();
    if files_to_delete.is_empty() {
        interaction_field!("No files selected for deletion.\r")?;
        return Ok(());
    }

    let mut allowed_files = Vec::new();
    for path in &files_to_delete {
        if app_state.check_operation_allowed(path, "delete") {
            allowed_files.push(path);
        } else {
            let _ = clear_interaction_field();
            interaction_field!("Deletion not allowed\r",)?;
        }
    }

    if allowed_files.is_empty() {
        return Ok(());
    }

    if !skip_confirm {
        let _ = clear_interaction_field();
        interaction_field!(
            "Are you sure you want to delete {} file(s)? (y/n)\r",
            allowed_files.len()
        )?;
        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            if key.code != KeyCode::Char('y') {
                let _ = clear_interaction_field();
                interaction_field!("Deletion cancelled.\r")?;
                return Ok(());
            }
        }
    }

    for path in &allowed_files {
        let (storage, size) = copy_to_storage(path)?;
        app_state.undo_manager.add_tome_entry(UndoEntry {
            operation: Operation::Delete {
                timestamp: SystemTime::now(),
            },
            storage,
            original_path: path.to_path_buf(),
            size,
        })?;

        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    let _ = clear_interaction_field();
    interaction_field!("{} file(s) deleted successfully.\r", allowed_files.len())?;
    app_state.clear_selection();

    Ok(())
}

use std::fmt::write;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
pub fn copy_to_temp(path: &Path, temp_dir: &Path) -> io::Result<(PathBuf, u64)> {
    // Generate unique timestamp-based filename
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros();

    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;

    let temp_path = temp_dir.join(format!("{}_{}", timestamp, file_name));

    #[cfg(windows)]
    let temp_path = if temp_path.to_string_lossy().len() > 260 {
        PathBuf::from(format!("\\\\?\\{}", temp_path.to_string_lossy()))
    } else {
        temp_path
    };

    let mut total_size = 0;

    if path.is_dir() {
        fs::create_dir_all(&temp_path)?;

        for entry in fs::read_dir(path)?.filter_map(Result::ok) {
            let source_path = entry.path();
            let relative_path = source_path
                .strip_prefix(path)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let dest_path = temp_path.join(relative_path);

            #[cfg(windows)]
            let dest_path = if dest_path.to_string_lossy().len() > 260 {
                PathBuf::from(format!("\\\\?\\{}", dest_path.to_string_lossy()))
            } else {
                dest_path
            };

            if source_path.is_dir() {
                fs::create_dir_all(&dest_path)?;
                let entries = fs::read_dir(&source_path)?
                    .filter_map(Result::ok)
                    .collect::<Vec<_>>();

                for sub_entry in entries {
                    let sub_source = sub_entry.path();
                    let sub_dest = dest_path.join(sub_source.clone());

                    #[cfg(windows)]
                    {
                        if let Ok(metadata) = sub_source.metadata() {
                            if metadata.file_attributes() & 0x2 != 0 {
                                continue;
                            }
                        }
                    }

                    if sub_source.is_file() {
                        total_size += fs::copy(&sub_source, &sub_dest)?;
                    }
                }
            } else {
                #[cfg(windows)]
                {
                    if let Ok(metadata) = source_path.metadata() {
                        if metadata.file_attributes() & 0x2 != 0 {
                            continue;
                        }
                    }
                }

                total_size += fs::copy(&source_path, &dest_path)?;
            }
        }
    } else {
        #[cfg(windows)]
        {
            if let Ok(metadata) = path.metadata() {
                if metadata.file_attributes() & 0x2 != 0 {
                    return Err(io::Error::new(
                        std::io::ErrorKind::PermissionDenied,
                        "Cannot copy hidden file",
                    ));
                }
            }
        }

        total_size = fs::copy(path, &temp_path)?;

        #[cfg(windows)]
        {
            // hopefully is writable on Windows
            if let Ok(metadata) = temp_path.metadata() {
                let mut perms = metadata.permissions();
                perms.set_readonly(false);
                fs::set_permissions(&temp_path, perms)?;
            }
        }
    }

    Ok((temp_path, total_size))
}
fn copy_dir_contents(src: &Path, dst: &Path) -> io::Result<u64> {
    let mut total_size = 0;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if entry_path.is_dir() {
            fs::create_dir(&dst_path)?;
            total_size += copy_dir_contents(&entry_path, &dst_path)?;
        } else {
            let size = fs::copy(&entry_path, &dst_path)?;
            total_size += size;
        }
    }
    Ok(total_size)
}
pub fn undo_last_operation(app_state: &mut AppState, stdout: &mut impl Write) -> io::Result<()> {
    if let Some(entry) = app_state.undo_manager.undo_haunting_regret() {
        match entry.operation {
            Operation::Move {
                old_path, new_path, ..
            } => {
                if new_path.exists() {
                    if old_path.exists() {
                        let _ = clear_interaction_field();
                        interaction_field!(
                            "Cannot undo move: both source and destination exist.\r"
                        )?;
                    } else {
                        match fs::rename(&new_path, &old_path) {
                            Ok(_) => {
                                let _ = clear_interaction_field();
                                interaction_field!(
                                    "Undid move. Moved back: {} -> {}\r",
                                    new_path.display(),
                                    old_path.display()
                                )?;
                                if let Some(parent) = old_path.parent() {
                                    app_state.current_dir = parent.to_path_buf();
                                }
                            }
                            Err(e) => {
                                let _ = clear_interaction_field();
                                interaction_field!("Error undoing move: {}\r", e)?;
                            }
                        }
                    }
                } else {
                    let _ = clear_interaction_field();
                    interaction_field!("Cannot undo move: destination no longer exists.\r")?;
                }
            }
            Operation::Delete { .. } => {
                match entry.storage {
                    UndoStorage::Ram(data) => {
                        let mut file = File::create(&entry.original_path)?;
                        file.write_all(&data)?;
                    }
                    UndoStorage::Disk(temp_path) => {
                        fs::rename(temp_path, &entry.original_path)?;
                    }
                }
                let _ = clear_interaction_field();
                interaction_field!("Undid deletion. File/directory restored.\r")?;
            }
            Operation::Rename {
                path,
                old_name,
                new_name,
                ..
            } => {
                let current_path = path.join(&new_name);
                let old_path = path.join(&old_name);
                fs::rename(&current_path, &old_path)?;
                let _ = clear_interaction_field();
                interaction_field!(
                    "Undid rename. File/directory name restored to '{}'.\r",
                    old_name
                )?;
            }
            Operation::Create {
                path, is_directory, ..
            } => {
                if is_directory {
                    let _ = fs::remove_dir_all(path);
                } else {
                    let _ = fs::remove_file(path);
                }
            }
            Operation::Copy {
                source_path: _,
                dest_path,
                ..
            } => {
                if dest_path.is_dir() {
                    fs::remove_dir_all(&dest_path)?;
                } else {
                    fs::remove_file(&dest_path)?;
                }
                let _ = clear_interaction_field();
                interaction_field!("Undid copy. Removed: {}\r", dest_path.display())?;
            }
            Operation::Duplicate {
                original_path: _,
                new_path,
                ..
            } => {
                if new_path.is_dir() {
                    fs::remove_dir_all(&new_path)?;
                } else {
                    fs::remove_file(&new_path)?;
                }
                let _ = clear_interaction_field();
                interaction_field!("Undid duplication. Removed: {}\r", new_path.display())?;
            }
        }
        Ok(())
    } else {
        let _ = clear_interaction_field();
        interaction_field!("No operations to undo.\r")?;
        Ok(())
    }
}

pub fn execute_terminal_command(
    app_state: &mut AppState,
    stdout: &mut impl Write,
) -> io::Result<()> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;
    let start_y = 5;
    let max_lines = height - 15;
    let mut terminal_output: VecDeque<(String, bool)> = VecDeque::new();
    let mut autocomplete = Autocomplete::new();
    let dimming_config = DimmingConfig::new(5, &app_state.config); // visible suggestions. maybe more is better?
    let _ = clear_preview();
    // let _ = clear_interaction_field();
    queue!(stdout, MoveTo(preview_width + 2, height - 13))?;
    write!(stdout, "{}", "-".repeat((preview_width - 4).into()).green())?;
    queue!(stdout, MoveTo(preview_width + 2, height - 7))?;
    write!(stdout, "{}", "-".repeat((preview_width - 4).into()).green())?;

    execute!(stdout, cursor::Show)?;

    queue!(stdout, MoveTo(nav_width + 4, height - 10))?;
    write!(stdout, "> ")?;
    stdout.flush()?;

    fn display_suggestions(
        stdout: &mut impl Write,
        suggestions: &[String],
        current_index: usize,
        nav_width: u16,
        height: u16,
        dimming_config: &DimmingConfig,
        command_buffer: &str,
    ) -> io::Result<()> {
        let max_suggestions = 5;
        let suggestion_x = nav_width + 2;
        let suggestion_y = height - 10;

        for i in 0..max_suggestions {
            queue!(stdout, MoveTo(suggestion_x, suggestion_y + i as u16 - 2))?;
            write!(stdout, "{}", " ".repeat(40))?;
        }

        queue!(stdout, MoveTo(nav_width + 4, height - 10))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;
        write!(stdout, ">")?;

        for (i, c) in command_buffer.chars().enumerate() {
            let color = if i % 2 == 0 {
                Color::Rgb {
                    r: 200,
                    g: 100,
                    b: 200,
                }
            } else {
                Color::Rgb {
                    r: 150,
                    g: 250,
                    b: 150,
                }
            };
            execute!(stdout, SetForegroundColor(color))?;
            write!(stdout, "{}", c)?;
        }

        queue!(stdout, MoveTo(nav_width + 4, height - 10))?;
        execute!(stdout, SetForegroundColor(Color::Green))?;
        write!(stdout, ">")?;
        execute!(
            stdout,
            SetForegroundColor(Color::Rgb {
                r: 255,
                g: 255,
                b: 255
            })
        )?;
        write!(stdout, " {}", command_buffer)?;

        if suggestions.len() == 1 {
            let suggestion = &suggestions[0];
            let words: Vec<&str> = command_buffer.split_whitespace().collect();
            if let Some(last_word) = words.last() {
                let completion = if suggestion.starts_with(last_word) {
                    &suggestion[last_word.len()..]
                } else {
                    suggestion
                };

                queue!(
                    stdout,
                    MoveTo(nav_width + 4 + command_buffer.len() as u16 + 2, height - 10),
                    SetForegroundColor(Color::DarkGrey)
                )?;
                write!(stdout, "{}", completion)?;
            }
            return Ok(());
        }

        let window_size = max_suggestions;
        let half_window = window_size / 2;

        let start_index = if current_index >= half_window {
            if current_index + half_window < suggestions.len() {
                current_index - half_window
            } else {
                suggestions.len().saturating_sub(window_size)
            }
        } else {
            0
        };

        let get_color = |distance: i32| -> Color {
            match distance.abs() {
                0 => Color::Rgb {
                    r: 150,
                    g: 255,
                    b: 150,
                },
                1 => Color::Rgb {
                    r: 100,
                    g: 150,
                    b: 100,
                },
                2 => Color::Rgb {
                    r: 70,
                    g: 100,
                    b: 70,
                },
                _ => Color::Rgb {
                    r: 50,
                    g: 70,
                    b: 50,
                },
            }
        };

        for (i, suggestion) in suggestions
            .iter()
            .skip(start_index)
            .take(window_size)
            .enumerate()
        {
            let display_index = start_index + i;
            let relative_pos = display_index as i32 - current_index as i32;
            let color = get_color(relative_pos);

            queue!(stdout, MoveTo(suggestion_x, suggestion_y + i as u16 - 2))?;

            if display_index == current_index {
                execute!(stdout, SetForegroundColor(color))?;
                write!(stdout, "→ {}\r", suggestion.clone().dark_green())?;
                write!(stdout, "{}", " ".repeat(suggestion.len() / 35))?;
            } else {
                execute!(stdout, SetForegroundColor(color))?;
                write!(stdout, "  {}\r", suggestion)?;
            }
        }

        execute!(stdout, SetForegroundColor(Color::Reset))?;
        stdout.flush()?;
        Ok(())
    }
    fn handle_command(
        command: &str,
        current_dir: &mut Path,
        selected_index: &mut usize,
        app_state: &mut AppState,
        terminal_output: &mut VecDeque<(String, bool)>,
        stdout: &mut impl Write,
    ) -> io::Result<()> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        let sort = app_state.config.default_sort.clone();
        if parts.is_empty() {
            return Ok(());
        }

        match parts[0] {
            ".." => {
                let _ = handle_move_left(
                    app_state,
                    &mut current_dir.to_path_buf(),
                    selected_index,
                    &mut 0,
                    &mut false,
                    &sort,
                    stdout,
                );

                if let Ok(entries) = fs::read_dir(&app_state.current_dir) {
                    let entries: Vec<FileEntry> = entries
                        .filter_map(|entry| entry.ok())
                        .filter_map(|entry| FileEntry::new(entry.path()).ok())
                        .collect();
                    display_directory(
                        app_state,
                        &entries,
                        &app_state.current_dir.clone(),
                        app_state.selected_index,
                        stdout,
                        app_state.scroll_state.offset,
                        app_state.lines,
                        true,
                    )?;
                }
            }
            "cd" => {
                let new_dir = if parts.len() < 2 {
                    return Ok(());
                } else if parts[1] == ".." {
                    if let Some(parent) = app_state.current_dir.parent() {
                        parent.to_path_buf()
                    } else {
                        app_state.current_dir.clone()
                    }
                } else {
                    let path = PathBuf::from(parts[1]);
                    if path.is_absolute() {
                        path
                    } else {
                        app_state.current_dir.join(path)
                    }
                };

                match fs::canonicalize(&new_dir) {
                    Ok(canonical_path) => {
                        app_state.last_browsed_dir = app_state.current_dir.clone();
                        app_state.current_dir = canonical_path;
                        app_state.selected_index = 0;
                        app_state.scroll_state.offset = 0;

                        if let Ok(entries) = fs::read_dir(&app_state.current_dir) {
                            let entries: Vec<FileEntry> = entries
                                .filter_map(|entry| entry.ok())
                                .filter_map(|entry| FileEntry::new(entry.path()).ok())
                                .collect();

                            display_directory(
                                app_state,
                                &entries,
                                &app_state.current_dir.clone(),
                                app_state.selected_index,
                                stdout,
                                app_state.scroll_state.offset,
                                app_state.lines,
                                true,
                            )?;
                        }
                    }
                    Err(e) => {
                        terminal_output.push_back((format!("cd: {}: {}", parts[1], e), true));
                    }
                }
            }
            _ => {
                match Command::new(parts[0])
                    .args(&parts[1..])
                    .current_dir(&app_state.current_dir)
                    .output()
                {
                    Ok(output) => {
                        if !output.stdout.is_empty() {
                            String::from_utf8_lossy(&output.stdout)
                                .lines()
                                .for_each(|line| {
                                    terminal_output.push_back((line.to_string(), false));
                                });
                        }
                        if !output.stderr.is_empty() {
                            String::from_utf8_lossy(&output.stderr)
                                .lines()
                                .for_each(|line| {
                                    terminal_output.push_back((line.to_string(), true));
                                });
                        }

                        if let Ok(entries) = fs::read_dir(&app_state.current_dir) {
                            let entries: Vec<FileEntry> = entries
                                .filter_map(|entry| entry.ok())
                                .filter_map(|entry| FileEntry::new(entry.path()).ok())
                                .collect();

                            display_directory(
                                app_state,
                                &entries,
                                &app_state.current_dir.clone(),
                                app_state.selected_index,
                                stdout,
                                app_state.scroll_state.offset,
                                app_state.lines,
                                true,
                            )?;
                        }
                    }
                    Err(e) => {
                        terminal_output.push_back((format!("Error: {}", e), true));
                    }
                }
            }
        }
        Ok(())
    }
    queue!(stdout, MoveTo(preview_width + 2, height - 13))?;
    write!(stdout, "{}", "-".repeat((preview_width - 4).into()).green())?;
    queue!(stdout, MoveTo(preview_width + 2, height - 7))?;
    write!(stdout, "{}", "-".repeat((preview_width - 4).into()).green())?;
    let mut command_buffer = String::new();
    let mut last_tab_word = String::new();
    let mut tab_count = 0;
    while let Ok(Event::Key(key)) = event::read() {
        match key.code {
            KeyCode::Esc => {
                execute!(stdout, cursor::Hide)?;
                let _ = clear_interaction_field();

                if let Ok(entries) = fs::read_dir(&app_state.last_browsed_dir) {
                    let entries: Vec<FileEntry> = entries
                        .filter_map(|entry| entry.ok())
                        .filter_map(|entry| FileEntry::new(entry.path()).ok())
                        .collect();

                    display_directory(
                        app_state,
                        &entries,
                        &app_state.current_dir.clone(),
                        app_state.selected_index,
                        stdout,
                        app_state.scroll_state.offset,
                        app_state.lines,
                        true,
                    )?;
                }
                return Ok(());
            }
            KeyCode::Tab => {
                if autocomplete.suggestions.is_empty() {
                    autocomplete.get_suggestions(&command_buffer, &app_state.current_dir)?;
                }

                if !autocomplete.suggestions.is_empty() {
                    let words: Vec<&str> = command_buffer.split_whitespace().collect();
                    let current_word = words.last().map_or("", |w| *w);

                    if tab_count == 0 || last_tab_word != current_word {
                        last_tab_word = current_word.to_string();
                        tab_count = 0;
                    }

                    autocomplete.next_suggestion();
                    tab_count += 1;

                    if let Some(suggestion) = autocomplete.get_current_suggestion() {
                        if let Some(last_word) = words.last() {
                            let prefix = command_buffer[..command_buffer.len() - last_word.len()]
                                .to_string();
                            command_buffer = format!("{}{}", prefix, suggestion);
                        } else {
                            command_buffer = suggestion.clone();
                        }

                        queue!(stdout, MoveTo(nav_width + 4, height - 10))?;
                        write!(stdout, "> {}", command_buffer)?;
                    }

                    display_suggestions(
                        stdout,
                        &autocomplete.suggestions,
                        autocomplete.current_index,
                        nav_width,
                        height,
                        &dimming_config,
                        &command_buffer,
                    )?;
                }
            }
            KeyCode::Char(c) => {
                command_buffer.push(c);
                tab_count = 0;

                queue!(stdout, MoveTo(nav_width + 4, height - 10))?;
                execute!(stdout, SetForegroundColor(Color::Green))?;
                write!(stdout, ">")?;
                execute!(
                    stdout,
                    SetForegroundColor(Color::Rgb {
                        r: 200,
                        g: 255,
                        b: 200
                    })
                )?;
                write!(stdout, " {}", command_buffer)?;

                autocomplete.get_suggestions(&command_buffer, &app_state.current_dir)?;
                display_suggestions(
                    stdout,
                    &autocomplete.suggestions,
                    autocomplete.current_index,
                    nav_width,
                    height,
                    &dimming_config,
                    &command_buffer,
                )?;
            }
            KeyCode::Backspace => {
                if !command_buffer.is_empty() {
                    command_buffer.pop();
                    tab_count = 0;

                    queue!(stdout, MoveTo(nav_width + 4, height - 10))?;
                    write!(stdout, "{}", " ".repeat(40))?;

                    queue!(stdout, MoveTo(nav_width + 4, height - 10))?;
                    execute!(stdout, SetForegroundColor(Color::Green))?;
                    write!(stdout, ">")?;
                    execute!(
                        stdout,
                        SetForegroundColor(Color::Rgb {
                            r: 200,
                            g: 255,
                            b: 200
                        })
                    )?;
                    write!(stdout, " {}", command_buffer)?;

                    autocomplete.get_suggestions(&command_buffer, &app_state.current_dir)?;
                    display_suggestions(
                        stdout,
                        &autocomplete.suggestions,
                        autocomplete.current_index,
                        nav_width,
                        height,
                        &dimming_config,
                        &command_buffer,
                    )?;
                }
            }
            KeyCode::Down => {
                autocomplete.next_suggestion();
                display_suggestions(
                    stdout,
                    &autocomplete.suggestions,
                    autocomplete.current_index,
                    nav_width,
                    height,
                    &dimming_config,
                    &command_buffer,
                )?;
            }
            KeyCode::Up => {
                autocomplete.prev_suggestion();
                display_suggestions(
                    stdout,
                    &autocomplete.suggestions,
                    autocomplete.current_index,
                    nav_width,
                    height,
                    &dimming_config,
                    &command_buffer,
                )?;
            }
            KeyCode::Enter => {
                let command = if autocomplete.showing_suggestions
                    && !autocomplete.suggestions.is_empty()
                {
                    if let Some(suggestion) = autocomplete.get_current_suggestion() {
                        let words: Vec<&str> = command_buffer.split_whitespace().collect();
                        if let Some(last_word) = words.last() {
                            let prefix = command_buffer[..command_buffer.len() - last_word.len()]
                                .to_string();
                            format!("{}{}", prefix, suggestion)
                        } else {
                            suggestion.clone()
                        }
                    } else {
                        command_buffer.trim().to_string()
                    }
                } else {
                    command_buffer.trim().to_string()
                };

                while terminal_output.len() > 1000 {
                    terminal_output.pop_front();
                }

                handle_command(
                    &command,
                    &mut app_state.current_dir.clone(),
                    &mut 0,
                    app_state,
                    &mut terminal_output,
                    stdout,
                )?;
                let _ = clear_preview();
                queue!(stdout, MoveTo(nav_width + 4, (height - 14) as u16))?;
                terminal_output.push_back((format!("> {}", command.clone().red()), false));
                for (i, (line, is_error)) in terminal_output
                    .iter()
                    .rev()
                    .take((max_lines - 2) as usize)
                    .enumerate()
                {
                    queue!(stdout, MoveTo(nav_width + 4, (height - 15) - (i) as u16))?;
                    if *is_error {
                        execute!(stdout, SetForegroundColor(Color::Red))?;
                        write!(stdout, "{}\r", line)?;
                        execute!(stdout, SetForegroundColor(Color::Reset))?;
                    } else {
                        write!(stdout, "{}\r", line.clone().green())?;
                    }
                }

                command_buffer.clear();
                queue!(stdout, MoveTo(preview_width + 2, height - 13))?;
                write!(stdout, "{}", "-".repeat((preview_width - 4).into()).green())?;
                queue!(stdout, MoveTo(preview_width + 2, height - 7))?;
                write!(stdout, "{}", "-".repeat((preview_width - 4).into()).green())?;
                queue!(stdout, MoveTo(nav_width + 4, height - 10))?;
                write!(stdout, "{}", "> ")?;
                autocomplete.showing_suggestions = false;
            }
            _ => {}
        }
        stdout.flush()?;
    }
    Ok(())
}
pub fn open_terminal_command(app_state: &mut AppState, stdout: &mut impl Write) -> io::Result<()> {
    let original_dir = env::current_dir()?;

    cleanup_terminal()?;
    execute!(stdout, LeaveAlternateScreen)?;

    let shell = env::var("SHELL").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {
            "cmd".to_string()
        } else {
            "/bin/sh".to_string()
        }
    });

    println!("Entering shell mode. Use your shell's exit command to return to StygianSift.");

    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(format!(
                "cd {} && {}",
                app_state.current_dir.display(),
                &shell
            ))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?
    } else {
        Command::new(&shell)
            // .arg("-c")
            .arg(format!(
                "cd {} && exec {}",
                app_state.current_dir.display(),
                &shell
            ))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?
    };

    if !status.success() {
        eprintln!("Shell exited with error status");
    }

    if let Ok(new_dir) = env::current_dir() {
        update_navigation_stack(app_state, new_dir.clone());
        app_state.current_dir = new_dir;
    }

    env::set_current_dir(&original_dir)?;

    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    draw_initial_border(stdout, &app_state.page_state)?;

    Ok(())
}

fn update_navigation_stack(app_state: &mut AppState, new_dir: PathBuf) {
    if new_dir != app_state.current_dir {
        app_state.nav_stack.push(NavigationInfo {
            dir_name: app_state
                .current_dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string(),
            index: 0, //
        });

        app_state.current_dir = new_dir;
        app_state.last_browsed_dir = app_state.current_dir.clone();

        while app_state.nav_stack.len()
            > app_state
                .nav_stack
                .iter()
                .position(|info| {
                    info.dir_name
                        == app_state
                            .current_dir
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("")
                })
                .unwrap_or(app_state.nav_stack.len())
                + 1
        {
            app_state.nav_stack.pop();
        }
    }
}

////////////////////////////////////////////////////////UNDOMANAGER////////////////////////////////////////////////////////////////////////////////
pub enum UndoStorage {
    Ram(Vec<u8>),
    Disk(PathBuf),
}
pub struct UndoEntry {
    pub operation: Operation,
    pub storage: UndoStorage,
    pub original_path: PathBuf,
    pub size: usize,
}

pub enum Operation {
    Create {
        path: PathBuf,
        is_directory: bool,
        timestamp: SystemTime,
    },
    Delete {
        timestamp: SystemTime,
    },
    Move {
        old_path: PathBuf,
        new_path: PathBuf,
        // is_directory: bool,
        timestamp: SystemTime,
    },
    Duplicate {
        original_path: PathBuf,
        new_path: PathBuf,
        timestamp: SystemTime,
    },
    Copy {
        source_path: PathBuf,
        dest_path: PathBuf,
        timestamp: SystemTime,
    },
    Rename {
        old_name: String,
        new_name: String,
        path: PathBuf,
        timestamp: SystemTime,
    },
}

pub struct UndoManager {
    pub entries: VecDeque<UndoEntry>,
    pub ram_storage: Vec<u8>,
    pub temp_dir: PathBuf,
    pub ram_limit: usize,
    pub disk_limit: u64,
    pub total_disk_size: u64,
    pub allow_disk_storage: bool,
    pub move_operations: VecDeque<UndoEntry>,
}

impl UndoManager {
    pub fn new(
        temp_dir: PathBuf,
        ram_limit: usize,
        disk_limit: u64,
        allow_disk_storage: bool,
    ) -> io::Result<Self> {
        fs::create_dir_all(&temp_dir)?;
        Ok(UndoManager {
            entries: VecDeque::new(),
            ram_storage: Vec::with_capacity(ram_limit),
            temp_dir,
            ram_limit,
            disk_limit,
            total_disk_size: 0,
            allow_disk_storage,
            move_operations: VecDeque::new(),
        })
    }

    pub fn add_move_operation(&mut self, old_path: PathBuf, new_path: PathBuf) -> io::Result<()> {
        let entry = UndoEntry {
            operation: Operation::Move {
                old_path: old_path.clone(),
                new_path,
                timestamp: SystemTime::now(),
            },
            storage: UndoStorage::Ram(Vec::new()), // Move operations don't need storage, unless to big i guess, then we might need a temp folder.
            original_path: old_path,
            size: 0, // Size is not relevant for move operations
        };
        self.add_tome_entry(entry)
    }

    pub fn undo_flirty_move(&mut self) -> Option<UndoEntry> {
        self.move_operations.pop_back()
    }

    pub fn add_tome_entry(&mut self, entry: UndoEntry) -> io::Result<()> {
        if entry.size <= self.ram_limit - self.ram_storage.len() {
            if let UndoStorage::Ram(data) = &entry.storage {
                self.ram_storage.extend(data);
            }
            self.entries.push_back(entry);
        } else if self.allow_disk_storage {
            let disk_path = if let UndoStorage::Ram(data) = &entry.storage {
                Some(self.store_to_disk(data)?)
            } else {
                None
            };

            if let Some(path) = disk_path {
                self.total_disk_size += entry.size as u64;
                self.entries.push_back(UndoEntry {
                    storage: UndoStorage::Disk(path),
                    ..entry
                });
            } else {
                self.entries.push_back(entry);
            }

            while self.total_disk_size > self.disk_limit {
                if let Some(removed_entry) = self.entries.pop_front() {
                    self.remove_entry(removed_entry)?;
                }
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Undo storage limit reached",
            ));
        }
        Ok(())
    }

    pub fn undo_haunting_regret(&mut self) -> Option<UndoEntry> {
        self.entries.pop_back().map(|entry| match entry.storage {
            UndoStorage::Ram(_) => {
                let start = self.ram_storage.len().saturating_sub(entry.size);
                let data = self.ram_storage.split_off(start);
                UndoEntry {
                    storage: UndoStorage::Ram(data),
                    ..entry
                }
            }
            UndoStorage::Disk(_) => {
                self.total_disk_size = self.total_disk_size.saturating_sub(entry.size as u64);
                entry
            }
        })
    }
    fn store_to_disk(&self, data: &[u8]) -> io::Result<PathBuf> {
        let file_name = format!(
            "undo_{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros()
        );
        let path = self.temp_dir.join(file_name);
        let mut file = File::create(&path)?;
        file.write_all(data)?;
        Ok(path)
    }

    fn remove_entry(&mut self, entry: UndoEntry) -> io::Result<()> {
        match entry.storage {
            UndoStorage::Ram(_) => {
                self.ram_storage
                    .truncate(self.ram_storage.len() - entry.size);
            }
            UndoStorage::Disk(path) => {
                fs::remove_file(path)?;
                self.total_disk_size -= entry.size as u64;
            }
        }
        Ok(())
    }
    pub fn undo_by_type(&mut self, operation_type: &str) -> Option<UndoEntry> {
        let index = self.entries.iter().rposition(|entry| {
            matches!(
                (&entry.operation, operation_type),
                (Operation::Delete { .. }, "delete")
                    | (Operation::Move { .. }, "move")
                    | (Operation::Duplicate { .. }, "duplicate")
                    | (Operation::Copy { .. }, "copy")
                    | (Operation::Rename { .. }, "rename")
            )
        })?;

        let entry = self.entries.remove(index).unwrap();

        match entry.storage {
            UndoStorage::Ram(_) => {
                let start = self.ram_storage.len().saturating_sub(entry.size);
                let data = self.ram_storage.split_off(start);
                Some(UndoEntry {
                    storage: UndoStorage::Ram(data),
                    ..entry
                })
            }
            UndoStorage::Disk(_) => {
                self.total_disk_size = self.total_disk_size.saturating_sub(entry.size as u64);
                Some(entry)
            }
        }
    }
}
pub fn set_color_rules(app_state: &mut AppState, stdout: &mut impl Write) -> io::Result<()> {
    let (width, height) = size()?;
    let nav_width = width / 2;

    loop {
        clear_nav()?;
        clear_preview()?;

        execute!(stdout, MoveTo(nav_width / 3 + 5, 9))?;
        writeln!(stdout, "{}", "Set Color Rules\r".green())?;
        execute!(stdout, MoveTo(nav_width / 3 + 5, 10))?;
        writeln!(stdout, "===============\r")?;

        let colors = [
            MarkerColor::Red,
            MarkerColor::Orange,
            MarkerColor::Yellow,
            MarkerColor::Blue,
            MarkerColor::Cyan,
            MarkerColor::Pink,
            MarkerColor::White,
            MarkerColor::Green,
            MarkerColor::Magenta,
            MarkerColor::Reset,
        ];
        for (i, color) in colors.iter().enumerate() {
            let rule = app_state
                .get_color_rule(&color.clone())
                .cloned()
                .unwrap_or_default();
            execute!(stdout, MoveTo(nav_width / 12, 12 + i as u16))?;
            execute!(stdout, SetForegroundColor(color.to_color()))?;
            if *color == MarkerColor::Red {
                write!(stdout, "1. {:<10}", color.as_str())?;
            } else {
                write!(stdout, "{}. {:<10}", i + 1, color.as_str())?;
            }
            execute!(stdout, ResetColor)?;
            writeln!(
                stdout,
                "Delete: {}, Rename: {}, Move: {}, Copy: {}, Search: {}\r",
                if rule.allow_delete {
                    "Yes".green()
                } else {
                    "No".red()
                },
                if rule.allow_rename {
                    "Yes".green()
                } else {
                    "No".red()
                },
                if rule.allow_move {
                    "Yes".green()
                } else {
                    "No".red()
                },
                if rule.allow_copy {
                    "Yes".green()
                } else {
                    "No".red()
                },
                if rule.include_in_search {
                    "Yes".green()
                } else {
                    "No".red()
                }
            )?;
        }

        execute!(stdout, MoveTo(nav_width / 8 + 5, height - 5))?;
        writeln!(
            stdout,
            "{}",
            "Enter color number to edit (1-8), press or escape to quit: \r".green()
        )?;
        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('1') => {
                    edit_color_rule(app_state, stdout, MarkerColor::Red).expect("color_error")
                }
                KeyCode::Char('2') => {
                    edit_color_rule(app_state, stdout, MarkerColor::Orange).expect("color_error")
                }
                KeyCode::Char('3') => {
                    edit_color_rule(app_state, stdout, MarkerColor::Yellow).expect("color_error")
                }
                KeyCode::Char('4') => {
                    edit_color_rule(app_state, stdout, MarkerColor::Blue).expect("color_error")
                }
                KeyCode::Char('5') => {
                    edit_color_rule(app_state, stdout, MarkerColor::Cyan).expect("color_error")
                }
                KeyCode::Char('6') => {
                    edit_color_rule(app_state, stdout, MarkerColor::Pink).expect("color_error")
                }
                KeyCode::Char('7') => {
                    edit_color_rule(app_state, stdout, MarkerColor::White).expect("color_error")
                }
                KeyCode::Char('8') => {
                    edit_color_rule(app_state, stdout, MarkerColor::Green).expect("color_error")
                }
                KeyCode::Char('9') => {
                    edit_color_rule(app_state, stdout, MarkerColor::Magenta).expect("color_error")
                }
                KeyCode::Char('0') => {
                    edit_color_rule(app_state, stdout, MarkerColor::Reset).expect("color_error")
                }
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }
    let _ = clear_nav();
    let _ = clear_preview();
    app_state.config.save_config()?;
    Ok(())
}

fn edit_color_rule(
    app_state: &mut AppState,
    stdout: &mut impl Write,
    color: MarkerColor,
) -> io::Result<()> {
    let mut rule = app_state
        .get_color_rule(&color)
        .cloned()
        .unwrap_or_default();
    let (width, _) = size()?;
    loop {
        clear_preview()?;
        execute!(stdout, MoveTo(width / 2 + 5, 10))?;
        writeln!(stdout, "Editing rules for {} color\r", color.as_str())?;
        execute!(stdout, MoveTo(width / 2 + 5, 12))?;
        writeln!(
            stdout,
            "1. Allow Delete: {}\r",
            if rule.allow_delete {
                "Yes".green()
            } else {
                "No".red()
            }
        )?;
        execute!(stdout, MoveTo(width / 2 + 5, 13))?;
        writeln!(
            stdout,
            "2. Allow Rename: {}\r",
            if rule.allow_rename {
                "Yes".green()
            } else {
                "No".red()
            }
        )?;
        execute!(stdout, MoveTo(width / 2 + 5, 14))?;
        writeln!(
            stdout,
            "3. Allow Move: {}\r",
            if rule.allow_move {
                "Yes".green()
            } else {
                "No".red()
            }
        )?;
        execute!(stdout, MoveTo(width / 2 + 5, 15))?;
        writeln!(
            stdout,
            "4. Allow Copy: {}\r",
            if rule.allow_copy {
                "Yes".green()
            } else {
                "No".red()
            }
        )?;
        execute!(stdout, MoveTo(width / 2 + 5, 16))?;
        writeln!(
            stdout,
            "5. Include in Search: {}\r",
            if rule.include_in_search {
                "Yes".green()
            } else {
                "No".red()
            }
        )?;
        execute!(stdout, MoveTo(width / 2 + 5, 18))?;
        writeln!(stdout, "Press enter or escape to go back: \r")?;
        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('1') => rule.allow_delete = !rule.allow_delete,
                KeyCode::Char('2') => rule.allow_rename = !rule.allow_rename,
                KeyCode::Char('3') => rule.allow_move = !rule.allow_move,
                KeyCode::Char('4') => rule.allow_copy = !rule.allow_copy,
                KeyCode::Char('5') => rule.include_in_search = !rule.include_in_search,
                KeyCode::Enter | KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    app_state.set_color_rule(color, rule);
    Ok(())
}
struct Autocomplete {
    suggestions: Vec<String>,
    current_index: usize,
    last_word: String,
    showing_suggestions: bool,
}
impl Autocomplete {
    fn new() -> Self {
        Self {
            suggestions: Vec::new(),
            current_index: 0,
            last_word: String::new(),
            showing_suggestions: false,
        }
    }

    fn get_suggestions(&mut self, input: &str, current_dir: &Path) -> io::Result<()> {
        self.suggestions.clear();
        let words: Vec<&str> = input.split_whitespace().collect();
        let current_word = words.last().unwrap_or(&"").to_lowercase();
        self.last_word = current_word.clone();

        if words.len() <= 1 {
            let builtin_commands = ["cd", "ls", "mkdir", "touch", "rm", "cp", "mv"];
            for cmd in builtin_commands.iter() {
                if cmd.starts_with(&current_word) {
                    self.suggestions.push((*cmd).to_string());
                }
            }

            if let Ok(path) = std::env::var("PATH") {
                for path_entry in path.split(':') {
                    if let Ok(entries) = fs::read_dir(path_entry) {
                        for entry in entries.filter_map(Result::ok) {
                            if let Ok(file_name) = entry.file_name().into_string() {
                                if file_name.to_lowercase().starts_with(&current_word) {
                                    self.suggestions.push(file_name);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if let Ok(entries) = fs::read_dir(current_dir) {
                for entry in entries.filter_map(Result::ok) {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        if file_name.to_lowercase().starts_with(&current_word) {
                            let mut suggestion = file_name;
                            if entry.file_type()?.is_dir() {
                                suggestion.push('/');
                            }
                            self.suggestions.push(suggestion);
                        }
                    }
                }
            }
        }

        self.suggestions.sort();
        self.suggestions.dedup();
        self.current_index = 0;
        self.showing_suggestions = !self.suggestions.is_empty();
        Ok(())
    }

    fn get_current_suggestion(&self) -> Option<&String> {
        self.suggestions.get(self.current_index)
    }

    fn next_suggestion(&mut self) {
        if !self.suggestions.is_empty() {
            self.current_index = (self.current_index + 1) % self.suggestions.len();
        }
    }

    fn prev_suggestion(&mut self) {
        if !self.suggestions.is_empty() {
            self.current_index = self
                .current_index
                .checked_sub(1)
                .unwrap_or(self.suggestions.len() - 1);
        }
    }
}
