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
///////////////////////////////////////////////////////Action///////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    IncreaseDimDistance,
    DecreaseDimDistance,
    IncreaseDimIntensity,
    DecreaseDimIntensity,
    BorderStyle,
    CastCommandLineSpell,
    ToggleFilters,
    GoToTop,
    GoToBottom,
    ExecuteFile,
    GiveBirthDir,
    GiveBirthFile,
    GitMenu,
    SetColorRules,
    CycleItemColor,
    RemoveItemColor,
    SelectAll,
    SearchFiles,
    ShowShortcuts,
    TerminalCommand,
    Search,
    Undo,
    ToggleSelect,
    MultiSelectUp,
    MultiSelectDown,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Enter,
    TogglePreview,
    ToggleCount,
    Rename,
    RenameWithoutExtension,
    Murder,
    Copy,
    Paste,
    Duplicate,
    MoveItem,
    Quit,
    Help,
    RenameLayer,
    // Switch shortcut layers
    SwitchLayer0,
    SwitchLayer1,
    SwitchLayer2,
    SwitchLayer3,
    SwitchLayer4,
    SwitchLayer5,
    SwitchLayer6,
    SwitchLayer7,
    SwitchLayer8,
    SwitchLayer9,
    // Set shortcuts
    SetShortcut1,
    SetShortcut2,
    SetShortcut3,
    SetShortcut4,
    SetShortcut5,
    SetShortcut6,
    SetShortcut7,
    SetShortcut8,
    SetShortcut9,
    SetShortcut0,
    // Use shortcuts
    UseShortcut1,
    UseShortcut2,
    UseShortcut3,
    UseShortcut4,
    UseShortcut5,
    UseShortcut6,
    UseShortcut7,
    UseShortcut8,
    UseShortcut9,
    UseShortcut0,
    SetLineAmount,
    OpenInEditor,
    EditConfig,
    SortCycleForward,
}

