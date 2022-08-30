

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Message {
    device: String,
    message: String,
    timestamp: String,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "📝 {} {:?}", self.device, self.message).unwrap();

        if !self.extra.is_empty() {
            write!(f, "{:?}\n", self.extra).unwrap();
        }

        return Ok(());
    }
}