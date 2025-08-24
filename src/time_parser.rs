use chrono::{Local, NaiveTime, Timelike};
use std::time::Duration;

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat(String),
    InvalidDuration(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
            ParseError::InvalidDuration(s) => write!(f, "Invalid duration: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse_duration_or_time(input: &str) -> Result<Duration, ParseError> {
    if let Ok(duration) = parse_duration_string(input) {
        return Ok(duration);
    }
    
    if let Ok(duration) = parse_time_string(input) {
        return Ok(duration);
    }
    
    Err(ParseError::InvalidFormat(input.to_string()))
}

fn parse_duration_string(input: &str) -> Result<Duration, ParseError> {
    let input = input.trim();
    
    if input.is_empty() {
        return Err(ParseError::InvalidDuration("Empty string".to_string()));
    }
    
    let mut total_seconds = 0u64;
    let mut current_number = String::new();
    
    for ch in input.chars() {
        match ch {
            '0'..='9' => {
                current_number.push(ch);
            }
            'h' | 'H' => {
                if current_number.is_empty() {
                    return Err(ParseError::InvalidDuration("Missing number before 'h'".to_string()));
                }
                let hours: u64 = current_number.parse()
                    .map_err(|_| ParseError::InvalidDuration(format!("Invalid number: {}", current_number)))?;
                total_seconds += hours * 3600;
                current_number.clear();
            }
            'm' | 'M' => {
                if current_number.is_empty() {
                    return Err(ParseError::InvalidDuration("Missing number before 'm'".to_string()));
                }
                let minutes: u64 = current_number.parse()
                    .map_err(|_| ParseError::InvalidDuration(format!("Invalid number: {}", current_number)))?;
                total_seconds += minutes * 60;
                current_number.clear();
            }
            's' | 'S' => {
                if current_number.is_empty() {
                    return Err(ParseError::InvalidDuration("Missing number before 's'".to_string()));
                }
                let seconds: u64 = current_number.parse()
                    .map_err(|_| ParseError::InvalidDuration(format!("Invalid number: {}", current_number)))?;
                total_seconds += seconds;
                current_number.clear();
            }
            ' ' => {
            }
            _ => {
                return Err(ParseError::InvalidDuration(format!("Invalid character: {}", ch)));
            }
        }
    }
    
    if !current_number.is_empty() {
        return Err(ParseError::InvalidDuration("Number without unit".to_string()));
    }
    
    if total_seconds == 0 {
        return Err(ParseError::InvalidDuration("Zero duration".to_string()));
    }
    
    Ok(Duration::from_secs(total_seconds))
}

fn parse_time_string(input: &str) -> Result<Duration, ParseError> {
    let input = input.trim().to_uppercase();
    
    let target_time = if input.ends_with("AM") || input.ends_with("PM") {
        let time_str = input.trim_end_matches("AM").trim_end_matches("PM").trim();
        let time_with_ampm = format!("{} {}", time_str, &input[input.len() - 2..]);
        
        NaiveTime::parse_from_str(&time_with_ampm, "%I:%M %p")
            .or_else(|_| NaiveTime::parse_from_str(&time_with_ampm, "%I:%M%p"))
            .or_else(|_| NaiveTime::parse_from_str(&time_with_ampm, "%l:%M %p"))
            .or_else(|_| NaiveTime::parse_from_str(&time_with_ampm, "%l:%M%p"))
            .map_err(|_| ParseError::InvalidFormat(input.clone()))?
    } else {
        NaiveTime::parse_from_str(&input, "%H:%M")
            .or_else(|_| NaiveTime::parse_from_str(&input, "%H%M"))
            .map_err(|_| ParseError::InvalidFormat(input.clone()))?
    };
    
    let now = Local::now().time();
    let now_seconds = now.hour() * 3600 + now.minute() * 60 + now.second();
    let target_seconds = target_time.hour() * 3600 + target_time.minute() * 60;
    
    let duration_seconds = if target_seconds <= now_seconds {
        24 * 3600 + target_seconds - now_seconds
    } else {
        target_seconds - now_seconds
    };
    
    Ok(Duration::from_secs(duration_seconds as u64))
}