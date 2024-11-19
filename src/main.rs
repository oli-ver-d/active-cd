mod app;

use app::App;
use anyhow::Result;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use crossterm::{cursor::{MoveTo}, execute, queue, event::{read, Event, KeyCode, KeyEvent}, terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, size}, style::Print, cursor};
use crate::app::OutputType;

fn main() -> Result<()> {
    let current_dir = env::current_dir()?;
    let mut app = App::new(current_dir, false);
    let mut stderr = io::stderr();
    terminal::enable_raw_mode()?;
    execute!(stderr, EnterAlternateScreen, cursor::Hide)?;

    let (cols, rows) = size()?;

    loop {
        queue!(stderr, Clear(ClearType::All))?;
        queue!(stderr, MoveTo(0,0), Print(format!("Current Directory: {}", app.current_dir.display())))?;

        for (i, (key, subdirectory)) in app.subdirectories.iter().enumerate() {
            if let Some(subdir_osstr) = subdirectory.file_name() {
                if let Some(subdir) = subdir_osstr.to_str() {
                    queue!(stderr, MoveTo(0, (i + 2) as u16), Print(format!("{key} -> {subdir}")))?;
                }
            }
        }

        queue!(stderr, MoveTo(0, rows - 1), Print(format!("{}", app.user_input)))?;
        stderr.flush()?;

        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Esc => {
                    app.output = OutputType::Start;
                    break;
                }
                KeyCode::Tab => {
                    app.show_hidden = !app.show_hidden;
                    app.update_subdirectories();
                }
                KeyCode::Char('/') => {
                    app.set_current_dir(PathBuf::from("/"));
                }
                KeyCode::Char('~') => {
                    if let Some(home_dir) = dirs::home_dir() {
                        app.set_current_dir(home_dir);
                    }
                }
                KeyCode::Char('u') => {
                    if let Some(parent) = app.current_dir.parent() {
                        app.set_current_dir(parent.to_path_buf());
                    }
                }
                KeyCode::Backspace => {
                    app.delete_input_letter();
                }
                KeyCode::Char(c) => {
                    app.input_letter(c);
                }
                _ => {}
            }
        }
    }

    execute!(stderr, LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    match app.output {
        OutputType::Start => {
            print!("{}", app.start_dir.display());
        },
        OutputType::Current => {
            print!("{}", app.current_dir.display());
        }
    }
    Ok(())
}
