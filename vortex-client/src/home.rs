// -- leaked by @azixi0 on github
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomeState {
    LoadingGames,
    Ready,
    Joining,
    ConnectionFailed { detail: String, attempt: u32 },
    Kicked { detail: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameSummary {
    pub id: u64,
    pub name: String,
    pub visits: u64,
    pub active: u32,
    pub player_count: u32,
}

impl HomeState {
    pub fn retry(&mut self) {
        if let Self::ConnectionFailed { detail, attempt } = self {
            *attempt += 1;
            detail.clear();
        }
    }
}
