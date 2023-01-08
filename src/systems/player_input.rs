use legion::world::SubWorld;
pub use legion::*;

use crate::{
    components::{Player},
    resources::Controller, prelude::*,
};

const MAX_ACCELERATION: f32 = 25.0;
const MAX_VELOCITY: f32 = 10.0;
const MAX_JUMP_ACCELERATION: f32 = 8.0;

#[system]
#[write_component(Velocity)]
#[read_component(Point)]
pub fn player_input(ecs: &mut SubWorld, #[resource] frame_input: &FrameInput) {
    let controller = frame_input.controller;
    let dt = frame_input.dt;
    let mut players = <(Entity, &mut Velocity, &Point)>::query().filter(component::<Player>());
    players.iter_mut(ecs).for_each(|(_, velocity, point)| {
        velocity.vel_x = velocity.vel_x + (controller.x * MAX_ACCELERATION * dt);
        println!("{}", velocity.vel_x);
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
                velocity.vel_x = f32::max(0.0, velocity.vel_x - (MAX_ACCELERATION * dt));
            }
            else {
                velocity.vel_x = f32::min(0.0, velocity.vel_x + (MAX_ACCELERATION * dt));
            }
            
        }

        if controller.btn_a && point.y >= (SCREEN_HEIGHT - PLAYER_HEIGHT - 5) as f32  {
            velocity.vel_y = -MAX_JUMP_ACCELERATION;
        }

    });
}


//point.x = f32::min(f32::max(point.x + (controller.x*SPEED), 0.0), (SCREEN_WIDTH-PLAYER_WIDTH) as f32);
//point.y = f32::min(f32::max(point.y + (controller.y*SPEED), 0.0), (SCREEN_HEIGHT-PLAYER_HEIGHT) as f32);