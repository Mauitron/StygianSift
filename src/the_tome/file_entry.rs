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

use super::*;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitStatus {
    Unmodified,
    Modified,
    Added,
    Deleted,
    Renamed,
    Untracked,
    Ignored,
}

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub file_type: FileType,
    pub size: u64,
    pub admin_required: bool,
    pub read_only: bool,
    pub git_status: Option<GitStatus>,
}

impl FileEntry {
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let metadata = fs::metadata(&path)?;

        let name = path
            .file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_default()
            .into_owned();

        let git_info = GitInfo::new();
        let git_status = if git_info.is_git_repo {
            git_info.file_statuses.get(&path).cloned()
        } else {
            None
        };

        let file_type = if metadata.is_dir() {
            FileType::Directory
        } else {
            Self::determine_file_type(&path)
        };

        let (admin_required, read_only) = Self::check_permissions(&path, &metadata);

        Ok(Self {
            path,
            name,
            file_type,
            size: metadata.len(),
            admin_required,
            read_only,
            git_status,
        })
    }

    #[inline]
    fn determine_file_type(path: &Path) -> FileType {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => FileType::Rust,
            Some("zig") => FileType::Zig,
            Some("nix") => FileType::Nix,
            Some("txt") => FileType::Text,
            Some("log") => FileType::Log,
            Some("doc" | "docx" | "odt") => FileType::Document,
            Some("jpg" | "jpeg" | "png" | "svg") => FileType::Image,
            Some("bin") => FileType::Binary,
            Some("ini" | "config" | "yml" | "yaml" | "toml") => FileType::Config,
            Some("exe" | "sh" | "bat") => FileType::Executable,
            _ => FileType::Unknown,
        }
    }

    #[cfg(unix)]
    fn check_permissions(path: &Path, metadata: &fs::Metadata) -> (bool, bool) {
        use std::os::unix::fs::MetadataExt;
        let mode = metadata.mode();
        let uid = metadata.uid();

        let admin_required = if uid == 0 { false } else { (mode & 0o200) == 0 };

        (admin_required, metadata.permissions().readonly())
    }

    #[cfg(windows)]
    fn check_permissions(path: &Path, metadata: &fs::Metadata) -> (bool, bool) {
        use std::os::windows::fs::MetadataExt;

        if let Ok(attrs) = metadata.file_attributes() {
            let readonly = (attrs & 0x1) != 0;
            let system = (attrs & 0x4) != 0;
            let hidden = (attrs & 0x2) != 0;

            let admin_required = system || hidden;

            (admin_required, readonly)
        } else {
            (false, metadata.permissions().readonly())
        }
    }
}
