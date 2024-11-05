/*
 * Stygian Sift - A Terminal-based File Manager
 * Copyright (c) 2024 Maui The Magnificent (Charon)
 *
 * This software is released under the Stygian Sift License.
 * See LICENSE file in the project root or visit:
 * https://github.com/Mauitron/StygianSift.git
 *
 * Created by: Maui The Magnificent (Charon)
 * Contact: Stygian.Ferryman69@gmail.com
 *
 * When using, modifying, or distributing this software,
 * please maintain this attribution notice and provide a link
 * to the original project.
 */

use std::env::current_dir;

use super::*;
//////////////////////////////////////////////////////Search/////////////////////////////////////////////////////////////////////////
fn parse_color_search(search_term: &str) -> (Option<MarkerColor>, String) {
    let color_prefixes = [
        ("red:", MarkerColor::Red),
        ("orange:", MarkerColor::Orange),
        ("yellow:", MarkerColor::Yellow),
        ("blue:", MarkerColor::Blue),
        ("cyan:", MarkerColor::Cyan),
        ("pink:", MarkerColor::Pink),
        ("white:", MarkerColor::White),
        ("green:", MarkerColor::Green),
        ("magenta:", MarkerColor::Magenta),
        ("reset:", MarkerColor::Reset),
        ("colored:", MarkerColor::Reset),
        ("nocolor:", MarkerColor::Reset),
        ("hidecolor:", MarkerColor::Reset),
    ];

    color_prefixes
        .par_iter()
        .find_first(|(prefix, _)| search_term.to_lowercase().starts_with(prefix))
        .map(|(prefix, color)| (Some(*color), search_term[prefix.len()..].to_string()))
        .unwrap_or((None, search_term.to_string()))
}

pub struct SearchFilters {
    pub color_filters: HashMap<MarkerColor, bool>,
    pub show_uncolored: bool,
    pub show_filters: bool,
    pub hide_all_colors: bool,
}

impl SearchFilters {
    pub fn new() -> Self {
        let mut color_filters = HashMap::new();
        for color in [
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
        ] {
            color_filters.insert(color, false);
        }
        Self {
            color_filters,
            show_uncolored: true,
            show_filters: false,
            hide_all_colors: false,
        }
    }
}

pub fn fuzzy_navigate(
    current_dir: &Path,
    input: &str,
    max_results: usize,
) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(current_dir)?
        .filter_map(|entry| entry.ok())
        .collect::<Vec<_>>();

    let results: Vec<_> = entries
        .into_par_iter()
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            let score = fuzzy_match(&name, input);
            if score > 0 {
                Some((entry.path(), score))
            } else {
                None
            }
        })
        .collect();

    let mut sorted_results = results;
    sorted_results.sort_by(|a, b| b.1.cmp(&a.1));

    Ok(sorted_results
        .into_iter()
        .take(max_results)
        .map(|(path, _)| path)
        .collect())
}

