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
    writeln!(
        stdout,
        "Opening file in {}. Press F12 to exit when done.\r",
        editor
    )?;
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

    let (width, _height) = size()?;
    let nav_width = width / 2;
    let _preview_width = width - nav_width - 2;
    if !config_path.exists() {
        create_default_config(&config_path, app_state)?;
    }
    let _ = clear_nav();
    let _ = clear_preview();
    // let (width, height) = size()?;

    loop {
        // execute!(stdout, Clear(ClearType::All))?;

        // execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, MoveTo(nav_width / 3, 7))?;
        writeln!(stdout, "Edit Configuration\r")?;
        execute!(stdout, MoveTo(nav_width / 3, 8))?;
        writeln!(stdout, "===================\r")?;
        execute!(stdout, MoveTo(nav_width / 8, 10))?;
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
        execute!(stdout, MoveTo(nav_width / 8, 12))?;
        writeln!(stdout, "{}. Set Current Directory as Home\r", "1".green())?;

        execute!(stdout, MoveTo(nav_width / 8, 13))?;
        writeln!(
            stdout,
            "{}. Lines Shown: {}\r",
            "2".green(),
            app_state.config.lines_shown
        )?;
        execute!(stdout, MoveTo(nav_width / 8, 14))?;
        writeln!(
            stdout,
            "{}. Default Sort: {:?}\r",
            "3".green(),
            app_state.config.default_sort
        )?;
        execute!(stdout, MoveTo(nav_width / 8, 15))?;
        writeln!(stdout, "{}. Remap Keys\r", "4".green())?;
        execute!(stdout, MoveTo(nav_width / 8, 16))?;
        writeln!(stdout, "{}. Manage Shortcuts\r", "5".green())?;
        execute!(stdout, MoveTo(nav_width / 8, 17))?;
        writeln!(
            stdout,
            "{}. Set Text Editor (current: {})\r",
            "6".green(),
            app_state.config.text_editor
        )?;
        execute!(stdout, MoveTo(nav_width / 8, 18))?;
        writeln!(
            stdout,
            "{}. Set Search Depth Limit (current: {})\r",
            "7".green(),
            app_state.search_depth_limit
        )?;
        execute!(stdout, MoveTo(nav_width / 8, 19))?;
        writeln!(stdout, "{}. Exit\r", "Esc".green())?;
        execute!(stdout, MoveTo(nav_width / 8, 20))?;
        writeln!(stdout, "\nEnter your choice (1-7): \r")?;

        stdout.flush()?;
        execute!(stdout, MoveTo(nav_width / 8, 23))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('1') => {
                    app_state.set_home_folder(Some(current_dir.to_path_buf()))?;
                    execute!(stdout, MoveTo(nav_width / 8, 24))?;
                    writeln!(
                        stdout,
                        "Current directory set as home folder: {:?}\r",
                        current_dir
                    )?;
                }
                KeyCode::Char('2') => match read_new_lines(stdout) {
                    Ok(new_lines) => {
                        app_state.config.lines_shown = new_lines;
                        app_state.save_config()?;
                    }
                    Err(e) => writeln!(stdout, "Error: {}\r", e)?,
                },
                KeyCode::Char('3') => match read_new_sort(stdout) {
                    Ok(new_sort) => {
                        app_state.config.default_sort = new_sort;
                        app_state.save_config()?;
                    }
                    Err(e) => writeln!(stdout, "Error: {}\r", e)?,
                },

                KeyCode::Char('4') => {
                    if let Some(_key_bindings) = &app_state.config.keybindings {
                        let _ = manage_keybindings(app_state, stdout);
                    }
                }
                KeyCode::Char('5') => {
                    if let Some(shortcuts) = &app_state.config.shortcuts {
                        for (key, path) in shortcuts {
                            execute!(stdout, MoveTo(nav_width / 8, 24))?;
                            writeln!(stdout, "Shortcut {}: {}\r", key, path.0.display())?;
                        }
                    } else {
                        execute!(stdout, MoveTo(nav_width / 8, 24))?;
                        writeln!(stdout, "No shortcuts set\r")?;
                    }
                    manage_shortcuts(app_state, stdout)?;
                    let _ = clear_nav();
                    let _ = clear_preview();
                }
                KeyCode::Char('6') => {
                    execute!(stdout, MoveTo(nav_width / 8, 24))?;
                    write!(stdout, "Enter new text editor command: \r")?;
                    stdout.flush()?;
                    execute!(stdout, MoveTo(nav_width / 8, 25))?;
                    let new_editor = read_line()?;
                    if !new_editor.is_empty() {
                        app_state.config.text_editor = new_editor;
                        app_state.config.save_config()?;
                        execute!(stdout, MoveTo(nav_width / 8, 26))?;
                        writeln!(stdout, "Text editor updated successfully.\r")?;
                    }
                }
                KeyCode::Char('7') => {
                    execute!(stdout, MoveTo(nav_width / 8, 24))?;
                    write!(
                        stdout,
                        "Enter new search depth limit (0 No depth search): \r"
                    )?;
                    stdout.flush()?;
                    execute!(stdout, MoveTo(nav_width / 8, 25))?;
                    let new_depth = read_line()?;
                    if let Ok(depth) = new_depth.trim().parse() {
                        app_state.search_depth_limit = depth;
                        app_state.config.search_depth_limit = depth;
                        app_state.config.save_config()?;
                        execute!(stdout, MoveTo(nav_width / 8, 26))?;
                        writeln!(stdout, "Search depth limit updated to: {}\r", depth)?;
                    } else {
                        execute!(stdout, MoveTo(nav_width / 8, 26))?;
                        writeln!(stdout, "Invalid input. Search depth limit not changed.\r")?;
                    }
                }
                KeyCode::F(3) | KeyCode::Esc => {
                    let _ = clear_nav();
                    let _ = clear_preview();
                    break;
                }
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
    let _preview_width = width - nav_width - 2;
    // let _ = clear_nav();
    let _ = clear_preview();

    if !app_state.check_operation_allowed(&entry.path, "rename") {
        queue!(stdout, MoveTo(nav_width + 20, 7 + height / 2 + 1 as u16))?;
        writeln!(stdout, "Rename not allowed for file: {}\r", entry.name)?;
        return Ok({});
    }
    let (width, _height) = size()?;
    let nav_width = width / 2;
    let start_y = 11;
    execute!(stdout, MoveTo(nav_width + 4, start_y - 3))?;
    writeln!(stdout, "Renaming file: {}\r", entry.name)?;
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
                    print!("{}", c);
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
                                writeln!(stdout, "\nFile renamed successfully.\r")?;
                            }
                            Err(e) => {
                                execute!(stdout, MoveTo(nav_width + 4, start_y + 2))?;
                                writeln!(stdout, "\nError renaming file: {}\r", e)?;
                            }
                        }
                    }
                    break;
                }
                KeyCode::Esc => {
                    execute!(stdout, MoveTo(nav_width + 4, start_y + 3))?;
                    writeln!(stdout, "\nRename cancelled.\r")?;
                    break;
                }
                _ => {}
            }
        }
    }
    execute!(stdout, MoveTo(nav_width + 4, start_y + 3))?;
    writeln!(stdout, "Press any key to continue...\r")?;
    stdout.flush()?;
    event::read()?;
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
        return;
    };

    let allowed_files: Vec<PathBuf> = files_to_copy
        .into_iter()
        .filter(|path| app_state.check_operation_allowed(path, "copy"))
        .collect();

    if !allowed_files.is_empty() {
        app_state.clipboard = Some(allowed_files);
        println!(
            "{} file(s) copied to clipboard",
            app_state.clipboard.as_ref().unwrap().len()
        );
    } else {
        println!("No files were copied (permission denied or no selection)");
    }
}

