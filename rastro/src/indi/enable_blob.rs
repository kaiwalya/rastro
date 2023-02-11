
#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum EnableBLOBValue {
    #[serde(rename = "None")] None,
    #[serde(rename = "Only")] Only,
    #[serde(rename = "Also")] Also,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct EnableBLOB {
    #[serde(rename = "$text")]
    pub value: EnableBLOBValue
}