pub fn fuzzy_search_entries(
    app_state: &mut AppState,
    search_term: &str,
) -> io::Result<Vec<FileEntry>> {
    let (color_filter, actual_search_term) = parse_color_search(search_term);

    fn search_directory(
        app_state: &AppState,
        dir: &Path,
        search_term: &str,
        color_filter: Option<MarkerColor>,
        depth: usize,
        max_depth: usize,
    ) -> Vec<FileEntry> {
        if depth > max_depth {
            return Vec::new();
        }

        let mut results = Vec::new();

        if let Ok(entries) = fs::read_dir(dir) {
            let accessible_entries: Vec<_> = entries.filter_map(Result::ok).collect();

            let mut entry_results: Vec<_> = accessible_entries
                .par_iter()
                .filter_map(|entry| {
                    let path = entry.path();
                    if !app_state.is_searchable(&path) {
                        return None;
                    }

                    let item_color = app_state.get_item_color(&path);
                    if app_state.search_filters.hide_all_colors && item_color.is_some() {
                        return None;
                    } else if search_term.to_lowercase().starts_with("hidecolor:") {
                        if item_color.is_some() {
                            return None;
                        }
                    }

                    match color_filter {
                        Some(filter_color) => {
                            if app_state.search_filters.hide_all_colors && item_color.is_some() {
                                return None;
                            }
                            if search_term.to_lowercase().starts_with("hidecolor:")
                                && item_color.is_some()
                            {
                                return None;
                            }
                            if search_term.to_lowercase().starts_with("colored:")
                                && item_color.is_none()
                            {
                                return None;
                            }
                            if search_term.to_lowercase().starts_with("nocolor:")
                                && item_color.is_some()
                            {
                                return None;
                            }
                            if !search_term.to_lowercase().starts_with("colored:")
                                && !search_term.to_lowercase().starts_with("nocolor:")
                                && item_color != Some(filter_color)
                            {
                                return None;
                            }
                        }
                        None => {
                            let any_filters_active =
                                app_state.search_filters.color_filters.values().any(|&v| v);
                            if any_filters_active {
                                match item_color {
                                    Some(color) => {
                                        if !app_state
                                            .search_filters
                                            .color_filters
                                            .get(&color)
                                            .copied()
                                            .unwrap_or(false)
                                        {
                                            return None;
                                        }
                                    }
                                    None => {
                                        if !app_state.search_filters.show_uncolored {
                                            return None;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    let score = if search_term.is_empty() {
                        1
                    } else {
                        fuzzy_match(&name, search_term)
                    };

                    if score > 0 {
                        FileEntry::new(path).ok()
                    } else {
                        None
                    }
                })
                .collect();

            results.append(&mut entry_results);

            let subdirs: Vec<_> = accessible_entries
                .par_iter()
                .filter(|entry| entry.path().is_dir() && app_state.is_searchable(&entry.path()))
                .map(|entry| entry.path())
                .collect();

            let subdir_results: Vec<_> = subdirs
                .par_iter()
                .flat_map(|path| {
                    search_directory(
                        app_state,
                        path,
                        search_term,
                        color_filter,
                        depth + 1,
                        max_depth,
                    )
                })
                .collect();

            results.extend(subdir_results);
        }

        results
    }

    let results = search_directory(
        app_state,
        &app_state.current_dir,
        &actual_search_term,
        color_filter,
        0,
        app_state.search_depth_limit,
    );

    let mut scored_results: Vec<_> = results
        .into_par_iter()
        .map(|entry| {
            let score = if actual_search_term.is_empty() {
                1
            } else {
                fuzzy_match(&entry.name, &actual_search_term)
            };
            (entry, score)
        })
        .collect();

    scored_results.par_sort_unstable_by(|a, b| b.1.cmp(&a.1));

    Ok(scored_results.into_iter().map(|(entry, _)| entry).collect())
}

pub fn fuzzy_match(name: &str, input: &str) -> usize {
    let name_lower = name.to_lowercase();
    let input_lower = input.to_lowercase();

    if input_lower.len() > 10 {
        let chars: Vec<_> = name_lower.chars().collect();
        let input_chars: Vec<_> = input_lower.chars().collect();

        let score = chars
            .par_windows(input_chars.len())
            .map(|window| {
                let mut local_score = 0;
                let mut consecutive = 0;

                for (window_c, input_c) in window.iter().zip(input_chars.iter()) {
                    if window_c == input_c {
                        local_score += 1 + consecutive;
                        consecutive += 1;
                    } else {
                        consecutive = 0;
                    }
                }

                local_score
            })
            .max()
            .unwrap_or(0);

        let boundary_bonus = if name_lower.starts_with(&input_lower)
            || name_lower
                .split(|c| c == ' ' || c == '_' || c == '-')
                .any(|word| word.starts_with(&input_lower))
        {
            5
        } else {
            0
        };

        (score + boundary_bonus).try_into().unwrap_or(0)
    } else {
        let mut score: i32 = 0;
        let mut name_iter = name_lower.chars().peekable();
        let mut input_iter = input_lower.chars().peekable();
        let mut consecutive = 0;
        let mut last_matched_pos = None;

        while let (Some(name_c), Some(input_c)) = (name_iter.peek(), input_iter.peek()) {
            if name_c == input_c {
                score += 1 + consecutive;
                consecutive += 1;
                last_matched_pos = Some(name_lower.len() - name_iter.clone().count());
                name_iter.next();
                input_iter.next();
            } else {
                consecutive = 0;
                name_iter.next();
            }
        }

        score = score.saturating_sub(input_iter.count() as i32);

        if let Some(pos) = last_matched_pos {
            if pos == 0
                || name_lower
                    .chars()
                    .nth(pos.saturating_sub(1))
                    .map_or(false, |c| c == ' ' || c == '_' || c == '-')
            {
                score += 5;
            }
        }

        score.try_into().unwrap_or(0)
    }
}
// Remember to fix so that these entries can be sorted.
pub fn read_search_input(stdout: &mut impl Write, app_state: &mut AppState) -> io::Result<String> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 1;
    let mut input = String::new();
    let prompt = "Search: ";
    let prompt_length = prompt.len() as u16;
    execute!(
        stdout,
        MoveTo(preview_width + 11, 1),
        SetForegroundColor(Color::Green),
        Hide
    )?;
    queue!(stdout, MoveTo(preview_width + 2, height - 12))?;
    write!(stdout, "{}", "-".repeat((preview_width - 5).into()))?;
    queue!(stdout, MoveTo(preview_width + 3, height - 10))?;
    write!(stdout, "{}", prompt)?;
    queue!(stdout, MoveTo(preview_width + 2, height - 8))?;
    write!(stdout, "{}", "-".repeat((preview_width - 5).into()))?;
    stdout.flush()?;

    let mut cursor_pos = prompt_length;

    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Enter => {
                    app_state.last_search_term = input.clone();
                    break;
                }
                KeyCode::Char(c) => {
                    input.push(c);
                    queue!(stdout, MoveTo(preview_width + 3 + cursor_pos, height - 10))?;
                    write!(stdout, "{}", c)?;
                    cursor_pos += 1;
                }
                KeyCode::Backspace => {
                    if !input.is_empty() {
                        input.pop();
                        cursor_pos -= 1;
                        queue!(
                            stdout,
                            MoveTo(preview_width + 3 + cursor_pos, height - 10),
                            crossterm::terminal::Clear(
                                crossterm::terminal::ClearType::UntilNewLine
                            )
                        )?;
                    }
                }
                KeyCode::Esc => {
                    while !input.is_empty() {
                        input.pop();
                        cursor_pos -= 1;
                        queue!(
                            stdout,
                            MoveTo(preview_width + 11 + cursor_pos, height - 10),
                            crossterm::terminal::Clear(
                                crossterm::terminal::ClearType::UntilNewLine
                            )
                        )?;
                    }
                    break;
                }
                _ => {}
            }
            stdout.flush()?;
        }
    }
    execute!(stdout, ResetColor, Show)?;
    Ok(input)
}

pub fn display_search_results(
    app_state: &mut AppState,
    mut results: Vec<FileEntry>,
    stdout: &mut impl Write,
) -> io::Result<Option<PathBuf>> {
    let (width, height) = crossterm::terminal::size()?;
    let start_y = 11;
    let end_y = height - 2;
    let visible_lines = (end_y - start_y) as usize;
    let mut selected_index = 0;
    let mut scroll_offset = 0;
    let mut last_selected_index = 0;
    let mut last_scroll_offset = 0;
    let mut g_pressed = false;
    let mut sort_order = app_state.config.default_sort.clone();
    draw_search_results(
        app_state,
        stdout,
        &results,
        visible_lines,
        start_y,
        end_y,
        width,
        selected_index,
        scroll_offset,
        true,
    )?;
    execute!(stdout, cursor::Hide)?;
    loop {
        if let Event::Key(key) = event::read()? {
            if let Some(action) = app_state.config.clone().get_action(&key) {
                if app_state.search_filters.show_filters
                    && handle_search_filter_keys(app_state, key)
                {
                    results = fuzzy_search_entries(app_state, &app_state.last_search_term.clone())?;
                    draw_search_results(
                        app_state,
                        stdout,
                        &results,
                        visible_lines,
                        start_y,
                        end_y,
                        width,
                        selected_index,
                        scroll_offset,
                        true,
                    )?;
                    continue;
                }
                match action {
                    Action::GoToTop => {
                        if g_pressed {
                            selected_index = 0;
                            scroll_offset = 0;
                            g_pressed = false;
                        } else {
                            g_pressed = true;
                        }
                    }

                    Action::GoToBottom => {
                        if g_pressed {
                            selected_index = (results.len() - 1);
                            if results.len() > (visible_lines) as usize {
                                scroll_offset = (results.len()) - visible_lines as usize;
                            }
                            g_pressed = false;
                        } else {
                            g_pressed = false;
                        }
                    }

                    Action::MoveUp => {
                        let _ = handle_move_updown(
                            action.clone(),
                            &mut selected_index,
                            &mut scroll_offset,
                            &results,
                            visible_lines / 2,
                            visible_lines,
                            app_state,
                            app_state.is_search,
                        );
                    }
                    Action::SortCycleForward => {
                        handle_sort_cycle(
                            app_state,
                            &mut sort_order,
                            &mut selected_index,
                            &mut scroll_offset,
                        );
                        let mut a = current_dir()?;
                        get_sorted_entries(&app_state, a.as_path(), &sort_order)?;
                    }
                    Action::MoveDown => {
                        let _ = handle_move_updown(
                            action.clone(),
                            &mut selected_index,
                            &mut scroll_offset,
                            &results,
                            visible_lines / 2,
                            visible_lines,
                            app_state,
                            app_state.is_search,
                        );
                    }
                    Action::MultiSelectUp => {
                        handle_multi_select(
                            app_state,
                            &mut selected_index,
                            &mut scroll_offset,
                            &results,
                            action.clone(),
                            visible_lines / 2,
                            visible_lines,
                        );
                    }
                    Action::MultiSelectDown => {
                        handle_multi_select(
                            app_state,
                            &mut selected_index,
                            &mut scroll_offset,
                            &results,
                            action.clone(),
                            visible_lines / 2,
                            visible_lines,
                        );
                    }
                    Action::ToggleFilters => {
                        app_state.search_filters.show_filters =
                            !app_state.search_filters.show_filters;
                        if app_state.search_filters.show_filters {
                            draw_search_results(
                                app_state,
                                stdout,
                                &results,
                                visible_lines,
                                start_y,
                                end_y,
                                width,
                                selected_index,
                                scroll_offset,
                                true,
                            )?;
                        }
                    }
                    Action::ToggleSelect => {
                        app_state.select_mode = !app_state.select_mode;
                        if app_state.select_mode {
                            app_state.selection_amont = None;
                        }
                    }
                    Action::SelectAll => {
                        app_state.select_all(&results);
                    }
                    Action::Enter => {
                        let selected_path = &results[selected_index as usize].path;
                        if selected_path.is_dir() {
                            app_state.is_search = false;
                            app_state.current_dir = selected_path.clone();
                            return Ok(Some(selected_path.clone()));
                        } else {
                            return Ok(Some(selected_path.clone()));
                        }
                    }
                    Action::Quit => {
                        if app_state.multiple_selected_files.is_some()
                            && app_state.multiple_selected_files.as_ref().unwrap().len() > 0
                        {
                            app_state
                                .clear_multi_select()
                                .expect("could not clear multi select");
                        } else {
                            app_state.is_search = false;
                            clear_nav()?;
                            clear_preview()?;
                            draw_initial_border(stdout, &app_state.page_state)?;
                            let _ = app_state.config.save_config();

                            return Ok(None);
                        }
                    }
                    Action::Rename => {
                        if let Some(entry) = results.get(selected_index as usize) {
                            rename_file(stdout, entry, true, app_state)?;
                            if let Some(updated_entry) = results.get_mut(selected_index as usize) {
                                *updated_entry = FileEntry::new(updated_entry.path.clone())?;
                            }
                        }
                    }
                    Action::Murder => {
                        murder_files(app_state, stdout, &results, selected_index)?;
                        results.retain(|entry| entry.path.exists());
                        selected_index = selected_index.min((results.len() - 1));
                    }
                    Action::Copy => {
                        copy_files(app_state, &results, selected_index);
                    }
                    Action::MoveItem => {
                        if let Some(entry) = results.get(selected_index as usize) {
                            if app_state.is_moving {
                                app_state
                                    .finish_move(&app_state.current_dir.clone())
                                    .expect("could not move");
                            } else {
                                app_state
                                    .start_move(Some(entry))
                                    .expect("could not start moving file");
                            }
                        }
                    }
                    Action::Paste => {
                        let _ = paste_files(app_state, &app_state.current_dir.clone());
                        results =
                            fuzzy_search_entries(app_state, &app_state.last_search_term.clone())?;
                    }
                    Action::Duplicate => {
                        duplicate_files(stdout, app_state, &results, Some(selected_index))?;
                        results =
                            fuzzy_search_entries(app_state, &app_state.last_search_term.clone())?;
                    }
                    Action::CycleItemColor => {
                        app_state.changing_color = true;
                        let files_to_cycle =
                            if let Some(selected) = &app_state.multiple_selected_files {
                                selected.iter().cloned().collect::<Vec<_>>()
                            } else {
                                vec![results[selected_index as usize].path.clone()]
                            };

                        if !files_to_cycle.is_empty() {
                            let current_color = app_state.get_item_color(&files_to_cycle[0]);
                            let new_color = match current_color {
                                None => Some(MarkerColor::Red),
                                Some(MarkerColor::Red) => Some(MarkerColor::Orange),
                                Some(MarkerColor::Orange) => Some(MarkerColor::Yellow),
                                Some(MarkerColor::Yellow) => Some(MarkerColor::Blue),
                                Some(MarkerColor::Blue) => Some(MarkerColor::Cyan),
                                Some(MarkerColor::Cyan) => Some(MarkerColor::Pink),
                                Some(MarkerColor::Pink) => Some(MarkerColor::White),
                                Some(MarkerColor::White) => Some(MarkerColor::Green),
                                Some(MarkerColor::Green) => Some(MarkerColor::Magenta),
                                Some(MarkerColor::Magenta) => Some(MarkerColor::Reset),
                                Some(MarkerColor::Reset) => None,
                            };

                            for path in files_to_cycle {
                                match new_color {
                                    Some(color) => {
                                        app_state.set_item_color(path.clone(), color);
                                        if let Some(entry) =
                                            results.iter_mut().find(|e| e.path == path)
                                        {
                                            entry.git_status = Some(GitStatus::Unmodified);
                                        }
                                    }
                                    None => {
                                        app_state.remove_item_color(&path);
                                        if let Some(entry) =
                                            results.iter_mut().find(|e| e.path == path)
                                        {
                                            entry.git_status = None;
                                        }
                                    }
                                }
                            }

                            execute!(stdout, MoveTo(0, height - 1), Clear(ClearType::CurrentLine))?;
                            if let Some(color) = new_color {
                                write!(stdout, "New color: {}", color.as_str())?;
                            } else {
                                write!(stdout, "Color removed")?;
                            }
                            stdout.flush()?;

                            draw_search_results(
                                app_state,
                                stdout,
                                &results,
                                visible_lines,
                                start_y,
                                end_y,
                                width,
                                selected_index,
                                scroll_offset,
                                true,
                            )?;
                        }
                        let _ = app_state.config.save_config();
                    }
                    Action::OpenInEditor => {
                        if let Some(entry) = results.get(selected_index as usize) {
                            if entry.file_type != FileType::Directory {
                                open_file_with_editor(
                                    &entry.path,
                                    &app_state.config.text_editor,
                                    stdout,
                                )?;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        if selected_index as usize != last_selected_index || scroll_offset != last_scroll_offset {
            draw_search_results(
                app_state,
                stdout,
                &results,
                visible_lines,
                start_y,
                end_y,
                width,
                selected_index,
                scroll_offset,
                false,
            )?;
            last_selected_index = selected_index as usize;
            last_scroll_offset = scroll_offset;
        }
        execute!(stdout, cursor::Hide)?;
    }
}
fn draw_color_filters(
    stdout: &mut impl Write,
    filters: &SearchFilters,
    y_position: u16,
    width: u16,
) -> io::Result<()> {
    let colors = [
        (MarkerColor::Red, "Red", Color::Red),
        (MarkerColor::Orange, "Org", Color::DarkYellow),
        (MarkerColor::Yellow, "Yel", Color::Yellow),
        (MarkerColor::Blue, "Blu", Color::Blue),
        (MarkerColor::Cyan, "Cyn", Color::Cyan),
        (MarkerColor::Pink, "Pnk", Color::Magenta),
        (MarkerColor::White, "Wht", Color::White),
        (MarkerColor::Green, "Grn", Color::Green),
        (MarkerColor::Magenta, "Mag", Color::Magenta),
        (MarkerColor::Reset, "Rst", Color::Grey),
    ];

    let toggle_width = 8;
    let toggles_per_row = (width as usize / toggle_width).max(1);

    for (i, (color, label, display_color)) in colors.iter().enumerate() {
        let x = (i % toggles_per_row) * toggle_width;
        let y = y_position + (i / toggles_per_row) as u16;

        execute!(stdout, MoveTo(x as u16, y))?;

        let is_active = if filters.hide_all_colors {
            false
        } else {
            *filters.color_filters.get(color).unwrap_or(&false)
        };

        let bracket_color = if is_active {
            Color::Green
        } else {
            Color::DarkGrey
        };

        let actual_display_color = if filters.hide_all_colors {
            Color::DarkGrey
        } else {
            *display_color
        };

        execute!(stdout, SetForegroundColor(bracket_color))?;
        write!(stdout, "[")?;

        execute!(stdout, SetForegroundColor(actual_display_color))?;
        write!(stdout, "{}", label)?;

        execute!(stdout, SetForegroundColor(bracket_color))?;
        write!(stdout, "]")?;
    }

    let y = y_position + (colors.len() / toggles_per_row) as u16 + 1;
    execute!(
        stdout,
        MoveTo(0, y),
        SetForegroundColor(if filters.show_uncolored {
            Color::Green
        } else {
            Color::DarkGrey
        })
    )?;
    write!(stdout, "[Uncolored]")?;

    execute!(
        stdout,
        MoveTo(toggle_width as u16, y),
        SetForegroundColor(if filters.hide_all_colors {
            Color::Green
        } else {
            Color::DarkGrey
        })
    )?;
    write!(stdout, "[Hide Colors]")?;

    Ok(())
}

pub fn draw_search_results(
    app_state: &mut AppState,
    stdout: &mut impl Write,
    results: &[FileEntry],
    visible_lines: usize,
    start_y: u16,
    end_y: u16,
    width: u16,
    selected_index: usize,
    scroll_offset: usize,
    full_redraw: bool,
) -> io::Result<()> {
    let dimming_config = DimmingConfig::new(visible_lines);
    if full_redraw {
        execute!(stdout, Clear(ClearType::All))?;

        execute!(
            stdout,
            MoveTo(0, start_y - 6),
            SetForegroundColor(Color::Green)
        )?;
        writeln!(
            stdout,
            "Search Results for '{}' (Esc to cancel, Tab for filters, H to hide colors):",
            app_state.last_search_term
        )?;

        if app_state.search_filters.show_filters {
            draw_color_filters(stdout, &app_state.search_filters, start_y - 5, width)?;
        }
    }

    let entry_width = width / 2;
    let path_width = width - entry_width - 2;
    let selected_index_usize = selected_index as usize;

    let content_start_y = if app_state.search_filters.show_filters {
        start_y + 2
    } else {
        start_y
    };

    for (index, entry) in results
        .iter()
        .skip(scroll_offset)
        .take(visible_lines)
        .enumerate()
    {
        let absolute_index = index + scroll_offset;
        let y = content_start_y + index as u16;
        let distance_from_selected = absolute_index as i32 - selected_index_usize as i32;

        if app_state.search_filters.hide_all_colors
            && app_state.get_item_color(&entry.path).is_some()
        {
            continue;
        }

        execute!(stdout, MoveTo(0, y - 2))?;
        let is_selected = absolute_index == selected_index_usize;
        write_entry(
            app_state,
            stdout,
            entry,
            is_selected,
            distance_from_selected,
            entry_width,
            &dimming_config,
        )?;

        let path_start_column = entry_width;
        execute!(stdout, MoveTo(path_start_column - 10, y - 2))?;
        let relative_path = entry
            .path
            .strip_prefix(&app_state.current_dir)
            .unwrap_or(&entry.path);
        let relative_path_str = truncate_path(&relative_path, path_width as usize);

        let dim_factor = dimming_config.calculate_dimming(distance_from_selected);
        let path_color = if is_selected {
            Color::Green
        } else {
            dim_color(Color::DarkGrey, dim_factor)
        };

        if is_selected {
            execute!(stdout, SetForegroundColor(Color::Red))?;
            write!(stdout, "=====>")?;
        } else {
            let dimmed_red = dim_color(Color::Red, dim_factor);
            execute!(stdout, SetForegroundColor(dimmed_red))?;
            write!(stdout, "=====>")?;
        }

        if !app_state.search_filters.hide_all_colors {
            if let Some(color) = app_state.get_item_color(&entry.path) {
                let color_indicator = match color {
                    MarkerColor::Red => "🔴",
                    MarkerColor::Orange => "🟠",
                    MarkerColor::Yellow => "🟡",
                    MarkerColor::Blue => "🔵",
                    MarkerColor::Cyan => "🔷",
                    MarkerColor::Pink => "💗",
                    MarkerColor::White => "⚪",
                    MarkerColor::Green => "🟢",
                    MarkerColor::Magenta => "🟣",
                    MarkerColor::Reset => "⭕",
                };
                write!(stdout, " {}", color_indicator)?;
            }
        }

        execute!(stdout, SetForegroundColor(path_color))?;
        write!(
            stdout,
            " {:width$}",
            relative_path_str,
            width = path_width as usize
        )?;
    }

    // Draw footer
    execute!(stdout, MoveTo(0, end_y), SetForegroundColor(Color::Yellow))?;
    writeln!(
        stdout,
        "Found {} results. ({}/{})",
        results.len(),
        selected_index_usize + 1,
        results.len()
    )?;

    execute!(stdout, ResetColor)?;
    stdout.flush()?;
    Ok(())
}
fn dim_color(color: Color, dim_factor: u8) -> Color {
    match color {
        Color::Rgb { r, g, b } => {
            let dim = |v: u8| -> u8 {
                let factor = (100 - dim_factor) as f32 / 100.0;
                (v as f32 * factor) as u8
            };
            Color::Rgb {
                r: dim(r),
                g: dim(g),
                b: dim(b),
            }
        }
        Color::Red => dim_color(Color::Rgb { r: 255, g: 0, b: 0 }, dim_factor),
        Color::DarkGrey => {
            let base = 128u8;
            let dim = (base as f32 * (100 - dim_factor) as f32 / 100.0) as u8;
            Color::Rgb {
                r: dim,
                g: dim,
                b: dim,
            }
        }
        _ => color,
    }
}
pub fn set_search_depth_limit(app_state: &mut AppState, stdout: &mut impl Write) -> io::Result<()> {
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, MoveTo(0, 0))?;
    writeln!(
        stdout,
        "Current search depth limit: {}",
        app_state.search_depth_limit
    )?;
    writeln!(stdout, "Enter new search depth limit (0 for no limit):")?;
    stdout.flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if let Ok(limit) = input.trim().parse() {
        app_state.search_depth_limit = limit;
        writeln!(stdout, "Search depth limit updated to: {}", limit)?;
    } else {
        writeln!(stdout, "Invalid input. Depth limit not changed.")?;
    }

    writeln!(stdout, "Press any key to continue...")?;
    stdout.flush()?;
    event::read()?;

    Ok(())
}
