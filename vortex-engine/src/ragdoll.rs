// -- leaked by @azixi0 on github
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RagdollState {
    Animated,
    Entering,
    Simulated,
    Recovering,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ragdoll {
    pub state: RagdollState,
    pub elapsed: f32,
}

impl Default for Ragdoll {
    fn default() -> Self {
        Self { state: RagdollState::Animated, elapsed: 0.0 }
    }
}

impl Ragdoll {
    pub fn transition(&mut self, state: RagdollState) {
        self.state = state;
        self.elapsed = 0.0;
    }

    pub fn tick(&mut self, delta_seconds: f32) {
        self.elapsed += delta_seconds.max(0.0);
    }
}

