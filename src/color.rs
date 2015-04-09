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

use std::ops::{Add, Sub, Mul};

#[derive(Debug,Copy,Clone)]
pub struct Color {
   pub r: f32,
   pub g: f32,
   pub b: f32,
}

impl Color {

   // Constructors
   pub fn new(r: f32, g: f32, b: f32) -> Color {
      Color { r: r, g: g, b: b }
   }

   pub fn to_bytes(self) -> (u8, u8, u8) {
      let r = (self.r / 1.0) * 255.0;
      let g = (self.g / 1.0) * 255.0;
      let b = (self.b / 1.0) * 255.0;
      (r as u8, g as u8, b as u8)
   }

}

// Operator overloads

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b
        }
    }
}


