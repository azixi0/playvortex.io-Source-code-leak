// -- leaked by @azixi0 on github
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Track { pub title: String, pub source: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MusicState { Idle, Fetching { attempt: u8 }, Playing(Track), Failed { attempts: u8 } }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MusicController { pub state: MusicState, pub max_attempts: u8 }

impl Default for MusicController {
    fn default() -> Self { Self { state: MusicState::Idle, max_attempts: 3 } }
}

impl MusicController {
    pub fn begin_fetch(&mut self) { self.state = MusicState::Fetching { attempt: 1 }; }
    pub fn fail_fetch(&mut self) -> bool {
        let attempt = match self.state { MusicState::Fetching { attempt } => attempt, _ => 1 };
        if attempt >= self.max_attempts { self.state = MusicState::Failed { attempts: attempt }; false }
        else { self.state = MusicState::Fetching { attempt: attempt + 1 }; true }
    }
    pub fn play(&mut self, track: Track) { self.state = MusicState::Playing(track); }
}

