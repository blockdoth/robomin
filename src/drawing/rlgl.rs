use std::ops::{Deref, DerefMut};

use raylib::color::Color;
use raylib::ffi::{self, rlBegin, rlColor4ub, rlEnd, rlVertex3f};
use raylib::math::Vector3;
use raylib::prelude::{RaylibDrawHandle, RaylibMode3D};

pub struct Raylib3DHandle<'a, 'b>(pub RaylibMode3D<'a, RaylibDrawHandle<'b>>);

impl<'a, 'b> Deref for Raylib3DHandle<'a, 'b> {
    type Target = RaylibMode3D<'a, RaylibDrawHandle<'b>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'b> DerefMut for Raylib3DHandle<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, 'b> Raylib3DHandle<'a, 'b> {
    #![allow(non_snake_case)]
    pub fn draw_triangle_strip3D_wires(&mut self, points: &[Vector3], color: Color) {
        if points.len() < 3 {
            return;
        }

        unsafe {
            rlBegin(1);
            rlColor4ub(color.r, color.g, color.b, color.a);

            for i in 2..points.len() {
                if i % 2 == 0 {
                    let (a, b, c) = (points[i], points[i - 2], points[i - 1]);
                    rlVertex3f(a.x, a.y, a.z);
                    rlVertex3f(b.x, b.y, b.z);
                    rlVertex3f(b.x, b.y, b.z);
                    rlVertex3f(c.x, c.y, c.z);
                    rlVertex3f(c.x, c.y, c.z);
                    rlVertex3f(a.x, a.y, a.z);
                } else {
                    let (a, b, c) = (points[i], points[i - 1], points[i - 2]);
                    rlVertex3f(a.x, a.y, a.z);
                    rlVertex3f(b.x, b.y, b.z);
                    rlVertex3f(b.x, b.y, b.z);
                    rlVertex3f(c.x, c.y, c.z);
                    rlVertex3f(c.x, c.y, c.z);
                    rlVertex3f(a.x, a.y, a.z);
                }
            }

            rlEnd();
        }
    }
}

impl<'a, 'b> From<RaylibMode3D<'a, RaylibDrawHandle<'b>>> for Raylib3DHandle<'a, 'b> {
    fn from(value: RaylibMode3D<'a, RaylibDrawHandle<'b>>) -> Self {
        Raylib3DHandle(value)
    }
}

impl<'a, 'b> Raylib3DHandle<'a, 'b> {
    pub fn into_inner(self) -> RaylibMode3D<'a, RaylibDrawHandle<'b>> {
        self.0
    }
}