impl Action {
    pub fn iter() -> Iter<'static, Action> {
        static ACTIONS: [Action; 76] = [
            Action::IncreaseDimDistance,
            Action::DecreaseDimDistance,
            Action::IncreaseDimIntensity,
            Action::DecreaseDimIntensity,
            Action::BorderStyle,
            Action::CastCommandLineSpell,
            Action::ToggleFilters,
            Action::GoToTop,
            Action::GoToBottom,
            Action::ExecuteFile,
            Action::GiveBirthDir,
            Action::GiveBirthFile,
            Action::GitMenu,
            Action::SetColorRules,
            Action::CycleItemColor,
            Action::RemoveItemColor,
            Action::SelectAll,
            Action::SearchFiles,
            Action::ShowShortcuts,
            Action::TerminalCommand,
            Action::Search,
            Action::Undo,
            Action::ToggleSelect,
            Action::MultiSelectUp,
            Action::MultiSelectDown,
            Action::MoveUp,
            Action::MoveDown,
            Action::MoveLeft,
            Action::MoveRight,
            Action::Enter,
            Action::TogglePreview,
            Action::ToggleCount,
            Action::Rename,
            Action::RenameWithoutExtension,
            Action::Murder,
            Action::Copy,
            Action::Paste,
            Action::Duplicate,
            Action::MoveItem,
            Action::Quit,
            Action::Help,
            Action::RenameLayer,
            // Switch
            Action::SwitchLayer0,
            Action::SwitchLayer1,
            Action::SwitchLayer2,
            Action::SwitchLayer3,
            Action::SwitchLayer4,
            Action::SwitchLayer5,
            Action::SwitchLayer6,
            Action::SwitchLayer7,
            Action::SwitchLayer8,
            Action::SwitchLayer9,
            // Set
            Action::SetShortcut1,
            Action::SetShortcut2,
            Action::SetShortcut3,
            Action::SetShortcut4,
            Action::SetShortcut5,
            Action::SetShortcut6,
            Action::SetShortcut7,
            Action::SetShortcut8,
            Action::SetShortcut9,
            Action::SetShortcut0,
            // Use
            Action::UseShortcut1,
            Action::UseShortcut2,
            Action::UseShortcut3,
            Action::UseShortcut4,
            Action::UseShortcut5,
            Action::UseShortcut6,
            Action::UseShortcut7,
            Action::UseShortcut8,
            Action::UseShortcut9,
            Action::UseShortcut0,
            Action::SetLineAmount,
            Action::OpenInEditor,
            Action::EditConfig,
            Action::SortCycleForward,
        ];
        ACTIONS.iter()
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IncreaseDimDistance" => Ok(Action::IncreaseDimDistance),
            "DecreaseDimDistance" => Ok(Action::DecreaseDimDistance),
            "IncreaseDimIntensity" => Ok(Action::IncreaseDimIntensity),
            "DecreaseDimIntensity" => Ok(Action::DecreaseDimIntensity),
            "BorderStyle" => Ok(Action::BorderStyle),
            "CastCommandLineSpell" => Ok(Action::CastCommandLineSpell),
            "ToggleFilters" => Ok(Action::ToggleFilters),
            "GoToTop" => Ok(Action::GoToTop),
            "GoToBottam" => Ok(Action::GoToBottom),
            "ExecuteFile" => Ok(Action::ExecuteFile),
            "GiveBirthDir" => Ok(Action::GiveBirthDir),
            "GiveBirthFile" => Ok(Action::GiveBirthFile),
            "GitMenu" => Ok(Action::GitMenu),
            "SetColorRules" => Ok(Action::SetColorRules),
            "CycleItemColor" => Ok(Action::CycleItemColor),
            "RemoveItemColor" => Ok(Action::RemoveItemColor),
            "SelectAll" => Ok(Self::SelectAll),
            "SearchFiles" => Ok(Action::SearchFiles),
            "ShowShortcuts" => Ok(Self::ShowShortcuts),
            "TerminalCommand" => Ok(Action::TerminalCommand),
            "Search" => Ok(Action::Search),
            "Undo" => Ok(Action::Undo),
            "ToggleSelect" => Ok(Action::ToggleSelect),
            "MultiSelectUp" => Ok(Action::MultiSelectUp),
            "MultiSelectDown" => Ok(Action::MultiSelectDown),
            "MoveUp" => Ok(Action::MoveUp),
            "MoveDown" => Ok(Action::MoveDown),
            "MoveLeft" => Ok(Action::MoveLeft),
            "MoveRight" => Ok(Action::MoveRight),
            "Enter" => Ok(Action::Enter),
            "TogglePreview" => Ok(Action::TogglePreview),
            "ToggleCount" => Ok(Action::ToggleCount),
            "Rename" => Ok(Action::Rename),
            "RenameWithoutExtension" => Ok(Action::RenameWithoutExtension),
            "Delete" => Ok(Action::Murder),
            "Copy" => Ok(Action::Copy),
            "Paste" => Ok(Action::Paste),
            "Duplicate" => Ok(Action::Duplicate),
            "MoveItem" => Ok(Action::MoveItem),
            "Quit" => Ok(Action::Quit),
            "Help" => Ok(Action::Help),
            "SetLineAmount" => Ok(Action::SetLineAmount),
            "OpenInEditor" => Ok(Action::OpenInEditor),
            "EditConfig" => Ok(Action::EditConfig),
            "SortCycleForward" => Ok(Action::SortCycleForward),
            "RenameLayer" => Ok(Action::RenameLayer),
            s if s.starts_with("SwitchLayer") => {
                let num = s
                    .chars()
                    .last()
                    .and_then(|c| c.to_digit(10))
                    .ok_or_else(|| format!("Invalid layer: {}", s))?;
                match num {
                    0 => Ok(Action::SwitchLayer0),
                    1 => Ok(Action::SwitchLayer1),
                    2 => Ok(Action::SwitchLayer2),
                    3 => Ok(Action::SwitchLayer3),
                    4 => Ok(Action::SwitchLayer4),
                    5 => Ok(Action::SwitchLayer5),
                    6 => Ok(Action::SwitchLayer6),
                    7 => Ok(Action::SwitchLayer7),
                    8 => Ok(Action::SwitchLayer8),
                    9 => Ok(Action::SwitchLayer9),
                    _ => Err(format!("Invalid layer number: {}", num)),
                }
            }
            s if s.starts_with("SetShortcut") => {
                let num = s
                    .chars()
                    .last()
                    .and_then(|c| c.to_digit(10))
                    .ok_or_else(|| format!("Invalid SetShortcut action: {}", s))?;
                match num {
                    0 => Ok(Action::SetShortcut0),
                    1 => Ok(Action::SetShortcut1),
                    2 => Ok(Action::SetShortcut2),
                    3 => Ok(Action::SetShortcut3),
                    4 => Ok(Action::SetShortcut4),
                    5 => Ok(Action::SetShortcut5),
                    6 => Ok(Action::SetShortcut6),
                    7 => Ok(Action::SetShortcut7),
                    8 => Ok(Action::SetShortcut8),
                    9 => Ok(Action::SetShortcut9),
                    _ => Err(format!("Invalid SetShortcut number: {}", num)),
                }
            }
            s if s.starts_with("UseShortcut") => {
                let num = s
                    .chars()
                    .last()
                    .and_then(|c| c.to_digit(10))
                    .ok_or_else(|| format!("Invalid UseShortcut action: {}", s))?;
                match num {
                    0 => Ok(Action::UseShortcut0),
                    1 => Ok(Action::UseShortcut1),
                    2 => Ok(Action::UseShortcut2),
                    3 => Ok(Action::UseShortcut3),
                    4 => Ok(Action::UseShortcut4),
                    5 => Ok(Action::UseShortcut5),
                    6 => Ok(Action::UseShortcut6),
                    7 => Ok(Action::UseShortcut7),
                    8 => Ok(Action::UseShortcut8),
                    9 => Ok(Action::UseShortcut9),
                    _ => Err(format!("Invalid UseShortcut number: {}", num)),
                }
            }
            _ => Err(format!("Unknown action: {}", s)),
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> OtherResult {
        let s = match self {
            Action::IncreaseDimDistance => "IncreaseDimDistance",
            Action::DecreaseDimDistance => "DecreaseDimDistance",
            Action::IncreaseDimIntensity => "IncreaseDimIntensity",
            Action::DecreaseDimIntensity => "DecreaseDimIntensity",
            Action::BorderStyle => "BorderStyle",
            Action::CastCommandLineSpell => "CastCommandLineSpell",
            Action::ToggleFilters => "ToggleFilters",
            Action::GoToTop => "GoToTop",
            Action::GoToBottom => "GoToBottom",
            Action::ExecuteFile => "ExecuteFile",
            Action::GiveBirthDir => "GiveBirthDir",
            Action::GiveBirthFile => "GiveBirthFile",
            Action::GitMenu => "GitMenu",
            Action::SetColorRules => "SetColorRules",
            Action::CycleItemColor => "CycleItemColor",
            Action::RemoveItemColor => "RemoveItemColor",
            Action::SelectAll => "SelectAll",
            Action::SearchFiles => "SearchFiles",
            Action::ShowShortcuts => "ShowShortcuts",
            Action::TerminalCommand => "TerminalCommand",
            Action::Search => "Search",
            Action::Undo => "Undo",
            Action::ToggleSelect => "ToggleSelect",
            Action::MultiSelectUp => "MultiSelectUp",
            Action::MultiSelectDown => "MultiSelectDown",
            Action::MoveUp => "MoveUp",
            Action::MoveDown => "MoveDown",
            Action::MoveLeft => "MoveLeft",
            Action::MoveRight => "MoveRight",
            Action::Enter => "Enter",
            Action::TogglePreview => "TogglePreview",
            Action::ToggleCount => "ToggleCount",
            Action::Rename => "Rename",
            Action::RenameWithoutExtension => "RenameWithoutExtension",
            Action::Murder => "Delete",
            Action::Copy => "Copy",
            Action::Paste => "Paste",
            Action::Duplicate => "Duplicate",
            Action::MoveItem => "MoveItem",
            Action::Quit => "Quit",
            Action::Help => "Help",
            Action::RenameLayer => "RenameLayer",
            // switch
            Action::SwitchLayer0 => "SwitchLayer0",
            Action::SwitchLayer1 => "SwitchLayer1",
            Action::SwitchLayer2 => "SwitchLayer2",
            Action::SwitchLayer3 => "SwitchLayer3",
            Action::SwitchLayer4 => "SwitchLayer4",
            Action::SwitchLayer5 => "SwitchLayer5",
            Action::SwitchLayer6 => "SwitchLayer6",
            Action::SwitchLayer7 => "SwitchLayer7",
            Action::SwitchLayer8 => "SwitchLayer8",
            Action::SwitchLayer9 => "SwitchLayer9",
            // set
            Action::SetShortcut1 => "SetShortcut1",
            Action::SetShortcut2 => "SetShortcut2",
            Action::SetShortcut3 => "SetShortcut3",
            Action::SetShortcut4 => "SetShortcut4",
            Action::SetShortcut5 => "SetShortcut5",
            Action::SetShortcut6 => "SetShortcut6",
            Action::SetShortcut7 => "SetShortcut7",
            Action::SetShortcut8 => "SetShortcut8",
            Action::SetShortcut9 => "SetShortcut9",
            Action::SetShortcut0 => "SetShortcut0",
            // use
            Action::UseShortcut1 => "UseShortcut1",
            Action::UseShortcut2 => "UseShortcut2",
            Action::UseShortcut3 => "UseShortcut3",
            Action::UseShortcut4 => "UseShortcut4",
            Action::UseShortcut5 => "UseShortcut5",
            Action::UseShortcut6 => "UseShortcut6",
            Action::UseShortcut7 => "UseShortcut7",
            Action::UseShortcut8 => "UseShortcut8",
            Action::UseShortcut9 => "UseShortcut9",
            Action::UseShortcut0 => "UseShortcut0",
            Action::SetLineAmount => "SetLineAmount",
            Action::OpenInEditor => "OpenInEditor",
            Action::EditConfig => "EditConfig",
            Action::SortCycleForward => "SortCycleForward",
        };
        write!(f, "{}", s)
    }
}
