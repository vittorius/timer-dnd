use std::time::Duration;

use clap::{Parser, Subcommand};
use smol::stream::StreamExt;
use smol::Timer;

#[derive(Debug, Parser)]
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
        session_time_m: u64,
    },
}

pub enum TimerEvent {
    Update(Duration),
    Finish,
}

pub struct SessionTimer<'a> {
    time_left: Duration,
    timer: Timer,
    on_event: Box<dyn Fn(TimerEvent) + 'a>,
}

impl<'a> SessionTimer<'a> {
    pub fn new(session_time_m: u64, on_event: impl Fn(TimerEvent) + 'a) -> Self {
        SessionTimer {
            time_left: Duration::from_secs(session_time_m * 60),
            timer: Timer::interval(Duration::from_secs(1)),
            on_event: Box::new(on_event),
        }
    }

    pub async fn start(&mut self) {
        while (self.timer.next().await).is_some() {
            self.time_left -= Duration::from_secs(1);
            (self.on_event)(TimerEvent::Update(self.time_left));

            if self.time_left == Duration::ZERO {
                (self.on_event)(TimerEvent::Finish);
                break;
            }
        }
    }
}
