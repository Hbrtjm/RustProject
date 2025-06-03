use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::fs;
use std::path::PathBuf;
use crate::converter::formats::ImageFormat;
use crate::converter::main_converter;

#[derive(Debug)]
pub enum AppEvent {
    Input(KeyEvent),
    Tick,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    SelectMode,
    ConvertMode,
}

#[derive(Debug)]
pub struct AppState {
    pub cwd: PathBuf,
    pub entries: Vec<fs::DirEntry>,
    pub selected_index: usize,
    pub selected_file: Option<PathBuf>,
    pub mode: AppMode,
    pub selected_format_index: usize,
    pub to_convert: Vec<(PathBuf, ImageFormat)>,
    pub status_message: Option<String>,
    pub command_buffer: String,
}

impl AppState {
    pub fn new(cwd: PathBuf) -> Self {
        let mut app = Self {
            cwd: cwd.clone(),
            entries: Vec::new(),
            selected_index: 0,
            selected_file: None,
            mode: AppMode::SelectMode,
            selected_format_index: 0,
            to_convert: Vec::new(),
            status_message: None,
            command_buffer: String::new(),
        };
        app.refresh_entries();
        app
    }

    pub fn refresh_entries(&mut self) {
        match fs::read_dir(&self.cwd) {
            Ok(dir_entries) => {
                self.entries = dir_entries
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| {
                        if let Ok(metadata) = entry.metadata() {
                            if metadata.is_dir() {
                                return true;
                            }
                            // Check if file extension matches ImageFormat
                            if let Some(filename) = entry.file_name().to_str() {
                                return ImageFormat::from_extension(Some(filename)).is_some();
                            }
                        }
                        false
                    })
                    .collect();

                // Sort entries: directories first, then files, both alphabetically
                self.entries.sort_by(|a, b| {
                    let a_is_dir = a.metadata().map(|m| m.is_dir()).unwrap_or(false);
                    let b_is_dir = b.metadata().map(|m| m.is_dir()).unwrap_or(false);
                    
                    match (a_is_dir, b_is_dir) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => a.file_name().cmp(&b.file_name()),
                    }
                });

                if self.selected_index >= self.entries.len() {
                    self.selected_index = self.entries.len().saturating_sub(1);
                }
            }
            Err(e) => {
                self.status_message = Some(format!("Error reading directory: {}", e));
                // Switch to home directory on error
                if let Some(home) = dirs::home_dir() {
                    self.cwd = home;
                    self.refresh_entries();
                }
            }
        }
    }

    pub fn can_go_up(&self) -> bool {
        self.cwd.parent().is_some()
    }

    pub fn go_up(&mut self) {
        if let Some(parent) = self.cwd.parent() {
            self.cwd = parent.to_path_buf();
            self.refresh_entries();
            self.selected_index = 0;
        }
    }

    pub fn enter_selected(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        let selected_entry = &self.entries[self.selected_index];
        
        if let Ok(metadata) = selected_entry.metadata() {
            if metadata.is_dir() {
                // Enter directory
                self.cwd = selected_entry.path();
                self.refresh_entries();
                self.selected_index = 0;
            } else {
                // Select file
                self.selected_file = Some(selected_entry.path());
            }
        }
    }

    pub fn move_up(&mut self) {
        if !self.entries.is_empty() {
            self.selected_index = self.selected_index.saturating_sub(1);
        }
    }

    pub fn move_down(&mut self) {
        if !self.entries.is_empty() && self.selected_index < self.entries.len() {
            self.selected_index += 1;
        }
    }

    pub fn move_format_up(&mut self) {
        self.selected_format_index = self.selected_format_index.saturating_sub(1);
    }

    pub fn move_format_down(&mut self) {
        if self.selected_format_index < 5 {
            self.selected_format_index += 1;
        }
    }

    pub fn confirm_conversion(&mut self) {
        if let Some(ref file_path) = self.selected_file {
            let formats = [
                ImageFormat::PNG,
                ImageFormat::JPEG,
                ImageFormat::WEBP,
                ImageFormat::GIF,
                ImageFormat::BMP,
            ];
            
            let selected_format = formats[self.selected_format_index].clone();
            self.to_convert.push((file_path.clone(), selected_format.clone()));
            
            self.status_message = Some(format!(
                "Will convert {} → {}",
                file_path.display(),
                format!("{:?}", selected_format).to_uppercase()
            ));

            main_converter::convert(
                &file_path,
                &selected_format,
            ).unwrap_or_else(|e| {
                self.status_message = Some(format!("Conversion error: {}", e));
            });
            self.refresh_entries();
        }
    }
}

