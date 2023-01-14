use super::common::{IndiState};


#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct DefLightValue {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@label")]
    pub label: String,

    #[serde(rename = "$text")]
    pub value: IndiState,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct DefLightVector {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@label")]
    pub label: String,
    #[serde(rename = "@group")]
    pub group: String,
    #[serde(rename = "@state")]
    pub state: IndiState,
    #[serde(rename = "@device")]
    pub device: String,
    #[serde(rename = "@timestamp")]
    pub timestamp: String,

    #[serde(rename = "defLight")]
    pub lights: Vec<DefLightValue>,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}    

impl std::fmt::Display for DefLightVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}::{}(Light)\n", self.state, self.device, self.name).unwrap();
        for v in &self.lights {
            write!(f, "\t{}\n", v).unwrap();
        }
        if !self.extra.is_empty() {
            write!(f, "{:?}\n", self.extra).unwrap();
        }

        return std::fmt::Result::Ok(());
    }
}

impl std::fmt::Display for DefLightValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.name)
    }
}