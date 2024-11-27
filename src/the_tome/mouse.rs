use super::*;
pub struct ConfirmationWindow {
    pub message: String,
    pub selected: usize,
    pub position: (u16, u16),
    pub hovered: Option<usize>,
    pub result: Option<bool>,
    buttons: Vec<Button>,
}
pub struct NavButtons {
    _back_pos: (u16, u16),
    is_visible: bool,
}

impl NavButtons {
    pub fn new() -> Self {
        Self {
            _back_pos: (0, 0),
            is_visible: false,
        }
    }

    pub fn draw(&self, stdout: &mut impl Write, nav_width: u16) -> io::Result<()> {
        if !self.is_visible {
            return Ok(());
        }

        queue!(stdout, MoveTo(nav_width / 9, 7))?;
        write!(stdout, "{}", "⫷⭅".green())?;

        stdout.flush()
    }

    pub fn handle_click(&self, x: u16, y: u16, nav_width: u16) -> Option<NavigationAction> {
        if !self.is_visible || y != 7 {
            return None;
        }

        if x >= 9 && x <= 11 {
            Some(NavigationAction::Back)
        } else {
            None
        }
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }
}

pub enum NavigationAction {
    Back,
}

struct Button {
    text: String,
    x: u16,
    width: u16,
    value: bool,
}

impl ConfirmationWindow {
    pub fn new(message: &str, position: (u16, u16)) -> Self {
        Self {
            message: message.to_string(),
            selected: 0,
            position,
            buttons: vec![
                Button {
                    text: "Yes".to_string(),
                    x: 0,
                    width: 6,
                    value: true,
                },
                Button {
                    text: "No".to_string(),
                    x: 0,
                    width: 6,
                    value: false,
                },
            ],
            hovered: None,
            result: None,
        }
    }

    pub fn draw(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        let (menu_x, menu_y) = self.position;
        let message_width = self.message.len() as u16 + 4;
        let menu_width = message_width.max(20);

        let button_spacing = 2;
        let total_buttons_width: u16 = self.buttons.iter().map(|b| b.width).sum();
        let total_spacing = (self.buttons.len() - 1) as u16 * button_spacing;
        let buttons_start = menu_x + (menu_width - total_buttons_width - total_spacing) / 2;

        let mut current_x = buttons_start;
        for button in &mut self.buttons {
            button.x = current_x;
            current_x += button.width + button_spacing;
        }

        queue!(
            stdout,
            MoveTo(menu_x, menu_y),
            SetBackgroundColor(Color::Black),
            SetForegroundColor(Color::White)
        )?;
        write!(
            stdout,
            "╭{}╮",
            "─".repeat(menu_width as usize - 2).dark_yellow()
        )?;

        queue!(
            stdout,
            MoveTo(menu_x, menu_y + 1),
            SetForegroundColor(Color::White)
        )?;
        write!(
            stdout,
            "│ {} {}│",
            self.message.to_string().grey(),
            " ".repeat(menu_width as usize - self.message.len() - 4)
        )?;

        queue!(stdout, MoveTo(menu_x, menu_y + 2))?;
        write!(stdout, "│{}│", " ".repeat(menu_width as usize - 2))?;

        for (i, button) in self.buttons.iter().enumerate() {
            let is_selected = i == self.selected;
            let is_hovered = Some(i) == self.hovered;

            queue!(stdout, MoveTo(button.x, menu_y + 2))?;
            if is_selected || is_hovered {
                queue!(
                    stdout,
                    SetBackgroundColor(Color::Reset),
                    SetBackgroundColor(Color::DarkBlue),
                    SetForegroundColor(Color::White)
                )?;
            } else {
                queue!(
                    stdout,
                    SetBackgroundColor(Color::Reset),
                    SetBackgroundColor(Color::Black),
                    SetForegroundColor(Color::White)
                )?;
            }

            write!(stdout, " {} |", button.text.to_string())?;
        }

        // bottom border
        queue!(
            stdout,
            MoveTo(menu_x, menu_y + 3),
            SetBackgroundColor(Color::Reset),
            SetBackgroundColor(Color::Black),
            SetForegroundColor(Color::DarkYellow)
        )?;
        write!(stdout, "╰{}╯", "─".repeat(menu_width as usize - 2).yellow())?;

        queue!(
            stdout,
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::Reset)
        )?;