pub fn handle_input(app: &mut AppState, key: KeyEvent) -> bool {
    // Handle Ctrl+C for quit
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return true; // Signal to quit
    }

    match key.code {
        KeyCode::Char(c) => {
            let ch = c.to_lowercase().next().unwrap_or(c);
            match ch {
                'k' => {
                    app.command_buffer.clear();
                    match app.mode {
                        AppMode::SelectMode => app.move_up(),
                        AppMode::ConvertMode => app.move_format_up(),
                    }
                }
                'j' => {
                    app.command_buffer.clear();
                    match app.mode {
                        AppMode::SelectMode => app.move_down(),
                        AppMode::ConvertMode => app.move_format_down(),
                    }
                }
                _ => {
                    app.command_buffer.push(ch);

                    // Check for complete commands
                    let buffer_lower = app.command_buffer.to_lowercase();

                    // Quit commands
                    if buffer_lower == "q" || buffer_lower == "quit" {
                        return true; // Signal to quit
                    }

                    // Select mode commands
                    if buffer_lower == "s" || buffer_lower == "sel" || 
                       buffer_lower == "select" || buffer_lower == "selection" {
                        app.mode = AppMode::SelectMode;
                        app.command_buffer.clear();
                        return false;
                    }

                    // Convert mode commands
                    if buffer_lower == "c" || buffer_lower == "conv" || 
                       buffer_lower == "convert" || buffer_lower == "conversion" {
                        if app.selected_file.is_some() {
                            app.mode = AppMode::ConvertMode;
                        }
                        app.command_buffer.clear();
                        return false;
                    }

                    // Clear buffer if it gets too long without matching
                    if app.command_buffer.len() > 10 {
                        app.command_buffer.clear();
                    }
                }
            }
        }
        KeyCode::Up => {
            app.command_buffer.clear();
            match app.mode {
                AppMode::SelectMode => app.move_up(),
                AppMode::ConvertMode => app.move_format_up(),
            }
        }
        KeyCode::Down => {
            app.command_buffer.clear();
            match app.mode {
                AppMode::SelectMode => app.move_down(),
                AppMode::ConvertMode => app.move_format_down(),
            }
        }
        KeyCode::Enter => {
            app.command_buffer.clear();
            match app.mode {
                AppMode::SelectMode => {
                    // Handle ".." entry for going up
                    if app.selected_index == 0 && app.can_go_up() && !app.entries.is_empty() {
                        // Check if first entry is actually ".." (we'll add this in UI)
                        app.go_up();
                    } else {
                        // Adjust index if we have ".." entry
                        let actual_index = if app.can_go_up() && !app.entries.is_empty() {
                            if app.selected_index == 0 {
                                return false; // Already handled above
                            }
                            app.selected_index - 1
                        } else {
                            app.selected_index
                        };
                        
                        if actual_index < app.entries.len() {
                            let temp_index = app.selected_index;
                            app.selected_index = actual_index;
                            app.enter_selected();
                            if app.selected_file.is_none() {
                                app.selected_index = temp_index;
                            }
                        }
                    }
                }
                AppMode::ConvertMode => {
                    app.confirm_conversion();
                }
            }
        }
        KeyCode::Esc => {
            app.command_buffer.clear();
            app.mode = AppMode::SelectMode;
        }
        _ => {
            app.command_buffer.clear();
        }
    }
    
    false // Don't quit
}