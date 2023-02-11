use super::common::{IndiState, IndiPermission};


#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct DefBlobValue {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@label")]
    pub label: String,

    #[serde(rename = "$text")]
    pub value: String,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct DefBlobVector {
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

    #[serde(rename = "defBLOB", default)]
    pub blobs: Vec<DefBlobValue>,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

impl std::fmt::Display for DefBlobVector {
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

impl std::fmt::Display for DefBlobValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}


#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct SetBlobValue {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@size")]
    pub size: usize,

    #[serde(rename = "@len")]
    pub len: usize,

    #[serde(rename = "@format")]
    pub format: String,

    #[serde(rename = "$text")]
    pub value: String,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct SetBlobVector {
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

    #[serde(rename = "oneBLOB")]
    pub blobs: Vec<SetBlobValue>,

    #[serde(flatten)]
    extra: std::collections::HashMap<String, String>,
}



impl std::fmt::Display for SetBlobVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}::{}(blob)\n", self.state, self.device, self.name).unwrap();
        for v in &self.blobs {
            write!(f, "\tformat: {}, len: {}\n", v.format, v.len).unwrap();
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

