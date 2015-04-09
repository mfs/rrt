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

use std::f32;
use ray::Ray;
use vector::Vector;

#[derive(Debug,Copy,Clone)]
pub struct Camera {
   v_fov: f32,
   h_fov: f32,
   h_pixels: f32,
   v_pixels: f32,
   l: f32,
   r: f32,
   t: f32,
   b: f32,
   vp: f32,
}

impl Camera {

   pub fn new(v_fov: f32, h_pixels: f32, v_pixels: f32) -> Camera {
      let h_fov = v_fov * h_pixels / v_pixels;
      let v_fov_rad = v_fov * f32::consts::PI / 180.0;
      let h_fov_rad = h_fov * f32::consts::PI / 180.0;
      Camera {
         v_fov: v_fov,
         h_fov: v_fov * h_pixels / v_pixels,
         h_pixels: h_pixels,
         v_pixels: v_pixels,
         l: -(h_fov_rad / 2.0).tan(),
         r: (h_fov_rad / 2.0).tan(),
         b: -(v_fov_rad / 2.0).tan(),
         t: (v_fov_rad / 2.0).tan(),
         vp: -1.0,
      }
   }

   pub fn ray(self, x: f32, y: f32) -> Ray {
      let u = self.l + (self.r - self.l) * ((x + 0.5) / self.h_pixels);
      let v = self.b + (self.t - self.b) * ((y + 0.5) / self.v_pixels);

      let eye = Vector::zero();
      let screen = Vector::new(u, v, self.vp);

      Ray {
         origin: eye,
         direction: screen.normalize(),
      }
   }
}
