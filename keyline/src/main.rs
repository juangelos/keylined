//! Keyline - Fast command execution tool
//! 
//! This is the main binary crate that ties together all the components.

use keyline_core::error::Result;
use keyline_config::Config;
use keyline_ui::app::App;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting Keyline...");

    // Load configuration
    let config = Config::load()?;

    // Start UI
    let app = App::new(config);
    app.run().await?;

    Ok(())
}
