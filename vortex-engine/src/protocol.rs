// -- leaked by @azixi0 on github
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3Data { pub x: f32, pub y: f32, pub z: f32 }

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuatData { pub x: f32, pub y: f32, pub z: f32, pub w: f32 }

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorData { pub r: f32, pub g: f32, pub b: f32, pub a: f32 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaceData { Front, Back, Top, Bottom, Left, Right }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialKindData { SmoothPlastic, Wood, Metal, Grass, Ice, Paint }

#[derive(Debug, Clone, PartialEq)]
pub enum TextureKindData { Studs, Inlets }

#[derive(Debug, Clone, PartialEq)]
pub struct TextureData {
    pub face: FaceData,
    pub kind: TextureKindData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartData {
    pub name: String,
    pub position: Vec3Data,
    pub rotation: QuatData,
    pub scale: Vec3Data,
    pub material: MaterialKindData,
    pub group: Option<u64>,
    pub cast_shadow: bool,
    pub anchored: bool,
    pub can_collide: bool,
    pub spawn_location: bool,
    pub baseplate: bool,
    pub custom_appearance: Option<String>,
    pub truss: bool,
    pub textures: Vec<TextureData>,
    pub point_light: Option<PointLightData>,
    pub spot_light: Option<SpotLightData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointLightData { pub color: ColorData, pub intensity: f32, pub range: f32 }

#[derive(Debug, Clone, PartialEq)]
pub struct SpotLightData {
    pub color: ColorData,
    pub intensity: f32,
    pub range: f32,
    pub angle: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LightingData {
    pub ambient_color: ColorData,
    pub brightness: f32,
    pub sun_color: ColorData,
    pub sun_illuminance: f32,
    pub sun_shadow_maps_enabled: bool,
    pub sun_rotation: QuatData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupData { pub parent_group: Option<u64> }

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectData {
    pub project_id: u64,
    pub parts: Vec<PartData>,
    pub lighting: LightingData,
    pub groups: Vec<GroupData>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassNameData {
    Workspace, Lighting, Part, Model, Folder, PointLight, SpotLight,
    LocalScript, Script, ModuleScript,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptData { pub source: String, pub enabled: bool }

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValueData {
    Bool(bool), F32(f32), Vec3(Vec3Data), Color(ColorData), Text(String), Enum(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InstanceData {
    pub id: u64,
    pub class_name: ClassNameData,
    pub parent: Option<u64>,
    pub part: Option<PartData>,
    pub script: Option<ScriptData>,
    pub attributes: BTreeMap<String, PropertyValueData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectDataV2 {
    pub project_id: u64,
    pub instances: Vec<InstanceData>,
    pub lighting: LightingData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerState {
    pub id: u64,
    pub game_id: u64,
    pub instance_id: String,
    pub username: String,
    pub role: String,
    pub yaw: f32,
    pub moving: bool,
    pub grounded: bool,
    pub animation_times: Vec<f32>,
    pub shirt_id: Option<u64>,
    pub pant_id: Option<u64>,
    pub body_type: String,
    pub body_colors: Vec<ColorData>,
    pub face_id: Option<u64>,
    pub dead: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerFrame { pub player_id: u64, pub position: Vec3Data, pub rotation: QuatData }

#[derive(Debug, Clone, PartialEq)]
pub struct MoveUpdate { pub had_input: bool, pub frame: PlayerFrame }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerInfo { pub id: u64, pub username: String, pub role: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage { pub sender_id: Option<u64>, pub text: String }

#[derive(Debug, Clone, PartialEq)]
pub enum Packet {
    State(PlayerState),
    States(Vec<PlayerState>),
    Chat(ChatMessage),
    ChatWarning(String),
    System(String),
    Kick(String),
    Kicked(String),
    Auth { token: String },
    Leave { player_id: u64 },
    Move(MoveUpdate),
    Frames(Vec<PlayerFrame>),
    Info(PlayerInfo),
    Sync,
    AuthAck,
    InstanceFull,
    Frozen(bool),
    TeleportTo { game_id: u64, instance_id: String, ticket: String },
    Kill { player_id: u64 },
}
