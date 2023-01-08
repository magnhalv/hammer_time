use legion::world::SubWorld;
pub use legion::*;

use crate::prelude::*;

const GRAVITY: f32 = 1.0;
const TERMINAL_VELOCITY: f32 = 5.0;

#[system]
#[write_component(Velocity)]
pub fn gravity(ecs: &mut SubWorld) {
    let mut players = <(Entity, &mut Velocity)>::query().filter(component::<Gravity>());
    players.iter_mut(ecs).for_each(|(_, velocity)| {
        velocity.vel_y = f32::min(velocity.vel_y + GRAVITY, TERMINAL_VELOCITY);
    });
}
