use bevy::prelude::Component;

#[derive(Component)]
pub struct Food {
    initial_value: f32,
    pub value: f32,
}
impl Food {
    pub fn new(last_for_turns: u32) -> Self {
        let initial_value = last_for_turns as f32 / 10.0;
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

pub struct FoodConfig {
    pub last_for_turns: u32, // The number of turns the food will last for
    pub growth_amount: i32,  // Amount of growth gained for eating fresh food
}
