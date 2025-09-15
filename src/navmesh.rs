use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

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

impl Triangle {
    fn contains_point(&self, point: Vector3) -> bool {
        let side1 = Self::side(self.corner_1, self.corner_2, point);
        let side2 = Self::side(self.corner_2, self.corner_3, point);
        let side3 = Self::side(self.corner_3, self.corner_1, point);

        side1 >= 0.0 && side2 >= 0.0 && side3 >= 0.0
    }

    fn side(p1: Vector3, p2: Vector3, point: Vector3) -> f32 {
        (p2.y - p1.y) * (point.x - p1.x) - (-p2.x + p1.x) * (point.y * p1.y)
    }
}

struct Traversal {
    open: BinaryHeap<(Reverse<NotNan<f64>>, usize)>,
    start: usize,
    end: usize,
    predecessors: HashMap<usize, usize>,
}
