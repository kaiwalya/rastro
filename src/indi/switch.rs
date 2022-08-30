use super::common::{IndiState, IndiPermission};


#[derive(Debug, serde::Deserialize, PartialEq)]
pub enum IndiSwitchOptions {
    AnyOfMany, OneOfMany, AtMostOne
}


#[derive(Debug, serde::Deserialize, PartialEq)]
pub enum IndiSwitch {
    On,
    Off,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct DefSwitchValue {
    pub name: String,
    pub label: String,

    #[serde(alias = "$value")]
    pub value: IndiSwitch,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct DefSwitchVector {
    pub name: String,
    pub label: String,
    pub group: String,
    pub state: IndiState,
    pub rule: IndiSwitchOptions,
    pub device: String,
    pub perm: IndiPermission,
    pub timeout: f64,
    pub timestamp: String,

    #[serde(rename = "defSwitch")]
    pub switches: Vec<DefSwitchValue>,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}    

impl std::fmt::Display for DefSwitchVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}::{}(switch)\n", self.state, self.device, self.label).unwrap();
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
            write!(f, "{} *", self.label)
        }
        else {
            write!(f, "{}", self.label)
        }
    }
}


#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct SetSwitchValue {
    pub name: String,

    #[serde(alias = "$value", default)]
    pub value: String,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct SetSwitchVector {
    pub name: String,
    pub state: IndiState,
    pub device: String,
    pub timeout: f64,
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
