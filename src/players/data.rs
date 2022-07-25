use std::cmp::Ordering;

use bevy::{
    prelude::{Color, Component, Deref, DerefMut},
    utils::HashMap,
};

#[derive(Component, Debug, Deref, DerefMut, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player {
    pub id: u32,
}

#[derive(Deref, DerefMut)]
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

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Score {
    pub kills: i32,
    pub deaths: u32,
    pub max_length: usize,
    pub current_length: usize,
}
impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.max_length
            .cmp(&other.max_length)
            .then_with(|| self.kills.cmp(&other.kills))
            .then_with(|| self.deaths.cmp(&other.deaths).reverse())
            .then_with(|| self.current_length.cmp(&other.current_length))
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct Scoreboard {
    scores: HashMap<Player, Score>,
}
impl Scoreboard {
    pub fn new() -> Self {
        Self {
            scores: HashMap::default(),
        }
    }

    pub fn insert_new(&mut self, player: Player) {
        if !self.scores.contains_key(&player) {
            self.scores.insert(player, Score::default());
        }
    }
}
