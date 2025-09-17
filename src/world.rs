use std::cmp::{Ordering, max};
use std::hash::Hash;

use ordered_float::Float;
use rand::Rng;
use raylib::math::Vector3;

use crate::navmesh::Navmesh;

pub struct World {
    pub bot: Bot,
    pub obstacles: Vec<Obstacle>,
    pub border: Vec<Vector3>,
    pub navmesh: Option<Navmesh>,
    pub border_margin: f32,
    pub obstacle_margin: f32,
}

impl World {
    fn all_points(&self) -> Vec<Vector3> {
        let obstacle_points: Vec<Vector3> = self.obstacles.iter().flat_map(|obstacle| obstacle.to_points()).collect();
        let mut all_points = self.border.clone();
        all_points.extend(obstacle_points);
        all_points
    }
    fn all_edges(&self) -> Vec<Vector3> {
        let obstacle_points: Vec<Vector3> = self.obstacles.iter().flat_map(|obstacle| obstacle.to_points()).collect();
        let mut all_points = self.border.clone();
        all_points.extend(obstacle_points);
        all_points
    }

    fn span(&self) -> f32 {
        let points = self.all_points();
        if let Some(p) = points.iter().map(|p| p.x).reduce(|p1, p2| p1.max(p2)) {
            p
        } else {
            f32::NAN
        }
    }

    // TODO decide wether to include obstacles points or not
    fn border_center(&self) -> Vector3 {
        let sum = self.border.iter().cloned().reduce(|a, b| a + b).expect("World has no border points");

        sum / (self.border.len() as f32)
    }

    fn scale_border_2d(&self, factor: f32) -> Vec<Vector3> {
        self.border
            .iter()
            .map(|point| {
                let mut point = *point;
                point.scale(factor);
                point
            })
            .collect()
    }

    fn circumcirlce(p1: Vector3, p2: Vector3, p3: Vector3) -> Vector3 {


      
    }

    fn split_triangle(&self, p1: Vector3, p2: Vector3, p3: Vector3) {
        // let new_point =
    }

    fn split_seg(&self) {}

    fn build_delaunay_mesh(&self) {
        let span = 1.5 * self.span();
        let bb_center = self.border_center();

        let bb_p1 = bb_center + Vector3 { x: span, y: 0.0, z: span };
        let bb_p2 = bb_center + Vector3 { x: -span, y: 0.0, z: span };
        let bb_p3 = bb_center + Vector3 { x: -span, y: 0.0, z: -span };
        let bb_p4 = bb_center + Vector3 { x: span, y: 0.0, z: -span };

        let edge_list = self.all_edges();
        let node_list = self.all_points();

        // let bounding_
    }
}

pub struct Obstacle {
    pub pos: Vector3,
    pub size: Vector3,
}

impl Obstacle {
    fn to_points(&self) -> Vec<Vector3> {
        let obstacle_width = self.size.x;
        let obstacle_length = self.size.y;
        vec![
            self.pos
                + Vector3 {
                    x: obstacle_width,
                    y: 0.0,
                    z: obstacle_length,
                },
            self.pos
                + Vector3 {
                    x: obstacle_width,
                    y: 0.0,
                    z: -obstacle_length,
                },
            self.pos
                + Vector3 {
                    x: -obstacle_width,
                    y: 0.0,
                    z: -obstacle_length,
                },
            self.pos
                + Vector3 {
                    x: -obstacle_width,
                    y: 0.0,
                    z: obstacle_length,
                },
        ]
    }
}

pub struct Bot {
    pub pos: Vector3,
    pub size: Vector3,
}
