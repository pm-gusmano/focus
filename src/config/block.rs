use serde::Serialize;

// Top level struct to hold the TOML data.
#[derive(Serialize)]
pub struct Config {
    pub website_list_path: String,
}
