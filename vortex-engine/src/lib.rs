// -- leaked by @azixi0 on github
//! Behavioral reconstruction of the proprietary `vortex_engine` crate.

pub mod avatar;
pub mod chat;
pub mod health;
pub mod instance;
pub mod map;
pub mod music;
pub mod network;
pub mod physics;
pub mod protocol;
pub mod ragdoll;
pub mod scripting;
pub mod session;
pub mod settings;
pub mod sfx;

pub const PRODUCT_NAME: &str = "Vortex";
pub const RECOVERED_VERSION: &str = "0.4.2";
pub const WEBSITE: &str = "https://playvortex.io";
pub const DEFAULT_GAME_SERVER: &str = "connect.playvortex.io:7777";
pub const DEVELOPMENT_GAME_SERVER: &str = "127.0.0.1:7777";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    Home,
    Joining,
    LoadingMap,
    LoadingPlayerInfo,
    Playing,
    Frozen,
    Kicked,
    ConnectionFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub phase: GamePhase,
    pub detail: String,
}

impl Default for GameState {
    fn default() -> Self {
        Self { phase: GamePhase::Home, detail: String::new() }
    }
}

impl GameState {
    pub fn transition(&mut self, phase: GamePhase, detail: impl Into<String>) {
        self.phase = phase;
        self.detail = detail.into();
    }
}
