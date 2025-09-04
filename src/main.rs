use std::{
    cell,
    cmp::{Reverse, min},
    collections::HashMap,
    f64,
    hash::Hash,
    iter::Map,
    os::unix::thread,
    time::{Duration, Instant},
};

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use ordered_float::NotNan;
use rand::Rng;
use raylib::prelude::*;

struct Stats {
    fps: f64,
    tps: f64,
    last_frame: Instant,
    last_tick: Instant,
}
struct State {
    stats: Stats,
    grid: Grid,
    traversal: Traversal,
}

struct Traversal {
    open: BinaryHeap<(Reverse<NotNan<f64>>, usize)>,
    start: usize,
    end: usize,
    predecessors: HashMap<usize, usize>,
}
#[derive(Clone, Debug)]
struct Grid {
    cells: Vec<Cell>,
    rows: usize,
    columns: usize,
}

impl Grid {
    fn init(columns: usize, rows: usize) -> Self {
        let mut rng = rand::rng();

        let mut cells = Vec::with_capacity(columns * rows);
        for y in 0..rows {
            for x in 0..columns {
                let wall = rng.random_bool(WALL_CHANGE);
                cells.push(Cell {
                    typ: if wall {
                        CellTyp::Wall
                    } else {
                        CellTyp::Background
                    },
                    x,
                    y,
                    f: 0.0,
                    g: if wall { 0.0 } else { f64::INFINITY },
                    h: 0.0,
                });
            }
        }

        Grid {
            cells,
            rows,
            columns,
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.columns + x
    }

    fn get(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y * self.columns + x]
    }
    fn set(&mut self, x: usize, y: usize, typ: CellTyp) -> usize {
        let idx = self.idx(x, y);
        self.cells[idx].typ = typ;
        idx
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<usize> {
        let mut neighbors = vec![];

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && ny >= 0 && (nx as usize) < self.columns && (ny as usize) < self.rows {
                    neighbors.push(self.idx(nx as usize, ny as usize));
                }
            }
        }

        neighbors
    }
}

#[derive(Clone, Debug, Hash)]
enum CellTyp {
    Start,
    End,
    Path,
    Wall,
    Background,
}

#[derive(Clone, Debug)]
struct Cell {
    typ: CellTyp,
    x: usize,
    y: usize,
    g: f64,
    h: f64,
    f: f64,
}

impl Hash for Cell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Eq for Cell {}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.partial_cmp(&self.f).unwrap_or(Ordering::Equal)
    }
}

const FPS: f64 = 60.0;
const TICKRATE: f64 = 60.0;
const SMOOTHING: f64 = 0.9;
const CELL_COUNT: usize = 50;
const WALL_CHANGE: f64 = 0.5;
fn main() {
    let (mut rlhandle, thread) = raylib::init()
        .size(800, 800)
        .build();

    let time_per_frame = Duration::from_secs_f64(1.0 / FPS);
    let time_per_tick = Duration::from_secs_f64(1.0 / TICKRATE);

    let mut grid = Grid::init(CELL_COUNT, CELL_COUNT);

    let startcell = grid.set(5, 3, CellTyp::Start);
    let endcell = grid.set(45, 48, CellTyp::End);

    let mut state = State {
        stats: Stats {
            fps: FPS,
            tps: TICKRATE,
            last_frame: Instant::now(),
            last_tick: Instant::now(),
        },
        grid,
        traversal: Traversal {
            start: startcell,
            end: endcell,
            open: BinaryHeap::new(),
            predecessors: HashMap::new(),
        },
    };

    let start_cell = &mut state.grid.cells[startcell];
    start_cell.g = 0.0;
    start_cell.h = euclidean(start_cell.x, start_cell.y, 45, 48);
    start_cell.f = start_cell.g + start_cell.h;

    state
        .traversal
        .open
        .push((Reverse(NotNan::new(start_cell.f).unwrap()), startcell));

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
    let mut d = handle.begin_drawing(thread);
    d.clear_background(Color::WHITE);

    let height = d.get_screen_height();
    let width = d.get_screen_width();
    let cell_size = min(
        width / (state.grid.columns as i32),
        height / (state.grid.rows as i32),
    );

    for cell in &state.grid.cells {
        let color = match cell.typ {
            CellTyp::Start => Color::BLUE,
            CellTyp::End => Color::RED,
            CellTyp::Wall => Color::BLACK,
            CellTyp::Background => Color::GRAY,
            CellTyp::Path => Color::GREEN,
        };

        d.draw_rectangle(
            (cell.x as i32) * cell_size,
            (cell.y as i32) * cell_size,
            cell_size - 1,
            cell_size - 1,
            color,
        );
    }

    // let text = format!(
    //     "FPS: {}\nTPS: {}",
    //     state.stats.fps.round() as i64,
    //     state.stats.tps.round() as i64
    // );
    // d.draw_text(&text, 12, 12, 20, Color::BLACK);
}

fn tick(state: &mut State) {
    let end_x = state.grid.cells[state.traversal.end].x;
    let end_y = state.grid.cells[state.traversal.end].y;
    while let Some((_, parent_idx)) = state.traversal.open.pop() {
        if parent_idx == state.traversal.end {
            println!("Found end");

            let mut prev = state.traversal.end;
            while let Some(prev_cell) = state.traversal.predecessors.get(&prev)
                && prev != state.traversal.start
            {
                state.grid.cells[*prev_cell].typ = CellTyp::Path;
                prev = *prev_cell;
            }
            println!("No path found");

            break;
        }
        let parent_cell = &state.grid.cells[parent_idx];
        let neighbours = state.grid.neighbours(parent_cell.x, parent_cell.y);

        let possible_new_g = parent_cell.g + 1.0;

        for neighbour_idx in neighbours {
            let neighbour = &mut state.grid.cells[neighbour_idx];

            if possible_new_g < neighbour.g {
                neighbour.g = possible_new_g;
                neighbour.h = euclidean(neighbour.x, neighbour.y, end_x, end_y);
                neighbour.f = neighbour.g + neighbour.h;

                state
                    .traversal
                    .predecessors
                    .insert(neighbour_idx, parent_idx);

                state
                    .traversal
                    .open
                    .push((Reverse(NotNan::new(neighbour.f).unwrap()), neighbour_idx));
            }
        }
    }
}

fn euclidean(x1: usize, y1: usize, x2: usize, y2: usize) -> f64 {
    let dx = x1 as f64 - x2 as f64;
    let dy = y1 as f64 - y2 as f64;
    (dx * dx + dy * dy).sqrt()
}
