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
use geometry::ShadeRec;
use toml::Value;

pub trait Light {
   fn direction(&self, hr: ShadeRec) -> Vector;
   fn radiance(&self, hr: ShadeRec) -> Color;
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

   pub fn from_vec(v: Vec<f32>) -> AmbientLight {
      AmbientLight { color: Color::new(v[0], v[1], v[2]), ls: v[3] }
   }

   pub fn import(obj: &Value) -> Result<AmbientLight, String> {
      let elements = vec!["color.0", "color.1", "color.2", "ls"];
      let mut values = Vec::new();

      for e in elements {
         let value = try!(obj.lookup(e).ok_or("Missing element."));
         values.push(try!(value.as_float().ok_or("Invalid float.")) as f32);
      }

      Ok(AmbientLight::from_vec(values))
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

   pub fn from_vec(v: Vec<f32>) -> PointLight {
      PointLight { location: Vector::new(v[0], v[1], v[2]), color: Color::new(v[3], v[4], v[5]), ls: v[6] }
   }

   pub fn import(obj: &Value) -> Result<PointLight, String> {
      let elements = vec!["location.0", "location.1", "location.2", "color.0", "color.1", "color.2", "ls"];
      let mut values = Vec::new();

      for e in elements {
         let value = try!(obj.lookup(e).ok_or("Missing element."));
         values.push(try!(value.as_float().ok_or("Invalid float.")) as f32);
      }

      Ok(PointLight::from_vec(values))
   }
}

impl Light for AmbientLight {
   fn direction(&self, hr: ShadeRec) -> Vector {
      Vector::zero()
   }

   fn radiance(&self, hr: ShadeRec) -> Color {
      self.color * self.ls
   }
}

impl Light for PointLight {
   fn direction(&self, hr: ShadeRec) -> Vector {
     (self.location - hr.hit_point).normalize()
   }

   fn radiance(&self, hr: ShadeRec) -> Color {
      self.color * self.ls
   }
}
