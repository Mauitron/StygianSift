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

use std::usize;

use super::*;
pub fn handle_multi_select(
    app_state: &mut AppState,
    selected_index: &mut usize,
    scroll_offset: &mut usize,
    entries: &[FileEntry],
    action: Action,
    middle_line: usize,
    visible_lines: usize,
) {
    let new_index = if matches!(action, Action::MultiSelectUp) {
        selected_index.saturating_sub(1)
    } else {
        (*selected_index + 1).min(entries.len().saturating_sub(1))
    };

    if app_state.selection_amont.is_none() {
        app_state.selection_amont = Some(*selected_index);
    }

    let _ = app_state.select_range(
        *selected_index,
        app_state.selection_amont.unwrap(),
        new_index,
        entries,
    );

    *selected_index = new_index;

    if *selected_index >= *scroll_offset + middle_line
        && *scroll_offset + visible_lines < entries.len()
    {
        *scroll_offset = scroll_offset.saturating_add(1);
    } else if *selected_index < *scroll_offset + middle_line && *scroll_offset > 0 {
        *scroll_offset = scroll_offset.saturating_sub(1);
    }
}
pub fn handle_layer_switch(
    app_state: &mut AppState,
    layer_index: usize,
    stdout: &mut impl Write,
    preview_width: u16,
    height: u16,
) -> io::Result<()> {
    let layer_name = app_state.config.shortcut_layers[layer_index].name.clone();
    app_state.config.current_layer = layer_index;

    queue!(stdout, MoveTo(preview_width + 3, height - 12))?;
    write!(
        stdout,
        "{}",
        "-".green().to_string().repeat((preview_width - 4).into())
    )?;

    queue!(stdout, MoveTo(preview_width + 28, height - 10))?;
    writeln!(stdout, "Switched to shortcut layer: {}", layer_name.green())?;

    queue!(stdout, MoveTo(preview_width + 3, height - 8))?;
    write!(
        stdout,
        "{}",
        "-".green().to_string().repeat((preview_width - 4).into())
    )?;
    let _ = clear_interaction_field();
    app_state.config.save_config()?;
    Ok(())
}

pub fn handle_layer_rename(
    app_state: &mut AppState,
    stdout: &mut impl Write,
    preview_width: u16,
    height: u16,
) -> io::Result<()> {
    let current_layer = app_state.config.current_layer;

    let _ = interaction_field!("Enter new layer name: ");
    stdout.flush()?;

    queue!(stdout, MoveTo(preview_width + 45, height - 10))?;
    queue!(stdout, SetForegroundColor(Color::Red))?;
    let new_name = read_line()?;
    if !new_name.trim().is_empty() {
        app_state
            .config
            .rename_layer(current_layer, new_name.trim().to_string())?;
        app_state.config.save_config()?;
    }
    let _ = clear_interaction_field();
    queue!(stdout, SetForegroundColor(Color::Reset))?;
    Ok(())
}

