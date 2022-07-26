use bevy::prelude::Component;

#[derive(Component)]
pub struct Food {
    initial_value: f32,
    pub value: f32,
}
impl Food {
    pub fn new(last_for_turns: u32) -> Self {
        let initial_value = (10 * last_for_turns) as f32;
        Self {
            initial_value,
            value: initial_value,
        }
    }

    pub fn get_factor(&self) -> f32 {
        (self.value / self.initial_value) * 2. - 1.
    }
}

#[derive(Component)]
pub struct Rottable;
