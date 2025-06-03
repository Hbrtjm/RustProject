use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use crate::frontend::events::{AppMode, AppState};
use crate::converter::formats::ImageFormat;

const OAK_BROWN: Color = Color::Rgb(101, 67, 33);
const DARK_GREEN: Color = Color::Rgb(0, 100, 0);

pub fn draw<B: Backend>(f: &mut Frame, app: &AppState) {
    let size = f.size();
    
    // Check minimum width
    if size.width < 80 {
        let error_msg = Paragraph::new("Terminal too narrow. Please resize to at least 80×24.")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red));
        
        let centered_area = centered_rect(60, 20, size);
        f.render_widget(error_msg, centered_area);
        return;
    }

    // Create main layout: left pane, right pane, status bar
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(size);

    let pane_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[0]);

    // Draw left pane (directory browser)
    draw_directory_pane::<B>(f, app, pane_chunks[0]);
    
    // Draw right pane (file selection and conversion)
    draw_conversion_pane::<B>(f, app, pane_chunks[1]);
    
    // Draw status bar
    draw_status_bar::<B>(f, app, main_chunks[1]);


}
fn draw_directory_pane<B: Backend>(f: &mut Frame, app: &AppState, area: Rect) {
    let mut items = Vec::new();
    let mut list_state = ListState::default();
    
    // Add ".." entry if we can go up
    let mut index_offset = 0;
    if app.can_go_up() {
        items.push(ListItem::new("..").style(Style::default().fg(Color::Yellow)));
        index_offset = 1;
    }
    
    // Add directory entries
    for entry in &app.entries {
        let name = entry.file_name().to_string_lossy().to_string();
        let display_name = if entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
            format!("{}/", name)
        } else {
            name
        };
        
        items.push(ListItem::new(display_name));
    }
    
    // Handle empty directory
    if items.is_empty() || (items.len() == 1 && index_offset == 1) {
        items.push(ListItem::new("<empty>").style(Style::default().fg(Color::DarkGray)));
    }
    
    // Set selected index
    if !items.is_empty() {
        list_state.select(Some(app.selected_index));
    }
    
    let border_style = if app.mode == AppMode::SelectMode {
        Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };
    
    let block = Block::default()
        .title("Select File")
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(Style::default().bg(OAK_BROWN));
    
    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().bg(DARK_GREEN).fg(Color::White))
        .highlight_symbol("▶ ");
    
    f.render_stateful_widget(list, area, &mut list_state);
}

fn draw_conversion_pane<B: Backend>(f: &mut Frame, app: &AppState, area: Rect) {
    let border_style = if app.mode == AppMode::ConvertMode {
        Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };
    
    let block = Block::default()
        .title("Convert File")
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(Style::default().bg(OAK_BROWN));
    
    let inner_area = block.inner(area);
    f.render_widget(block, area);
    
    match (&app.selected_file, &app.mode) {
        (None, _) => {
            // No file selected
            let msg = Paragraph::new("No file selected")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::White));
            
            let centered_area = centered_rect(50, 20, inner_area);
            f.render_widget(msg, centered_area);
        }
        (Some(file_path), AppMode::SelectMode) => {
            // File selected but in select mode
            let text = vec![
                Line::from("Selected file:"),
                Line::from(file_path.display().to_string()),
                Line::from(""),
                Line::from("(Type \"conv\" to choose format)"),
            ];
            
            let paragraph = Paragraph::new(text)
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left);
            
            f.render_widget(paragraph, inner_area);
        }
        (Some(file_path), AppMode::ConvertMode) => {
            // File selected and in convert mode
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(inner_area);
            
            // File info section
            let text = vec![
                Line::from("Selected file:"),
                Line::from(file_path.display().to_string()),
            ];
            
            let file_info = Paragraph::new(text)
                .style(Style::default().fg(Color::White));
            
            f.render_widget(file_info, chunks[0]);
            
            // Format selection section
            draw_format_selection::<B>(f, app, chunks[1]);
        }
    }
}

fn draw_format_selection<B: Backend>(f: &mut Frame, app: &AppState, area: Rect) {
    let formats = [
        ImageFormat::PNG,
        ImageFormat::JPEG,
        ImageFormat::WEBP,
        ImageFormat::GIF,
        ImageFormat::ICO,
        ImageFormat::BMP,
    ];
    
    let format_names = ["PNG", "JPEG", "WEBP", "GIF", "ICO", "BMP"];
    
    let items: Vec<ListItem> = format_names
        .iter()
        .map(|&name| ListItem::new(format!("• {}", name)))
        .collect();
    
    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_format_index));
    
    let block = Block::default()
        .title("Target format:")
        .borders(Borders::ALL)
        .style(Style::default().bg(OAK_BROWN));
    
    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().bg(DARK_GREEN).fg(Color::White))
        .highlight_symbol("▶ ");
    
    f.render_stateful_widget(list, area, &mut list_state);
}

fn draw_status_bar<B: Backend>(f: &mut Frame, app: &AppState, area: Rect) {
    let status_text = if let Some(ref message) = app.status_message {
        message.clone()
    } else if !app.to_convert.is_empty() {
        let (path, format) = &app.to_convert[app.to_convert.len() - 1];
        format!("Will convert {} → {:?}", path.display(), format).replace("\"", "")
    } else {
        String::new()
    };
    
    let status = Paragraph::new(status_text)
        .style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Left);
    
    f.render_widget(status, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}