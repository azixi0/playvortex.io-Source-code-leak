// -- leaked by @azixi0 on github
use crate::protocol::ColorData;

#[derive(Debug, Clone, PartialEq)]
pub struct AvatarDescriptor {
    pub shirt_id: Option<u64>, pub pant_id: Option<u64>, pub face_id: Option<u64>,
    pub body_type: String, pub body_colors: Vec<ColorData>,
}

impl AvatarDescriptor {
    pub fn outfit_query(ids: &[u64]) -> String { format!("/api/users/outfits?ids={}", join_ids(ids)) }
    pub fn clothing_query(ids: &[u64]) -> String { format!("/api/clothing/images?ids={}", join_ids(ids)) }
    pub fn mesh_query(ids: &[u64]) -> String { format!("/api/meshes?ids={}", join_ids(ids)) }
}

fn join_ids(ids: &[u64]) -> String { ids.iter().map(u64::to_string).collect::<Vec<_>>().join(",") }

