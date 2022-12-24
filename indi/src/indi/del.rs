#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct DelProperty {
    pub device: String,

    pub name: Option<String>,

    pub timestamp: Option<String>,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}



impl std::fmt::Display for DelProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(name) => {
                write!(f, "⛔ {}::{} (vector)\n", self.device, name).unwrap();
            }

            None => {
                write!(f, "⛔ {} (device)\n", self.device).unwrap();
            }
        }


        if !self.extra.is_empty() {
            write!(f, "{:?}\n", self.extra).unwrap();
        }

        return Ok(());
    }
}