pub fn handle_layer_actions(
    action: &Action,
    stdout: &mut impl Write,
    preview_width: u16,
    height: u16,
    app_state: &mut AppState,
) -> io::Result<()> {
    match action {
        Action::SwitchLayer0 => handle_layer_switch(app_state, 0, stdout, preview_width, height),
        Action::SwitchLayer1 => handle_layer_switch(app_state, 1, stdout, preview_width, height),
        Action::SwitchLayer2 => handle_layer_switch(app_state, 2, stdout, preview_width, height),
        Action::SwitchLayer3 => handle_layer_switch(app_state, 3, stdout, preview_width, height),
        Action::SwitchLayer4 => handle_layer_switch(app_state, 4, stdout, preview_width, height),
        Action::SwitchLayer5 => handle_layer_switch(app_state, 5, stdout, preview_width, height),
        Action::SwitchLayer6 => handle_layer_switch(app_state, 6, stdout, preview_width, height),
        Action::SwitchLayer7 => handle_layer_switch(app_state, 7, stdout, preview_width, height),
        Action::SwitchLayer8 => handle_layer_switch(app_state, 8, stdout, preview_width, height),
        Action::SwitchLayer9 => handle_layer_switch(app_state, 9, stdout, preview_width, height),
        Action::RenameLayer => handle_layer_rename(app_state, stdout, preview_width, height),
        _ => Ok(()),
    }
}
pub fn handle_set_shortcut(
    app_state: &mut AppState,
    stdout: &mut impl Write,
    current_dir: &PathBuf,
    selected_index: usize,
    action: Action,
    height: u16,
    preview_width: u16,
) -> io::Result<()> {
    let shortcut_key = match action {
        Action::SetShortcut1 => '1',
        Action::SetShortcut2 => '2',
        Action::SetShortcut3 => '3',
        Action::SetShortcut4 => '4',
        Action::SetShortcut5 => '5',
        Action::SetShortcut6 => '6',
        Action::SetShortcut7 => '7',
        Action::SetShortcut8 => '8',
        Action::SetShortcut9 => '9',
        Action::SetShortcut0 => '0',
        _ => unreachable!(),
    };

    interaction_field!("Enter a name for this shortcut:")?;
    queue!(stdout, SetForegroundColor(Color::Red))?;
    queue!(
        stdout,
        MoveTo((preview_width * 11 / 8) + 18 as u16, height - 10)
    )?;
    let shortcut_name = read_line()?.red();

    queue!(stdout, SetForegroundColor(Color::Reset))?;

    let current_layer = app_state.config.current_layer;
    app_state.config.set_shortcut_in_layer(
        current_layer,
        shortcut_key,
        current_dir.clone(),
        shortcut_name.to_string(),
        selected_index,
    )?;
    app_state.config.save_config()?;

    let layer_name = &app_state.config.shortcut_layers[current_layer].name;
    queue!(stdout, MoveTo(preview_width + 3, height - 10))?;
    writeln!(
        stdout,
        "'{}' set to current directory in layer '{}': {}",
        shortcut_key.red(),
        layer_name.clone().green(),
        current_dir.to_string_lossy().green(),
    )?;

    Ok(())
}
pub fn handle_use_shortcut(
    app_state: &mut AppState,
    current_dir: &mut PathBuf,
    selected_index: &mut usize,
    scroll_offset: &mut usize,
    action: Action,
    sort_order: &SortOrder,
    stdout: &mut impl Write,
    height: u16,
) -> io::Result<()> {
    let (width, _) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;
    let c = match action {
        Action::UseShortcut1 => '1',
        Action::UseShortcut2 => '2',
        Action::UseShortcut3 => '3',
        Action::UseShortcut4 => '4',
        Action::UseShortcut5 => '5',
        Action::UseShortcut6 => '6',
        Action::UseShortcut7 => '7',
        Action::UseShortcut8 => '8',
        Action::UseShortcut9 => '9',
        Action::UseShortcut0 => '0',
        _ => unreachable!(),
    };

    let current_layer = app_state.config.current_layer;
    if let Some((path, _, index)) = app_state.config.get_shortcut_from_layer(current_layer, c) {
        if path.is_dir() {
            *current_dir = path.clone();
            app_state.current_dir = current_dir.clone();
            *selected_index = *index;
            *scroll_offset = 0;
            let _ = clear_nav();
        } else if let Some(parent) = path.parent() {
            *current_dir = parent.to_path_buf();
            app_state.current_dir = current_dir.clone();
            let entries = get_sorted_entries(&app_state, &current_dir, sort_order)?;
            *selected_index = entries.iter().position(|e| e.path == *path).unwrap_or(0);
            *scroll_offset = 0;
        } else {
            let _ = clear_interaction_field();
            interaction_field!("{}", "Error: Invalid shortcut path".red())?;
        }
    } else {
        let _ = clear_interaction_field();
        return interaction_field!(
            "No shortcut '{}' in layer '{}'",
            c.green(),
            app_state.config.shortcut_layers[current_layer].name
        );
    }
    Ok(())
}
pub fn handle_quit(app_state: &mut AppState, stdout: &mut impl Write) -> io::Result<bool> {
    if app_state.multiple_selected_files.iter().count() > 0 {
        app_state.clear_multi_select()?;
        return Ok(false);
    }

    let _ = clear_interaction_field();
    if app_state.is_moving {
        let (width, height) = size()?;
        let nav_width = width / 2;
        let preview_width = width - nav_width;
        app_state.cancel_move();

        let _ = clear_interaction_field();
        interaction_field!("File move cancelled.")?;
        Ok(false)
    } else {
        let (width, height) = size()?;
        let nav_width = width / 2;
        let preview_width = width - nav_width;
        queue!(stdout, MoveTo(preview_width, height - 12))?;
        write!(stdout, "{}", "-".repeat((preview_width - 6).into()).green())?;
        queue!(stdout, MoveTo((preview_width * 11) / 8, height - 10),)?;
        write!(stdout, "Press {} you want to quit", "Y".red())?;
        queue!(stdout, MoveTo(preview_width, height - 8))?;
        write!(stdout, "{}", "-".repeat((preview_width - 6).into()).green())?;

        stdout.flush()?;

        let mut buffer = [0; 1];
        io::stdin().read_exact(&mut buffer)?;
        let input = buffer[0] as char;

        match input.to_ascii_lowercase() {
            'y' => {
                execute!(stdout, DisableMouseCapture)?;
                system_functions::cleanup_terminal()?;
                stdout.flush()?;
                Ok(true)
            }
            _ => {
                let (width, _height) = size()?;
                let nav_width = width / 2;
                let _preview_width = width - nav_width - 2;
                let _ = clear_interaction_field();
                stdout.flush()?;
                Ok(false)
            }
        }
    }
}

