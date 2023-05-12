use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt::format::Format;
use tracing_subscriber::fmt::{self, time};

pub fn trace_init() {
    let filter = EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into());
    let formatter = Format::default()
        .with_timer(time::SystemTime::default())
        .with_thread_names(true);

    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(filter)
        .event_format(formatter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");
}
