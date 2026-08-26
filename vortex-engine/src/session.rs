// -- leaked by @azixi0 on github
use std::collections::BTreeMap;
use std::fmt;

use crate::{DEFAULT_GAME_SERVER, DEVELOPMENT_GAME_SERVER};

pub const SESSION_FILE: &str = "Vortex/session.json";
pub const TOKEN_HEADER: &str = "X-App-Token";
pub const HARDWARE_HEADER: &str = "X-Hardware-Id";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchRequest { pub game_id: u64, pub instance_id: String, pub ticket: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedUser {
    pub app_token: String,
    pub username: String,
    pub user_id: u64,
    pub expired: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionProfile {
    pub friend_ids: Vec<u64>,
    pub following_ids: Vec<u64>,
    pub follower_ids: Vec<u64>,
    pub sent_request_ids: Vec<u64>,
    pub incoming_requests: Vec<u64>,
    pub body_colors: Vec<String>,
    pub game_name: String,
    pub game_creator: String,
    pub is_owner: bool,
    pub is_staff: bool,
    pub is_moderator: bool,
    pub is_booster: bool,
    pub client_token: String,
    pub shirt_id: Option<u64>,
    pub pant_id: Option<u64>,
    pub body_type: String,
    pub face_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaunchParseError { WrongScheme, Missing(&'static str), InvalidGameId, BadEscape }

impl fmt::Display for LaunchParseError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongScheme => write!(output, "malformed vortex URL scheme"),
            Self::Missing(field) => write!(output, "missing launch parameter {field}"),
            Self::InvalidGameId => write!(output, "game_id is not an unsigned integer"),
            Self::BadEscape => write!(output, "invalid percent escape"),
        }
    }
}

impl LaunchRequest {
    pub fn parse(url: &str) -> Result<Self, LaunchParseError> {
        let rest = url.strip_prefix("vortex://").ok_or(LaunchParseError::WrongScheme)?;
        let query = rest.split_once('?').map(|(_, query)| query).unwrap_or(rest);
        let mut values = BTreeMap::new();
        for pair in query.split('&').filter(|part| !part.is_empty()) {
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            values.insert(percent_decode(key)?, percent_decode(value)?);
        }
        let game_id = values.get("game_id").ok_or(LaunchParseError::Missing("game_id"))?
            .parse().map_err(|_| LaunchParseError::InvalidGameId)?;
        let instance_id = values.remove("instance_id").ok_or(LaunchParseError::Missing("instance_id"))?;
        let ticket = values.remove("ticket").ok_or(LaunchParseError::Missing("ticket"))?;
        Ok(Self { game_id, instance_id, ticket })
    }

    pub fn queue_join_path(&self) -> String {
        format!("/api/queue/join?game_id={}&instance_id={}&ticket={}", self.game_id, self.instance_id, self.ticket)
    }

    pub fn queue_status_path(&self) -> String {
        format!("/api/queue/status?game_id={}&instance_id={}&ticket={}", self.game_id, self.instance_id, self.ticket)
    }
}

fn percent_decode(value: &str) -> Result<String, LaunchParseError> {
    let bytes = value.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => { output.push(b' '); index += 1; }
            b'%' if index + 2 < bytes.len() => {
                let high = hex(bytes[index + 1]).ok_or(LaunchParseError::BadEscape)?;
                let low = hex(bytes[index + 2]).ok_or(LaunchParseError::BadEscape)?;
                output.push(high * 16 + low); index += 3;
            }
            b'%' => return Err(LaunchParseError::BadEscape),
            byte => { output.push(byte); index += 1; }
        }
    }
    String::from_utf8(output).map_err(|_| LaunchParseError::BadEscape)
}

fn hex(byte: u8) -> Option<u8> {
    match byte { b'0'..=b'9' => Some(byte-b'0'), b'a'..=b'f' => Some(byte-b'a'+10), b'A'..=b'F' => Some(byte-b'A'+10), _ => None }
}

pub fn game_server(development: bool) -> &'static str {
    if development { DEVELOPMENT_GAME_SERVER } else { DEFAULT_GAME_SERVER }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_launch_url() {
        let request = LaunchRequest::parse("vortex://launch?game_id=42&instance_id=abc%2D1&ticket=t%2B1").unwrap();
        assert_eq!(request.game_id, 42);
        assert_eq!(request.instance_id, "abc-1");
        assert_eq!(request.ticket, "t+1");
    }
}
