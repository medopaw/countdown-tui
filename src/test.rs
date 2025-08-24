use std::time::Duration;
use crate::time_parser::parse_duration_or_time;
use crate::timer::{CountdownState, TimerMode};
use crate::font::get_font;

pub fn test_duration_parsing() {
    println!("Testing duration parsing...");
    
    let test_cases = vec![
        ("5s", 5),
        ("1m30s", 90),
        ("1h2m3s", 3723),
        ("30s", 30),
        ("2m", 120),
        ("1h", 3600),
    ];
    
    for (input, expected_seconds) in test_cases {
        match parse_duration_or_time(input) {
            Ok(duration) => {
                if duration.as_secs() == expected_seconds {
                    println!("✓ {} -> {}s (correct)", input, duration.as_secs());
                } else {
                    println!("✗ {} -> {}s (expected {}s)", input, duration.as_secs(), expected_seconds);
                }
            }
            Err(e) => {
                println!("✗ {} -> Error: {}", input, e);
            }
        }
    }
    
    println!("\nTesting time format parsing...");
    let time_test_cases = vec![
        "14:15",   // 24-hour format
        "02:30PM", // 12-hour format with PM  
        "10:00AM", // 12-hour format with AM
    ];
    
    for input in time_test_cases {
        match parse_duration_or_time(input) {
            Ok(duration) => {
                let hours = duration.as_secs() / 3600;
                let minutes = (duration.as_secs() % 3600) / 60;
                let seconds = duration.as_secs() % 60;
                println!("✓ {} -> {}h {}m {}s", input, hours, minutes, seconds);
            }
            Err(e) => {
                println!("✗ {} -> Error: {}", input, e);
            }
        }
    }
    
    println!("\nTesting error cases...");
    let error_cases = vec![
        "invalid",
        "5",     // number without unit
        "s",     // unit without number
        "",      // empty string
    ];
    
    for input in error_cases {
        match parse_duration_or_time(input) {
            Ok(duration) => {
                println!("✗ {} -> {}s (should have been an error)", input, duration.as_secs());
            }
            Err(e) => {
                println!("✓ {} -> Error: {} (correct)", input, e);
            }
        }
    }
}

pub fn test_timer_logic() {
    println!("\nTesting timer logic...");
    
    let mut countdown = CountdownState::new(Duration::from_secs(5), TimerMode::CountDown);
    
    println!("Initial display: {}s", countdown.display_duration().as_secs());
    
    countdown.tick();
    println!("After 1 tick: {}s", countdown.display_duration().as_secs());
    
    countdown.pause();
    println!("After pause: paused = {}", countdown.is_paused());
    
    countdown.resume();
    println!("After resume: paused = {}", countdown.is_paused());
    
    let mut countup = CountdownState::new(Duration::from_secs(10), TimerMode::CountUp);
    println!("\nCount-up mode:");
    println!("Initial display: {}s", countup.display_duration().as_secs());
    
    countup.tick();
    println!("After 1 tick: {}s", countup.display_duration().as_secs());
}

pub fn test_font_display() {
    println!("\nTesting font system...");
    
    let font = get_font();
    
    println!("Available characters: {:?}", font.keys().collect::<Vec<_>>());
    
    if let Some(zero) = font.get(&'0') {
        println!("Character '0':");
        for line in zero {
            println!("  {}", line);
        }
    }
    
    if let Some(colon) = font.get(&':') {
        println!("Character ':':");
        for line in colon {
            println!("  {}", line);
        }
    }
    
    if let Some(one) = font.get(&'1') {
        println!("Character '1':");
        for (i, line) in one.iter().enumerate() {
            println!("  [{}] '{}' (len: {}, chars: {})", i, line, line.len(), line.chars().count());
            // Show each character individually
            for (j, ch) in line.chars().enumerate() {
                println!("    [{}]: '{}' (U+{:04X})", j, ch, ch as u32);
            }
        }
    }
}