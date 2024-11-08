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

// Look into adding more shortcuts. they are very useful
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
    queue!(stdout, MoveTo(preview_width + 3, height - 12))?;
    write!(
        stdout,
        "{}",
        "-".green().to_string().repeat((preview_width - 4).into())
    )?;
    queue!(stdout, MoveTo(preview_width + 23, height - 10))?;
    writeln!(stdout, "Enter a name for this shortcut:")?;
    queue!(stdout, MoveTo(preview_width + 3, height - 8))?;
    write!(
        stdout,
        "{}",
        "-".green().to_string().repeat((preview_width - 4).into())
    )?;
    stdout.flush()?;
    queue!(stdout, MoveTo(preview_width + 37, height - 9))?;
    queue!(stdout, SetForegroundColor(Color::Red))?;
    let shortcut_name = read_line()?.red();
    queue!(stdout, SetForegroundColor(Color::Reset))?;
    if app_state.config.shortcuts.is_none() {
        app_state.config.shortcuts = Some(std::collections::HashMap::new());
    }
    if let Some(shortcuts) = &mut app_state.config.shortcuts {
        shortcuts.insert(
            shortcut_key,
            (
                current_dir.clone(),
                shortcut_name.to_string(),
                selected_index,
            ),
        );
        app_state.config.save_config()?;
        queue!(stdout, MoveTo(preview_width + 3, height - 10))?;
        writeln!(
            stdout,
            "'{}' set to current directory: {}",
            shortcut_key.red(),
            current_dir.to_string_lossy().green(),
            // selected_index
        )?;
    }
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
    if let Some(shortcuts) = &app_state.config.shortcuts {
        if let Some((path, _, index)) = shortcuts.get(&c) {
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
                *selected_index = entries
                    .iter()
                    .position(|e| e.path == *path)
                    .map(|i| i)
                    .unwrap_or(0);
                *scroll_offset = 0;
            } else {
                // writeln!(stdout, "                              ",)?;
                queue!(stdout, MoveTo(preview_width + 30, height - 10))?;
                writeln!(stdout, "{}", "Error: Invalid shortcut path".red())?;
            }
        } else {
            queue!(stdout, MoveTo(preview_width + 30, height - 10))?;
            writeln!(stdout, "{}", "Error: Shortcut not found".red())?;
        }
    }
    Ok(())
}

pub fn handle_quit(
    app_state: &mut AppState,
    stdout: &mut impl Write,
    preview_width: u16,
    height: u16,
) -> io::Result<bool> {
    let _ = preview_width;
    let (width, _) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;
    if app_state.multiple_selected_files.iter().count() > 0 {
        app_state.clear_multi_select()?;
        return Ok(false);
    }
    if app_state.is_moving {
        app_state.cancel_move();
        // execute!(stdout, MoveTo(4, 3))?;
        queue!(stdout, MoveTo(preview_width + 28, height - 12))?;
        writeln!(stdout, "File move cancelled.")?;
        Ok(false)
    } else {
        // Uncomment to add back confirmation to quit

        // queue!(stdout, MoveTo(preview_width + 27, height - 10))?;
        // write!(stdout, "{}", "Are you sure you want to quit?".green())?;
        // queue!(stdout, MoveTo(preview_width + 40, height - 9))?;
        // write!(stdout, "{}/{}", "Y".red(), "N".red())?;
        // let quit = read_line().expect("error");
        // let quit_options = vec!["y", "Y", "yes", "Yes", "YES"];
        // Ok(quit_options.contains(&quit.as_str()))
        cleanup_terminal()?;
        Ok(true)
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
    let (width, _) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;
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
                        queue!(stdout, MoveTo(preview_width + 28, height - 12))?;
                        writeln!(stdout, "Error opening file: {}", e)?;
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
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;
    if let Some(_entry) = entries.get(selected_index as usize) {
        match duplicate_files(stdout, app_state, entries, Some(selected_index)) {
            Ok(_) => {
                queue!(stdout, MoveTo(preview_width + 28, height - 12))?;
                writeln!(stdout, "File duplicated successfully.")?;
            }
            Err(e) => {
                queue!(stdout, MoveTo(preview_width + 28, height - 12))?;
                writeln!(stdout, "Error duplicating file: {}", e)?;
            }
        }
        let _ = clear_nav();
    }
    Ok(())
}

pub fn handle_move_item(
    app_state: &mut AppState,
    stdout: &mut impl Write,
    entries: &[FileEntry],
    selected_index: usize,
    current_dir: &PathBuf,
) -> io::Result<()> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;
    if let Some(entry) = entries.get(selected_index as usize) {
        if app_state.is_moving {
            match app_state.finish_move(current_dir) {
                Ok(_) => {
                    queue!(stdout, MoveTo(preview_width + 28, height - 12))?;
                    writeln!(stdout, "File(s) moved successfully.")?;
                }
                Err(e) => {
                    queue!(stdout, MoveTo(preview_width + 28, height - 12))?;
                    writeln!(stdout, "Error moving file(s): {}", e)?;
                    app_state.cancel_move();
                }
            }
        } else {
            match app_state.start_move(Some(entry)) {
                Ok(_) => {
                    queue!(stdout, MoveTo(preview_width + 28, height - 12))?;
                    writeln!(
                        stdout,
                        "Moving file(s). Press 'm' again to place them in the current directory."
                    )?;
                }
                Err(e) => {
                    queue!(stdout, MoveTo(preview_width + 28, height - 12))?;
                    writeln!(stdout, "Error: {}", e)?;
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
        execute!(stdout, MoveTo(0, height - 2))?;
        writeln!(stdout, "Not a Git repository")?;
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
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;
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
        queue!(stdout(), MoveTo(preview_width + 3, height - 12))?;
        write!(
            stdout(),
            "{}",
            " ".green().to_string().repeat((preview_width - 4).into())
        )?;
        queue!(stdout(), MoveTo(preview_width + 3, height - 11))?;
        write!(
            stdout(),
            "{}",
            " ".green().to_string().repeat((preview_width - 4).into())
        )?;
        queue!(stdout(), MoveTo(preview_width + 3, height - 10))?;
        write!(
            stdout(),
            "{}",
            " ".green().to_string().repeat((preview_width - 4).into())
        )?;
        queue!(stdout(), MoveTo(preview_width + 3, height - 9))?;
        write!(
            stdout(),
            "{}",
            " ".green().to_string().repeat((preview_width - 4).into())
        )?;
        queue!(stdout(), MoveTo(preview_width + 3, height - 8))?;
        write!(
            stdout(),
            "{}",
            " ".green().to_string().repeat((preview_width - 4).into())
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
