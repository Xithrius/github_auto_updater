use log::{info};
use std::fs;

use env_logger::Env;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct InstalledAsset {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
struct InstalledAssets {
    assets: Vec<InstalledAsset>
}

fn get_assets() -> Result<()> {
    let contents = fs::read_to_string("installed_assets.json")
        .expect("Couldn't read the file");
    
    let installed_assets: InstalledAssets = serde_json::from_str(&contents)?;

    for asset in &installed_assets.assets {
        info!("Found the asset {:} ({:})", asset.name, asset.version)
    }

    Ok(())
}

fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let _assets = get_assets()
        .expect("Foo");
}