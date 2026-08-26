// -- leaked by @azixi0 on github
use std::collections::BTreeMap;

use crate::protocol::{Packet, PlayerFrame, PlayerInfo, PlayerState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Authenticating,
    Synchronizing,
    Active,
    Frozen,
    Kicked(String),
    InstanceFull,
}

#[derive(Debug)]
pub struct NetworkSession {
    pub state: ConnectionState,
    pub local_player: Option<PlayerInfo>,
    pub players: BTreeMap<u64, PlayerState>,
    pub frames: BTreeMap<u64, PlayerFrame>,
    pub system_messages: Vec<String>,
}

impl Default for NetworkSession {
    fn default() -> Self {
        Self {
            state: ConnectionState::Disconnected,
            local_player: None,
            players: BTreeMap::new(),
            frames: BTreeMap::new(),
            system_messages: Vec::new(),
        }
    }
}

impl NetworkSession {
    pub fn begin_authentication(&mut self) { self.state = ConnectionState::Authenticating; }

    pub fn apply(&mut self, packet: Packet) {
        match packet {
            Packet::State(player) => { self.players.insert(player.id, player); }
            Packet::States(players) => {
                self.players = players.into_iter().map(|player| (player.id, player)).collect();
            }
            Packet::Frames(frames) => {
                self.frames.extend(frames.into_iter().map(|frame| (frame.player_id, frame)));
            }
            Packet::Info(info) => { self.local_player = Some(info); }
            Packet::Sync => { self.state = ConnectionState::Synchronizing; }
            Packet::AuthAck => { self.state = ConnectionState::Active; }
            Packet::Frozen(true) => { self.state = ConnectionState::Frozen; }
            Packet::Frozen(false) => { self.state = ConnectionState::Active; }
            Packet::Kick(reason) | Packet::Kicked(reason) => { self.state = ConnectionState::Kicked(reason); }
            Packet::InstanceFull => { self.state = ConnectionState::InstanceFull; }
            Packet::Leave { player_id } => {
                self.players.remove(&player_id);
                self.frames.remove(&player_id);
            }
            Packet::System(message) | Packet::ChatWarning(message) => self.system_messages.push(message),
            Packet::Kill { player_id } => {
                if let Some(player) = self.players.get_mut(&player_id) { player.dead = true; }
            }
            Packet::Chat(_) | Packet::Auth { .. } | Packet::Move(_) | Packet::TeleportTo { .. } => {}
        }
    }
}
