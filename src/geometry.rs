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

use vector::{Vector, dot};
use ray::Ray;

pub trait Geometry {
   fn intersect(&self, r: Ray) -> bool;
}

#[derive(Debug,Copy,Clone)]
pub struct Sphere {
   origin: Vector,
   radius: f32,
}

impl Sphere {
   pub fn new(x: f32, y: f32, z: f32, r: f32) -> Sphere {
      Sphere { origin: Vector::new(x, y, z), radius: r }
   }
}

impl Geometry for Sphere {
   fn intersect(&self, r: Ray) -> bool {

      let l = self.origin - r.origin;
      let s = dot(l, r.direction);
      let ll = dot(l, l);
      let rr = self.radius * self.radius;

      if s < 0.0 && ll > rr {
         return false;
      }

      let mm = ll - s * s;
      if mm > rr {
         return false;
      }

      true
   }
}


