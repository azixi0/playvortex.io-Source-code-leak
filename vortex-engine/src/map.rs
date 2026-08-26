// -- leaked by @azixi0 on github
use crate::protocol::ProjectDataV2;

pub const EXTENSION: &str = "vrtx";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapLoadError {
    Empty,
    UnsupportedVersion(u32),
    Truncated,
    Decode(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapEnvelope<'a> { pub version: u32, pub payload: &'a [u8] }

impl<'a> MapEnvelope<'a> {
    /// Recovers the observed version-gated envelope. The inner serializer was stripped.
    pub fn parse(bytes: &'a [u8]) -> Result<Self, MapLoadError> {
        if bytes.is_empty() { return Err(MapLoadError::Empty); }
        if bytes.len() < 4 { return Err(MapLoadError::Truncated); }
        let version = u32::from_le_bytes(bytes[..4].try_into().expect("four-byte slice"));
        if version != 2 { return Err(MapLoadError::UnsupportedVersion(version)); }
        Ok(Self { version, payload: &bytes[4..] })
    }
}

#[derive(Debug, Default)]
pub struct MapWorld { pub project: Option<ProjectDataV2>, pub collider_count: usize }

impl MapWorld {
    pub fn install(&mut self, project: ProjectDataV2) {
        self.collider_count = project.instances.iter()
            .filter(|item| item.part.as_ref().is_some_and(|part| part.can_collide))
            .count();
        self.project = Some(project);
    }
}

