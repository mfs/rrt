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

#[derive(Debug,Copy,Clone)]
pub struct Light {
   pub origin: Vector,
   pub amb: Color,
   pub diff: Color,
   pub spec: Color,
}

impl Light {
   pub fn default() -> Light {
      Light {
         origin: Vector::zero(),
         amb: Color::new(0.2, 0.2, 0.2),
         diff: Color::new(0.7, 0.7, 0.7),
         spec: Color::new(0.3, 0.3, 0.3),
      }
   }
}

