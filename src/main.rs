/*
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 * */

extern crate image;

mod vector;
mod ray;
mod geometry;

use std::fs::File;
use std::path::Path;
use vector::Vector;
use ray::Ray;
use geometry::Sphere;
use geometry::Geometry;
use geometry::Triangle;

fn main() {

   let imgx = 500;
   let imgy = 500;

   let mut img = image::ImageBuffer::new(imgx, imgy);

   let dir = Vector::new(0.0, 0.0, -1.0);
   let s = Sphere::new(250.0, 250.0, -1000.0, 150.0);
   let t = Triangle {
      v0: Vector::new(300.0, 600.0, -800.0),
      v1: Vector::new(0.0, 100.0, -1000.0),
      v2: Vector::new(450.0, 20.0, -1000.0),
   };

   let mut shapes: Vec<&Geometry> = Vec::new();
   shapes.push(&s);
   shapes.push(&t);

   for y in (0 .. imgx) {
      for x in (0 .. imgy) {
         let r = Ray { origin: Vector::new(y as f32, x as f32, 0.0), direction: dir };

         let color = match s.intersect(r) {
            None => image::Rgb([0, 0, 0]),
            Some(hr) => image::Rgb([hr.color.x as u8, hr.color.y as u8, hr.color.z as u8]),
         };

         img.put_pixel(x, y, color);
      }
   }

   let ref mut fout = File::create(&Path::new("trace.png")).unwrap();
   match image::ImageRgb8(img).save(fout, image::PNG) {
      Ok(_) => (),
      Err(err) => {println!("{}", err)},
   }

}
