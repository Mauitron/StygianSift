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

#[allow(dead_code)]
// mod hydra;
use crossterm::{
    event::{Event, KeyCode},
    terminal,
};
use std::{env::current_dir, io};
use the_tome::main_nav_loop::{browse_fuzzy_file, BrowseResult};
/////////////////////////////////////////////////
mod the_tome;
use the_tome::tome_state::AppState;
/////////////////////////////////////////////////
//sort by color
//search for colors
//global extantions rules
//plugin system
//color rules that persist outside of the app?

fn main() -> io::Result<()> {
    let mut state = AppState::new()?;
    if state.config.home_folder.is_none() {
        let current_dir = current_dir();
        state.config.home_folder = Some(current_dir?);
    }

    loop {
        if !state.current_file_selected {
            match browse_fuzzy_file(&mut state)? {
                BrowseResult::FileSelected => {
                    state.current_file_selected = true;
                }
                BrowseResult::Exit => break,
                BrowseResult::Continue => {}
            }
        } else {
            terminal::enable_raw_mode()?;
            if let Ok(Event::Key(key)) = crossterm::event::read() {
                match key.code {
                    KeyCode::Tab => {
                        state.current_file_selected = false;
                        continue;
                    }
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => state.input.push(c as u8),
                    _ => {}
                }
            }
            terminal::disable_raw_mode()?;

            // redraw_interface(&mut stdout, &mut state)?;
        }
    }

    Ok(())
}
// Part of my search algorithm. might be added later, or will be another project?

// fn redraw_interface(stdout: &mut Stdout, state: &mut AppState) -> io::Result<()> {
//     let current_count = if state.show_count { 0 } else { 0 };

//     if current_count != state.last_count || state.last_time_stop != state.avg_time {
//         // execute!(stdout, Clear(ClearType::All))?; // Clear screen before redrawing
//         execute!(stdout, MoveTo(0, 5))?;
//         writeln!(stdout, "[F1: Verify Data]\r")?;
//         writeln!(stdout, "[F2: Memory Stats]")?;
//         execute!(stdout, MoveTo(0, 41))?;
//         println!("|{}|", String::from_utf8_lossy(&state.input));
//         execute!(stdout, MoveTo(0, 0))?;
//         state.last_count = current_count;
//         state.last_time_stop = state.avg_time;
//     }
//     state.no_match_len = state.input.len();
//     Ok(())
// }

// fn update_timing_stats(state: &mut AppState, time_stop: Duration) {
//     if time_stop > state.max_time {
//         state.max_time = time_stop;
//     }
//     if time_stop < state.min_time && time_stop > Duration::ZERO || state.min_time.is_zero() {
//         state.min_time = time_stop;
//     }
//     if time_stop > Duration::ZERO {
//         state.sum_time += time_stop;
//         state.avg_time = state.sum_time / state.time_iter;
//         state.time_iter += 1;
//     }
// }

// fn wait_for_enter() -> io::Result<()> {
//     loop {
//         if let Event::Key(key) = event::read()? {
//             if key.code == KeyCode::Enter {
//                 break;
//             }
//         }
//     }
//     Ok(())
// }
