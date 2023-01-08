use legion::world::SubWorld;
pub use legion::*;

use crate::{
    components::{Player},
    resources::Controller, prelude::*,
};

const MAX_ACCELERATION: f32 = 1.0;
const MAX_VELOCITY: f32 = 5.0;
const MAX_JUMP_ACCELERATION: f32 = 3.0;

#[system]
#[write_component(Velocity)]
pub fn player_input(ecs: &mut SubWorld, #[resource] controller: &Controller) {
    let mut players = <(Entity, &mut Velocity)>::query().filter(component::<Player>());
    players.iter_mut(ecs).for_each(|(_, velocity)| {
        velocity.vel_x = velocity.vel_x + controller.x * MAX_ACCELERATION;

        let direction = if velocity.vel_x > 0.0 {
            1.0
        }
        else {
            -1.0
        };

        if f32::abs(velocity.vel_x) > MAX_VELOCITY {
            velocity.vel_x = MAX_VELOCITY * direction;
        }

        
        if controller.x == 0.0 {
            if velocity.vel_x > 0.0 {
                velocity.vel_x = f32::max(0.0, velocity.vel_x - MAX_ACCELERATION);
            }
            else {
                velocity.vel_x = f32::min(0.0, velocity.vel_x + MAX_ACCELERATION);
            }
            
        }

        if controller.y < 0.0 {
            velocity.vel_y = f32::max(velocity.vel_y - MAX_JUMP_ACCELERATION, -MAX_VELOCITY);
        }

    });
}


//point.x = f32::min(f32::max(point.x + (controller.x*SPEED), 0.0), (SCREEN_WIDTH-PLAYER_WIDTH) as f32);
//point.y = f32::min(f32::max(point.y + (controller.y*SPEED), 0.0), (SCREEN_HEIGHT-PLAYER_HEIGHT) as f32);