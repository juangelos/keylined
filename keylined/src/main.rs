//! Keylined - Fast command execution tool
//!
//! This is the main binary crate that ties together all the components.

use keylined_config::Config;
use keylined_core::error::Result;
use keylined_ui::app::App;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting Keylined...");

    // Load configuration
    let config = Config::load()?;

    // Start UI
    let app = App::new(config);
    app.run().await?;

    Ok(())
}
