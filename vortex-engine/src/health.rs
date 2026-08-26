// -- leaked by @azixi0 on github
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Health { pub current: f32, pub maximum: f32 }

impl Health {
    pub fn new(maximum: f32) -> Self { let maximum = maximum.max(0.0); Self { current: maximum, maximum } }
    pub fn damage(&mut self, amount: f32) { self.current = (self.current - amount.max(0.0)).max(0.0); }
    pub fn heal(&mut self, amount: f32) { self.current = (self.current + amount.max(0.0)).min(self.maximum); }
    pub fn dead(self) -> bool { self.current <= 0.0 }
}

