use super::common::{IndiState, IndiPermission};


#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct DefBlobValue {
    pub name: String,
    pub label: String,

    #[serde(alias = "$value", default)]
    pub value: String,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct DefBlobVector {
    pub name: String,
    pub label: String,
    pub group: String,
    pub state: IndiState,
    pub device: String,
    pub perm: IndiPermission,
    pub timeout: f64,
    pub timestamp: String,

    #[serde(rename = "defBLOB")]
    pub blobs: Vec<DefBlobValue>,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

impl std::fmt::Display for DefBlobVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}::{}(blob)\n", self.state, self.device, self.label).unwrap();
        for v in &self.blobs {
            write!(f, "\t{}\n", v).unwrap();
        }
        if !self.extra.is_empty() {
            write!(f, "{:?}\n", self.extra).unwrap();
        }

        return Ok(());
    }
}

impl std::fmt::Display for DefBlobValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.label, self.value)
    }
}


#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct SetBlobValue {
    pub name: String,

    pub size: usize,

    pub enclen: usize,

    pub format: String,

    #[serde(alias = "$value", default)]
    pub value: String,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct SetBlobVector {
    pub name: String,
    pub state: IndiState,
    pub device: String,
    pub timeout: f64,
    pub timestamp: String,

    #[serde(rename = "oneBLOB")]
    pub blobs: Vec<SetBlobValue>,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}



impl std::fmt::Display for SetBlobVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}::{}(blob)\n", self.state, self.device, self.name).unwrap();
        for v in &self.blobs {
            write!(f, "\t{}\n", v).unwrap();
        }
        if !self.extra.is_empty() {
            write!(f, "{:?}\n", self.extra).unwrap();
        }

        return Ok(());
    }
}

impl std::fmt::Display for SetBlobValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {} bytes = ", self.name, self.value.len()).unwrap();
        write!(f, "{}", self.value)
    }
}

