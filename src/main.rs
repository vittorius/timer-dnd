use clap::Parser;
use macro_rules_attribute::apply;
use mindful_timer_rs::{Cli, Command, SessionTimer};
use smol_macros::main;

#[apply(main!)]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Start { duration } => {
            let mut timer = SessionTimer::new(duration);
            timer.start().await;
        }
    }
}
