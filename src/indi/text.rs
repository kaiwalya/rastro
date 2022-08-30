use super::common::{IndiState, IndiPermission};


#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct DefTextValue {
    pub name: String,
    pub label: String,

    #[serde(alias = "$value", default)]
    pub value: String,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct DefTextVector {
    pub name: String,
    pub label: String,
    pub group: String,
    pub state: IndiState,
    pub device: String,
    pub perm: IndiPermission,
    pub timeout: f64,
    pub timestamp: String,

    #[serde(rename = "defText")]
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


#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct SetTextValue {
    pub name: String,

    #[serde(alias = "$value", default)]
    pub value: String,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct SetTextVector {
    pub name: String,
    pub state: IndiState,
    pub device: String,
    pub timeout: f64,
    pub timestamp: String,

    #[serde(rename = "oneText")]
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

