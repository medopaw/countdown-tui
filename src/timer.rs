use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub enum TimerMode {
    CountDown,
    CountUp,
}

#[derive(Debug)]
pub struct CountdownState {
    total_duration: Duration,
    remaining_duration: Duration,
    mode: TimerMode,
    paused: bool,
}

impl CountdownState {
    pub fn new(duration: Duration, mode: TimerMode) -> Self {
        Self {
            total_duration: duration,
            remaining_duration: duration,
            mode,
            paused: false,
        }
    }
    
    pub fn tick(&mut self) {
        if self.paused {
            return;
        }
        
        // Simple tick like original: just subtract one second
        if self.remaining_duration > Duration::from_secs(1) {
            self.remaining_duration -= Duration::from_secs(1);
        } else {
            self.remaining_duration = Duration::ZERO;
        }
    }
    
    pub fn pause(&mut self) {
        if !self.paused {
            self.paused = true;
        }
    }
    
    pub fn resume(&mut self) {
        if self.paused {
            self.paused = false;
        }
    }
    
    pub fn is_paused(&self) -> bool {
        self.paused
    }
    
    pub fn display_duration(&self) -> Duration {
        match self.mode {
            TimerMode::CountDown => self.remaining_duration,
            TimerMode::CountUp => self.total_duration - self.remaining_duration,
        }
    }
    
    pub fn remaining_duration(&self) -> Duration {
        self.remaining_duration
    }
}