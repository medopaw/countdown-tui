use crate::font::{get_font, get_paused_text, char_height, paused_text_height, paused_text_width};
use crossterm::{
    cursor, execute,
    style::Print,
    terminal::{Clear, ClearType, size},
};
use std::io::{stdout, Write};
use std::time::Duration;

pub struct Display {
    width: u16,
    height: u16,
}

impl Display {
    pub fn new() -> Self {
        let (width, height) = size().unwrap_or((80, 24));
        Self { width, height }
    }
    
    pub fn update_size(&mut self) {
        let (width, height) = size().unwrap_or((80, 24));
        self.width = width;
        self.height = height;
    }
}

pub fn clear_screen() -> Result<(), Box<dyn std::error::Error>> {
    execute!(stdout(), Clear(ClearType::All))?;
    Ok(())
}

pub fn render_countdown(
    display: &mut Display,
    duration: Duration,
    title: &Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    display.update_size();
    
    let time_string = format_duration(duration);
    let font = get_font();
    
    let mut char_maps = Vec::new();
    let mut total_width = 0;
    
    for ch in time_string.chars() {
        if let Some(char_map) = font.get(&ch) {
            char_maps.push(char_map);
            // Use the same method as original Go: count runes in first line
            total_width += char_map[0].chars().count();
        }
    }
    
    let height = char_height();
    
    let start_x = (display.width as usize).saturating_sub(total_width) / 2;
    let start_y = (display.height as usize).saturating_sub(height) / 2;
    
    execute!(stdout(), cursor::MoveTo(0, 0))?;
    
    // Render exactly like original Go echo() function - character by character
    let mut current_x = start_x;
    for char_map in &char_maps {
        for row in 0..height {
            if row < char_map.len() {
                let y = start_y + row;
                let line = char_map[row];
                
                // Set each character individually like termbox.SetCell
                for (i, ch) in line.chars().enumerate() {
                    execute!(stdout(), cursor::MoveTo((current_x + i) as u16, y as u16))?;
                    execute!(stdout(), Print(ch))?;
                }
            }
        }
        // Move to next character position
        current_x += char_map[0].chars().count();
    }
    
    if let Some(title_text) = title {
        let title_y = start_y + height + 2;
        let title_x = (display.width as usize).saturating_sub(title_text.len()) / 2;
        
        execute!(stdout(), cursor::MoveTo(title_x as u16, title_y as u16))?;
        execute!(stdout(), Print(title_text))?;
    }
    
    stdout().flush()?;
    Ok(())
}

pub fn render_paused(display: &mut Display) -> Result<(), Box<dyn std::error::Error>> {
    display.update_size();
    
    // Clear screen first like original
    clear_screen()?;
    
    let paused_text = get_paused_text();
    let text_width = paused_text_width();
    let text_height = paused_text_height();
    
    let start_x = (display.width as usize).saturating_sub(text_width) / 2;
    let start_y = (display.height as usize * 3 / 4).saturating_sub(text_height / 2);
    
    for (row, line) in paused_text.iter().enumerate() {
        execute!(stdout(), cursor::MoveTo(start_x as u16, (start_y + row) as u16))?;
        execute!(stdout(), Print(line))?;
    }
    
    stdout().flush()?;
    Ok(())
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    
    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}