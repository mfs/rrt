/*
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use vector::{Vector, dot, cross};
use ray::Ray;
use color::Color;

pub trait Geometry {
   fn intersect(&self, r: Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

#[derive(Debug,Copy,Clone)]
pub struct Sphere {
   origin: Vector,
   radius: f32,
   color: Color,
}

impl Sphere {
   pub fn new(x: f32, y: f32, z: f32, r: f32, cr: f32, cg: f32, cb: f32) -> Sphere {
      Sphere { origin: Vector::new(x, y, z), radius: r, color: Color::new(cr, cg, cb) }
   }
}

impl Geometry for Sphere {
   fn intersect(&self, r: Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {

      let l = self.origin - r.origin;
      let s = dot(l, r.direction);
      let ll = dot(l, l);
      let rr = self.radius * self.radius;

      if s < 0.0 && ll > rr {
         return None;
      }

      let mm = ll - s * s;
      if mm > rr {
         return None;
      }

      let q = (rr - mm).sqrt();
      let t = if ll > rr { s - q } else { s + q };

      if t < tmin || t > tmax {
         return None;
      }

      let p = r.origin + r.direction * t;
      let n = (p - self.origin).normalize();

      Some(HitRecord {
         t: t,
         normal: n,
         color: self.color,
      })
   }
}

#[derive(Debug,Copy,Clone)]
pub struct Triangle {
   pub v0: Vector,
   pub v1: Vector,
   pub v2: Vector,
   pub color: Color,
}

impl Geometry for Triangle {
   fn intersect(&self, r: Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
      const EPS: f32 = 1e-5;
      let e1 = self.v1 - self.v0;
      let e2 = self.v2 - self.v0;
      let p = cross(r.direction, e2);
      let a = dot(e1, p);
      if a > -EPS && a < EPS {
         return None;
      }
      let f = 1.0 / a;
      let s = r.origin - self.v0;
      let u = f * dot(s, p);
      if u < 0.0 || u > 1.0 {
         return None;
      }
      let q = cross(s, e1);
      let v = f * dot(r.direction, q);
      if v < 0.0 || (u + v) > 1.0 {
         return None;
      }
      let t = f * dot(e2, q);

      if t < tmin || t > tmax {
         return None;
      }

      Some(HitRecord {
         t: t,
         normal: Vector::zero(),
         color: self.color,
      })
   }
}

pub struct HitRecord {
   pub t: f32,
   pub normal: Vector,
   pub color: Color,
}