pub fn handle_search_files(
    app_state: &mut AppState,
    current_dir: &mut PathBuf,
    selected_index: &mut usize,
    scroll_offset: &mut usize,
    stdout: &mut impl Write,
    sort_order: &SortOrder,
) -> io::Result<()> {
    app_state.is_search = true;
    let search_term = read_search_input(stdout, app_state)?;
    if !search_term.is_empty() {
        let search_results = fuzzy_search_entries(app_state, &search_term)?;
        if let Ok(Some(selected_path)) = display_search_results(app_state, search_results, stdout) {
            app_state.nav_stack.push(NavigationInfo {
                dir_name: app_state
                    .current_dir
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string(),
                index: *selected_index,
            });
            if selected_path.is_dir() {
                app_state.current_dir = selected_path;
                *selected_index = 0;
            } else if let Some(parent) = selected_path.parent() {
                app_state.current_dir = parent.to_path_buf();
                let entries = get_sorted_entries(&app_state, &app_state.current_dir, sort_order)?;
                *selected_index = entries
                    .iter()
                    .position(|e| e.path == selected_path)
                    .map(|i| i)
                    .unwrap_or(0);
            }
            *current_dir = app_state.current_dir.clone();
            *scroll_offset = 0;
            app_state.last_browsed_dir = app_state.current_dir.clone();
        }
    }
    Ok(())
}

pub fn handle_open_in_editor(
    app_state: &AppState,
    entries: &[FileEntry],
    selected_index: usize,
    stdout: &mut impl Write,
    height: u16,
    g_pressed: &mut bool,
) -> io::Result<()> {
    if !*g_pressed {
        if let Some(entry) = entries.get(selected_index as usize) {
            if entry.file_type != FileType::Directory {
                match open_file_with_editor(&entry.path, &app_state.config.text_editor, stdout) {
                    Ok(_) => {
                        terminal::enable_raw_mode()?;
                        execute!(
                            stdout,
                            terminal::EnterAlternateScreen,
                            terminal::Clear(ClearType::All),
                            cursor::Hide
                        )?;
                    }
                    Err(e) => {
                        let _ = clear_interaction_field();
                        interaction_field!("Error opening file: {}", e)?;
                    }
                }
            }
        }
    }
    *g_pressed = false;
    Ok(())
}

