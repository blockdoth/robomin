mod drawing;
mod navmesh;
mod stats;
mod world;

use std::cmp::{Ordering, Reverse, min};
use std::collections::{BinaryHeap, HashMap, hash_set};
use std::hash::{Hash, Hasher};
use std::iter::Map;
use std::os::unix::thread;
use std::time::{Duration, Instant};
use std::{cell, f64};

use log::LevelFilter;
use ordered_float::NotNan;
use rand::Rng;
use raylib::ffi::{Mesh, Model};
use raylib::prelude::*;
use simplelog::{Config, SimpleLogger};

use crate::drawing::draw::{DisplayInfo, draw};
use crate::navmesh::{Navmesh, Triangle};
use crate::stats::Stats;
use crate::world::{Bot, Obstacle, World};

struct State {
    pub stats: Stats,
    pub display: DisplayInfo,
    pub world: World,
}

const FPS: f64 = 60.0;
const TICKRATE: f64 = 60.0;
const SMOOTHING: f64 = 0.9;
const CELL_COUNT: usize = 50;

fn main() {
    let _ = SimpleLogger::init(LevelFilter::Info, Config::default());

    let initial_width = 800;
    let initial_height = 800;

    let (mut rlhandle, thread) = raylib::init().size(initial_width, initial_height).build();

    let nodes = vec![
        Triangle {
            id: 0,
            corner_1: Vector3 { x: 10.0, y: 0.0, z: 0.0 },
            corner_2: Vector3 { x: 10.0, y: 0.0, z: 10.0 },
            corner_3: Vector3 { x: 0.0, y: 0.0, z: 10.0 },
        },
        Triangle {
            id: 0,
            corner_1: Vector3 { x: 10.0, y: 0.0, z: 0.0 },
            corner_2: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            corner_3: Vector3 { x: 0.0, y: 0.0, z: 10.0 },
        },
    ];

    let mut state = State {
        world: World {
            bot: Bot {
                pos: Vector3 { x: 0.0, y: 0.25, z: 0.0 },
                size: Vector3 { x: 1.0, y: 0.5, z: 1.3 },
            },
            obstacles: vec![Obstacle {
                pos: Vector3 { x: 5.0, y: 0.5, z: 5.0 },
                size: Vector3 { x: 1.0, y: 1.0, z: 1.0 },
            }],
            navmesh: Navmesh { nodes, edges: vec![] },
        },
        display: DisplayInfo {
            screen_height: initial_height,
            screen_width: initial_width,
            cam_angle: 0.0,
            cam_radius: 0.0,
            camera: Camera3D::perspective(
                Vector3 { x: 0.0, y: 40.0, z: 0.0 },
                Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                Vector3 { x: 0.0, y: 1.0, z: 0.0 },
                45.0,
            ),
        },
        stats: Stats {
            fps: FPS,
            tps: TICKRATE,
            last_frame: Instant::now(),
            last_tick: Instant::now(),
        },
    };
    state.display.cam_radius = state.display.camera.position.length();

    let time_per_frame = Duration::from_secs_f64(1.0 / FPS);
    let time_per_tick = Duration::from_secs_f64(1.0 / TICKRATE);

    while !rlhandle.window_should_close() {
        let now = Instant::now();

        // Handle updates
        if now.duration_since(state.stats.last_tick) >= time_per_tick {
            let delta = now.duration_since(state.stats.last_tick).as_secs_f64();
            let instant_tps = 1.0 / delta;
            state.stats.tps = SMOOTHING * state.stats.tps + (1.0 - SMOOTHING) * instant_tps;

            handle_input(&mut rlhandle, &mut state);
            tick(&mut rlhandle, &mut state);

            state.stats.last_tick = now;
        }

        // Draw updates
        if now.duration_since(state.stats.last_frame) >= time_per_frame {
            let delta = now.duration_since(state.stats.last_frame).as_secs_f64();
            let instant_fps = 1.0 / delta;
            state.stats.fps = SMOOTHING * state.stats.fps + (1.0 - SMOOTHING) * instant_fps;

            // TODO figure out where to place this

            draw(&mut rlhandle.begin_drawing(&thread), &state);

            state.stats.last_frame = now;
        }
    }
}

fn handle_input(h: &mut RaylibHandle, state: &mut State) {
    if h.is_key_down(KeyboardKey::KEY_UP) {
        state.display.camera.position.scale(1.1);
        state.display.cam_radius += 0.5;
    }

    if h.is_key_down(KeyboardKey::KEY_DOWN) {
        state.display.camera.position.scale(0.9);
        state.display.cam_radius -= 0.5;
    }

    if h.is_key_down(KeyboardKey::KEY_LEFT) {
        state.display.cam_angle += 0.02;
    }
    if h.is_key_down(KeyboardKey::KEY_RIGHT) {
        state.display.cam_angle -= 0.02;
    }
}

fn tick(h: &mut RaylibHandle, state: &mut State) {
  update_camera(state);
  state.display.screen_height = h.get_screen_height();
  state.display.screen_width = h.get_screen_width();  


}

fn update_camera(state: &mut State) {
    state.display.camera.position.x = state.display.camera.target.x + state.display.cam_angle.cos() * state.display.cam_radius;
    state.display.camera.position.z = state.display.camera.target.z + state.display.cam_angle.sin() * state.display.cam_radius;
    state.display.camera.position.y = state.display.camera.target.y + 5.0;

}