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

extern crate byteorder;
extern crate toml;

mod vector;
mod ray;
mod geometry;
mod camera;
mod color;
mod light;
mod raytracer;

use std::env;
use camera::Camera;
use raytracer::RayTracer;

fn main() {
   let mut args = env::args();
   if args.len() != 2 {
      println!("usage: rrt <scene.toml>");
      return;
   }

   args.next();
   let filename = args.next().unwrap();

   let camera = Camera::new(90.0, 500.0, 500.0);

   let mut rt = RayTracer::new(camera);

   rt.import_scene(filename);

   rt.trace();

   rt.render();
}