pub fn handle_duplicate(
    app_state: &mut AppState,
    stdout: &mut impl Write,
    entries: &[FileEntry],
    selected_index: usize,
) -> io::Result<()> {
    let _ = clear_interaction_field();
    if let Some(_entry) = entries.get(selected_index as usize) {
        match duplicate_files(stdout, app_state, entries, Some(selected_index)) {
            Ok(_) => {
                let _ = clear_interaction_field();
                interaction_field!("File duplicated successfully.")?;
            }
            Err(e) => {
                let _ = clear_interaction_field();
                interaction_field!("Error duplicating file: {}", e)?;
            }
        }
        let _ = clear_nav();
    }
    Ok(())
}

pub fn handle_move_item(
    app_state: &mut AppState,
    entries: &[FileEntry],
    selected_index: usize,
    current_dir: &PathBuf,
) -> io::Result<()> {
    let (width, _height) = size()?;
    let nav_width = width / 2;
    let _preview_width = width - nav_width - 2;
    let _ = clear_interaction_field();
    if let Some(entry) = entries.get(selected_index as usize) {
        if app_state.is_moving {
            match app_state.finish_move(current_dir) {
                Ok(_) => {
                    let _ = clear_interaction_field();
                    interaction_field!("File(s) moved successfully.")?;
                }
                Err(e) => {
                    let _ = clear_interaction_field();
                    interaction_field!("Error moving file(s): {}", e)?;
                    app_state.cancel_move();
                }
            }
        } else {
            match app_state.start_move(Some(entry)) {
                Ok(_) => {
                    let _ = clear_interaction_field();
                    interaction_field!(
                        "Moving file(s). Press 'm' again to place them in the current directory."
                    )?;
                }
                Err(e) => {
                    let _ = clear_interaction_field();
                    interaction_field!("Error: {}", e)?;
                }
            }
        }
        // let _ = clear_nav();
    }
    Ok(())
}

pub fn handle_git_menu(
    app_state: &mut AppState,
    stdout: &mut impl Write,
    current_dir: &PathBuf,
    nav_width: u16,
    preview_width: u16,
    start_y: u16,
    end_y: u16,
    height: u16,
) -> io::Result<()> {
    if is_git_repository(current_dir) {
        if app_state.git_menu.is_none() {
            app_state.git_menu = Some(GitMenu::new());
        }

        loop {
            if let Some(git_menu) = &app_state.git_menu {
                display_git_menu(git_menu, stdout, nav_width, preview_width, start_y, end_y)?;
            }

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        if let Some(git_menu) = &mut app_state.git_menu {
                            git_menu.move_up();
                        }
                    }
                    KeyCode::Down => {
                        if let Some(git_menu) = &mut app_state.git_menu {
                            git_menu.move_down();
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(git_menu) = &app_state.git_menu {
                            let command = git_menu.get_selected_command().to_string();
                            app_state.execute_git_command(&command)?;
                        }
                    }
                    KeyCode::Esc => {
                        app_state.git_menu = None;
                        break;
                    }
                    _ => {}
                }
            }
        }
    } else {
        let _ = clear_interaction_field();
        interaction_field!("Not a Git repository")?;
    }
    Ok(())
}
pub fn handle_sort_cycle(
    app_state: &mut AppState,
    sort_order: &mut SortOrder,
    selected_index: &mut usize,
    scroll_offset: &mut usize,
) {
    app_state.config.default_sort = match app_state.config.default_sort {
        SortOrder::NameAsc => SortOrder::NameDesc,
        SortOrder::NameDesc => SortOrder::TypeAsc,
        SortOrder::TypeAsc => SortOrder::TypeDesc,
        SortOrder::TypeDesc => SortOrder::SizeAsc,
        SortOrder::SizeAsc => SortOrder::SizeDesc,
        SortOrder::SizeDesc => SortOrder::DateModifiedAsc,
        SortOrder::DateModifiedAsc => SortOrder::DateModifiedDesc,
        SortOrder::DateModifiedDesc => SortOrder::ColorAsc,
        SortOrder::ColorAsc => SortOrder::ColorDesc,
        SortOrder::ColorDesc => SortOrder::NameAsc,
    };
    *sort_order = app_state.config.default_sort.clone();
    *selected_index = 0;
    *scroll_offset = 0;

    let _ = clear_nav();
}

