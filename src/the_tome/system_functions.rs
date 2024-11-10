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
pub struct TerminalState {
    width: u16,
    height: u16,
    min_width: u16,
    min_height: u16,
}

impl TerminalState {
    pub fn new(min_width: u16, min_height: u16) -> io::Result<Self> {
        let (width, height) = size()?;

        let mut state = Self {
            width,
            height,
            min_width,
            min_height,
        };

        let _ = state.enforce_min_size();

        Ok(state)
    }

    pub fn has_size_changed(&self) -> io::Result<bool> {
        let (current_width, current_height) = size()?;
        Ok(self.width != current_width || self.height != current_height)
    }

    pub fn update(&mut self) -> io::Result<()> {
        let (width, height) = size()?;
        self.width = width;
        self.height = height;

        let _ = self.enforce_min_size();
        Ok(())
    }

    fn enforce_min_size(&mut self) -> io::Result<()> {
        let (current_width, current_height) = size()?;

        if current_width < self.min_width || current_height < self.min_height {
            #[cfg(not(target_os = "windows"))]
            {
                execute!(io::stdout(), SetSize(self.min_width, self.min_height))?;
            }

            #[cfg(target_os = "windows")]
            {
                let _ = execute!(io::stdout(), SetSize(self.min_width, self.min_height));
            }

            self.width = self.min_width;
            self.height = self.min_height;
        }

        Ok(())
    }

    pub fn get_dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn get_min_dimensions(&self) -> (u16, u16) {
        (self.min_width, self.min_height)
    }

    pub fn get_actual_dimensions(&self) -> io::Result<(u16, u16)> {
        size()
    }
}
pub struct GitInfo {
    pub is_git_repo: bool,
    pub file_statuses: HashMap<PathBuf, GitStatus>,
}

impl GitInfo {
    pub fn new() -> Self {
        GitInfo {
            is_git_repo: false,
            file_statuses: HashMap::new(),
        }
    }
}
pub struct GitMenuItem {
    pub label: &'static str,
    pub description: &'static str,
    pub command: &'static str,
}

pub struct GitMenu {
    pub items: Vec<GitMenuItem>,
    pub selected: usize,
}
impl GitMenu {
    pub fn new() -> Self {
        GitMenu {
            items: vec![
                GitMenuItem {
                    label: "status",
                    description: "Show the working tree status",
                    command: "status",
                },
                GitMenuItem {
                    label: "Commit",
                    description: "Commit to the repository",
                    command: "commit",
                },
                GitMenuItem {
                    label: "Push",
                    description: "Push to the repository",
                    command: "push",
                },
                GitMenuItem {
                    label: "Pull",
                    description: "Pull branch",
                    command: "pull",
                },
                GitMenuItem {
                    label: "Fetch",
                    description: "fetch from repository",
                    command: "fetch",
                },
            ],
            selected: 0,
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected < self.items.len() - 1 {
            self.selected += 1;
        }
    }

    pub fn get_selected_command(&self) -> &str {
        self.items[self.selected].command
    }
}
pub fn is_git_repo(path: &Path) -> bool {
    path.join(".git").is_dir()
}

pub fn get_git_statuses(path: &Path) -> io::Result<HashMap<PathBuf, GitStatus>> {
    let output = Command::new("git")
        .args(&["status", "--porcelain", "-z"])
        .current_dir(path)
        .output()?;

    if !output.status.success() {
        return Ok(HashMap::new());
    }

    let mut statuses = HashMap::new();
    let entries = output
        .stdout
        .split(|&b| b == 0)
        .filter(|entry| !entry.is_empty());

    for entry in entries {
        if entry.len() < 3 {
            continue;
        }
        let status_code = &entry[0..2];
        let file_path = Path::new(std::str::from_utf8(&entry[3..]).unwrap());
        let status = match status_code {
            b" M" => GitStatus::Modified,
            b"A " => GitStatus::Added,
            b"D " => GitStatus::Deleted,
            b"R " => GitStatus::Renamed,
            b"??" => GitStatus::Untracked,
            _ => GitStatus::Unmodified,
        };
        statuses.insert(path.join(file_path), status);
    }

    Ok(statuses)
}
pub fn handle_permission_issue(
    stdout: &mut impl Write,
    item_name: &str,
    path: &Path,
) -> io::Result<Option<Vec<String>>> {
    let _ = item_name;
    let _ = clear_preview();
    execute!(stdout, MoveTo(0, 0))?;
    writeln!(
        stdout,
        "Permission denied when trying to access: {}\r",
        path.display()
    )?;
    writeln!(
        stdout,
        "\nThis item requires elevated permissions to access.\r"
    )?;
    writeln!(
        stdout,
        "Do you want to attempt to access it with sudo? (y/n): "
    )?;
    stdout.flush()?;

    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    if cfg!(unix) {
                        let output = Command::new("sudo")
                            .arg("ls")
                            .arg("-la")
                            .arg(path)
                            .output()?;

                        if output.status.success() {
                            let contents = String::from_utf8_lossy(&output.stdout)
                                .lines()
                                .map(String::from)
                                .collect::<Vec<String>>();
                            writeln!(stdout, "Sudo access granted. Entering directory...\r")?;
                            return Ok(Some(contents));
                        } else {
                            writeln!(stdout, "Failed to access directory even with sudo.\r")?;
                            writeln!(
                                stdout,
                                "Error: {}\r",
                                String::from_utf8_lossy(&output.stderr)
                            )?;
                        }
                    } else {
                        writeln!(stdout, "Sudo is not available on this system.\r")?;
                    }
                    return Ok(None);
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    writeln!(stdout, "Cancelling access attempt.\r")?;
                    writeln!(
                        stdout,
                        "Press any key to go back to the parent directory...\r"
                    )?;
                    stdout.flush()?;
                    loop {
                        if event::poll(Duration::from_millis(100))? {
                            if let Event::Key(_) = event::read()? {
                                break;
                            }
                        }
                    }
                    return Ok(None);
                }
                _ => writeln!(stdout, "Invalid input. Please enter 'y' or 'n'.\r")?,
            }
        }
        stdout.flush()?;
    }
}

