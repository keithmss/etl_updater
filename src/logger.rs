extern crate tracing;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[cfg(debug_assertions)]
pub fn init() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default log subscriber failed");
}

#[cfg(not(debug_assertions))]
pub fn init() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default log subscriber failed");
}
