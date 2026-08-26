// -- leaked by @azixi0 on github
#[derive(Debug, Clone, PartialEq)]
pub struct PersistedSettings {
    pub sensitivity: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub shiftlock_enabled: bool,
    pub discord_rpc: bool,
    pub graphics_quality: u8,
    pub fps_cap: u16,
}

impl Default for PersistedSettings {
    fn default() -> Self {
        Self { sensitivity: 1.0, music_volume: 0.7, sfx_volume: 0.8, shiftlock_enabled: false, discord_rpc: true, graphics_quality: 2, fps_cap: 60 }
    }
}

impl PersistedSettings {
    pub fn normalize(&mut self) {
        self.sensitivity = self.sensitivity.clamp(0.05, 10.0);
        self.music_volume = self.music_volume.clamp(0.0, 1.0);
        self.sfx_volume = self.sfx_volume.clamp(0.0, 1.0);
        self.graphics_quality = self.graphics_quality.min(3);
        self.fps_cap = self.fps_cap.clamp(30, 1000);
    }
}

