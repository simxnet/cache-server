use tracing::Level;
use tracing_subscriber;

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
}
