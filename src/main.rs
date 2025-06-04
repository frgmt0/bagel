use anyhow::Result;

mod browser;
mod extensions;
mod search;
mod storage;
mod ui;
mod updates;
mod utils;

use utils::{init_logger, BrowserConfig};

fn main() -> Result<()> {
    init_logger();

    let config = BrowserConfig::load()?;
    config.ensure_directories()?;

    ui::run_app(config).map_err(|e| anyhow::anyhow!("Failed to run app: {}", e))?;

    Ok(())
}
