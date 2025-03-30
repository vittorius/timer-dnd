use clap::Parser;
use helpers::clear_line_and_write;
use helpers::DurationExt;
use macro_rules_attribute::apply;
use smol_macros::main;
use timer_dnd::{Cli, Command, SessionTimer, TimerEvent};

mod helpers;

#[apply(main!)]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Start { session_time_m } => {
            let mut timer = SessionTimer::new(session_time_m, |event| match event {
                TimerEvent::Update(time_left) => {
                    clear_line_and_write(&format!(
                        "🕐 {:02}:{:02}:{:02}",
                        time_left.to_hms().0,
                        time_left.to_hms().1,
                        time_left.to_hms().2
                    ));
                }
                TimerEvent::Finish => clear_line_and_write("✅ Finished\n"),
            });

            println!(
                "Starting timer for{} {}m",
                Some(session_time_m / 60)
                    .filter(|&x| x != 0)
                    .map_or("".to_owned(), |x| format!(" {}h", x)),
                session_time_m % 60,
            );

            timer.start().await;
        }
    }
}
