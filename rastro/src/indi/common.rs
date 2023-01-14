
#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Default)]
pub enum IndiState {
    #[default]
    Idle, 
    Ok,
    Busy,
    Alert
}


#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub enum IndiPermission {
    #[serde(rename = "ro")] RO,
    #[serde(rename = "rw")] RW,
    #[serde(rename = "wo")] WO,
}



impl std::fmt::Display for IndiState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IndiState::Idle => "⚪",
                IndiState::Ok => "🟢",
                IndiState::Busy => "🟡",
                IndiState::Alert => "🔴"
            }
        )
    }
}

