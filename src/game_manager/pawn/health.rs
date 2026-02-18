use crate::prelude::*;

#[derive(Component, Reflect, Debug)]
pub struct Health {
    current: f32,
    max: f32,
}
impl Health {
    pub fn _new(current: f32, max: f32) -> Self {
        Self { current, max }
    }
    pub fn is_alive(&self) -> bool {
        self.current > 0.
    }
    pub fn take_damage(&mut self, amount: f32) {
        self.current = self.current - amount;
    }
    pub fn _heal(&mut self, amount: f32) {
        self.current = self.current + amount;
    }
    pub fn new_full(max: f32) -> Self {
        Self { current: max, max }
    }
    pub fn get_current(&self) -> f32 {
        self.current
    }
    pub fn get_max(&self) -> f32 {
        self.max
    }
}
pub(crate) fn plugin(_app: &mut App) {}
