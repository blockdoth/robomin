use std::{os::unix::thread, time::{Duration, Instant}};

use raylib::prelude::*;


struct State {
  ticks_a_sec: u64,
  frames_a_sec:u64,
  now_second:Instant,
  h_cells:usize,
  v_cells:usize
}

const FPS:u64 = 60;
const TICKRATE:u64 = 60;


fn main() {
    let (mut rlhandle, thread) = raylib::init()
        .size(800, 800)
        // .title("Hello, World")
        .build();
    
    let time_per_frame = Duration::new(1 / FPS,0);
    let time_per_tick = Duration::new(1 / TICKRATE,0);


    let mut state =State {
        ticks_a_sec: 0,
        frames_a_sec: 0,
        now_second: Instant::now(),
        h_cells: 100,
        v_cells: 100,
    };

    // let mut 
    while !rlhandle.window_should_close() {
      
      let now_ticks = Instant::now();
      if now_ticks.elapsed() > time_per_tick {
        tick(&mut state);
      };      

      let now_fps = Instant::now();
      if now_fps.elapsed() > time_per_frame {
        draw(&mut rlhandle, &thread, &mut state);
      };

      if state.now_second.elapsed() > Duration::new(1, 0) {
        state.frames_a_sec = 0;
        state.ticks_a_sec = 0;
        state.now_second = Instant::now();
      }
    }
  }
  
  
fn draw(handle: &mut RaylibHandle, thread: &RaylibThread, state: &mut State) {
  let mut d = handle.begin_drawing(&thread);
  let text = format!("FPS: {}\nTPS: {}", state.frames_a_sec, state.ticks_a_sec);
  d.draw_text(&text, 12, 12, 20, Color::BLACK);

  d.clear_background(Color::WHITE);
  state.frames_a_sec += 1;
}

fn tick(state:&mut State){
  
  state.ticks_a_sec += 1;
}