fn handle_elevated_access(stdout: &mut impl Write, path: &Path) -> io::Result<bool> {
    let _ = stdout;
    let status = Command::new("sudo").arg("-v").status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            let access_granted = Command::new("sudo")
                .arg("ls")
                .arg(path)
                .status()
                .map(|s| s.success())
                .unwrap_or(false);

            if access_granted {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
    }
}

pub fn truncate_str(s: &str, max_width: usize) -> String {
    if s.chars().count() > max_width {
        format!("{}…", s.chars().take(max_width - 1).collect::<String>())
    } else {
        s.to_string()
    }
}

pub fn get_sorted_entries(
    app_state: &AppState,
    dir: &Path,
    sort_order: &SortOrder,
) -> io::Result<Vec<FileEntry>> {
    let entries: Vec<_> = fs::read_dir(dir)?
        .par_bridge()
        .filter_map(Result::ok)
        .collect();

    let mut entries: Vec<FileEntry> = entries
        .par_iter()
        .with_min_len(256)
        .filter_map(|entry| FileEntry::new(entry.path()).ok())
        .collect();

    match sort_order {
        SortOrder::ColorAsc => entries.par_sort_by(|a, b| {
            let a_color = app_state.get_item_color(&a.path);
            let b_color = app_state.get_item_color(&b.path);
            MarkerColor::color_order(a_color)
                .cmp(&MarkerColor::color_order(b_color))
                .then_with(|| a.name.cmp(&b.name))
        }),
        SortOrder::ColorDesc => entries.par_sort_by(|a, b| {
            let a_color = app_state.get_item_color(&a.path);
            let b_color = app_state.get_item_color(&b.path);
            MarkerColor::color_order(b_color)
                .cmp(&MarkerColor::color_order(a_color))
                .then_with(|| a.name.cmp(&b.name))
        }),
        SortOrder::NameAsc => entries.par_sort_unstable_by(|a, b| a.name.cmp(&b.name)),
        SortOrder::NameDesc => entries.par_sort_unstable_by(|a, b| b.name.cmp(&a.name)),
        SortOrder::TypeAsc => entries.par_sort_unstable_by(|a, b| {
            file_type_order(&a.file_type)
                .cmp(&file_type_order(&b.file_type))
                .then_with(|| a.name.cmp(&b.name))
        }),
        SortOrder::TypeDesc => entries.par_sort_unstable_by(|a, b| {
            file_type_order(&b.file_type)
                .cmp(&file_type_order(&a.file_type))
                .then_with(|| a.name.cmp(&b.name))
        }),
        SortOrder::SizeAsc => entries
            .par_sort_unstable_by(|a, b| a.size.cmp(&b.size).then_with(|| a.name.cmp(&b.name))),
        SortOrder::SizeDesc => entries
            .par_sort_unstable_by(|a, b| b.size.cmp(&a.size).then_with(|| a.name.cmp(&b.name))),
        SortOrder::DateModifiedAsc => {
            entries.par_sort_unstable_by(|a, b| compare_modified_times(a, b, Ordering::Less))
        }
        SortOrder::DateModifiedDesc => {
            entries.par_sort_unstable_by(|a, b| compare_modified_times(a, b, Ordering::Greater))
        }
    }
    Ok(entries)
}