pub fn handle_move_right(
    app_state: &mut AppState,
    current_dir: &mut PathBuf,
    selected_index: &mut usize,
    scroll_offset: &mut usize,
    preview_active: &mut bool,
    entries: &[FileEntry],
    stdout: &mut impl Write,
) -> io::Result<()> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 1;
    if let Some(entry) = entries.get(*selected_index as usize) {
        if entry.file_type == FileType::Directory {
            match fs::read_dir(&entry.path) {
                Ok(_) => {
                    if let Some(nav_info) = app_state.nav_stack.pop() {
                        if nav_info.dir_name == entry.name {
                            *selected_index = nav_info.index;
                        } else {
                            app_state.nav_stack.push(nav_info);
                            app_state.nav_stack.push(NavigationInfo {
                                dir_name: current_dir
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .map(String::from)
                                    .unwrap_or_default(),
                                index: *selected_index,
                            });
                            *selected_index = 0;
                        }
                    } else {
                        *selected_index = 0;
                    }

                    *current_dir = entry.path.clone();
                    app_state.current_dir = current_dir.clone();
                    *scroll_offset = 0;
                    *preview_active = false;
                    app_state.page_state.move_right();
                    let _ = clear_nav();
                    stdout.flush()?;
                }
                Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                    match handle_permission_issue(stdout, &entry.name, &entry.path)? {
                        Some(contents) => {
                            let current_dir_name = current_dir
                                .file_name()
                                .and_then(|n| n.to_str())
                                .map(String::from)
                                .unwrap_or_default();
                            app_state.nav_stack.push(NavigationInfo {
                                dir_name: current_dir_name,
                                index: *selected_index,
                            });

                            *scroll_offset = 0;
                            *preview_active = false;
                            let _last_entries = parse_ls_output(&contents, current_dir);
                        }
                        None => {}
                    }
                }
                Err(e) => return Err(e),
            }
        }
        if !app_state.is_search && !app_state.preview_active && !app_state.select_mode {
            queue!(stdout, MoveTo(preview_width + 34, height - 10))?;
            writeln!(stdout, "                              ",)?;
            queue!(stdout, MoveTo(preview_width + 1, height - 8))?;
        }
    }
    Ok(())
}

pub fn handle_move_left(
    app_state: &mut AppState,
    current_dir: &mut PathBuf,
    selected_index: &mut usize,
    scroll_offset: &mut usize,
    preview_active: &mut bool,
    sort_order: &SortOrder,
    stdout: &mut impl Write,
) -> io::Result<()> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 1;
    if let Some(parent) = current_dir.parent() {
        let current_dir_name = current_dir
            .file_name()
            .and_then(|n| n.to_str())
            .map(String::from)
            .unwrap_or_default();

        app_state.nav_stack.push(NavigationInfo {
            dir_name: current_dir_name.clone(),
            index: *selected_index,
        });

        app_state.last_browsed_dir = current_dir.clone();
        *current_dir = parent.to_path_buf();
        app_state.current_dir = current_dir.clone();

        let parent_entries =
            get_sorted_entries(&app_state, current_dir, sort_order).unwrap_or_default();

        *selected_index = parent_entries
            .iter()
            .position(|e| e.name == current_dir_name)
            .map(|i| i)
            .unwrap_or(0);

        *scroll_offset = 0;
        *preview_active = false;
        app_state.page_state.move_left();
        let _ = clear_nav();
        stdout.flush()?;
        if !app_state.is_search && !app_state.preview_active && !app_state.select_mode {
            queue!(stdout, MoveTo(preview_width + 34, height - 10))?;
            writeln!(stdout, "                              ",)?;
            queue!(stdout, MoveTo(preview_width + 1, height - 8))?;
        }
    }
    Ok(())
}

