extern crate sdl2;

use legion::*;
use sdl2::controller::Axis;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, SystemTime};

mod components;
mod resources;
mod systems;

mod prelude {
    pub use legion::*;
    pub const SCREEN_WIDTH: u32 = 800;
    pub const SCREEN_HEIGHT: u32 = 600;
    pub const PLAYER_WIDTH: u32 = 40;
    pub const PLAYER_HEIGHT: u32 = 80;
    
    pub use crate::components::*;
    pub use crate::resources::*;
}

use crate::systems::*;
use prelude::*;

struct GameState {
    player_id: Entity,
    ecs: World,
    resources: Resources,
    player_systems: Schedule,
}

const CONTROLLER_DEAD_ZONE: i16 = 4096;

impl GameState {
    fn new() -> Self {
        let mut ecs = World::default();
        let resources = Resources::default();

        let player_id = ecs.push((Player, Point { x: 10.0, y: 10.0 }, Velocity { vel_x: 0.0, vel_y: 0.0 }, Gravity));

        Self {
            player_id,
            ecs,
            resources,
            player_systems: build_player_scheduler(),
        }
    }
}

fn update(state: &mut GameState, controller: &Controller, dt: f32) {
    state.resources.insert(FrameInput {
        controller: controller.clone(),
        dt
    });

    state
        .player_systems
        .execute(&mut state.ecs, &mut state.resources);
}

fn process_input_stick_value(stick_value: i16, dead_zone: i16) -> f32 {
    if stick_value < -dead_zone {
        return (stick_value as f32) / 32768.0;
    } else if stick_value > dead_zone {
        return (stick_value as f32) / 32767.0;
    } else {
        return 0.0;
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let game_controller_subsystem = sdl_context.game_controller().unwrap();

    // Game controllers
    let available = game_controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))
        .unwrap();

    
    println!("{} joysticks available", available);

    // Iterate over all available joysticks and look for game controllers.
    let mut controller = (0..available)
        .find_map(|id| {
            if !game_controller_subsystem.is_game_controller(id) {
                println!("{} is not a game controller", id);
                return None;
            }

            println!("Attempting to open controller {}", id);

            match game_controller_subsystem.open(id) {
                Ok(c) => {
                    // We managed to find and open a game controller,
                    // exit the loop
                    println!("Success: opened \"{}\"", c.name());
                    Some(c)
                }
                Err(e) => {
                    println!("failed: {:?}", e);
                    None
                }
            }
        })
        .expect("Couldn't open any controller");

    println!("Controller mapping: {}", controller.mapping());

    // End Game controllers

    let window = video_subsystem
        .window("rust-sdl2 demo", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut state = GameState::new();
    let mut controller = Controller {
        x: 0.0,
        y: 0.0,
        dx: 0.0,
        dy: 0.0,
    };

    let mut frame_duration: Duration;
    let mut dt = 0.0;
    
    'running: loop {
        let start_frame_time = SystemTime::now();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 0, 0));        

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::ControllerAxisMotion {
                    axis: Axis::LeftX,
                    value: val,
                    ..
                } => {
                    controller.dx = controller.x - (val as f32);
                    controller.x = process_input_stick_value(val, CONTROLLER_DEAD_ZONE);
                }
                Event::ControllerAxisMotion {
                    axis: Axis::LeftY,
                    value: val,
                    ..
                } => {
                    controller.dy = controller.y - (val as f32);
                    controller.y = process_input_stick_value(val, CONTROLLER_DEAD_ZONE);
                }

                _ => {}
            }
        }                

        update(&mut state, &controller, dt);        
        // The rest of the game loop goes here...
        let player = state.ecs.entry(state.player_id).unwrap();        
        let point = player.get_component::<Point>().unwrap();
        

        canvas.fill_rect(Rect::new(point.x as i32, point.y as i32, PLAYER_WIDTH, PLAYER_HEIGHT)).unwrap();
        canvas.present();


        let end_frame_time = SystemTime::now();
        frame_duration = end_frame_time.duration_since(start_frame_time).unwrap();        
        
        let max_frame_duration = 1_000_000_000u128 / 60;

        if max_frame_duration > frame_duration.as_nanos() {
            let sleep_duration = (max_frame_duration - frame_duration.as_nanos()) as u32;            
            ::std::thread::sleep(Duration::new(0, sleep_duration));
            dt = 1.0 / 60.0;
            println!("Sleeping {} ns!", {dt});            
        }
        else {
            dt = frame_duration.as_secs_f32();
            println!("WARNING: Exceeded max frame duration!")
        }
        
    }
}
