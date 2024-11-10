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
pub use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    env,
    fmt::{Arguments, Display, Formatter, Result as OtherResult},
    fs::{self, File, OpenOptions},
    io::{self, stdout, BufRead, Read, Write},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    result::Result,
    slice::Iter,
    str::FromStr,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
    usize,
};

pub use self::{
    browser_commands::*, config::*, file_entry::*, marvelous_actions::*, nav_functions::*,
    system_functions::*, the_search::*, tome_state::*, ui_components::*,
};

pub const INPUT_TIMEOUT: Duration = Duration::from_secs(30);
pub const MAX_NAME_LENGTH: usize = 255;
pub const VISIBLE_LINES: usize = 35;
pub const DEFAULT_RAM_LIMIT: usize = 500 * 1024 * 1024;
pub const DEFAULT_DISK_LIMIT: u64 = 2 * 1024 * 1024 * 1024;
pub const PREVIEW_LIMIT: usize = 5 * 1024;
