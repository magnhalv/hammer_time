use legion::world::SubWorld;
pub use legion::*;

use crate::{
    components::{Player, Point},
    prelude::*,
    resources::Controller,
};

const GRAVITY: f32 = 3.0;

#[system]
#[write_component(Point)]
#[read_component(Velocity)]
pub fn velocity(ecs: &mut SubWorld) {
    let mut players = <(Entity, &Velocity, &mut Point)>::query().filter(component::<Gravity>());
    players.iter_mut(ecs).for_each(|(_, velocity, point)| {
        point.x = f32::min(f32::max(point.x + velocity.vel_x, 0.0), (SCREEN_WIDTH-PLAYER_WIDTH) as f32);
        point.y = f32::min(f32::max(point.y + velocity.vel_y, 0.0), (SCREEN_HEIGHT-PLAYER_HEIGHT) as f32);
    });
}
