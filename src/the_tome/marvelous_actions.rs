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
///////////////////////////////////////////////////////Action///////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
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
        static ACTIONS: [Action; 59] = [
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
