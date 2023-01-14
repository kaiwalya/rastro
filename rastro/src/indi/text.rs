use super::common::{IndiState, IndiPermission};


#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct DefTextValue {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@label")]
    pub label: String,

    #[serde(alias = "$text")]
    pub value: String,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct DefTextVector {
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
    #[serde(rename = "@perm")]
    pub perm: IndiPermission,
    #[serde(rename = "@timeout")]
    pub timeout: f64,
    #[serde(rename = "@timestamp")]
    pub timestamp: String,

    #[serde(rename = "defText", default)]
    pub texts: Vec<DefTextValue>,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

impl std::fmt::Display for DefTextVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}::{}(text)\n", self.state, self.device, self.name).unwrap();
        for v in &self.texts {
            write!(f, "\t{}\n", v).unwrap();
        }
        if !self.extra.is_empty() {
            write!(f, "{:?}\n", self.extra).unwrap();
        }

        return Ok(());
    }
}

impl std::fmt::Display for DefTextValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}


#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct SetTextValue {
    #[serde(alias = "@name")]
    pub name: String,

    #[serde(rename = "$text")]
    pub value: String,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct SetTextVector {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@state")]
    pub state: IndiState,
    #[serde(rename = "@device")]
    pub device: String,
    #[serde(rename = "@timeout")]
    pub timeout: f64,
    #[serde(rename = "@timestamp")]
    pub timestamp: String,

    #[serde(rename = "oneText", default)]
    pub texts: Vec<SetTextValue>,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}



impl std::fmt::Display for SetTextVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}::{}(text)\n", self.state, self.device, self.name).unwrap();
        for v in &self.texts {
            write!(f, "\t{}\n", v).unwrap();
        }
        if !self.extra.is_empty() {
            write!(f, "{:?}\n", self.extra).unwrap();
        }

        return Ok(());
    }
}

impl std::fmt::Display for SetTextValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}

