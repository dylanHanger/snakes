use super::data::*;
use crate::grid::prelude::*;

use bevy::prelude::{Commands, Entity, Query};

pub fn movement_system(
    mut commands: Commands,
    mut q: Query<(Entity, &mut GridPosition, &MoveIntent)>,
) {
    for (e, mut pos, intent) in q.iter_mut() {
        let dx = intent.delta_x();
        let dy = intent.delta_y();

        pos.x += dx;
        pos.y += dy;

        commands.entity(e).remove::<MoveIntent>();
    }
}
