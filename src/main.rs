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

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::io::Read;
use geometry::Sphere;
use geometry::Geometry;
use byteorder::{LittleEndian, WriteBytesExt};
use camera::Camera;
use vector::dot;
use toml::Parser;

fn main() {
   let mut args = env::args();
   if args.len() != 2 {
      println!("usage: rrt <scene.toml>");
      return;
   }

   args.next();
   let filename = args.next().unwrap();

   let imgx = 500;
   let imgy = 500;

   let camera = Camera::new(90.0, imgx as f32, imgy as f32);

   let mut img = vec![(0, 0, 0); imgx * imgy];

   let path = Path::new(&filename);
   let mut fin = File::open(&path).unwrap();
   let mut scene = String::new();
   fin.read_to_string(&mut scene).unwrap();

   let mut shapes: Vec<Box<Geometry>> = Vec::new();

   let mut p = Parser::new(&scene);
   let toml = p.parse().unwrap();
   let objects = toml.get("object").unwrap().as_slice().unwrap();
   for obj in objects {
      let t = obj.lookup("type").unwrap().as_str().unwrap();
      match t {
         "sphere" => {
            let s = Sphere::new(
               obj.lookup("origin.0").unwrap().as_float().unwrap() as f32,
               obj.lookup("origin.1").unwrap().as_float().unwrap() as f32,
               obj.lookup("origin.2").unwrap().as_float().unwrap() as f32,
               obj.lookup("radius").unwrap().as_float().unwrap() as f32,
               obj.lookup("color.0").unwrap().as_float().unwrap() as f32,
               obj.lookup("color.1").unwrap().as_float().unwrap() as f32,
               obj.lookup("color.2").unwrap().as_float().unwrap() as f32,
            );
            shapes.push(Box::new(s));
         }
         _ => { println!("Unknown object {} ignored.", t); }
      }
   }

   for y in (0 .. imgx) {
      for x in (0 .. imgy) {
         let mut tmax = 100000.0;
         let mut color = (0, 102, 205);
         let r = camera.ray(x as f32, y as f32);

         for sh in shapes.iter() {
            match sh.intersect(r, 0.00001, tmax) {
               None => {},
               Some(hr) => {
                  tmax = hr.t;
                  let c = hr.color * dot(-r.direction, hr.normal);
                  color = c.to_bytes();
               },
            }
         }
         img[x + imgx * (imgy - y - 1)] = color;
      }
   }

   // Write file out as a 24 bit uncompressed TGA.
   // http://en.wikipedia.org/wiki/Truevision_TGA
   let ref mut fout = File::create(&Path::new("trace.tga")).unwrap();
   // field 1,2,3,4
   fout.write(&[0, 0, 2, 0, 0, 0, 0, 0]).unwrap();
   // field 5
   fout.write_u16::<LittleEndian>(0 as u16).unwrap();
   fout.write_u16::<LittleEndian>(0 as u16).unwrap();
   fout.write_u16::<LittleEndian>(imgx as u16).unwrap();
   fout.write_u16::<LittleEndian>(imgy as u16).unwrap();
   fout.write(&[24, 32]).unwrap();
   // image data
   for pix in img {
      let (p0, p1, p2) = pix;
      fout.write(&[p2, p1, p0]).unwrap();
   }
}
