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

use vector::Vector;
use color::Color;
use geometry::HitRecord;

pub trait Light {
   fn direction(&self, hr: HitRecord) -> Vector;
   fn radiance(&self, hr: HitRecord) -> Color;
}

#[derive(Debug,Copy,Clone)]
pub struct AmbientLight {
   pub color: Color,
   pub ls: f32, // radiance scaling factor
}

#[derive(Debug,Copy,Clone)]
pub struct PointLight {
   pub location: Vector,
   pub color: Color,
   pub ls: f32, // radiance scaling factor
}

impl AmbientLight {
   pub fn new() -> AmbientLight {
      AmbientLight {
         color: Color::new(1.0, 1.0, 1.0),
         ls: 0.2,
      }
   }
}

impl PointLight {
   pub fn new() -> PointLight {
      PointLight {
         location: Vector::zero(),
         color: Color::new(1.0, 1.0, 1.0),
         ls: 0.2,
      }
   }
}

impl Light for AmbientLight {
   fn direction(&self, hr: HitRecord) -> Vector {
      Vector::zero()
   }

   fn radiance(&self, hr: HitRecord) -> Color {
      self.color * self.ls
   }
}

impl Light for PointLight {
   fn direction(&self, hr: HitRecord) -> Vector {
     //(self.location - hr.hit_point).normalize()
     Vector::zero() // fix this
   }

   fn radiance(&self, hr: HitRecord) -> Color {
      self.color * self.ls
   }
}