pub fn handle_go_to_top_bottom(
    action: Action,
    g_pressed: &mut bool,
    last_key_time: &mut Instant,
    selected_index: &mut usize,
    scroll_offset: &mut usize,
    entries: &[FileEntry],
) {
    match action {
        Action::GoToTop => {
            if *g_pressed && last_key_time.elapsed() < Duration::from_millis(500) {
                *selected_index = 0;
                *scroll_offset = 0;
                *g_pressed = false;
            } else {
                *g_pressed = true;
                *last_key_time = Instant::now();
            }
        }
        Action::GoToBottom => {
            if *g_pressed && last_key_time.elapsed() < Duration::from_millis(500) {
                *selected_index = entries.len() - 1;
                if entries.len() > VISIBLE_LINES as usize {
                    *scroll_offset = entries.len() - VISIBLE_LINES as usize;
                }
                *g_pressed = false;
            } else {
                *g_pressed = false;
            }
        }
        _ => {}
    }
}

pub fn handle_move_updown(
    action: Action,
    selected_index: &mut usize,
    scroll_offset: &mut usize,
    entries: &[FileEntry],
    middle_line: usize,
    visible_lines: usize,
    app_state: &mut AppState,
    is_search: bool,
) -> io::Result<()> {
    app_state.changing_color = false;
    let _ = clear_interaction_field();
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width;
    let direction = if matches!(action, Action::MoveUp) {
        if *selected_index < 1 {
            0
        } else {
            1
        }
    } else {
        if *selected_index >= entries.len().saturating_sub(1) {
            0
        } else {
            1
        }
    };
    if matches!(action, Action::MoveUp) {
        *selected_index = selected_index.saturating_sub(direction);
    } else {
        *selected_index = (*selected_index + direction).min(entries.len().saturating_sub(1));
    }
    if *selected_index >= *scroll_offset + middle_line
        && *scroll_offset + visible_lines < entries.len()
    {
        *scroll_offset += 1;
    } else if *selected_index < *scroll_offset + middle_line && *scroll_offset > 0 {
        *scroll_offset = scroll_offset.saturating_sub(1);
    }

    if !is_search && !app_state.preview_active && !app_state.select_mode {
        queue!(stdout(), MoveTo(preview_width, height - 12))?;
        write!(
            stdout(),
            "{}",
            " ".green().to_string().repeat((preview_width - 6).into())
        )?;
        queue!(stdout(), MoveTo(preview_width, height - 11))?;
        write!(
            stdout(),
            "{}",
            " ".green().to_string().repeat((preview_width - 6).into())
        )?;
        queue!(stdout(), MoveTo(preview_width, height - 10))?;
        write!(
            stdout(),
            "{}",
            " ".green().to_string().repeat((preview_width - 6).into())
        )?;
        queue!(stdout(), MoveTo(preview_width, height - 9))?;
        write!(
            stdout(),
            "{}",
            " ".green().to_string().repeat((preview_width - 6).into())
        )?;
        queue!(stdout(), MoveTo(preview_width, height - 8))?;
        write!(
            stdout(),
            "{}",
            " ".green().to_string().repeat((preview_width - 6).into())
        )?;
    }

    if !app_state.select_mode {
        app_state.selection_amont = None;
        app_state.clear_multi_select()?;
    }
    Ok(())
}

