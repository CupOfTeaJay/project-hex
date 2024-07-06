use bevy::prelude::*;

use crate::components::common::is_movable::IsMovable;
use crate::components::movement::movement_queue::MovementQueue;

#[derive(Bundle)]
pub struct MovableBundle {
    pub is_movable: IsMovable,
    pub movement_queue: MovementQueue,
}

impl MovableBundle {
    pub fn new(is_movable: bool) -> Self {
        MovableBundle {
            is_movable: IsMovable::new(is_movable),
            movement_queue: (MovementQueue::new()),
        }
    }
}
