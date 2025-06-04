use log::{error, info, warn};
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_logger() {
    INIT.call_once(|| {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
        
        info!("Bagel Browser logger initialized");
    });
}

pub fn log_error(context: &str, error: &dyn std::error::Error) {
    error!("{}: {}", context, error);
    
    let mut source = error.source();
    while let Some(err) = source {
        error!("  Caused by: {}", err);
        source = err.source();
    }
}

pub fn log_navigation(url: &str) {
    info!("Navigation: {}", url);
}

pub fn log_security_event(event: &str, details: &str) {
    warn!("Security Event - {}: {}", event, details);
}

pub fn log_performance(operation: &str, duration_ms: u64) {
    info!("Performance - {}: {}ms", operation, duration_ms);
}