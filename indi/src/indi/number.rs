use super::common::{IndiState, IndiPermission};


#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct DefNumberValue {
    pub name: String,
    pub label: String,

    #[serde(alias = "$value")]
    pub value: f64,
    pub format: String,
    pub min: f64,
    pub max: f64,
    pub step: f64,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct DefNumberVector {
    pub name: String,
    pub label: String,
    pub group: String,
    pub state: IndiState,
    pub device: String,
    pub perm: IndiPermission,
    pub timeout: f64,
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


#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct SetNumberValue {
    pub name: String,

    #[serde(alias = "$value", default)]
    pub value: String,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct SetNumberVector {
    pub name: String,
    pub state: IndiState,
    pub device: String,
    pub timeout: f64,
    pub timestamp: String,

    #[serde(rename = "oneNumber")]
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