fn compare_modified_times(a: &FileEntry, b: &FileEntry, order: Ordering) -> Ordering {
    let a_time = fs::metadata(&a.path).and_then(|m| m.modified()).ok();
    let b_time = fs::metadata(&b.path).and_then(|m| m.modified()).ok();
    match (a_time, b_time) {
        (Some(a_time), Some(b_time)) => {
            if order == Ordering::Less {
                a_time.cmp(&b_time).then_with(|| a.name.cmp(&b.name))
            } else {
                b_time.cmp(&a_time).then_with(|| a.name.cmp(&b.name))
            }
        }
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => a.name.cmp(&b.name),
    }
} // clear_preview();

pub fn parse_ls_output(contents: &[String], dir: &Path) -> Vec<FileEntry> {
    contents
        .iter()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 9 {
                let name = parts[8..].join(" ");
                let path = dir.join(&name);
                Some(FileEntry {
                    path: path.clone(),
                    name,
                    file_type: if parts[0].starts_with('d') {
                        FileType::Directory
                    } else {
                        FileType::Unknown
                    },
                    size: parts[4].parse().unwrap_or(0),
                    admin_required: false,
                    git_status: None,
                    read_only: parts[0].chars().nth(1) != Some('w'),
                })
            } else {
                None
            }
        })
        .collect()
}

/////////////////////////////////////////////////////////////////CONFIG////////////////////////////////////////////////////////////////////////////////////
pub fn create_default_config(config_path: &PathBuf, app_state: &mut AppState) -> io::Result<()> {
    let mut file = File::create(config_path)?;
    writeln!(
        file,
        "home_folder = {}",
        app_state.current_dir.to_string_lossy()
    )?;
    writeln!(file, "lines_shown = {}", app_state.config.lines_shown)?;
    writeln!(file, "default_sort = {:?}", app_state.config.default_sort)?;
    writeln!(file, "# Shortcuts")?;
    writeln!(file, "# Format: shortcut_X = /path/to/directory")?;
    writeln!(file, "# Example: shortcut_1 = /home/user/Documents")?;
    Ok(())
}

pub fn read_line() -> io::Result<String> {
    let mut input = String::new();
    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Enter => {
                    println!();
                    break;
                }
                KeyCode::Char(c) => {
                    input.push(c);
                    print!("{}", c);
                }
                KeyCode::Backspace => {
                    if !input.is_empty() {
                        input.pop();
                        print!("\x08 \x08");
                    }
                }
                _ => {}
            }
            io::stdout().flush()?;
        }
    }
    Ok(input)
}

pub fn get_current_branch(path: &Path) -> String {
    Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}

pub fn get_git_status(path: &Path) -> String {
    Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("status")
        .arg("--porcelain")
        .output()
        .map(|output| {
            if output.stdout.is_empty() {
                "Clean".to_string()
            } else {
                "Modified".to_string()
            }
        })
        .unwrap_or_else(|_| "Unknown".to_string())
}

pub fn is_git_repository(path: &Path) -> bool {
    Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
pub fn truncate_path(path: &Path, max_length: usize) -> String {
    let path_str = path.to_string_lossy();
    if path_str.len() <= max_length {
        return format!("{:width$}", path_str, width = max_length);
    }
    let components: Vec<&str> = path_str.split(std::path::MAIN_SEPARATOR).collect();
    let mut result = String::new();
    let ellipsis = "./";
    for (i, component) in components.iter().rev().enumerate() {
        let new_path = if result.is_empty() {
            component.to_string()
        } else {
            format!("{}{}{}", component, std::path::MAIN_SEPARATOR, result)
        };
        if (new_path.len() + ellipsis.len()) > max_length {
            if i == 0 {
                let truncated = &new_path[new_path.len() - (max_length - ellipsis.len())..];
                return format!(
                    "{:width$}",
                    format!("{}{}", ellipsis, truncated),
                    width = max_length
                );
            } else {
                return format!(
                    "{:width$}",
                    format!("{}{}", ellipsis, result),
                    width = max_length
                );
            }
        }
        result = new_path;
    }
    format!("{:width$}", result, width = max_length)
}
