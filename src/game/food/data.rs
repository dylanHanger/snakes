use bevy::{
    prelude::{Component, Deref, DerefMut, Entity, Resource},
    utils::HashSet,
};

#[derive(Component)]
pub struct Food {
    initial_lifetime: u32,
    pub lifetime: u32,
}
impl Food {
    pub fn new(lifetime: u32) -> Self {
        Self {
            initial_lifetime: lifetime,
            lifetime,
        }
    }

    pub fn get_factor(&self) -> f32 {
        (self.lifetime as f32 / self.initial_lifetime as f32) * 2. - 1.
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
