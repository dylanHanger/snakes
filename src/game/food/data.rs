use bevy::{
    prelude::{Component, Deref, DerefMut, Entity, Resource},
    utils::HashSet,
};

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

#[derive(Deref, DerefMut, Resource)]
pub struct DespawnedFoods(HashSet<Entity>);
impl DespawnedFoods {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
}
impl Default for DespawnedFoods {
    fn default() -> Self {
        Self::new()
    }
}
