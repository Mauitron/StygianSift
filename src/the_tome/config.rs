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
#[derive(Debug, Clone)]
pub struct Config {
    pub home_folder: Option<PathBuf>,
    pub lines_shown: usize,
    pub text_editor: String,
    pub keybindings: Option<HashMap<KeyEvent, Action>>,
    pub shortcuts: Option<HashMap<char, (PathBuf, String, usize)>>,
    pub default_sort: SortOrder,
    pub ram_undo_limit: usize,
    pub disk_undo_limit: u64,
    pub allow_disk_undo: bool,
    pub search_depth_limit: usize,
    pub colored_items: HashMap<MarkerColor, HashSet<PathBuf>>,
    pub color_rules: HashMap<MarkerColor, ColorRule>,
    pub shortcut_layers: Vec<ShortcutLayer>,
    pub current_layer: usize,
}

impl Config {
    pub fn new() -> Self {
        Config {
            home_folder: None,
            lines_shown: 40,
            default_sort: SortOrder::TypeAsc,
            shortcuts: None,
            text_editor: String::from(""),
            keybindings: Some(Self::default_keybindings()),
            ram_undo_limit: DEFAULT_RAM_LIMIT,
            disk_undo_limit: DEFAULT_DISK_LIMIT,
            allow_disk_undo: false,
            search_depth_limit: 3,
            colored_items: HashMap::new(),
            color_rules: HashMap::new(),
            shortcut_layers: (0..10)
                .map(|i| ShortcutLayer::new(format!("Layer {}", i)))
                .collect(),
            current_layer: 0,
        }
    }
    //////////////////////////////////////////////////KeyBindings//////////////////////////////////////////////////////////////////////
#[rustfmt::skip]
pub fn default_keybindings() -> HashMap<KeyEvent, Action> {
    let mut keybindings = HashMap::new();

    //------------------------------------------------Navigation---------------------------------------------------------------------\\
    keybindings.insert(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE), Action::MoveUp);
    keybindings.insert(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE), Action::MoveDown);
    keybindings.insert(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE), Action::MoveLeft);
    keybindings.insert(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE), Action::MoveRight);
    keybindings.insert(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE), Action::MoveUp);
    keybindings.insert(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE), Action::MoveDown);
    keybindings.insert(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE), Action::MoveLeft);
    keybindings.insert(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE), Action::MoveRight);
    keybindings.insert(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE), Action::GoToTop);
    keybindings.insert(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE), Action::GoToBottom);
    keybindings.insert(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE), Action::Enter);

    //-------------------------------------------------Selection---------------------------------------------------------------------\\
    keybindings.insert(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::CONTROL), Action::ToggleSelect);
    keybindings.insert(KeyEvent::new(KeyCode::Char('K'), KeyModifiers::SHIFT), Action::MultiSelectUp);
    keybindings.insert(KeyEvent::new(KeyCode::Char('J'), KeyModifiers::SHIFT), Action::MultiSelectDown);
    keybindings.insert(KeyEvent::new(KeyCode::Up, KeyModifiers::SHIFT), Action::MultiSelectUp);
    keybindings.insert(KeyEvent::new(KeyCode::Down, KeyModifiers::SHIFT), Action::MultiSelectDown);
    keybindings.insert(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL), Action::SelectAll);

    //----------------------------------------------File Operations------------------------------------------------------------------\\
    keybindings.insert(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE), Action::Rename);
    keybindings.insert(KeyEvent::new(KeyCode::Char('R'), KeyModifiers::SHIFT), Action::RenameWithoutExtension);
    keybindings.insert(KeyEvent::new(KeyCode::Char('D'), KeyModifiers::SHIFT), Action::Murder);
    keybindings.insert(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE), Action::Copy);
    keybindings.insert(KeyEvent::new(KeyCode::Char('P'), KeyModifiers::SHIFT), Action::Paste);
    keybindings.insert(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE), Action::Duplicate);
    keybindings.insert(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE), Action::MoveItem);
    keybindings.insert(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::CONTROL), Action::GiveBirthDir);
    keybindings.insert(KeyEvent::new(KeyCode::Char('B'), KeyModifiers::SHIFT), Action::GiveBirthFile);

    //----------------------------------------------View and Display-----------------------------------------------------------------\\
    keybindings.insert(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE), Action::TogglePreview);
    keybindings.insert(KeyEvent::new(KeyCode::Char('L'), KeyModifiers::SHIFT), Action::SetLineAmount);
    keybindings.insert(KeyEvent::new(KeyCode::Char('C'), KeyModifiers::SHIFT), Action::CycleItemColor);
    keybindings.insert(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL), Action::RemoveItemColor);

    //---------------------------------------------Search and Sort-------------------------------------------------------------------\\
    keybindings.insert(KeyEvent::new(KeyCode::Char('S'), KeyModifiers::SHIFT), Action::Search);
    keybindings.insert(KeyEvent::new(KeyCode::Char('F'), KeyModifiers::SHIFT), Action::SearchFiles);
    keybindings.insert(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE), Action::ToggleCount);
    keybindings.insert(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE), Action::SortCycleForward);
    keybindings.insert(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE), Action::ToggleFilters);

    //---------------------------------------------System and Tools------------------------------------------------------------------\\
    keybindings.insert(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE), Action::TerminalCommand);
    keybindings.insert(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::CONTROL), Action::Undo);
    keybindings.insert(KeyEvent::new(KeyCode::Char(']'), KeyModifiers::NONE), Action::GitMenu);
    keybindings.insert(KeyEvent::new(KeyCode::Char('|'), KeyModifiers::NONE), Action::ExecuteFile);
    keybindings.insert(KeyEvent::new(KeyCode::Char('.'), KeyModifiers::NONE), Action::OpenInEditor);

    //-----------------------------------------Help and Configuration----------------------------------------------------------------\\
    keybindings.insert(KeyEvent::new(KeyCode::F(12), KeyModifiers::NONE), Action::Help);
    keybindings.insert(KeyEvent::new(KeyCode::F(11), KeyModifiers::NONE), Action::ShowShortcuts);
    keybindings.insert(KeyEvent::new(KeyCode::F(1),  KeyModifiers::SHIFT),Action::RenameLayer);
    keybindings.insert(KeyEvent::new(KeyCode::F(2), KeyModifiers::SHIFT), Action::SetColorRules);
    keybindings.insert(KeyEvent::new(KeyCode::Char('~'),KeyModifiers::NONE), Action::EditConfig);

    //-------------------------------------------------Shortcuts---------------------------------------------------------------------\\
    
    //                                          <|Choose Shortcut Layer|>
    keybindings.insert(KeyEvent::new(KeyCode::F(1), KeyModifiers::NONE),Action::SwitchLayer1);
    keybindings.insert(KeyEvent::new(KeyCode::F(2), KeyModifiers::NONE),Action::SwitchLayer2);
    keybindings.insert(KeyEvent::new(KeyCode::F(3), KeyModifiers::NONE),Action::SwitchLayer3);
    keybindings.insert(KeyEvent::new(KeyCode::F(4), KeyModifiers::NONE),Action::SwitchLayer4);
    keybindings.insert(KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE),Action::SwitchLayer5);
    keybindings.insert(KeyEvent::new(KeyCode::F(6), KeyModifiers::NONE),Action::SwitchLayer6);
    keybindings.insert(KeyEvent::new(KeyCode::F(7), KeyModifiers::NONE),Action::SwitchLayer7);
    keybindings.insert(KeyEvent::new(KeyCode::F(8), KeyModifiers::NONE),Action::SwitchLayer8);
    keybindings.insert(KeyEvent::new(KeyCode::F(9), KeyModifiers::NONE),Action::SwitchLayer9);
    keybindings.insert(KeyEvent::new(KeyCode::F(10), KeyModifiers::NONE),Action::SwitchLayer0);


    //                                             <|Set Shortcuts|>
    keybindings.insert(KeyEvent::new(KeyCode::Char('!'), KeyModifiers::NONE), Action::SetShortcut1);
    keybindings.insert(KeyEvent::new(KeyCode::Char('@'), KeyModifiers::NONE), Action::SetShortcut2);
    keybindings.insert(KeyEvent::new(KeyCode::Char('#'), KeyModifiers::NONE), Action::SetShortcut3);
    keybindings.insert(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::NONE), Action::SetShortcut4);
    keybindings.insert(KeyEvent::new(KeyCode::Char('%'), KeyModifiers::NONE), Action::SetShortcut5);
    keybindings.insert(KeyEvent::new(KeyCode::Char('^'), KeyModifiers::NONE), Action::SetShortcut6);
    keybindings.insert(KeyEvent::new(KeyCode::Char('&'), KeyModifiers::NONE), Action::SetShortcut7);
    keybindings.insert(KeyEvent::new(KeyCode::Char('*'), KeyModifiers::NONE), Action::SetShortcut8);
    keybindings.insert(KeyEvent::new(KeyCode::Char('('), KeyModifiers::NONE), Action::SetShortcut9);
    keybindings.insert(KeyEvent::new(KeyCode::Char(')'), KeyModifiers::NONE), Action::SetShortcut0);

    //                                             <|Use Shortcuts|>
    keybindings.insert(KeyEvent::new(KeyCode::Char('1'), KeyModifiers::NONE), Action::UseShortcut1);
    keybindings.insert(KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE), Action::UseShortcut2);
    keybindings.insert(KeyEvent::new(KeyCode::Char('3'), KeyModifiers::NONE), Action::UseShortcut3);
    keybindings.insert(KeyEvent::new(KeyCode::Char('4'), KeyModifiers::NONE), Action::UseShortcut4);
    keybindings.insert(KeyEvent::new(KeyCode::Char('5'), KeyModifiers::NONE), Action::UseShortcut5);
    keybindings.insert(KeyEvent::new(KeyCode::Char('6'), KeyModifiers::NONE), Action::UseShortcut6);
    keybindings.insert(KeyEvent::new(KeyCode::Char('7'), KeyModifiers::NONE), Action::UseShortcut7);
    keybindings.insert(KeyEvent::new(KeyCode::Char('8'), KeyModifiers::NONE), Action::UseShortcut8);
    keybindings.insert(KeyEvent::new(KeyCode::Char('9'), KeyModifiers::NONE), Action::UseShortcut9);
    keybindings.insert(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE), Action::UseShortcut0);

    //---------------------------------------------------MISC------------------------------------------------------------------------\\
    keybindings.insert(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE), Action::Quit);
    keybindings.insert(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::ALT), Action::CastCommandLineSpell);

    keybindings
}
    fn compare_key_events(a: &KeyEvent, b: &KeyEvent) -> bool {
        a.code == b.code && a.modifiers == b.modifiers
    }

    pub fn set_keybinding(&mut self, key: KeyEvent, action: Action) {
        self.keybindings
            .get_or_insert_with(HashMap::new)
            .insert(key, action);
    }

    pub fn remove_keybinding(&mut self, key: &KeyEvent) {
        if let Some(kb) = self.keybindings.as_mut() {
            kb.retain(|k, _| !Self::compare_key_events(k, key));
        }
    }
    pub fn reset_keybindings(&mut self) {
        self.keybindings = Some(Self::default_keybindings());
    }

    pub fn get_keybindings(&self) -> Option<&HashMap<KeyEvent, Action>> {
        self.keybindings.as_ref()
    }

    /////////////////////////////////////////////////!KeyBindings!/////////////////////////////////////////////////////////////////////
     pub fn get_current_layer_info(&self) -> (usize, &str) {
        (
            self.current_layer,
            &self.shortcut_layers[self.current_layer].name
        )
    }

    pub fn switch_layer(&mut self, index: usize) -> io::Result<()> {
        // Ensure the layer exists
        while self.shortcut_layers.len() <= index {
            let new_layer_name = format!("Layer {}", self.shortcut_layers.len());
            self.shortcut_layers.push(ShortcutLayer::new(new_layer_name));
        }

        self.current_layer = index;
        self.save_config()
    }
    
       pub fn set_shortcut_in_layer(
        &mut self,
        layer_index: usize,
        key: char,
        path: PathBuf,
        name: String,
        index: usize,
    ) -> io::Result<()> {
        if layer_index < self.shortcut_layers.len() {
            let layer = &mut self.shortcut_layers[layer_index];
            layer.shortcuts.get_or_insert_with(HashMap::new)
                .insert(key, (path, name, index));
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid layer index",
            ))
        }
    }

    pub fn get_shortcut_from_layer(
        &self,
        layer_index: usize,
        key: char,
    ) -> Option<&(PathBuf, String, usize)> {
        self.shortcut_layers.get(layer_index)
            .and_then(|layer| layer.shortcuts.as_ref())
            .and_then(|shortcuts| shortcuts.get(&key))
    }


    pub fn add_new_layer(&mut self, name: String) -> usize {
        self.shortcut_layers.push(ShortcutLayer::new(name));
        self.shortcut_layers.len() - 1
    }

    pub fn rename_layer(&mut self, layer_index: usize, new_name: String) -> io::Result<()> {
        if layer_index < self.shortcut_layers.len() {
            self.shortcut_layers[layer_index].name = new_name;
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid: Layer Doesn't Exist",
            ))
        }
    }
    pub fn get_selected_path(app_state: &AppState, entries: &[FileEntry]) -> Option<PathBuf> {
        entries
            .get(app_state.current_file_selected as usize)
            .map(|entry| entry.path.clone())
    }

    pub fn get_action(&self, key: &KeyEvent) -> Option<&Action> {
        self.keybindings.as_ref().and_then(|kb| {
            kb.iter()
                .find(|(k, _)| Self::compare_key_events(k, key))
                .map(|(_, v)| v)
        })
    }
    pub fn set_home_folder(&mut self, path: Option<PathBuf>) {
        self.home_folder = path;
    }

    pub fn set_item_color(&mut self, path: PathBuf, color: MarkerColor) {
        for items in self.colored_items.values_mut() {
            items.remove(&path);
        }
        self.colored_items
            .entry(color)
            .or_insert_with(HashSet::new)
            .insert(path);
    }

    pub fn remove_item_color(&mut self, path: &Path) {
        for items in self.colored_items.values_mut() {
            items.remove(path);
        }
    }

    pub fn get_item_color(&self, path: &Path) -> Option<MarkerColor> {
        for (color, items) in &self.colored_items {
            if items.contains(path) {
                return Some(*color);
            }
        }
        None
    }
    pub fn save_config(&self) -> io::Result<()> {
        let config_path = Self::get_config_path()?;
        let mut file = File::create(config_path)?;

        writeln!(
            file,
            "home_folder = {:?}",
            self.home_folder
                .clone()
                .expect("home directory does not exist")
        )?;
        writeln!(file, "lines_shown = {}", self.lines_shown)?;
        writeln!(file, "default_sort = {}", self.default_sort.to_string())?;
        writeln!(file, "text_editor = {}", self.text_editor)?;
        writeln!(file, "ram_undo_limit = {}", self.ram_undo_limit)?;
        writeln!(file, "disk_undo_limit = {}", self.disk_undo_limit)?;
        writeln!(file, "allow_disk_undo = {}", self.allow_disk_undo)?;
        writeln!(file, "search_depth_limit = {}", self.search_depth_limit)?;

        if let Some(keybindings) = &self.keybindings {
            writeln!(file, "keybindings:")?;
            for (key, action) in keybindings {
                writeln!(file, "  {} = {}", key_event_to_string(key), action)?;
            }
        }

        writeln!(file, "colored_items:")?;
        for (color, paths) in &self.colored_items {
            writeln!(file, "  {}:", color.as_str())?;
            for path in paths {
                writeln!(file, "    {}", path.display())?;
            }
            if let Some(rule) = self.color_rules.get(color) {
                writeln!(
                    file,
                    "  {}_rule = {},{},{},{},{}",
                    color.as_str(),
                    rule.allow_delete,
                    rule.allow_rename,
                    rule.allow_move,
                    rule.allow_copy,
                    rule.include_in_search
                )?;
            }
        }
        if let Some(shortcuts) = &self.shortcuts {
            writeln!(file, "shortcuts:")?;
            for (key, (path, name, index)) in shortcuts {
                writeln!(file, "  {} = {}|{}|{}", key, path.display(), name, index)?;
            }
        }
            writeln!(file, "shortcut_layers:")?;
            for (i, layer) in self.shortcut_layers.iter().enumerate() {
                writeln!(file, "  layer_{}_name = {}", i, layer.name)?;

            if let Some(shortcuts) = &layer.shortcuts {
                for (key, (path, name, index)) in shortcuts {
                    writeln!(
                        file,
                        "  layer_{}_shortcut_{} = {}|{}|{}",
                        i,
                        key,
                        path.display(),
                        name,
                        index
                    )?;
                }
            }
        }
        writeln!(file, "current_layer = {}", self.current_layer)?;

        Ok(())
    }
        
    pub fn load_config() -> io::Result<Self> {
        let mut config = Config::new();
        let config_path = Self::get_config_path()?;
        let content = fs::read_to_string(config_path)?;

        let mut current_section: Option<&str> = None;
        let mut current_color: Option<MarkerColor> = None;

        for line in content.lines() {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                continue;
            }
            if trimmed_line == "keybindings:" {
                current_section = Some("keybindings");
                continue;
            } else if trimmed_line == "colored_items:" {
                current_section = Some("colored_items");
                continue;
            }
            if trimmed_line == "shortcut_layers:" {
                current_section = Some("shortcut_layers");
                continue;
            }
            if trimmed_line == "shortcuts:" {
                current_section = Some("shortcuts");
                continue;
            } else if trimmed_line == "keybindings:" || trimmed_line == "colored_items:" {
                current_section = Some(trimmed_line.trim_end_matches(':'));
                continue;
            }

            match current_section {
                Some("shortcut_layers") => {
                    if trimmed_line.starts_with("layer_") {
                        if let Some((key, value)) = trimmed_line.split_once('=') {
                            let key = key.trim();
                            let value = value.trim();

                            if key.ends_with("_name") {
                                let layer_index = key
                                    .strip_prefix("layer_")
                                    .and_then(|s| s.strip_suffix("_name"))
                                    .and_then(|s| s.parse::<usize>().ok());

                                if let Some(index) = layer_index {
                                    while config.shortcut_layers.len() <= index {
                                        config.add_new_layer("New Layer".to_string());
                                    }
                                    config.shortcut_layers[index].name = value.to_string();
                                }
                            } else if key.contains("_shortcut_") {
                                let parts: Vec<&str> = key.split('_').collect();
                                if parts.len() >= 4 {
                                    if let (Ok(layer_index), Some(shortcut_key)) =
                                        (parts[1].parse::<usize>(), parts[3].chars().next())
                                    {
                                        let parts: Vec<&str> = value.splitn(3, '|').collect();
                                        if parts.len() == 3 {
                                            let path = PathBuf::from(parts[0].trim());
                                            let name = parts[1].trim().to_string();
                                            let index = parts[2].trim().parse().unwrap_or(0);
                                            let _ = config.set_shortcut_in_layer(
                                                layer_index,
                                                shortcut_key,
                                                path,
                                                name,
                                                index,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    } else if trimmed_line.starts_with("current_layer = ") {
                        if let Some(value) = trimmed_line.strip_prefix("current_layer = ") {
                            config.current_layer = value.parse().unwrap_or(0);
                        }
                    }
                }
                Some("shortcuts") => {
                    if let Some((key, value)) = trimmed_line.split_once('=') {
                        let key = key.trim().chars().next().unwrap();
                        let parts: Vec<&str> = value.trim().splitn(3, '|').collect();
                        if parts.len() == 3 {
                            let path = PathBuf::from(parts[0].trim());
                            let name = parts[1].trim().to_string();
                            let index = parts[2].trim().parse().unwrap_or(0);
                            config
                                .shortcuts
                                .get_or_insert_with(HashMap::new)
                                .insert(key, (path, name, index));
                        }
                    }
                }
                Some("keybindings") => {
                    if let Some((key, action)) = trimmed_line.split_once('=') {
                        let key_event = parse_key_event(key.trim());
                        if let Ok(action) = Action::from_str(action.trim()) {
                            config
                                .keybindings
                                .get_or_insert_with(HashMap::new)
                                .insert(key_event, action);
                        }
                    }
                }
                Some("colored_items") => {
                    if trimmed_line.ends_with(':') {
                        current_color =
                            MarkerColor::from_str(&trimmed_line[..trimmed_line.len() - 1]);
                    } else if trimmed_line.contains("_rule =") {
                        if let Some(color) = current_color {
                            let parts: Vec<&str> = trimmed_line.split('=').collect();
                            if parts.len() == 2 {
                                let values: Vec<&str> = parts[1].split(',').collect();
                                if values.len() == 5 {
                                    let rule = ColorRule {
                                        allow_delete: values[0].trim().parse().unwrap_or(true),
                                        allow_rename: values[1].trim().parse().unwrap_or(true),
                                        allow_move: values[2].trim().parse().unwrap_or(true),
                                        allow_copy: values[3].trim().parse().unwrap_or(true),
                                        include_in_search: values[4].trim().parse().unwrap_or(true),
                                    };
                                    config.color_rules.insert(color, rule);
                                }
                            }
                        }
                    } else if let Some(color) = current_color {
                        config
                            .colored_items
                            .entry(color)
                            .or_insert_with(HashSet::new)
                            .insert(PathBuf::from(trimmed_line));
                    }
                }
                _ => {
                    if let Some((key, value)) = trimmed_line.split_once('=') {
                        let key = key.trim();
                        let value = value.trim();
                        match key {
                            "home_folder" => {
                                config.home_folder = if value == "None" {
                                    None
                                } else {
                                    Some(PathBuf::from(value.trim_matches('"')))
                                }
                            }
                            "lines_shown" => config.lines_shown = value.parse().unwrap_or(40),
                            "default_sort" => config.default_sort = SortOrder::from_str(value),
                            "text_editor" => config.text_editor = value.to_string(),
                            "ram_undo_limit" => {
                                config.ram_undo_limit = value.parse().unwrap_or(DEFAULT_RAM_LIMIT)
                            }
                            "disk_undo_limit" => {
                                config.disk_undo_limit = value.parse().unwrap_or(DEFAULT_DISK_LIMIT)
                            }
                            "allow_disk_undo" => {
                                config.allow_disk_undo = value.parse().unwrap_or(false)
                            }
                            "search_depth_limit" => {
                                config.search_depth_limit = value.parse().unwrap_or(3)
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok(config)
    }

    pub fn get_config_path() -> std::io::Result<PathBuf> {
        let mut path = std::env::current_dir()?;
        path.push(".maui");
        Ok(path)
    }
}
pub fn manage_keybindings(app_state: &mut AppState, stdout: &mut impl Write) -> io::Result<()> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;
    let _ = clear_nav();
    let _ = clear_preview();
    loop {
        // execute!(stdout, Clear(ClearType::All))?;
        execute!(stdout, MoveTo(nav_width / 3, 7))?;
        writeln!(stdout, "Manage Keybindings\r")?;
        execute!(stdout, MoveTo(nav_width / 3, 8))?;
        writeln!(stdout, "==================\r")?;

        if let Some(keybindings) = &app_state.config.keybindings {
            for (i, (key, action)) in keybindings.iter().enumerate() {
                if i >= (height - 11) as usize {
                    execute!(
                        stdout,
                        MoveTo(nav_width / 2 + 7, i as u16 - (height - 11) + 8)
                    )?;
                    writeln!(
                        stdout,
                        "{} : {}\r",
                        key_event_to_string(key).trim_matches('"').red(),
                        action.to_string().green()
                    )?;
                } else {
                    execute!(stdout, MoveTo(nav_width / 12, i as u16 + 8))?;
                    writeln!(
                        stdout,
                        "{} : {}\r",
                        key_event_to_string(key).trim_matches('"').red(),
                        action.to_string().green()
                    )?;
                }
            }
        } else {
            writeln!(stdout, "No custom keybindings set. Using defaults.\r")?;
        }

        execute!(
            stdout,
            MoveTo(preview_width + preview_width / 3, height / 8)
        )?;
        writeln!(stdout, "\nEnter your choice (1-3): \r")?;
        execute!(
            stdout,
            MoveTo(preview_width + preview_width / 3, (height / 8) + 3)
        )?;
        writeln!(stdout, "{}. Add/Edit Keybinding\r", "1".green())?;
        execute!(
            stdout,
            MoveTo(preview_width + preview_width / 3, (height / 8) + 4)
        )?;
        writeln!(stdout, "{}. Remove Keybinding\r", "2".green())?;
        execute!(
            stdout,
            MoveTo(preview_width + preview_width / 3, (height / 8) + 5)
        )?;
        writeln!(stdout, "{}. Reset to Default Keybindings\r", "3".green())?;
        execute!(
            stdout,
            MoveTo(preview_width + preview_width / 3, (height / 8) + 6)
        )?;
        writeln!(stdout, "{}. Return to Main Menu\r", "Esc".green())?;
        stdout.flush()?;
        if let Event::Key(key) = event::read()? {
            let _ = clear_nav();
            let _ = clear_preview();
            match key.code {
                KeyCode::Char('1') => {
                    let (key_event, action) = read_new_keybinding(stdout, &app_state)?;
                    app_state.config.set_keybinding(key_event, action);
                    app_state.config.save_config()?;
                }
                KeyCode::Char('2') => {
                    let key_event = read_keybinding_to_remove(stdout, &app_state.config)?;
                    app_state.config.remove_keybinding(&key_event.unwrap());
                    app_state.config.save_config()?;
                }
                KeyCode::Char('3') => {
                    app_state.config.reset_keybindings();
                    app_state.config.save_config()?;
                    writeln!(stdout, "Keybindings reset to default.\r")?;
                }
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }
    Ok(())
}
fn read_new_keybinding(
    stdout: &mut impl Write,
    app_state: &AppState,
) -> io::Result<(KeyEvent, Action)> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;
    let _ = clear_nav();
    let _ = clear_preview();
    execute!(
        stdout,
        MoveTo(preview_width + preview_width / 3, (height / 5) + 6)
    )?;
    write!(stdout, "Press the key you want to bind: \r")?;
    stdout.flush()?;
    let key_event = event::read()?;
    if let Event::Key(key) = key_event {
        execute!(
            stdout,
            MoveTo(preview_width + preview_width / 3, (height / 5) + 4)
        )?;
        writeln!(stdout, "You pressed: {}\r", key_event_to_string(&key))?;
        execute!(
            stdout,
            MoveTo(preview_width + preview_width / 3, (height / 5) + 6)
        )?;
        writeln!(stdout, "Select an action to bind to this key:\r")?;

        if let Some(keybindings) = &app_state.config.keybindings {
            for (i, (_key, action)) in keybindings.iter().enumerate() {
                if i >= (height - 11) as usize {
                    execute!(
                        stdout,
                        MoveTo(nav_width / 2 + 7, i as u16 - (height - 11) + 8)
                    )?;
                    writeln!(stdout, "{}. {}\r", i, action.to_string().green())?;
                } else {
                    execute!(stdout, MoveTo(nav_width / 12, i as u16 + 8))?;
                    writeln!(stdout, "{}. {}\r", i, action.to_string().green())?;
                }
            }
        }
        execute!(
            stdout,
            MoveTo(preview_width + preview_width / 3, (height / 5) + 6)
        )?;
        write!(stdout, "Enter the number of the action: \r")?;
        stdout.flush()?;
        execute!(
            stdout,
            MoveTo(preview_width + preview_width / 3, (height / 5) + 8)
        )?;
        let action_index: usize = read_line()?
            .trim()
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid action number"))?;
        if let Some(action) = Action::iter().nth(action_index - 1) {
            let _ = clear_nav();
            let _ = clear_preview();
            Ok((key, action.clone()))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid action number",
            ))
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid key event",
        ))
    }
}

fn read_keybinding_to_remove(
    stdout: &mut impl Write,
    config: &Config,
) -> io::Result<Option<KeyEvent>> {
    if let Some(keybindings) = config.get_keybindings() {
        writeln!(stdout, "Current keybindings:")?;
        for (key, action) in keybindings {
            writeln!(stdout, "{} : {:?}", key_event_to_string(key), action)?;
        }
    } else {
        writeln!(stdout, "No custom keybindings set. Using defaults.")?;
        return Ok(None);
    }

    writeln!(
        stdout,
        "Press the key of the binding you want to remove (or Esc to cancel): "
    )?;
    stdout.flush()?;

    loop {
        if let Ok(Event::Key(key)) = event::read() {
            match key.code {
                KeyCode::Esc => {
                    writeln!(stdout, "Cancelled keybinding removal.")?;
                    return Ok(None);
                }
                _ => {
                    if config
                        .get_keybindings()
                        .map_or(false, |kb| kb.contains_key(&key))
                    {
                        writeln!(
                            stdout,
                            "Removing keybinding for: {}",
                            key_event_to_string(&key)
                        )?;
                        return Ok(Some(key));
                    } else {
                        writeln!(
                            stdout,
                            "No keybinding found for: {}. Try again or press Esc to cancel.",
                            key_event_to_string(&key)
                        )?;
                    }
                }
            }
        }
    }
}
////////////////////////////////////////////////////////////////Shortcuts///////////////////////////////////////////////////////////////////////////////



pub fn manage_shortcuts(app_state: &mut AppState, stdout: &mut impl Write) -> io::Result<()> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;
    let _ = clear_nav();
    loop {
        // execute!(stdout, Clear(ClearType::All))?;
        queue!(stdout, MoveTo(preview_width + 30, height - 38))?;
        writeln!(stdout, "Manage Shortcuts\r")?;
        queue!(stdout, MoveTo(preview_width + 30, height - 37))?;
        writeln!(stdout, "================\r")?;
        queue!(stdout, MoveTo(nav_width / 3 + 5, height - 38))?;
        writeln!(stdout, "Shortcuts\r")?;
        queue!(stdout, MoveTo(nav_width / 3 + 5 - 3, height - 37))?;
        writeln!(stdout, "================\r")?;

        if let Some(shortcuts) = &app_state.config.shortcuts {
            let sorted_shortcuts: BTreeMap<_, _> = shortcuts.iter().collect();

            for (i, (key, (path, name, _index))) in sorted_shortcuts.iter().enumerate() {
                if name.is_empty() {
                    queue!(stdout, MoveTo(nav_width / 8, (height - 35) + i as u16))?;
                    writeln!(stdout, "{}: {}\r", key.red(), path.display())?;
                } else {
                    queue!(stdout, MoveTo(nav_width / 8, (height - 33) + i as u16))?;
                    writeln!(
                        stdout,
                        "{}: {} ({})\r",
                        key.red(),
                        name,
                        path.to_string_lossy().green(),
                    )?;
                }
            }
        } else {
            writeln!(stdout, "No shortcuts set\r")?;
        }

        queue!(stdout, MoveTo(preview_width + 26, height - 34))?;
        writeln!(stdout, "1. Add/Edit Shortcut\r")?;
        queue!(stdout, MoveTo(preview_width + 26, height - 33))?;
        writeln!(stdout, "2. Remove Shortcut\r")?;
        queue!(stdout, MoveTo(preview_width + 26, height - 32))?;
        writeln!(stdout, "{}. Return to Main Menu\r", "ESC".red())?;
        queue!(stdout, MoveTo(preview_width + 26, height - 31))?;
        writeln!(stdout, "Enter your choice (1-3): \r")?;
        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('1') => {
                    let (key, path, name, index) = read_new_shortcut(stdout, &app_state)?;
                    if let Some(shortcuts) = app_state.config.shortcuts.as_mut() {
                        shortcuts.insert(key, (path, name, index));
                    } else {
                        let mut shortcuts = HashMap::new();
                        shortcuts.insert(key, (path, name, index));
                        app_state.config.shortcuts = Some(shortcuts);
                    }
                    app_state.config.save_config()?;
                }
                KeyCode::Char('2') => {
                    let key = read_shortcut_to_remove(stdout)?;
                    if let Some(shortcuts) = app_state.config.shortcuts.as_mut() {
                        shortcuts.remove(&key);
                        if shortcuts.is_empty() {
                            app_state.config.shortcuts = None;
                        }
                    }
                    app_state.config.save_config()?;
                }
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }
    Ok(())
}

pub fn read_new_shortcut(
    stdout: &mut impl Write,
    app_state: &AppState,
) -> io::Result<(char, PathBuf, String, usize)> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;
    // execute!(stdout, Clear(ClearType::All))?;

    queue!(stdout, MoveTo(preview_width + 3, height - 10))?;
    write!(stdout, "Enter shortcut key (0 - 9): \r")?;
    stdout.flush()?;
    let key = loop {
        if let Event::Key(key_event) = event::read()? {
            if let KeyCode::Char(c) = key_event.code {
                writeln!(stdout, "{}", c)?;
                break c;
            }
        }
    };

    queue!(stdout, MoveTo(preview_width + 3, height - 10))?;
    write!(stdout, "Enter path for shortcut: \r")?;
    stdout.flush()?;
    queue!(stdout, MoveTo(preview_width + 3, height - 9))?;
    let path_str = read_line()?;
    let path = PathBuf::from(path_str);

    queue!(stdout, MoveTo(preview_width + 3, height - 10))?;
    write!(stdout, "Enter name for shortcut (optional): \r")?;
    stdout.flush()?;
    queue!(stdout, MoveTo(preview_width + 30 + 34, height - 38))?;
    let name = read_line()?;
    let index = app_state.nav_stack[app_state.nav_stack.len() - 1].index;
    if path.exists() {
        let _ = clear_nav();
        queue!(stdout, MoveTo(preview_width + 3, height - 10))?;
        Ok((key, path, name, index))
    } else {
        let _ = clear_nav();
        queue!(stdout, MoveTo(preview_width + 3, height - 10))?;
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid path\r",
        ))
    }
}

pub fn read_shortcut_to_remove(stdout: &mut impl Write) -> io::Result<char> {
    let (width, height) = size()?;
    let nav_width = width / 2;
    let preview_width = width - nav_width - 2;
    queue!(stdout, MoveTo(preview_width + 3, height - 10))?;
    write!(stdout, "Enter shortcut key to remove: \r")?;
    stdout.flush()?;
    loop {
        if let Event::Key(key_event) = event::read()? {
            if let KeyCode::Char(c) = key_event.code {
                queue!(stdout, MoveTo(preview_width + 33, height - 10))?;
                writeln!(stdout, "{}", c)?;
                return Ok(c);
            }
            if let KeyCode::Esc = key_event.code {
                break Ok('e');
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShortcutLayer {
    pub name: String,
    pub shortcuts: Option<HashMap<char, (PathBuf, String, usize)>>,
}

impl ShortcutLayer {
    pub fn new(name: String) -> Self {
        ShortcutLayer {
            name,
            shortcuts: None,
        }
    }
}

///////////////////////////////////////////////////////////////////Colors///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorRule {
    pub allow_delete: bool,
    pub allow_rename: bool,
    pub allow_move: bool,
    pub allow_copy: bool,
    pub include_in_search: bool,
}

impl Default for ColorRule {
    fn default() -> Self {
        ColorRule {
            allow_delete: true,
            allow_rename: true,
            allow_move: true,
            allow_copy: true,
            include_in_search: true,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarkerColor {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Cyan,
    Pink,
    White,
    Magenta,
    Reset,
}

impl MarkerColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            MarkerColor::Red => "red",
            MarkerColor::Orange => "orange",
            MarkerColor::Yellow => "yellow",
            MarkerColor::Green => "green",
            MarkerColor::Blue => "blue",
            MarkerColor::Cyan => "cyan",
            MarkerColor::Pink => "pink",
            MarkerColor::White => "white",
            MarkerColor::Magenta => "magenta",
            MarkerColor::Reset => "reset",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "red" => Some(MarkerColor::Red),
            "orange" => Some(MarkerColor::Orange),
            "yellow" => Some(MarkerColor::Yellow),
            "green" => Some(MarkerColor::Green),
            "blue" => Some(MarkerColor::Blue),
            "cyan" => Some(MarkerColor::Cyan),
            "pink" => Some(MarkerColor::Pink),
            "white" => Some(MarkerColor::White),
            "magenta" => Some(MarkerColor::Magenta),
            "reset" => Some(MarkerColor::Reset),
            _ => None,
        }
    }

    pub fn to_highlight_color(&self) -> Color {
        match self {
            MarkerColor::Red => Color::Rgb { r: 255, g: 0, b: 0 },
            MarkerColor::Orange => Color::Rgb {
                r: 220,
                g: 125,
                b: 0,
            },
            MarkerColor::Yellow => Color::Rgb {
                r: 255,
                g: 255,
                b: 0,
            },
            MarkerColor::Green => Color::Rgb { r: 0, g: 250, b: 0 },
            MarkerColor::Blue => Color::Rgb {
                r: 0,
                g: 100,
                b: 255,
            },
            MarkerColor::Cyan => Color::Rgb {
                r: 0,
                g: 255,
                b: 255,
            },
            MarkerColor::Pink => Color::Rgb {
                r: 214,
                g: 134,
                b: 148,
            },
            MarkerColor::White => Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
            MarkerColor::Magenta => Color::Rgb {
                r: 255,
                g: 0,
                b: 255,
            },
            MarkerColor::Reset => Color::Rgb {
                r: 180,
                g: 180,
                b: 180,
            },
        }
    }
    pub fn color_order(color: Option<MarkerColor>) -> u8 {
        match color {
            None => 0,
            Some(color) => match color {
                MarkerColor::Red => 1,
                MarkerColor::Orange => 2,
                MarkerColor::Yellow => 3,
                MarkerColor::Green => 4,
                MarkerColor::Blue => 5,
                MarkerColor::Cyan => 6,
                MarkerColor::Pink => 7,
                MarkerColor::White => 8,
                MarkerColor::Magenta => 9,
                MarkerColor::Reset => 10,
            },
        }
    }
    pub fn next(&self) -> Self {
        match self {
            MarkerColor::Red => MarkerColor::Orange,
            MarkerColor::Orange => MarkerColor::Yellow,
            MarkerColor::Yellow => MarkerColor::Green,
            MarkerColor::Green => MarkerColor::Blue,
            MarkerColor::Blue => MarkerColor::Cyan,
            MarkerColor::Cyan => MarkerColor::Pink,
            MarkerColor::Pink => MarkerColor::White,
            MarkerColor::White => MarkerColor::Magenta,
            MarkerColor::Magenta => MarkerColor::Reset,
            MarkerColor::Reset => MarkerColor::Red,
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            MarkerColor::Red => Color::Rgb { r: 255, g: 0, b: 0 },
            MarkerColor::Green => Color::Rgb { r: 0, g: 250, b: 0 },
            MarkerColor::Blue => Color::Rgb {
                r: 0,
                g: 100,
                b: 255,
            },
            MarkerColor::Yellow => Color::Rgb {
                r: 255,
                g: 255,
                b: 0,
            },
            MarkerColor::Magenta => Color::Rgb {
                r: 255,
                g: 0,
                b: 255,
            },
            MarkerColor::Cyan => Color::Rgb {
                r: 0,
                g: 255,
                b: 255,
            },
            MarkerColor::White => Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
            MarkerColor::Orange => Color::Rgb {
                r: 220,
                g: 125,
                b: 0,
            },
            MarkerColor::Pink => Color::Rgb {
                r: 214,
                g: 134,
                b: 148,
            },
            MarkerColor::Reset => Color::Rgb {
                r: 180,
                g: 180,
                b: 180,
            },
        }
    }
}