pub fn paste_files(app_state: &mut AppState, current_dir: &Path) -> io::Result<()> {
    if let Some(files_to_paste) = &app_state.clipboard {
        for source_path in files_to_paste {
            if !app_state.check_operation_allowed(source_path, "copy") {
                continue;
            }
            let file_name = source_path.file_name().unwrap_or_default();
            let destination = current_dir.join(file_name);

            if source_path.is_dir() {
                copy_dir_all(source_path, &destination)?;
            } else {
                fs::copy(source_path, &destination)?;
            }

            // For undo entry
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
) -> io::Result<()> {
    let files_to_delete = if let Some(selected) = &app_state.multiple_selected_files {
        selected.iter().cloned().collect::<Vec<_>>()
    } else {
        vec![entries[selected_index].path.clone()]
    };

    if files_to_delete.is_empty() {
        writeln!(stdout, "No files selected for deletion.\r")?;
        return Ok(());
    }

    let mut allowed_files = Vec::new();
    for path in &files_to_delete {
        if app_state.check_operation_allowed(path, "delete") {
            allowed_files.push(path);
        } else {
            writeln!(
                stdout,
                "Deletion not allowed for file: {}\r",
                path.display()
            )?;
        }
    }

    if allowed_files.is_empty() {
        writeln!(stdout, "No files are allowed to be deleted.\r")?;
        return Ok(());
    }

    write!(
        stdout,
        "Are you sure you want to delete {} file(s)? (y/n)\r",
        allowed_files.len()
    )?;
    stdout.flush()?;

    if let Event::Key(key) = event::read()? {
        if key.code == KeyCode::Char('y') {
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
            writeln!(
                stdout,
                "{} file(s) deleted successfully.\r",
                allowed_files.len()
            )?;
            app_state.clear_selection();
        } else {
            writeln!(stdout, "Deletion cancelled.\r")?;
        }
    }
    Ok(())
}

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
                        ErrorKind::PermissionDenied,
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
                        writeln!(
                            stdout,
                            "Cannot undo move: both source and destination exist.\r"
                        )?;
                    } else {
                        match fs::rename(&new_path, &old_path) {
                            Ok(_) => {
                                writeln!(
                                    stdout,
                                    "Undid move. Moved back: {} -> {}\r",
                                    new_path.display(),
                                    old_path.display()
                                )?;
                                if let Some(parent) = old_path.parent() {
                                    app_state.current_dir = parent.to_path_buf();
                                }
                            }
                            Err(e) => {
                                writeln!(stdout, "Error undoing move: {}\r", e)?;
                            }
                        }
                    }
                } else {
                    writeln!(stdout, "Cannot undo move: destination no longer exists.\r")?;
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
                writeln!(stdout, "Undid deletion. File/directory restored.\r")?;
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
                writeln!(
                    stdout,
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
                writeln!(stdout, "Undid copy. Removed: {}\r", dest_path.display())?;
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
                writeln!(
                    stdout,
                    "Undid duplication. Removed: {}\r",
                    new_path.display()
                )?;
            }
        }
        Ok(())
    } else {
        writeln!(stdout, "No operations to undo.\r")?;
        Ok(())
    }
}

pub fn open_terminal_command(app_state: &mut AppState, stdout: &mut impl Write) -> io::Result<()> {
    let current_dir = app_state.current_dir.clone();

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;

    let shell = env::var("SHELL").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {
            "cmd".to_string()
        } else {
            "/bin/sh".to_string()
        }
    });

    println!("Entering shell mode. Use your shell's exit command to return to StygianSift.");

    let _status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(&shell)
            .current_dir(&app_state.current_dir)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?
    } else {
        Command::new(&shell)
            .current_dir(&app_state.current_dir)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?
    };

    if let Ok(new_dir) = env::current_dir() {
        update_navigation_stack(app_state, new_dir.clone());
        app_state.current_dir = new_dir;
    } else {
        app_state.current_dir = current_dir;
    }
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

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
