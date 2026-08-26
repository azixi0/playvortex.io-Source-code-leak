// -- leaked by @azixi0 on github
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct SoundEffect {
    pub source: String,
    pub volume: f32,
    pub spatial: bool,
}

#[derive(Debug, Default)]
pub struct SfxController {
    queue: VecDeque<SoundEffect>,
}

impl SfxController {
    pub fn enqueue(&mut self, mut effect: SoundEffect) {
        effect.volume = effect.volume.clamp(0.0, 1.0);
        self.queue.push_back(effect);
    }

    pub fn next(&mut self) -> Option<SoundEffect> {
        self.queue.pop_front()
    }
}