#[rustfmt::skip]
pub fn handle_search_filter_keys(app_state: &mut AppState, key: KeyEvent) -> bool {
    if !app_state.search_filters.show_filters {
        return false;
    }
    let was_changed = match key.code {
        KeyCode::Char('1') => toggle_filter(app_state, MarkerColor::Red),
        KeyCode::Char('2') => toggle_filter(app_state, MarkerColor::Orange),
        KeyCode::Char('3') => toggle_filter(app_state, MarkerColor::Yellow),
        KeyCode::Char('4') => toggle_filter(app_state, MarkerColor::Blue),
        KeyCode::Char('5') => toggle_filter(app_state, MarkerColor::Cyan),
        KeyCode::Char('6') => toggle_filter(app_state, MarkerColor::Pink),
        KeyCode::Char('7') => toggle_filter(app_state, MarkerColor::White),
        KeyCode::Char('8') => toggle_filter(app_state, MarkerColor::Green),
        KeyCode::Char('9') => toggle_filter(app_state, MarkerColor::Magenta),
        KeyCode::Char('0') => {
            for enabled in app_state.search_filters.color_filters.values_mut() {
                *enabled = false;
            }
            app_state.search_filters.show_uncolored = true;
            app_state.search_filters.hide_all_colors = false;
            true
        },
        KeyCode::Char('h') => {   
            app_state.search_filters.hide_all_colors = !app_state.search_filters.hide_all_colors;
            if app_state.search_filters.hide_all_colors {
                for enabled in app_state.search_filters.color_filters.values_mut() {
                    *enabled = false;
                }
            }
            true
        },
        KeyCode::Char('u') => {
            app_state.search_filters.show_uncolored = !app_state.search_filters.show_uncolored;
            if app_state.search_filters.show_uncolored {
                for enabled in app_state.search_filters.color_filters.values_mut() {
                    *enabled = false;
                }
                app_state.search_filters.hide_all_colors = false;
            }
            true
        },
        KeyCode::Tab => {
            app_state.search_filters.show_filters = !app_state.search_filters.show_filters;
            true
        },
        _ => false,
    };
    was_changed
}

fn toggle_filter(app_state: &mut AppState, color: MarkerColor) -> bool {
    if let Some(current_state) = app_state.search_filters.color_filters.get(&color).copied() {
        app_state
            .search_filters
            .color_filters
            .insert(color, !current_state);

        let any_color_active = app_state
            .search_filters
            .color_filters
            .values()
            .any(|&enabled| enabled);

        app_state.search_filters.show_uncolored = !any_color_active;

        true
    } else {
        false
    }
}
// didn't work consistantly with just "border.config.draw_simple_borders = !border.config.draw_simple_borders"
pub fn handle_change_border(border: &mut AppState) {
    let mut stdout = stdout();
    if border.config.draw_simple_borders {
        let _ = execute!(stdout, Clear(ClearType::All));
        border.config.draw_simple_borders = !border.config.draw_simple_borders;
        let _ = draw_initial_border(&mut stdout, &border.page_state);
    } else {
        let _ = execute!(stdout, Clear(ClearType::All));
        border.config.draw_simple_borders = !border.config.draw_simple_borders;
        let _ = draw_simple_border(&mut stdout, &border.page_state);
    }
    let _ = border.config.save_config();
}
pub fn handle_dim_controls(app_state: &mut AppState, action: Action) -> io::Result<()> {
    match action {
        Action::IncreaseDimDistance => {
            app_state.config.max_distance = (app_state.config.max_distance + 1).min(100);
            interaction_field!("Dim distance: {}", app_state.config.max_distance)?;
        }
        Action::DecreaseDimDistance => {
            app_state.config.max_distance = (app_state.config.max_distance - 1).max(1);
            interaction_field!("Dim distance: {}", app_state.config.max_distance)?;
        }
        Action::IncreaseDimIntensity => {
            app_state.config.dim_step = (app_state.config.dim_step + 1).min(100);
            interaction_field!("Dim intensity: {}", app_state.config.dim_step)?;
        }
        Action::DecreaseDimIntensity => {
            app_state.config.dim_step = (app_state.config.dim_step - 1).max(1);
            interaction_field!("Dim intensity: {}", app_state.config.dim_step)?;
        }
        _ => {}
    }
    app_state.config.save_config()?;
    Ok(())
}
