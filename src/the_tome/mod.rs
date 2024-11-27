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

// Should be removed when the annoyence is no longer present.
// comlexity might always be there.

#[macro_export]
macro_rules! interaction_field {
    ($($arg:tt)*) => {
        interaction_field(format_args!($($arg)*))
    };
}

#[allow(clippy::complexity, dead_code, clippy::if_same_then_else)]
pub mod browser_commands;
pub mod config;
pub mod file_entry;
pub mod main_nav_loop;
pub mod marvelous_actions;
pub mod mouse;
pub mod nav_functions;
pub mod system_functions;
pub mod the_search;
pub mod tome_state;
pub mod ui_components;

//////////////////////////////////////////DEPENDENCIES///////////////////////////////////////////////
// If possible, do not add any more dependencies. Rather work to remove them.
pub use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    event::{DisableMouseCapture, EnableMouseCapture},
    event::{MouseButton, MouseEvent, MouseEventKind},
    style::{
        Attribute, Color, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
        StyledContent, Stylize,
    },
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen, SetSize,
    },
    {cursor, execute, queue, terminal},
};

pub use rayon::prelude::*;

/////////////////////////////////////////!DEPENDENCIES!//////////////////////////////////////////////
pub use self::{
    browser_commands::*, config::*, file_entry::*, main_nav_loop::*, marvelous_actions::*,
    mouse::*, nav_functions::*, system_functions::*, the_search::*, tome_state::*,
    ui_components::*,
};
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
pub use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    env,
    fmt::{Arguments, Display, Formatter, Result as OtherResult},
    fs::{self, File, OpenOptions},
    io::{self, stdout, BufRead, Read, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    result::Result,
    slice::Iter,
    str::FromStr,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
    usize,
};

pub const INPUT_TIMEOUT: Duration = Duration::from_secs(30);
pub const MAX_NAME_LENGTH: usize = 255;
pub const VISIBLE_LINES: usize = 35;
pub const DEFAULT_RAM_LIMIT: usize = 500 * 1024 * 1024;
pub const DEFAULT_DISK_LIMIT: u64 = 2 * 1024 * 1024 * 1024;
pub const PREVIEW_LIMIT: usize = 5 * 1024;
