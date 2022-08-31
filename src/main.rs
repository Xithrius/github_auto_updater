use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use tracing::{event, Level};

#[derive(Serialize, Deserialize)]
struct InstalledAsset {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
struct InstalledAssets {
    assets: Vec<InstalledAsset>,
}

fn get_assets() -> Result<()> {
    let contents = fs::read_to_string("installed_assets.json").expect("Couldn't read the file");

    let installed_assets: InstalledAssets = serde_json::from_str(&contents)?;

    for asset in &installed_assets.assets {
        event!(
            Level::DEBUG,
            "Found the asset {:} ({:})",
            asset.name,
            asset.version
        )
    }

    Ok(())
}

fn main() {
    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().pretty())
        .with_max_level(Level::DEBUG)
        .init();

    get_assets().expect("Foo");
}
