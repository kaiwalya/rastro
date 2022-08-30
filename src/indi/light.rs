use super::common::{IndiState};


#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct DefLightValue {
    pub name: String,
    pub label: String,

    #[serde(alias = "$value", default)]
    pub value: IndiState,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct DefLightVector {
    pub name: String,
    pub label: String,
    pub group: String,
    pub state: IndiState,
    pub device: String,
    pub timestamp: String,

    #[serde(rename = "defLight")]
    pub lights: Vec<DefLightValue>,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}    

impl std::fmt::Display for DefLightVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}::{}(Light)\n", self.state, self.device, self.label).unwrap();
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
        write!(f, "{} {}", self.value, self.label)
    }
}