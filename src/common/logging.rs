use chrono::Local;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    fmt::{self, time::FormatTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

// Custom timer struct that implements FormatTime
struct ChronoTime;

impl FormatTime for ChronoTime {
    fn format_time(
        &self,
        w: &mut tracing_subscriber::fmt::format::Writer<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

pub fn setup_file_logging() -> impl Drop {
    // Create logs directory if it doesn't exist
    std::fs::create_dir_all("logs").unwrap();

    // File appender with daily rotation
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "bevy-app.log");

    // Create non blocking writer and get guard
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::Layer::new()
        .with_ansi(false)
        .with_writer(non_blocking)
        .with_timer(ChronoTime)
        .with_target(true)
        .with_level(true)
        .with_thread_names(true)
        .with_span_events(fmt::format::FmtSpan::ENTER);

    // Console layer
    let console_layer = fmt::Layer::new()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_target(true)
        .with_level(true)
        .with_thread_names(true)
        .with_span_events(fmt::format::FmtSpan::ENTER)
        .with_timer(ChronoTime);

    // Filter based on RUST_LOG environment variable
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| {
            EnvFilter::try_new("debug,wgpu=warn,bevy=debug,bevy_ecs=debug,bevy_render=debug")
        })
        .unwrap();

    // Combine layers
    let subscriber = tracing_subscriber::registry()
        .with(filter_layer)
        .with(console_layer)
        .with(file_layer);

    // Initialize global subscriber
    subscriber.init();

    // Return guard to keep the file writer alive
    guard
}
