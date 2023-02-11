use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ConnectionProtocol {
    #[serde(rename = "indi")]
    InstrumentNeutralDistributedInterface
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionSpec {
    pub name: String,
    pub protocol: ConnectionProtocol,
    pub host: String,
    pub port: usize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub connections: Vec<ConnectionSpec>
}

impl ConfigFile {
    pub fn load_default() -> Result<ConfigFile, Box<dyn Error>> {
        toml::from_str(r###"
            #[[connections]]
            #name = "mobile-mini"
            #protocol = "indi"
            #host = "mobile-mini.local"
            #port = 7624

            [[connections]]
            name = "local"
            protocol = "indi"
            host = "localhost"
            port = 7624
        "###).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::config_file::ConfigFile;

    #[test]
    fn it_loads_default_config_file() -> Result<(), Box<dyn Error>> {
        ConfigFile::load_default().map(|_|())
    }
}