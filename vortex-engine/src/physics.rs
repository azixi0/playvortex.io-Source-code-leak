// -- leaked by @azixi0 on github
use crate::protocol::Vec3Data;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Humanoid {
    pub walk_speed: f32,
    pub jump_speed: f32,
    pub grounded: bool,
    pub velocity: Vec3Data,
}

impl Default for Humanoid {
    fn default() -> Self {
        Self {
            walk_speed: 16.0,
            jump_speed: 32.0,
            grounded: false,
            velocity: Vec3Data { x: 0.0, y: 0.0, z: 0.0 },
        }
    }
}

impl Humanoid {
    pub fn apply_input(&mut self, x: f32, z: f32, jump: bool) {
        let length = (x * x + z * z).sqrt().max(1.0);
        self.velocity.x = x / length * self.walk_speed;
        self.velocity.z = z / length * self.walk_speed;
        if jump && self.grounded {
            self.velocity.y = self.jump_speed;
            self.grounded = false;
        }
    }
}

