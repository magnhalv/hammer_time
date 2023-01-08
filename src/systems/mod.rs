use crate::prelude::*;

mod player_input;
mod gravity;
mod velocity;

use player_input::*;

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(player_input::player_input_system())
    .add_system(gravity::gravity_system())
    .add_system(velocity::velocity_system())
    .build()
}