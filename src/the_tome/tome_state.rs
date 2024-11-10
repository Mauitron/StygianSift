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
pub struct ScrollState {
    pub offset: usize,
    pub target_offset: usize,
}

impl ScrollState {
    pub fn new() -> Self {
        ScrollState {
            offset: 0,
            target_offset: 0,
        }
    }

    pub fn update(&mut self) {
        self.offset = self.target_offset;
    }
}
pub struct NavigationInfo {
    pub dir_name: String,
    pub index: usize,
}

pub struct AppState {
    pub input: Vec<u8>,
    pub loops: u64,
    pub lines: usize,
    pub time_iter: u32,
    pub avg_time: Duration,
    pub max_time: Duration,
    pub min_time: Duration,
    pub sum_time: Duration,
    pub last_count: usize,
    pub last_time_stop: Duration,
    pub no_match_len: usize,
    pub is_windows: bool,
    pub file_path: PathBuf,
    pub show_count: bool,
    pub preview_active: bool,
    pub current_file_selected: bool,
    pub multiple_selected_files: Option<HashSet<PathBuf>>,
    pub selected_index: usize,          // was f32
    pub selection_amont: Option<usize>, // was Option<f32>
    pub clipboard: Option<Vec<PathBuf>>,
    pub page_state: PageState,
    pub config: Config,
    pub scroll_state: ScrollState,
    pub file_to_move: Option<PathBuf>,
    pub is_moving: bool,
    pub undo_manager: UndoManager,
    pub current_dir: PathBuf,
    pub last_browsed_dir: PathBuf,
    pub nav_stack: Vec<NavigationInfo>,
    pub search_depth_limit: usize,
    pub colored_items: HashMap<PathBuf, MarkerColor>,
    pub colored_rules: HashMap<MarkerColor, HashSet<PathBuf>>,
    pub git_menu: Option<GitMenu>,
    pub is_git_repo: bool,
    pub git_info: GitInfo,
    pub last_search_term: String,
    pub select_mode: bool,
    pub is_search: bool,
    pub changing_color: bool,
    pub search_filters: SearchFilters,
}

impl AppState {
    pub fn new() -> io::Result<Self> {
        let config = Config::load_config().unwrap_or_else(|_| Config::new());
        let current_dir = config
            .home_folder
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap());

        let temp_dir = std::env::temp_dir().join("file_manager_undo");

