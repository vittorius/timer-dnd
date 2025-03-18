use std::time::Duration;

use clap::{Parser, Subcommand};
use smol::stream::StreamExt;
use smol::Timer;

#[derive(Debug, Parser)]
#[command(name = "mindful-timer")]
#[command(about = "A timer for distraction-free work", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Start a timer for a given duration
    #[command(arg_required_else_help = true)]
    Start {
        #[arg(default_value = "25")]
        duration: u64,
    },
}

pub struct SessionTimer {
    time_left: Duration,
    timer: Timer,
}

impl SessionTimer {
    pub fn new(session_time: u64) -> Self {
        SessionTimer {
            time_left: Duration::from_secs(session_time * 60),
            timer: Timer::interval(Duration::from_secs(1)),
        }
    }

    pub async fn start(&mut self) {
        while (self.timer.next().await).is_some() {
            println!("Timer fired!");
            self.time_left -= Duration::from_secs(1);
            println!("Time left: {:?}", self.time_left);
            if self.time_left == Duration::ZERO {
                println!("Session completed!");
                break;
            }
        }
    }
}
