use specs::prelude::*;

use crate::components::*;

use super::MovementCommand;

const PLAYER_MOVEMENT_SPEED: i32 = 20;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<MovementCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let movement_command = match &*data.0 {
            Some(movement_command) => movement_command,
            None => return,
        };
        (&data.1, &mut data.2)
            .par_join()
            .for_each(|(_, vel)| {
                match movement_command {
                    &MovementCommand::Move(direction) => {
                        vel.speed = PLAYER_MOVEMENT_SPEED;
                        vel.direction = direction;
                    },
                    MovementCommand::Stop => vel.speed = 0,
                }
            });
    }
}
