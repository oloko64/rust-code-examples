use std::error::Error;

use time::{macros::format_description, UtcOffset};
use tracing_subscriber::fmt::{time::OffsetTime, writer::MakeWriterExt};

fn main() -> Result<(), Box<dyn Error>> {
    let offset = UtcOffset::current_local_offset()?;
    let timer = OffsetTime::new(
        offset,
        format_description!("[day]-[month]-[year] [hour]:[minute]:[second]"),
    );
    let appender = tracing_appender::rolling::never("logs", "logs.log");
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(appender);
    let (non_blocking_stdout, _guard_stdout) = tracing_appender::non_blocking(std::io::stdout());
    let all_logs = non_blocking_file.and(non_blocking_stdout);

    // We can initialize the subscriber directly
    tracing_subscriber::fmt()
        .with_timer(timer)
        .with_writer(all_logs)
        .with_ansi(false)
        .with_max_level(tracing::Level::INFO)
        .init();

    // Or we can use set_global_default subscriber
    // let subscriber = tracing_subscriber::fmt()
    //     .with_timer(timer)
    //     .with_writer(all_logs)
    //     .with_ansi(false)
    //     .with_max_level(tracing::Level::INFO)
    //     .finish();
    // tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Hello, world!");
    tracing::warn!("Hello, world Two!");

    println!("Hello, world!");
    Ok(())
}
