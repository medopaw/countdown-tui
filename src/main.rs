use clap::Parser;
use std::time::Duration;
use tokio::time::interval;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, poll},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::stdout;

mod font;
mod ui;
mod timer;
mod time_parser;
mod test;

use ui::{Display, render_countdown, render_paused, clear_screen};
use timer::{CountdownState, TimerMode};
use time_parser::parse_duration_or_time;

#[derive(Parser)]
#[command(name = "countdown-tui")]
#[command(about = "A terminal countdown timer with big digital display", long_about = None)]
#[command(
    after_help = "Examples:
  countdown-tui 25s
  countdown-tui -t \"Coffee Break\" 14:15
  countdown-tui 02:15PM
  countdown-tui -u 30s
  countdown-tui -s 10s"
)]
struct Args {
    #[arg(value_name = "DURATION")]
    duration: Option<String>,

    #[arg(short = 'u', long = "up", help = "Count up from zero")]
    up: bool,

    #[arg(short = 's', long = "say", help = "Announce the time left")]
    say: bool,

    #[arg(short = 't', long = "title", value_name = "TEXT", help = "Display title below the countdown")]
    title: Option<String>,
    
    #[arg(long, help = "Run internal tests")]
    test: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    if args.test {
        test::test_duration_parsing();
        test::test_timer_logic();
        test::test_font_display();
        return Ok(());
    }
    
    let duration_str = args.duration.ok_or("error: DURATION is required\n\nUSAGE:\n    countdown-tui [OPTIONS] <DURATION>\n\nFor more information try '--help'")?;
    let duration = parse_duration_or_time(&duration_str)
        .map_err(|e| format!("error: invalid duration or time format '{}'\n\nSupported formats:\n  Duration: 25s, 1m30s, 1h2m3s\n  Time: 14:15, 02:30PM, 10:00AM\n\nOriginal error: {}", duration_str, e))?;
    
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(crossterm::cursor::Hide)?;
    
    let result = run_countdown(duration, args.up, args.say, args.title).await;
    
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    stdout().execute(crossterm::cursor::Show)?;
    
    result
}

async fn run_countdown(
    total_duration: Duration,
    count_up: bool,
    say_time: bool,
    title: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mode = if count_up { TimerMode::CountUp } else { TimerMode::CountDown };
    let mut state = CountdownState::new(total_duration, mode);
    let mut display = Display::new();
    
    let mut tick_interval = interval(Duration::from_secs(1));
    let mut timer_deadline = Box::pin(tokio::time::sleep(total_duration));
    let mut time_left = total_duration;
    
    let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();
    
    let event_handle = tokio::spawn(async move {
        loop {
            // Use non-blocking poll with timeout
            match poll(Duration::from_millis(100)) {
                Ok(true) => {
                    match event::read() {
                        Ok(event) => {
                            if event_tx.send(event).is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
                Ok(false) => {
                    // No event available, continue polling
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
                Err(_) => break,
            }
        }
    });
    
    clear_screen()?;
    
    // Initial draw
    let current_display_duration = if count_up { Duration::ZERO } else { time_left };
    render_countdown(&mut display, current_display_duration, &title)?;
    if say_time && !count_up && time_left.as_secs() <= 10 && time_left.as_secs() > 0 {
        tokio::spawn(say_countdown(time_left.as_secs()));
    }
    
    loop {
        tokio::select! {
            _ = tick_interval.tick() => {
                if state.is_paused() {
                    continue;
                }
                
                // Update time_left like original ticker.C logic
                if time_left > Duration::from_secs(1) {
                    time_left -= Duration::from_secs(1);
                } else {
                    time_left = Duration::ZERO;
                }
                
                let display_duration = if count_up { 
                    total_duration - time_left 
                } else { 
                    time_left 
                };
                render_countdown(&mut display, display_duration, &title)?;
                
                if say_time && !count_up && time_left.as_secs() <= 10 && time_left.as_secs() > 0 {
                    tokio::spawn(say_countdown(time_left.as_secs()));
                }
            }
            
            _ = &mut timer_deadline => {
                // Time's up! Exit like original timer.C - don't draw 00:00, just break
                break;
            }
            
            event = event_rx.recv() => {
                if let Some(event) = event {
                    match event {
                        Event::Key(key) if key.kind == KeyEventKind::Press => {
                            match key.code {
                                KeyCode::Char(' ') => {
                                    if state.is_paused() {
                                        state.resume();
                                        // Restart timer with remaining time_left like original start(timeLeft)
                                        timer_deadline = Box::pin(tokio::time::sleep(time_left));
                                        let display_duration = if count_up { 
                                            total_duration - time_left 
                                        } else { 
                                            time_left 
                                        };
                                        render_countdown(&mut display, display_duration, &title)?;
                                    } else {
                                        state.pause();
                                        // Stop the timer like original stop() function
                                        timer_deadline = Box::pin(tokio::time::sleep(Duration::from_secs(999999))); // Never expires
                                        render_paused(&mut display)?;
                                    }
                                }
                                KeyCode::Esc | KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                                    std::process::exit(1);
                                }
                                _ => {}
                            }
                        }
                        Event::Resize(_, _) => {
                            clear_screen()?;
                            if state.is_paused() {
                                render_paused(&mut display)?;
                            } else {
                                let current_display_duration = state.display_duration();
                                render_countdown(&mut display, current_display_duration, &title)?;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    
    // Abort the event reading task to prevent further input processing
    event_handle.abort();
    
    Ok(())
}

async fn say_countdown(seconds: u64) {
    if cfg!(target_os = "macos") {
        let _ = std::process::Command::new("say")
            .arg(seconds.to_string())
            .spawn();
    }
}
