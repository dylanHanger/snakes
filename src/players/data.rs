use bevy::{
    prelude::{Color, Component, Deref, DerefMut},
    utils::HashMap,
};

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player {
    pub id: u32,
}

pub struct PlayerColors {
    colors: HashMap<u32, Color>,
}
impl PlayerColors {
    pub fn new<I>(colors: I) -> Self
    where
        I: IntoIterator<Item = Color>,
    {
        Self {
            colors: colors
                .into_iter()
                .enumerate()
                .map(|(i, c)| (i as u32, c))
                .collect(),
        }
    }
    pub fn get(&self, player: &Player) -> Option<Color> {
        self.colors
            .get(&(player.id % self.colors.len() as u32))
            .cloned()
    }
}
impl Default for PlayerColors {
    fn default() -> Self {
        let colors = vec![
            Color::rgb(0.8, 0.5, 0.5), // Red
            Color::rgb(0.5, 0.8, 0.5), // Green
            Color::rgb(0.5, 0.5, 0.8), // Blue
            Color::rgb(0.8, 0.8, 0.5), // Yellow
            Color::rgb(0.8, 0.5, 0.8), // Magenta
            Color::rgb(0.5, 0.8, 0.8), // Cyan
        ];
        Self::new(colors)
    }
}
