use std::cmp::Ordering;

use bevy::{
    prelude::{Component, Deref, DerefMut},
    utils::HashMap,
};

use super::config::PlayerDetails;

#[derive(Component, Debug, Deref, DerefMut, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId {
    pub id: u32,
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

#[derive(Debug, Default, Deref, DerefMut)]
pub struct Players(pub HashMap<PlayerId, PlayerDetails>);
