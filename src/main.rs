use std::{cmp::min, os::unix::thread, time::{Duration, Instant}};

use raylib::prelude::*;


struct Stats {
    fps: f64,
    tps: f64,
    last_frame: Instant,
    last_tick: Instant,
}
struct State {
  stats: Stats,
  h_cells:usize,
  v_cells:usize
}

const FPS:f64 = 60.0;
const TICKRATE:f64 = 60.0;
const SMOOTHING: f64 = 0.9;

fn main() {
    let (mut rlhandle, thread) = raylib::init()
        .size(1000, 1000)
        // .title("Hello, World")
        // .resizable()
        .build();
    

    let time_per_frame = Duration::from_secs_f64(1.0 / (FPS  as f64));
    let time_per_tick = Duration::from_secs_f64(1.0 / (TICKRATE as f64));
    println!("{:?}",time_per_frame);

    let mut state = State {
        stats: Stats { 
          fps: FPS,
          tps: TICKRATE,
          last_frame: Instant::now(),
          last_tick: Instant::now(),
        },
        h_cells: 500,
        v_cells: 500,
    };

    while !rlhandle.window_should_close() {
      let now = Instant::now();      

      if now.duration_since(state.stats.last_tick) >= time_per_tick {
          let delta = now.duration_since(state.stats.last_tick).as_secs_f64();
          let instant_tps = 1.0 / delta;
          state.stats.tps = SMOOTHING * state.stats.tps + (1.0 - SMOOTHING) * instant_tps;
          tick(&mut state);
          state.stats.last_tick = now;
      }    
      

      if now.duration_since(state.stats.last_frame) >= time_per_frame {
          let delta = now.duration_since(state.stats.last_frame).as_secs_f64();
          let instant_fps = 1.0 / delta;
          state.stats.fps = SMOOTHING * state.stats.fps + (1.0 - SMOOTHING) * instant_fps;
          draw(&mut rlhandle, &thread, &state);
          state.stats.last_frame = now;
      }      
    }
  }
  

fn draw(handle: &mut RaylibHandle, thread: &RaylibThread, state: &State) {
  let mut d = handle.begin_drawing(&thread);
  let text = format!("FPS: {}\nTPS: {}", state.stats.fps.round() as i64, state.stats.tps.round() as i64);
  // d.clear_background(Color::WHITE);
  d.draw_text(&text, 12, 12, 20, Color::BLACK);

  let height= d.get_screen_height();
  let width = d.get_screen_width();
  let cell_size = min(width / (state.h_cells as i32), width / (state.v_cells as i32));

  for y in 0..state.v_cells {
    for x in 0..state.h_cells {
      d.draw_rectangle((x as  i32) * cell_size, (y as i32) * cell_size, cell_size - 1, cell_size - 1 , Color::GRAY);
    }
  }

}

fn tick(state:&mut State){
  
} 