use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    // cover database connection
    pub db_uri: Url,
    pub db_pass_path: PathBuf,
    // port on which the cover API will listen for incoming connections
    pub listen_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_uri: Url::parse("postgresql:://user@127.0.0.1:5432/mydb").unwrap(),
            db_pass_path: PathBuf::from("name_api/db/user"),
            listen_port: 10200,
        }
    }
}
