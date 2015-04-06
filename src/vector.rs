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

use std::ops::{Add, Sub, Neg, Mul};

#[derive(Debug,Copy,Clone)]
pub struct Vector {
   pub x: f32,
   pub y: f32,
   pub z: f32,
}

impl Vector {

   // Constructors
   pub fn new(x: f32, y: f32, z: f32) -> Vector {
      Vector {x: x, y: y, z: z }
   }

   pub fn zero() -> Vector {
      Vector::new(0.0, 0.0, 0.0)
   }

   // Magnitude related
   pub fn magnitude_sq(self) -> f32 {
      dot(self, self)
   }

   pub fn magnitude(self) -> f32 {
      self.magnitude_sq().sqrt()
   }

   pub fn normalize(self) -> Vector {
      let inv_magnitude = 1.0 / self.magnitude();
      Vector {
         x: self.x * inv_magnitude,
         y: self.y * inv_magnitude,
         z: self.z * inv_magnitude,
      }
   }
}

// Dot and cross products.

pub fn dot(a: Vector, b: Vector) -> f32 {
   a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross(a: Vector, b: Vector) -> Vector {
   Vector {
      x: a.y * b.z - a.z * b.y,
      y: a.z * b.x - a.x * b.z,
      z: a.x * b.y - a.y * b.x
   }
}

// Operator overloads

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}





