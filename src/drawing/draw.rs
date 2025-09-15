use std::cmp::min;

use log::{info, log};
use raylib::camera::Camera3D;
use raylib::ffi::{rlEnd, rlVertex3f};
use raylib::math::{Vector2, Vector3};
use raylib::prelude::{Color, RaylibDraw, RaylibDraw3D, RaylibDrawHandle, RaylibMode3D, RaylibMode3DExt};

use crate::State;
use crate::drawing::rlgl::Raylib3DHandle;
use crate::world::Obstacle;

const NAVMESH_COLOR: Color = Color { r: 0, g: 0, b: 255, a: 50 };
pub struct DisplayInfo {
    pub screen_height: i32,
    pub screen_width: i32,
    pub cam_angle: f32,
    pub cam_radius: f32,
    pub camera: Camera3D,
}

pub fn draw(h: &mut RaylibDrawHandle<'_>, state: &State) {
    h.clear_background(Color::WHITE);
    {
        let mut handle_3d: Raylib3DHandle = h.begin_mode3D(state.display.camera).into();
        draw_world(&mut handle_3d, state);
    }
    draw_ui(h, state);
}

fn draw_world(h: &mut Raylib3DHandle, state: &State) {
    let pos = Vector3 {
        x: 0.0,
        y: 0.5 / 2.0,
        z: 0.0,
    };

    let size = Vector3 { x: 1.0, y: 0.5, z: 1.3 };

    let points: Vec<Vector3> = state
        .world
        .navmesh
        .nodes
        .iter()
        .flat_map(|f| [f.corner_1, f.corner_2, f.corner_3])
        .collect();

    h.draw_triangle_strip3D(&points, NAVMESH_COLOR);
    h.draw_triangle_strip3D_wires(&points, NAVMESH_COLOR);

    for obstacle in &state.world.obstacles {
        h.draw_cube_v(obstacle.pos, obstacle.size, Color::RED);
        h.draw_cube_wires_v(obstacle.pos, obstacle.size, Color::BLACK);
    }

    h.draw_cube_v(state.world.bot.pos, state.world.bot.size, Color::GREEN);
    h.draw_cube_wires_v(state.world.bot.pos, state.world.bot.size, Color::BLACK);

    h.draw_grid(100, 1.0);
}

fn draw_ui(h: &mut RaylibDrawHandle<'_>, state: &State) {
    draw_stats(h, state);
}

fn draw_stats(h: &mut RaylibDrawHandle<'_>, state: &State) {
    let text = format!("FPS:\t{}\nTPS:\t{}", state.stats.fps.round() as i64, state.stats.tps.round() as i64);
    h.draw_rectangle(5, 5, 93, 45, Color::BLACK);
    h.draw_text(&text, 10, 9, 20, Color::WHITE);
}