        stdout.flush()
    }

    pub fn confirmation_handle_event(&mut self, event: Event) -> Option<bool> {
        match event {
            Event::Key(key) => self.confirmation_handle_key_event(key),
            Event::Mouse(mouse) => self.confirmation_handle_mouse_event(mouse),
            _ => None,
        }
    }

    fn confirmation_handle_key_event(&mut self, key: KeyEvent) -> Option<bool> {
        match key.code {
            KeyCode::Left | KeyCode::Right | KeyCode::Char('h') | KeyCode::Char('l') => {
                self.selected = 1 - self.selected;
                self.hovered = Some(self.selected);
                None
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.result = Some(self.buttons[self.selected].value);
                Some(self.buttons[self.selected].value)
            }
            KeyCode::Esc => {
                self.result = Some(false);
                Some(false)
            }
            _ => None,
        }
    }

    fn confirmation_handle_mouse_event(&mut self, event: MouseEvent) -> Option<bool> {
        let (_, menu_y) = self.position;

        match event.kind {
            MouseEventKind::Moved => {
                if event.row == menu_y + 2 {
                    let mut found_hover = None;
                    for (i, button) in self.buttons.iter().enumerate() {
                        if event.column >= button.x && event.column < button.x + button.width {
                            found_hover = Some(i);
                            break;
                        }
                    }
                    if self.hovered != found_hover {
                        self.hovered = found_hover;
                        if let Some(idx) = found_hover {
                            self.selected = idx;
                        }
                    }
                } else {
                    self.hovered = None;
                }
                None
            }
            MouseEventKind::Down(MouseButton::Left) => {
                if event.row == menu_y + 2 {
                    for (i, button) in self.buttons.iter().enumerate() {
                        if event.column >= button.x && event.column < button.x + button.width {
                            self.selected = i;
                            self.result = Some(button.value);
                            return Some(button.value);
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }
}
pub struct MouseState {
    pub _confirmation_window: Option<ConfirmationWindow>,
    pub context_menu: Option<ContextMenu>,
    pub drag_start: Option<(u16, u16)>,
    pub hovered_index: Option<usize>,
    pub is_dragging: bool,
    pub last_click_pos: Option<(u16, u16)>,
    pub last_click_time: Instant,
    pub nav_buttons: NavButtons,
    pub current_drag_pos: Option<(u16, u16)>,
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            last_click_time: Instant::now(),
            last_click_pos: None,
            drag_start: None,
            is_dragging: false,
            context_menu: None,
            hovered_index: None,
            _confirmation_window: None,
            nav_buttons: NavButtons::new(),
            current_drag_pos: None,
        }
    }
}
#[derive(Debug, Clone)]
pub struct ContextMenuItem {
    pub label: String,
    pub action: ContextMenuAction,
    pub shortcut: String,
}
impl ContextMenuItem {
    fn new(label: &str, action: ContextMenuAction, shortcut: &str) -> Self {
        Self {
            label: label.to_string(),
            action,
            shortcut: shortcut.to_string(),
        }
    }
}

pub struct ContextMenu {
    pub items: Vec<ContextMenuItem>,
    pub selected: usize,
    pub position: (u16, u16),
    pub hovered: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum ContextMenuAction {
    Open,
    Copy,
    Paste,
    Cut,
    Rename,
    Delete,
    Properties,
    Duplicate,
    CreateFile,
    CreateDirectory,
    SelectAll,
    Cancel,
}

impl ContextMenu {
    pub fn new(position: (u16, u16)) -> Self {
        let items = vec![
            ContextMenuItem::new("Open", ContextMenuAction::Open, "Enter"),
            ContextMenuItem::new("Copy", ContextMenuAction::Copy, "C"),
            ContextMenuItem::new("Paste", ContextMenuAction::Paste, "V"),
            ContextMenuItem::new("Cut", ContextMenuAction::Cut, "X"),
            ContextMenuItem::new("Rename", ContextMenuAction::Rename, "R"),
            ContextMenuItem::new("Delete", ContextMenuAction::Delete, "Del"),
            ContextMenuItem::new("Duplicate", ContextMenuAction::Duplicate, "D"),
            ContextMenuItem::new("Properties", ContextMenuAction::Properties, "P"),
            ContextMenuItem::new("New File", ContextMenuAction::CreateFile, "N"),
            ContextMenuItem::new("New Directory", ContextMenuAction::CreateDirectory, "D"),
            ContextMenuItem::new("Select All", ContextMenuAction::SelectAll, "A"),
        ];

        Self {
            items,
            selected: 0,
            position,
            hovered: None,
        }
    }

    pub fn draw(&self, stdout: &mut impl Write) -> io::Result<()> {
        let (menu_x, menu_y) = self.position;
        let menu_width = 30;

        // top border
        queue!(
            stdout,
            cursor::MoveTo(menu_x, menu_y),
            SetBackgroundColor(Color::DarkYellow)
        )?;
        write!(stdout, "╭{}╮", "─".repeat(menu_width - 2))?;

        for (i, item) in self.items.iter().enumerate() {
            queue!(stdout, cursor::MoveTo(menu_x, menu_y + 1 + i as u16))?;

            if Some(i) == self.hovered {
                queue!(
                    stdout,
                    SetBackgroundColor(Color::DarkBlue),
                    SetForegroundColor(Color::White)
                )?;
            } else {
                queue!(
                    stdout,
                    SetBackgroundColor(Color::DarkGrey),
                    SetForegroundColor(Color::Grey)
                )?;
            }

            write!(stdout, "│ {}", item.label)?;
            let padding = menu_width - item.label.len() - item.shortcut.len() - 4;
            write!(stdout, "{}{} │", " ".repeat(padding), item.shortcut)?;
        }

        // bottom border
        queue!(
            stdout,
            cursor::MoveTo(menu_x, menu_y + self.items.len() as u16 + 1),
            SetBackgroundColor(Color::DarkGrey),
            SetForegroundColor(Color::Grey)
        )?;
        write!(stdout, "╰{}╯", "─".repeat(menu_width - 2))?;

        queue!(
            stdout,
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::Reset)
        )?;

        stdout.flush()
    }

    pub fn context_handle_mouse_event(&mut self, event: MouseEvent) -> Option<ContextMenuAction> {
        match event.kind {
            MouseEventKind::Moved => {
                let (menu_x, menu_y) = self.position;
                if event.column >= menu_x
                    && event.column <= menu_x + 30
                    && event.row >= menu_y + 1
                    && event.row <= menu_y + self.items.len() as u16
                {
                    let hover_index = (event.row - menu_y - 1) as usize;
                    if hover_index < self.items.len() {
                        self.hovered = Some(hover_index);
                        let _ = self.draw(&mut std::io::stdout());
                    }
                } else {
                    if self.hovered.is_some() {
                        self.hovered = None;
                        let _ = self.draw(&mut std::io::stdout());
                    }
                }
            }
            MouseEventKind::Down(MouseButton::Left) => {
                let (menu_x, menu_y) = self.position;
                if event.column >= menu_x
                    && event.column <= menu_x + 30
                    && event.row >= menu_y + 1
                    && event.row <= menu_y + self.items.len() as u16
                {
                    let selected = (event.row - menu_y - 1) as usize;
                    if selected < self.items.len() {
                        return Some(self.items[selected].action.clone());
                    }
                }
            }
            MouseEventKind::Down(MouseButton::Right) => {
                return Some(ContextMenuAction::Cancel);
            }
            _ => {}
        }
        None
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Option<ContextMenuAction> {
        match key.code {
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                    self.hovered = Some(self.selected);
                }
                None
            }
            KeyCode::Down => {
                if self.selected < self.items.len() - 1 {
                    self.selected += 1;
                    self.hovered = Some(self.selected);
                }
                None
            }
            KeyCode::Enter => Some(self.items[self.selected].action.clone()),
            KeyCode::Esc => Some(ContextMenuAction::Cancel),
            _ => None,
        }
    }
}
pub fn handle_context_menu_action(
    app_state: &mut AppState,
    action: ContextMenuAction,
    entries: &[FileEntry],
) -> io::Result<()> {
    let sort_order = app_state.config.default_sort.clone();
    match action {
        ContextMenuAction::Open => {
            if let Some(entry) = entries.get(app_state.selected_index) {
                app_state.execute_file(&mut stdout(), &entry.path)?;
            }
        }
        ContextMenuAction::Copy => {
            copy_files(app_state, entries, app_state.selected_index);
        }
        ContextMenuAction::Cut => {
            if let Some(entry) = entries.get(app_state.selected_index) {
                app_state.file_to_move = Some(entry.path.clone());
                app_state.is_moving = true;
            }
        }
        ContextMenuAction::Paste => {
            paste_files(app_state, &app_state.current_dir.clone())?;
        }
        ContextMenuAction::Rename => {
            if let Some(entry) = entries.get(app_state.selected_index) {
                rename_file(&mut stdout(), entry, true, app_state)?;
            }
        }
        ContextMenuAction::Delete => {
            app_state.mouse_state.context_menu = None;

            let mut stdout = stdout();
            if app_state.config.draw_simple_borders {
                draw_simple_border(&mut stdout, &app_state.page_state)?;
            } else {
                draw_initial_border(&mut stdout, &app_state.page_state)?;
            }

            display_directory(
                app_state,
                entries,
                &app_state.current_dir.clone(),
                app_state.selected_index,
                &mut stdout,
                app_state.scroll_state.offset,
                (size()?.1 - 15) as usize,
                true,
            )?;

            let (width, _) = size()?;
            let selected_y = app_state.selected_index as u16 + 11;
            let dialog_x = (width / 2) - 15;
            let dialog_y = selected_y.saturating_sub(2);

            let mut dialog =
                ConfirmationWindow::new("Are you sure you want to delete?", (dialog_x, dialog_y));

            dialog.draw(&mut stdout)?;

            while dialog.result.is_none() {
                if let Ok(event) = event::read() {
                    if let Some(confirmed) = dialog.confirmation_handle_event(event) {
                        if confirmed {
                            murder_files(
                                app_state,
                                &mut stdout,
                                entries,
                                app_state.selected_index,
                                true,
                            )?;
                        }
                        display_directory(
                            app_state,
                            entries,
                            &app_state.current_dir.clone(),
                            app_state.selected_index,
                            &mut stdout,
                            app_state.scroll_state.offset,
                            (size()?.1 - 15) as usize,
                            true,
                        )?;
                        break;
                    }
                    dialog.draw(&mut stdout)?;
                }
            }

            return Ok(());
        }
        ContextMenuAction::Properties => {
            if let Some(entry) = entries.get(app_state.selected_index) {
                app_state.preview_active = !app_state.preview_active;
                let preview = app_state.preview_active;
                display_file_info_or_preview(
                    app_state,
                    &mut stdout(),
                    entry,
                    0,
                    0,
                    5,
                    20,
                    preview,
                )?;
            }
        }
        ContextMenuAction::Duplicate => {
            handle_duplicate(app_state, &mut stdout(), entries, app_state.selected_index)?;
        }
        ContextMenuAction::CreateFile => {
            app_state.create_file(&mut stdout())?;
        }
        ContextMenuAction::CreateDirectory => {
            app_state.create_directory(&mut stdout())?;
        }
        ContextMenuAction::SelectAll => {
            app_state.select_all(entries);
        }
        ContextMenuAction::Cancel => {}
    }

    app_state.mouse_state.context_menu = None;
    let mut stdout = stdout();
    if app_state.config.draw_simple_borders {
        draw_simple_border(&mut stdout, &app_state.page_state)?;
    } else {
        draw_initial_border(&mut stdout, &app_state.page_state)?;
    }

    display_directory(
        app_state,
        entries,
        &app_state.current_dir.clone(),
        app_state.selected_index,
        &mut stdout,
        app_state.scroll_state.offset,
        (size()?.1 - 15) as usize,
        true,
    )?;

    Ok(())
}

pub fn show_context_menu(app_state: &mut AppState, position: (u16, u16)) -> io::Result<()> {
    let mut stdout = stdout();
    let context_menu = ContextMenu::new(position);
    context_menu.draw(&mut stdout)?;
    app_state.mouse_state.context_menu = Some(context_menu);
    Ok(())
}
pub fn handle_mouse_event(
    app_state: &mut AppState,
    event: MouseEvent,
    entries: &[FileEntry],
    visible_lines: usize,
    start_y: u16,
    nav_width: u16,
) -> io::Result<Option<BrowseResult>> {
    // Handle back button click first
    if let MouseEventKind::Down(MouseButton::Left) = event.kind {
        if let Some(action) =
            app_state
                .mouse_state
                .nav_buttons
                .handle_click(event.column, event.row, nav_width)
        {
            match action {
                NavigationAction::Back => {
                    let mut current_dir = app_state.current_dir.clone();
                    let mut scroll_offset = app_state.scroll_state.offset;
                    let mut preview_active = app_state.preview_active;
                    let mut index = app_state.selected_index;
                    return handle_move_left(
                        app_state,
                        &mut current_dir,
                        &mut index,
                        &mut scroll_offset,
                        &mut preview_active,
                        &app_state.config.default_sort.clone(),
                        &mut stdout(),
                    )
                    .map(|_| Some(BrowseResult::Continue));
                }
            }
        }
    }

    // Handle context menu if active
    if let Some(context_menu) = &mut app_state.mouse_state.context_menu {
        if let Some(action) = context_menu.context_handle_mouse_event(event) {
            handle_context_menu_action(app_state, action, entries)?;
            return Ok(None);
        }
        return Ok(None);
    }
    let get_index_from_coordinates = |row: u16| -> Option<usize> {
        if row >= start_y + 2 {
            let index = (row - start_y - 2) as usize + app_state.scroll_state.offset;
            if index < entries.len() {
                Some(index)
            } else {
                None
            }
        } else {
            None
        }
    };

    match event.kind {
        MouseEventKind::Moved => {
            if event.column < nav_width {
                if app_state.input_mode != InputMode::Mouse {
                    app_state.input_mode = InputMode::Mouse;
                }

                let hover_index = get_index_from_coordinates(event.row);
                if app_state.mouse_state.hovered_index != hover_index {
                    app_state.mouse_state.hovered_index = hover_index;
                    display_directory(
                        app_state,
                        entries,
                        &app_state.current_dir.clone(),
                        app_state.selected_index,
                        &mut stdout(),
                        app_state.scroll_state.offset,
                        visible_lines,
                        false,
                    )?;
                }
            } else if app_state.mouse_state.hovered_index.is_some() {
                app_state.mouse_state.hovered_index = None;
                display_directory(
                    app_state,
                    entries,
                    &app_state.current_dir.clone(),
                    app_state.selected_index,
                    &mut stdout(),
                    app_state.scroll_state.offset,
                    visible_lines,
                    false,
                )?;
            }
        }

        MouseEventKind::Down(MouseButton::Left) => {
            if let Some(action) =
                app_state
                    .mouse_state
                    .nav_buttons
                    .handle_click(event.column, event.row, nav_width)
            {
                match action {
                    NavigationAction::Back => {
                        let mut current_dir = app_state.current_dir.clone();
                        let mut scroll_offset = app_state.scroll_state.offset;
                        let mut preview_active = app_state.preview_active;
                        let mut index = app_state.selected_index;
                        return handle_move_left(
                            app_state,
                            &mut current_dir,
                            &mut index,
                            &mut scroll_offset,
                            &mut preview_active,
                            &app_state.config.default_sort.clone(),
                            &mut stdout(),
                        )
                        .map(|_| Some(BrowseResult::Continue));
                    }
                }
            }

            if event.column < nav_width {
                if let Some(clicked_index) = get_index_from_coordinates(event.row) {
                    // Initialize selection if needed
                    if event.modifiers.contains(KeyModifiers::CONTROL) {
                        app_state.select_mode = true;
                        if app_state.multiple_selected_files.is_none() {
                            app_state.multiple_selected_files = Some(HashSet::new());
                        }
                    }

                    // Handle double-click
                    if let Some(last_pos) = app_state.mouse_state.last_click_pos {
                        if last_pos == (event.column, event.row)
                            && app_state.mouse_state.last_click_time.elapsed()
                                < Duration::from_millis(500)
                        {
                            app_state.selected_index = clicked_index;
                            let mut current_dir = app_state.current_dir.clone();
                            let mut scroll_offset = app_state.scroll_state.offset;
                            let mut preview_active = app_state.preview_active;

                            return handle_move_right(
                                app_state,
                                &mut current_dir,
                                &mut app_state.selected_index.clone(),
                                &mut scroll_offset,
                                &mut preview_active,
                                entries,
                                &mut stdout(),
                            )
                            .map(|_| Some(BrowseResult::Continue));
                        }
                    }

                    // Update selection state
                    app_state.selected_index = clicked_index;
                    app_state.mouse_state.last_click_pos = Some((event.column, event.row));
                    app_state.mouse_state.last_click_time = Instant::now();
                    app_state.mouse_state.drag_start = Some((event.column, event.row));

                    // Initialize or clear selection based on Ctrl state
                    if !event.modifiers.contains(KeyModifiers::CONTROL) {
                        app_state.multiple_selected_files = Some(HashSet::new());
                        app_state.select_mode = false;
                    }

                    // If Ctrl is held, toggle the clicked item
                    if event.modifiers.contains(KeyModifiers::CONTROL) {
                        if let Some(selected_files) = &mut app_state.multiple_selected_files {
                            if let Some(entry) = entries.get(clicked_index) {
                                if selected_files.contains(&entry.path) {
                                    selected_files.remove(&entry.path);
                                } else {
                                    selected_files.insert(entry.path.clone());
                                }
                            }
                        }
                    }

                    display_directory(
                        app_state,
                        entries,
                        &app_state.current_dir.clone(),
                        app_state.selected_index,
                        &mut stdout(),
                        app_state.scroll_state.offset,
                        visible_lines,
                        false,
                    )?;
                }
            }
        }

        MouseEventKind::Drag(MouseButton::Left) => {
            if event.column < nav_width {
                if let Some((start_x, start_y)) = app_state.mouse_state.drag_start {
                    if let (Some(start_idx), Some(current_idx)) = (
                        get_index_from_coordinates(start_y),
                        get_index_from_coordinates(event.row),
                    ) {
                        app_state.mouse_state.is_dragging = true;

                        // Initialize selection if needed
                        if app_state.multiple_selected_files.is_none() {
                            app_state.multiple_selected_files = Some(HashSet::new());
                        }

                        if let Some(selected) = &mut app_state.multiple_selected_files {
                            let (range_start, range_end) = if start_idx <= current_idx {
                                (start_idx, current_idx)
                            } else {
                                (current_idx, start_idx)
                            };

                            if event.modifiers.contains(KeyModifiers::CONTROL) {
                                // For Ctrl+drag, just add files to selection without clearing or toggling
                                for idx in range_start..=range_end {
                                    if let Some(entry) = entries.get(idx) {
                                        selected.insert(entry.path.clone());
                                    }
                                }
                            } else {
                                // Regular drag behavior - clear and select new range
                                selected.clear();
                                for idx in range_start..=range_end {
                                    if let Some(entry) = entries.get(idx) {
                                        selected.insert(entry.path.clone());
                                    }
                                }
                            }

                            display_directory(
                                app_state,
                                entries,
                                &app_state.current_dir.clone(),
                                app_state.selected_index,
                                &mut stdout(),
                                app_state.scroll_state.offset,
                                visible_lines,
                                false,
                            )?;
                        }
                    }
                }
            }
        }

        MouseEventKind::Up(MouseButton::Left) => {
            if !app_state.mouse_state.is_dragging && !app_state.select_mode {
                app_state.multiple_selected_files = None;
            }
            app_state.mouse_state.drag_start = None;
            app_state.mouse_state.is_dragging = false;
        }
        MouseEventKind::ScrollDown => {
            let new_offset = (app_state.scroll_state.offset + 3)
                .min(entries.len().saturating_sub(visible_lines));
            if new_offset != app_state.scroll_state.offset {
                app_state.scroll_state.offset = new_offset;
                display_directory(
                    app_state,
                    entries,
                    &app_state.current_dir.clone(),
                    app_state.selected_index,
                    &mut stdout(),
                    app_state.scroll_state.offset,
                    visible_lines,
                    false,
                )?;
            }
        }

        MouseEventKind::ScrollUp => {
            let new_offset = app_state.scroll_state.offset.saturating_sub(3);
            if new_offset != app_state.scroll_state.offset {
                app_state.scroll_state.offset = new_offset;
                display_directory(
                    app_state,
                    entries,
                    &app_state.current_dir.clone(),
                    app_state.selected_index,
                    &mut stdout(),
                    app_state.scroll_state.offset,
                    visible_lines,
                    false,
                )?;
            }
        }

        MouseEventKind::Down(MouseButton::Right) => {
            if event.column < nav_width {
                let mut stdout = stdout();
                if app_state.config.draw_simple_borders {
                    draw_simple_border(&mut stdout, &app_state.page_state)?;
                } else {
                    draw_initial_border(&mut stdout, &app_state.page_state)?;
                }
                stdout.flush()?;

                if let Some(clicked_index) = get_index_from_coordinates(event.row) {
                    app_state.selected_index = clicked_index;
                }

                show_context_menu(app_state, (event.column, event.row))?;
            }
        }

        _ => {}
    }

    Ok(None)
}
