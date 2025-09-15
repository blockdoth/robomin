use rand::Rng;
use raylib::math::Vector3;

use crate::navmesh::Navmesh;
use std::{cmp::Ordering, hash::Hash};

pub struct World {
    pub bot: Bot,
    pub obstacles: Vec<Obstacle>,
    pub navmesh: Navmesh,
}

pub struct Obstacle {
    pub pos: Vector3,
    pub size: Vector3,
}

pub struct Bot {
  pub pos: Vector3,
  pub size: Vector3,
}