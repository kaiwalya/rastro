use super::common::{IndiState, IndiPermission};


#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct DefNumberValue {

    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@label")]
    pub label: String,
    #[serde(rename = "$text")]
    pub value: f64,
    #[serde(rename = "@format")]
    pub format: String,
    #[serde(rename = "@min")]
    pub min: f64,
    #[serde(rename = "@max")]
    pub max: f64,
    #[serde(rename = "@step")]
    pub step: f64,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct DefNumberVector {
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

    #[serde(rename = "defNumber")]
    pub numbers: Vec<DefNumberValue>,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}    

impl std::fmt::Display for DefNumberVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}::{}(numbers)\n", self.state, self.device, self.name).unwrap();
        for v in &self.numbers {
            write!(f, "\t{}\n", v).unwrap();
        }
        if !self.extra.is_empty() {
            write!(f, "{:?}\n", self.extra).unwrap();
        }

        return std::fmt::Result::Ok(());
    }
}

impl std::fmt::Display for DefNumberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {} | [{}, {}], Δ{}, {}", self.name, self.value, self.min, self.max, self.step, self.format)
    }
}


#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct SetNumberValue {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "$text")]
    pub value: String,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct SetNumberVector {
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

    #[serde(rename = "oneNumber", default)]
    pub numbers: Vec<SetNumberValue>,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}



impl std::fmt::Display for SetNumberVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}::{}(text)\n", self.state, self.device, self.name).unwrap();
        for v in &self.numbers {
            write!(f, "\t{}\n", v).unwrap();
        }
        if !self.extra.is_empty() {
            write!(f, "{:?}\n", self.extra).unwrap();
        }

        return Ok(());
    }
}

impl std::fmt::Display for SetNumberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}
