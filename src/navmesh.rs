use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use ordered_float::NotNan;
use raylib::math::Vector3;


pub struct Navmesh {
    pub nodes: Vec<Triangle>,
    pub edges: Vec<(u64, u64)>,
}

pub struct Triangle {
    pub id: u64,
    pub corner_1: Vector3,
    pub corner_2: Vector3,
    pub corner_3: Vector3,
}

struct Traversal {
    open: BinaryHeap<(Reverse<NotNan<f64>>, usize)>,
    start: usize,
    end: usize,
    predecessors: HashMap<usize, usize>,
}
