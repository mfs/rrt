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

mod vector;
mod ray;
mod geometry;

use std::fs::File;
use std::path::Path;
use std::io::Write;
use vector::Vector;
use ray::Ray;
use geometry::Sphere;
use geometry::Geometry;
use geometry::Triangle;
use byteorder::{LittleEndian, WriteBytesExt};

fn main() {

   let imgx = 500;
   let imgy = 500;

   let mut img = vec![(0, 0, 0); imgx * imgy];

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
         let mut tmax = 100000.0;
         let mut color = (0, 0, 0);
         let r = Ray { origin: Vector::new(x as f32, y as f32, 0.0), direction: dir };

         for sh in shapes.iter() {
            match sh.intersect(r, 0.00001, tmax) {
               None => {},
               Some(hr) => {
                  tmax = hr.t;
                  color = (hr.color.x as u8, hr.color.y as u8, hr.color.z as u8);
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
      fout.write(&[p0, p1, p2]).unwrap();
   }
}
