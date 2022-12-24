
#[derive(Debug, serde::Deserialize, PartialEq, Default)]
pub enum IndiState {
    #[default]
    Idle, 
    Ok,
    Busy,
    Alert
}


#[derive(Debug, serde::Deserialize, PartialEq)]
pub enum IndiPermission {
    #[serde(alias = "ro")] RO,
    #[serde(alias = "rw")] RW,
    #[serde(alias = "wo")] WO,

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

