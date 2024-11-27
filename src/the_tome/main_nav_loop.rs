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
pub enum BrowseResult {
    FileSelected,
    Exit,
    Continue,
}

pub fn browse_fuzzy_file(app_state: &mut AppState) -> io::Result<BrowseResult> {
    stdout().flush()?;
    terminal::enable_raw_mode()?;
    execute!(stdout(), EnableMouseCapture)?;
    // execute!(stdout(), EnableMouseCapture)?;
    let mut selected_index = app_state.selected_index;
    let mut current_dir = app_state.current_dir.clone();
    let mut scroll_offset = 0;
    let mut stdout = io::stdout();
    let mut terminal_state = TerminalState::new(80, 24)?;
    let mut sort_order = app_state.config.default_sort.clone();
    let mut last_entries: Vec<FileEntry> = Vec::new();
    let mut last_key_time = Instant::now();
    let mut g_pressed = false;
    let mut preview_active = false;

    execute!(stdout, terminal::Clear(ClearType::All))?;

    let (width, height) = size()?;
    let visible_lines = (height - 9) as usize;
    let middle_line = visible_lines / 2;
    let _end_y = height - 2;
    let start_y = 7;
    let nav_width = width / 2;
    let preview_width = width - nav_width;

    loop {
        let entries = match get_sorted_entries(&app_state, &current_dir, &sort_order) {
            Ok(entries) => entries,
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                let dir_name = current_dir
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or("");

                let permission_result =
                    handle_permission_issue(&mut stdout, dir_name, &current_dir)?;

                match permission_result {
                    Some(contents) => parse_ls_output(&contents, &current_dir),
                    None => {
                        if let Some(parent) = current_dir.parent() {
                            current_dir = parent.to_path_buf();
                            continue;
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
            Err(e) => return Err(e),
        };

        let mut full_redraw = entries != last_entries;
        if terminal_state.has_size_changed()? {
            terminal_state.update()?;
            full_redraw = true;
            // draw_initial_border(&mut stdout, &app_state.page_state)?;
        }
        display_directory(
            app_state,
            &entries,
            &current_dir,
            match app_state.input_mode {
                InputMode::Keyboard => selected_index,
                InputMode::Mouse => 0,
            },
            &mut stdout,
            scroll_offset,
            visible_lines,
            full_redraw,
        )?;
        last_entries = entries.clone();
        if let Ok(event) = event::read() {
            match event {
                Event::Mouse(mouse_event) => {
                    if app_state.input_mode != InputMode::Mouse {
                        app_state.input_mode = InputMode::Mouse;
                        selected_index = 0;
                    }
                    if let Some(result) = handle_mouse_event(
                        app_state,
                        mouse_event,
                        &entries,
                        visible_lines,
                        start_y,
                        nav_width,
                    )? {
                        return Ok(result);
                    }
                }
                Event::Key(key) => {
                    if app_state.input_mode != InputMode::Keyboard {
                        app_state.input_mode = InputMode::Keyboard;
                        app_state.mouse_state.hovered_index = None;
                    }
                    if let Some(context_menu) = &mut app_state.mouse_state.context_menu {
                        if let Some(action) = context_menu.handle_key_event(key).clone() {
                            handle_context_menu_action(app_state, action, &entries)?;
                            continue;
                        }
                    }

                    if !cfg!(target_os = "windows") || key.kind == KeyEventKind::Press {
                        if let Some(action) = app_state.config.get_action(&key) {
                            match action {
                                Action::ExecuteFile => {
                                    if let Some(entry) = entries.get(selected_index as usize) {
                                        let _ = app_state.execute_file(&mut stdout, &entry.path);
                                    }
                                }
                                Action::TerminalCommand => {
                                    if let Err(_e) = open_terminal_command(app_state, &mut stdout) {
                                    }
                                    current_dir = app_state.current_dir.clone();
                                    selected_index = 0;
                                    scroll_offset = 0;
                                }
                                Action::MultiSelectUp | Action::MultiSelectDown => {
                                    handle_multi_select(
                                        app_state,
                                        &mut selected_index,
                                        &mut scroll_offset,
                                        &entries,
                                        action.clone(),
                                        middle_line,
                                        visible_lines,
                                    );
                                }
                                Action::IncreaseDimDistance
                                | Action::DecreaseDimDistance
                                | Action::IncreaseDimIntensity
                                | Action::DecreaseDimIntensity => {
                                    let _ = handle_dim_controls(app_state, action.clone());
                                }
                                Action::BorderStyle => {
                                    handle_change_border(app_state);
                                    // let _ = full_redraw = true;
                                }
                                Action::ToggleSelect => {
                                    app_state.select_mode = !app_state.select_mode;
                                    if app_state.select_mode {
                                        app_state.selection_amont = None;
                                    }
                                }
                                Action::TogglePreview => {
                                    let _ = clear_preview();
                                    app_state.preview_active = !app_state.preview_active;
                                    preview_active = !preview_active;
                                }
                                Action::ToggleFilters => {
                                    handle_search_filter_keys(app_state, key);
                                }
                                Action::RenameLayer => {
                                    handle_layer_actions(
                                        &Action::RenameLayer,
                                        &mut stdout,
                                        preview_width,
                                        height,
                                        app_state,
                                    )?;
                                }
                                Action::SwitchLayer1
                                | Action::SwitchLayer2
                                | Action::SwitchLayer3
                                | Action::SwitchLayer4
                                | Action::SwitchLayer5
                                | Action::SwitchLayer6
                                | Action::SwitchLayer7
                                | Action::SwitchLayer8
                                | Action::SwitchLayer9
                                | Action::SwitchLayer0 => {
                                    handle_layer_actions(
                                        &action.clone(),
                                        &mut stdout,
                                        preview_width,
                                        height,
                                        app_state,
                                    )?;

                                    app_state.display_current_layer(&mut stdout)?;
                                }

                                #[rustfmt::skip]
                        Action::SetShortcut1 | Action::SetShortcut2 | Action::SetShortcut3 |
                        Action::SetShortcut4 | Action::SetShortcut5 | Action::SetShortcut6 |
                        Action::SetShortcut7 | Action::SetShortcut8 | Action::SetShortcut9 |
                        Action::SetShortcut0 => {
                            handle_set_shortcut(
                                app_state,
                                &mut stdout,
                                &current_dir,
                                selected_index ,
                                action.clone(),
                                height,
                                preview_width,
                            )?;
                        }

                                #[rustfmt::skip]
                        Action::UseShortcut1 | Action::UseShortcut2 | Action::UseShortcut3 |
                        Action::UseShortcut4 | Action::UseShortcut5 | Action::UseShortcut6 |
                        Action::UseShortcut7 | Action::UseShortcut8 | Action::UseShortcut9 |
                        Action::UseShortcut0 => {
                            handle_use_shortcut(
                                app_state,
                                &mut current_dir,
                                &mut selected_index,
                                &mut scroll_offset,
                                action.clone(),
                                &sort_order,
                                &mut stdout,
                                height,
                            )?;
                        }
                                Action::SetColorRules => {
                                    let _ = set_color_rules(app_state, &mut stdout);
                                    let _ = app_state.config.save_config();
                                }
                                Action::ShowShortcuts => {
                                    display_shortcuts(app_state, &mut stdout)?;
                                    continue;
                                }
                                Action::ToggleCount => {
                                    app_state.show_count = !app_state.show_count;
                                    // not in use as of yet.
                                }
                                Action::SetLineAmount => {
                                    if let Ok(new_lines) =
                                        prompt_line_amount(app_state.lines, &app_state.page_state)
                                    {
                                        app_state.lines = new_lines;
                                    }
                                }
                                Action::Help => {
                                    display_help_screen(
                                        &mut stdout,
                                        &app_state.config,
                                        app_state,
                                        full_redraw,
                                    )?;
                                    let _ = clear_nav();
                                    let _ = clear_preview();
                                    continue;
                                }
                                Action::Quit => {
                                    if handle_quit(app_state, &mut stdout)? {
                                        return Err(io::Error::new(
                                            io::ErrorKind::Interrupted,
                                            "User quit",
                                        ));
                                    }
                                }
                                Action::CastCommandLineSpell => {
                                    let _ = execute_terminal_command(app_state, &mut stdout);
                                }
                                Action::GiveBirthFile => {
                                    let _ = app_state.create_file(&mut stdout);
                                }
                                Action::GiveBirthDir => {
                                    let _ = app_state.create_directory(&mut stdout);
                                }
                                Action::SelectAll => {
                                    let _ = app_state.select_all(&entries);
                                }
                                Action::CycleItemColor => {
                                    app_state.cycle_item_color(&entries, selected_index)?;
                                    let _ = app_state.config.save_config();
                                }
                                Action::RemoveItemColor => {
                                    if let Some(selected_path) =
                                        Config::get_selected_path(app_state, &entries)
                                    {
                                        app_state.remove_item_color(&selected_path);
                                        let _ = app_state.config.save_config();
                                    }
                                }
                                Action::Undo => {
                                    let _ = undo_last_operation(app_state, &mut stdout);
                                }
                                Action::SearchFiles => {
                                    handle_search_files(
                                        app_state,
                                        &mut current_dir,
                                        &mut selected_index,
                                        &mut scroll_offset,
                                        &mut stdout,
                                        &sort_order,
                                    )?;
                                }
                                Action::OpenInEditor => {
                                    handle_open_in_editor(
                                        app_state,
                                        &entries,
                                        selected_index,
                                        &mut stdout,
                                        height,
                                        &mut g_pressed,
                                    )?;
                                }
                                Action::EditConfig => {
                                    edit_config(app_state, &mut stdout, &current_dir)?;
                                }
                                Action::Rename => {
                                    if let Some(entry) = entries.get(selected_index as usize) {
                                        rename_file(&mut stdout, entry, true, app_state)?;
                                    }
                                }
                                Action::RenameWithoutExtension => {
                                    if let Some(entry) = entries.get(selected_index as usize) {
                                        rename_file(&mut stdout, entry, false, app_state)?;
                                    }
                                }
                                Action::Murder => {
                                    if let Some(_entry) = entries.get(selected_index as usize) {
                                        murder_files(
                                            app_state,
                                            &mut stdout,
                                            &entries,
                                            selected_index,
                                            false,
                                        )?;
                                    }
                                }
                                Action::Copy => {
                                    copy_files(app_state, &entries, selected_index);
                                }
                                Action::Paste => {
                                    paste_files(app_state, &current_dir)?;
                                }
                                Action::Duplicate => {
                                    handle_duplicate(
                                        app_state,
                                        &mut stdout,
                                        &entries,
                                        selected_index,
                                    )?;
                                }
                                Action::MoveItem => {
                                    handle_move_item(
                                        app_state,
                                        &entries,
                                        selected_index,
                                        &current_dir,
                                    )?;
                                }
                                // removed from code for the time being
                                Action::Search => {
                                    continue;
                                }
                                // Is unstable at the moment.
                                Action::GitMenu => {
                                    handle_git_menu(
                                        app_state,
                                        &mut stdout,
                                        &current_dir,
                                        nav_width,
                                        preview_width,
                                        start_y,
                                        _end_y,
                                        height,
                                    )?;
                                }
                                Action::SortCycleForward => {
                                    handle_sort_cycle(
                                        app_state,
                                        &mut sort_order,
                                        &mut selected_index,
                                        &mut scroll_offset,
                                    );
                                }
                                Action::MoveRight | Action::Enter => {
                                    handle_move_right(
                                        app_state,
                                        &mut current_dir,
                                        &mut selected_index,
                                        &mut scroll_offset,
                                        &mut preview_active,
                                        &entries,
                                        &mut stdout,
                                    )?;
                                }
                                Action::MoveLeft => {
                                    handle_move_left(
                                        app_state,
                                        &mut current_dir,
                                        &mut selected_index,
                                        &mut scroll_offset,
                                        &mut preview_active,
                                        &sort_order,
                                        &mut stdout,
                                    )?;
                                }
                                Action::GoToTop | Action::GoToBottom => {
                                    handle_go_to_top_bottom(
                                        action.clone(),
                                        &mut g_pressed,
                                        &mut last_key_time,
                                        &mut selected_index,
                                        &mut scroll_offset,
                                        &entries,
                                    );
                                }
                                Action::MoveDown | Action::MoveUp => {
                                    let _ = handle_move_updown(
                                        action.clone(),
                                        &mut selected_index,
                                        &mut scroll_offset,
                                        &entries,
                                        middle_line,
                                        visible_lines,
                                        app_state,
                                        app_state.is_search,
                                    );
                                    g_pressed = false;
                                }
                            }
                            // } else {
                            //     println!("action not mapped");
                            // }
                        }
                    }
                }
                Event::FocusGained => {
                    full_redraw = true;
                }
                Event::FocusLost => {}
                Event::Paste(data) => {}
                _ => {}
            }
        }
    }
}
