
#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct GetProperties {
    #[serde(rename = "@version")]
    pub version: String
}