        Ok(Self {
            lines: config.lines_shown,
            input: Vec::new(),
            //not in use ----------------------------|
            loops: 0,
            time_iter: 1,
            avg_time: Duration::ZERO,
            max_time: Duration::ZERO,
            min_time: Duration::ZERO,
            sum_time: Duration::ZERO,
            last_count: 0,
            last_time_stop: Duration::ZERO,
            no_match_len: 0,
            show_count: false,
            //not in use----------------------------|
            is_windows: cfg!(target_os = "windows"),
            file_path: PathBuf::new(),
            last_browsed_dir: current_dir.clone(),
            preview_active: false,
            selection_amont: None,
            current_file_selected: false,
            multiple_selected_files: None,
            clipboard: None,
            page_state: PageState::new(),
            file_to_move: None,
            is_moving: false,
            scroll_state: ScrollState::new(),
            undo_manager: UndoManager::new(
                temp_dir,
                config.ram_undo_limit,
                config.disk_undo_limit,
                config.allow_disk_undo,
            )?,
            search_depth_limit: config.search_depth_limit,
            nav_stack: Vec::new(),
            colored_items: HashMap::new(),
            colored_rules: config.colored_items.clone(),
            current_dir,
            git_menu: None,
            config,
            is_git_repo: false,
            git_info: GitInfo::new(),
            selected_index: 0,
            last_search_term: "".to_string(),
            select_mode: false,
            is_search: false,
            changing_color: false,
            search_filters: SearchFilters::new(),
        })
    }

    pub fn display_current_layer(&self, stdout: &mut impl Write) -> io::Result<()> {
        let (width, height) = size()?;
        let nav_width = width / 2;
        let preview_width = width - nav_width - 2;

        let (layer_index, layer_name) = self.config.get_current_layer_info();

        queue!(stdout, MoveTo(preview_width + 3, height - 12))?;
        write!(
            stdout,
            "{}",
            "-".green().to_string().repeat((preview_width - 4).into())
        )?;

        queue!(stdout, MoveTo(preview_width + 28, height - 10))?;
        writeln!(
            stdout,
            "Current Layer: {} ({})",
            layer_index,
            layer_name.green()
        )?;

        queue!(stdout, MoveTo(preview_width + 3, height - 8))?;
        write!(
            stdout,
            "{}",
            "-".green().to_string().repeat((preview_width - 4).into())
        )?;

        stdout.flush()?;
        Ok(())
    }
    pub fn execute_file(&self, stdout: &mut impl Write, file_path: &Path) -> io::Result<()> {
        if !file_path.exists() {
            writeln!(
                stdout,
                "Error: File '{}' does not exist.",
                file_path.display()
            )?;
            return Ok(());
        }

        writeln!(stdout, "Attempting to open file: {}", file_path.display())?;

        let desktop_env = env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| String::new());
        writeln!(stdout, "Detected desktop environment: {}", desktop_env)?;

        let opener = match desktop_env.to_lowercase().as_str() {
            "kde" => "kde-open5",
            "gnome" => "gio open",
            "xfce" => "exo-open",
            _ => {
                if self.command_exists("xdg-open") {
                    "xdg-open"
                } else if self.command_exists("gvfs-open") {
                    "gvfs-open"
                } else if self.command_exists("gnome-open") {
                    "gnome-open"
                } else {
                    writeln!(
                        stdout,
                        "No suitable opener found. Falling back to nix-shell method."
                    )?;
                    return self.nix_shell_open(file_path, stdout);
                }
            }
        };

        writeln!(stdout, "Using opener: {}", opener)?;
        self.run_command(opener, file_path, stdout)
    }

    fn run_command(
        &self,
        command: &str,
        file_path: &Path,
        stdout: &mut impl Write,
    ) -> io::Result<()> {
        let output = if command.contains(' ') {
            let mut parts = command.split_whitespace();
            let cmd = parts.next().unwrap();
            let arg = parts.next().unwrap();
            Command::new(cmd).arg(arg).arg(file_path).output()?
        } else {
            Command::new(command).arg(file_path).output()?
        };

        if output.status.success() {
            writeln!(stdout, "File opened successfully.")?;
        } else {
            writeln!(stdout, "Failed to open file. Error output:")?;
            stdout.write_all(&output.stderr)?;
        }

        Ok(())
    }

    fn nix_shell_open(&self, file_path: &Path, stdout: &mut impl Write) -> io::Result<()> {
        let output = Command::new("nix-shell")
            .args(&[
                "-p",
                "xdg-utils",
                "--run",
                &format!("xdg-open '{}'", file_path.display()),
            ])
            .output()?;

        if output.status.success() {
            writeln!(stdout, "File opened successfully using nix-shell.")?;
        } else {
            writeln!(stdout, "Failed to open file using nix-shell. Error output:")?;
            stdout.write_all(&output.stderr)?;
        }

        Ok(())
    }

    fn command_exists(&self, command: &str) -> bool {
        Command::new("which")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    pub fn create_directory(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        let name = match self.prompt_for_name(stdout, "Enter new directory name: ") {
            Ok(Some(name)) => name,
            Ok(None) => {
                writeln!(stdout, "Directory creation cancelled.")?;
                return Ok(());
            }
            Err(e) => {
                writeln!(stdout, "Error during name input: {}", e)?;
                return Ok(());
            }
        };

        let new_dir_path = self.current_dir.join(&name);

        if new_dir_path.exists() {
            writeln!(stdout, "Error: '{}' already exists.", name)?;
            return Ok(());
        }

        if !self.check_creation_allowed(&new_dir_path) {
            writeln!(
                stdout,
                "Creation not allowed due to color rules or permissions."
            )?;
            return Ok(());
        }

        match fs::create_dir(&new_dir_path) {
            Ok(_) => {
                self.add_create_undo_entry(&new_dir_path, true)?;
                writeln!(stdout, "Directory '{}' created successfully.", name)?;
            }
            Err(e) => writeln!(stdout, "Failed to create directory: {}", e)?,
        }

        Ok(())
    }

    pub fn create_file(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        let name = match self.prompt_for_name(stdout, "Enter new file name: ") {
            Ok(Some(name)) => name,
            Ok(None) => {
                writeln!(stdout, "File creation cancelled.")?;
                return Ok(());
            }
            Err(e) => {
                writeln!(stdout, "Error during name input: {}", e)?;
                return Ok(());
            }
        };

        let new_file_path = self.current_dir.join(&name);

        if new_file_path.exists() {
            writeln!(stdout, "Error: '{}' already exists.", name)?;
            return Ok(());
        }

        if !self.check_creation_allowed(&new_file_path) {
            writeln!(
                stdout,
                "Creation not allowed due to color rules or permissions."
            )?;
            return Ok(());
        }

        match fs::File::create(&new_file_path) {
            Ok(_) => {
                self.add_create_undo_entry(&new_file_path, false)?;
                writeln!(stdout, "File '{}' created successfully.", name)?;
            }
            Err(e) => writeln!(stdout, "Failed to create file: {}", e)?,
        }

        Ok(())
    }

    fn check_creation_allowed(&self, path: &Path) -> bool {
        let parent_path = path.parent().unwrap_or(Path::new(""));
        self.check_operation_allowed(parent_path, "create")
    }

    fn prompt_for_name(&self, stdout: &mut impl Write, prompt: &str) -> io::Result<Option<String>> {
        writeln!(stdout, "{}", prompt)?;
        stdout.flush()?;

        let mut name = String::new();
        let start_time = Instant::now();

        loop {
            if start_time.elapsed() > INPUT_TIMEOUT {
                return Ok(None);
            }

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Enter => {
                            if !name.is_empty() {
                                break;
                            }
                        }
                        KeyCode::Char(c) if name.len() < MAX_NAME_LENGTH => {
                            name.push(c);
                            write!(stdout, "{}", c)?;
                        }
                        KeyCode::Backspace => {
                            if !name.is_empty() {
                                name.pop();
                                write!(stdout, "\x08 \x08")?;
                            }
                        }
                        KeyCode::Esc => {
                            return Ok(None);
                        }
                        _ => {}
                    }
                    stdout.flush()?;
                }
            }
        }

        writeln!(stdout)?;
        Ok(Some(name))
    }

    fn add_create_undo_entry(&mut self, path: &Path, is_directory: bool) -> io::Result<()> {
        self.undo_manager.add_tome_entry(UndoEntry {
            operation: Operation::Create {
                path: path.to_path_buf(),
                is_directory,
                timestamp: std::time::SystemTime::now(),
            },
            storage: UndoStorage::Ram(Vec::new()),
            original_path: path.to_path_buf(),
            size: 0,
        })
    }

    fn is_git_repo(path: &Path) -> bool {
        path.join(".git").is_dir()
    }
    pub fn update_current_dir(&mut self, new_dir: PathBuf) {
        self.current_dir = new_dir;
        self.is_git_repo = is_git_repository(&self.current_dir);
    }
    pub fn execute_git_command(&mut self, command: &str) -> io::Result<()> {
        let mut full_command = String::from(command);

        if command == "commit\r" {
            let message = self.read_input("Enter commit message: \r")?;
            full_command = format!("commit -m '{}'\r", message);
        } else if command == "checkout\r" || command == "merge\r" {
            let branch = self.read_input("Enter branch name: \r")?;
            full_command = format!("\r{} {}\r", command, branch);
        }

        let output = Command::new("git")
            .args(full_command.split_whitespace())
            .current_dir(&self.current_dir)
            .output()?;

        let mut stdout = io::stdout();
        execute!(stdout, Clear(ClearType::All))?;
        writeln!(stdout, "Git command output:\r")?;
        writeln!(stdout, "{}\r", String::from_utf8_lossy(&output.stdout))?;
        writeln!(stdout, "{}\r", String::from_utf8_lossy(&output.stderr))?;
        writeln!(stdout, "Press any key to continue...\r")?;
        stdout.flush()?;
        event::read()?;

        Ok(())
    }

    pub fn read_input(&self, prompt: &str) -> io::Result<String> {
        let mut stdout = io::stdout();
        execute!(stdout, Clear(ClearType::CurrentLine))?;
        write!(stdout, "{}", prompt)?;
        stdout.flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    pub fn check_operation_allowed(&self, path: &Path, operation: &str) -> bool {
        if !self.check_single_item_allowed(path, operation) {
            return false;
        }

        if path.is_dir() {
            self.check_directory_contents_allowed(path, operation)
        } else {
            true
        }
    }
    fn check_single_item_allowed(&self, path: &Path, operation: &str) -> bool {
        if let Some(color) = self.get_item_color(path) {
            if let Some(rule) = self.config.color_rules.get(&color) {
                match operation {
                    "delete" => rule.allow_delete,
                    "rename" => rule.allow_rename,
                    "move" => rule.allow_move,
                    "copy" => rule.allow_copy,
                    _ => true,
                }
            } else {
                true
            }
        } else {
            true
        }
    }

    fn check_directory_contents_allowed(&self, dir_path: &Path, operation: &str) -> bool {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if !self.check_operation_allowed(&path, operation) {
                        return false;
                    }
                }
            }
        }
        true
    }
    pub fn is_searchable(&self, path: &Path) -> bool {
        if let Some(color) = self.get_item_color(path) {
            if let Some(rule) = self.config.color_rules.get(&color) {
                rule.include_in_search
            } else {
                true // If no rule is set for the color, include in search
            }
        } else {
            true // If the item has no color, include in search
        }
    }
    pub fn set_color_rule(&mut self, color: MarkerColor, rule: ColorRule) {
        self.config.color_rules.insert(color, rule);
    }

    pub fn get_color_rule(&self, color: &MarkerColor) -> Option<&ColorRule> {
        self.config.color_rules.get(color)
    }
    pub fn cycle_item_color(
        &mut self,
        entries: &[FileEntry],
        selected_index: usize,
    ) -> io::Result<()> {
        self.changing_color = true;
        let files_to_cycle = if let Some(selected) = &self.multiple_selected_files {
            selected.iter().cloned().collect::<Vec<_>>()
        } else {
            vec![entries[selected_index].path.clone()]
        };

        if files_to_cycle.is_empty() {
            return Ok(());
        }

        let current_color = self.get_item_color(&files_to_cycle[0]);
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
                Some(color) => self.set_item_color(path, color),
                None => self.remove_item_color(&path),
            }
        }

        Ok(())
    }
    pub fn set_item_color(&mut self, path: PathBuf, color: MarkerColor) {
        self.colored_items.insert(path.clone(), color);
        self.config.set_item_color(path, color);
    }

    pub fn remove_item_color(&mut self, path: &Path) {
        self.colored_items.remove(path);
        self.config.remove_item_color(path);
    }
    pub fn get_item_color(&self, path: &Path) -> Option<MarkerColor> {
        for (color, paths) in &self.config.colored_items {
            if paths.contains(path) {
                return Some(*color);
            }
        }
        None
    }

    pub fn open_terminal(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        open_terminal_command(self, stdout)
    }

    pub fn toggle_selection(&mut self, path: &Path, direction: i32) -> bool {
        let selected = self
            .multiple_selected_files
            .get_or_insert_with(HashSet::new);
        let was_selected = selected.contains(path);

        if (direction > 0 && !was_selected) || (direction < 0 && was_selected) {
            if was_selected {
                selected.remove(path);
            } else {
                selected.insert(path.to_path_buf());
            }
            true
        } else {
            false
        }
    }
    pub fn clear_multi_select(&mut self) -> io::Result<()> {
        self.multiple_selected_files = None;
        self.selection_amont = None;
        // let _ = clear_preview();
        Ok(())
    }
    pub fn select_range(
        &mut self,
        selected_index: usize,
        start: usize,
        end: usize,
        entries: &[FileEntry],
    ) -> io::Result<()> {
        let (width, height) = size()?;
        let nav_width = width / 2;
        let preview_width = width - nav_width - 2;
        let selected = self
            .multiple_selected_files
            .get_or_insert_with(HashSet::new);

        if self.select_mode {
            if let Some(entry) = entries.get(selected_index) {
                if selected.contains(&entry.path) {
                    selected.remove(&entry.path);
                } else {
                    selected.insert(entry.path.clone());
                }
            }
        } else {
            let (range_start, range_end) = if start <= end {
                (start, end)
            } else {
                (end, start)
            };

            selected.retain(|path| {
                entries[range_start..=range_end]
                    .iter()
                    .any(|entry| &entry.path == path)
            });

            selected.extend(
                entries[range_start..=range_end]
                    .iter()
                    .map(|entry| entry.path.clone()),
            );
        }
        queue!(stdout(), MoveTo(preview_width + 3, height - 12))?;
        write!(
            stdout(),
            "{}",
            "-".green().to_string().repeat((preview_width - 4).into())
        )?;
        queue!(stdout(), MoveTo(preview_width + 35, height - 10))?;
        writeln!(stdout(), "                                    ")?;
        queue!(stdout(), MoveTo(preview_width + 34, height - 10))?;
        writeln!(
            stdout(),
            "Selected Files: {}\r",
            selected.len().to_string().red()
        )?;
        queue!(stdout(), MoveTo(preview_width + 3, height - 8))?;
        write!(
            stdout(),
            "{}",
            "-".green().to_string().repeat((preview_width - 4).into())
        )?;

        Ok(())
    }
    pub fn select_all(&mut self, entries: &[FileEntry]) {
        self.multiple_selected_files =
            Some(entries.iter().map(|entry| entry.path.clone()).collect());
    }
    pub fn is_selected(&self, path: &PathBuf) -> bool {
        self.multiple_selected_files
            .as_ref()
            .map_or(false, |selected| selected.contains(path))
    }
    pub fn set_shortcut(
        &mut self,
        key: char,
        path: PathBuf,
        name: String,
        index: usize,
    ) -> io::Result<()> {
        self.config
            .shortcuts
            .get_or_insert_with(HashMap::new)
            .insert(key, (path, name, index));
        self.config.save_config()
    }

    pub fn has_shortcut_for_path(&self, path: &PathBuf) -> Option<char> {
        self.config.shortcuts.as_ref().and_then(|shortcuts| {
            shortcuts
                .iter()
                .find(|(_, &ref p)| p.0 == *path)
                .map(|(&k, _)| k)
        })
    }
    pub fn get_shortcut(&self, key: char) -> Option<&(PathBuf, String, usize)> {
        self.config
            .shortcuts
            .as_ref()
            .and_then(|shortcuts| shortcuts.get(&key))
    }

    pub fn remove_shortcut(&mut self, key: char) -> io::Result<()> {
        if let Some(shortcuts) = self.config.shortcuts.as_mut() {
            shortcuts.remove(&key);
            if shortcuts.is_empty() {
                self.config.shortcuts = None;
            }
        }
        self.save_config()
    }

    pub fn clear_shortcuts(&mut self) -> io::Result<()> {
        self.config.shortcuts = None;
        self.save_config()
    }

    pub fn save_config(&self) -> io::Result<()> {
        self.config.save_config()
    }

    pub fn reload_config(&mut self) -> io::Result<()> {
        let loaded_config = Config::load_config()?;

        self.config.home_folder = loaded_config.home_folder;
        self.config.lines_shown = loaded_config.lines_shown;
        self.config.default_sort = loaded_config.default_sort;

        if let Some(new_shortcuts) = loaded_config.shortcuts {
            if self.config.shortcuts.is_none() {
                self.config.shortcuts = Some(new_shortcuts);
            } else {
                let existing_shortcuts = self.config.shortcuts.as_mut().unwrap();
                for (key, path) in new_shortcuts {
                    existing_shortcuts.entry(key).or_insert(path);
                }
            }
        }

        Ok(())
    }

    pub fn set_home_folder(&mut self, path: Option<PathBuf>) -> io::Result<()> {
        self.config.set_home_folder(path.clone());
        if let Some(path) = path {
            self.current_dir = path.clone();
        }
        self.config.save_config()
    }

    pub fn set_current_dir(&mut self, path: PathBuf) -> std::io::Result<()> {
        self.current_dir = path.clone();
        self.config.home_folder = Some(path);
        self.save_config()
    }
    pub fn start_move(&mut self, current_entry: Option<&FileEntry>) -> Result<(), io::Error> {
        if self.multiple_selected_files.is_some() {
            for path in self.multiple_selected_files.as_ref().unwrap() {
                if !self.check_operation_allowed(path, "move") {
                    return Err(io::Error::new(
                        io::ErrorKind::PermissionDenied,
                        format!("Move not allowed for file: {}", path.display()),
                    ));
                }
            }
        } else if let Some(entry) = current_entry {
            if !self.check_operation_allowed(&entry.path, "move") {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    format!("Move not allowed for file: {}", entry.path.display()),
                ));
            }
            self.file_to_move = Some(entry.path.clone());
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "No files selected for moving",
            ));
        }

        self.is_moving = true;
        Ok(())
    }

    pub fn finish_move(&mut self, dest_dir: &Path) -> io::Result<()> {
        let files_to_move = if let Some(selected) = self.multiple_selected_files.take() {
            selected.into_iter().collect::<Vec<_>>()
        } else if let Some(file) = self.file_to_move.take() {
            vec![file]
        } else {
            return Ok(());
        };

        for source_path in files_to_move {
            let file_name = source_path.file_name().unwrap_or_default();
            let dest_path = dest_dir.join(file_name);

            fs::rename(&source_path, &dest_path)?;
            self.undo_manager.add_tome_entry(UndoEntry {
                operation: Operation::Move {
                    old_path: source_path.clone(),
                    new_path: dest_path.clone(),
                    timestamp: SystemTime::now(),
                },
                storage: UndoStorage::Ram(Vec::new()),
                original_path: source_path,
                size: 0,
            })?;
        }

        self.is_moving = false;
        self.clear_selection();
        Ok(())
    }
    pub fn clear_selection(&mut self) {
        self.multiple_selected_files = None;
        self.file_to_move = None;
        self.current_file_selected = false;
        self.selection_amont = None;
    }
    pub fn cancel_move(&mut self) {
        self.file_to_move = None;
        self.is_moving = false;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortOrder {
    DateModifiedAsc,
    DateModifiedDesc,
    NameAsc,
    NameDesc,
    SizeAsc,
    SizeDesc,
    TypeAsc,
    TypeDesc,
    ColorAsc,
    ColorDesc,
}

impl SortOrder {
    pub fn from_str(s: &str) -> Self {
        match s {
            "NameAsc" => SortOrder::NameAsc,
            "NameDesc" => SortOrder::NameDesc,
            "SizeAsc" => SortOrder::SizeAsc,
            "SizeDesc" => SortOrder::SizeDesc,
            "TypeAsc" => SortOrder::TypeAsc,
            "TypeDesc" => SortOrder::TypeDesc,
            "ColorAsc" => SortOrder::ColorAsc,
            "ColorDesc" => SortOrder::ColorDesc,
            "DateModifiedAsc" => SortOrder::DateModifiedAsc,
            "DateModifiedDesc" => SortOrder::DateModifiedDesc,
            _ => SortOrder::NameAsc,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            SortOrder::NameAsc => "NameAsc",
            SortOrder::NameDesc => "NameDesc",
            SortOrder::SizeAsc => "SizeAsc",
            SortOrder::SizeDesc => "SizeDesc",
            SortOrder::TypeAsc => "TypeAsc",
            SortOrder::TypeDesc => "TypeDesc",
            SortOrder::ColorAsc => "ColorAsc",
            SortOrder::ColorDesc => "ColorDesc",
            SortOrder::DateModifiedAsc => "DateModifiedAsc",
            SortOrder::DateModifiedDesc => "DateModifiedDesc",
        }
    }
}

pub struct PageState {
    pub left_page: u32,
    pub right_page: u32,
    pub total_pages: u32,
}

impl PageState {
    pub fn new() -> Self {
        let start_page = PageState::generate_random_start_page();
        PageState {
            left_page: start_page,
            right_page: 1,
            total_pages: 1000,
        }
    }

    fn generate_random_start_page() -> u32 {
        let now = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        (now % 401 + 200) as u32
    }

    pub fn move_left(&mut self) {
        if self.left_page > 1 {
            self.left_page -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.left_page < self.total_pages {
            self.left_page += 1;
        }
    }

    pub fn update_right_page(&mut self, selected_index: usize, total_entries: usize) {
        if total_entries > 0 {
            self.right_page = (selected_index as u32).saturating_add(1);
        } else {
            self.right_page = 1;
        }
    }
}
