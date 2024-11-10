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
#[allow(clippy::complexity, dead_code, clippy::if_same_then_else)]
pub mod browser_commands;
#[allow(clippy::complexity, dead_code, clippy::if_same_then_else)]
pub mod config;
#[allow(clippy::complexity, dead_code, clippy::if_same_then_else)]
pub mod file_entry;
#[allow(clippy::complexity, dead_code, clippy::if_same_then_else)]
pub mod main_nav_loop;
#[allow(clippy::complexity, dead_code, clippy::if_same_then_else)]
pub mod marvelous_actions;
#[allow(clippy::complexity, dead_code, clippy::if_same_then_else)]
pub mod nav_functions;
#[allow(clippy::complexity, dead_code, clippy::if_same_then_else)]
pub mod system_functions;
#[allow(clippy::complexity, dead_code, clippy::if_same_then_else)]
pub mod the_search;
#[allow(clippy::complexity, dead_code, clippy::if_same_then_else)]
pub mod tome_state;
#[allow(clippy::complexity, dead_code, clippy::if_same_then_else)]
pub mod ui_components;

//////////////////////////////////////////DEPENDENCIES///////////////////////////////////////////////
// If possible, do not add any more dependencies. Rather work to remove them.

pub use crossterm::cursor::{Hide, MoveTo, Show};
pub use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
pub use crossterm::style::{
    Attribute, Color, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    StyledContent, Stylize,
};
pub use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen, SetSize,
};
pub use crossterm::{cursor, execute, queue, terminal};

pub use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
pub use rayon::prelude::*;
/////////////////////////////////////////!DEPENDENCIES!//////////////////////////////////////////////

pub use std::cmp::Ordering;
pub use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
pub use std::env::{self};
pub use std::fmt::{Display, Formatter, Result as OtherResult};
pub use std::fs::{self, File, OpenOptions};
pub use std::io::{self, stdout, BufRead, Read, Write};
pub use std::os::unix::fs::MetadataExt;
pub use std::path::{Path, PathBuf};
pub use std::process::{Child, Command, Stdio};
pub use std::result::Result;
pub use std::slice::Iter;
pub use std::str::FromStr;
pub use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::usize;
use std::{io::Cursor, sync::Mutex};

pub use super::the_tome::browser_commands::*;
pub use super::the_tome::config::*;
pub use super::the_tome::file_entry::*;
pub use super::the_tome::marvelous_actions::*;
pub use super::the_tome::nav_functions::*;
pub use super::the_tome::system_functions::*;
pub use super::the_tome::the_search::*;
pub use super::the_tome::tome_state::*;
pub use super::the_tome::ui_components::*;

pub const INPUT_TIMEOUT: Duration = Duration::from_secs(30);
pub const MAX_NAME_LENGTH: usize = 255;
pub const VISIBLE_LINES: usize = 35;
pub const DEFAULT_RAM_LIMIT: usize = 500 * 1024 * 1024;
pub const DEFAULT_DISK_LIMIT: u64 = 2 * 1024 * 1024 * 1024;
pub const PREVIEW_LIMIT: usize = 5 * 1024; // 50 KB
                                           // pub const SCROLL_SPEED: f32 = 0.3; //Not enabled, will try to smooth things out later. play with this to control smoothness (0.0 to 1.0)
                                           // pub const MIDDLE_OFFSET: f32 = VISIBLE_LINES / 2.0;
                                           // pub const INPUT_TIMEOUT: Duration = Duration::from_secs(30);
                                           // pub const MAX_NAME_LENGTH: usize = 255;
