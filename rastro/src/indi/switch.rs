use super::common::{IndiState, IndiPermission};


#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum IndiSwitchOptions {
    AnyOfMany, OneOfMany, AtMostOne
}


#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum IndiSwitch {
    On,
    Off,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct DefSwitchValue {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@label")]
    pub label: String,

    #[serde(rename = "$text")]
    pub value: IndiSwitch,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct DefSwitchVector {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@label")]
    pub label: String,
    #[serde(rename = "@group")]
    pub group: String,
    #[serde(rename = "@state")]
    pub state: IndiState,
    #[serde(rename = "@rule")]
    pub rule: IndiSwitchOptions,
    #[serde(rename = "@device")]
    pub device: String,
    #[serde(rename = "@perm")]
    pub perm: IndiPermission,
    #[serde(rename = "@timeout")]
    pub timeout: f64,
    #[serde(rename = "@timestamp")]
    pub timestamp: String,

    #[serde(rename = "defSwitch")]
    pub switches: Vec<DefSwitchValue>,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}    

impl std::fmt::Display for DefSwitchVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}::{}(switch)\n", self.state, self.device, self.name).unwrap();
        for switch in &self.switches {
            write!(f, "\t{}\n", switch).unwrap();
        }
        if !self.extra.is_empty() {
            write!(f, "{:?}\n", self.extra).unwrap();
        }

        return std::fmt::Result::Ok(());
    }
}

impl std::fmt::Display for DefSwitchValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value == IndiSwitch::On {
            write!(f, "{} *", self.name)
        }
        else {
            write!(f, "{}", self.name)
        }
    }
}


#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct SetSwitchValue {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "$text")]
    pub value: String,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct SetSwitchVector {
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

    #[serde(rename = "oneSwitch")]
    pub switches: Vec<SetSwitchValue>,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}



impl std::fmt::Display for SetSwitchVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}::{}(text)\n", self.state, self.device, self.name).unwrap();
        for v in &self.switches {
            write!(f, "\t{}\n", v).unwrap();
        }
        if !self.extra.is_empty() {
            write!(f, "{:?}\n", self.extra).unwrap();
        }

        return Ok(());
    }
}

impl std::fmt::Display for SetSwitchValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}
