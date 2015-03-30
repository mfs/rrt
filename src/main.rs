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

use std::fs::File;
use std::path::Path;

fn main() {

   let imgx = 500;
   let imgy = 500;

   let mut img = image::ImageBuffer::new(imgx, imgy);

   for y in (0 .. imgx) {
      for x in (0 .. imgy) {
         let px: u8 = ((x * 255 )as f32 / imgx as f32) as u8;
         let py: u8 = ((y * 255 )as f32 / imgy as f32) as u8;
         img.put_pixel(x, y, image::Rgb([px, 0, py]));
      }
   }

   let ref mut fout = File::create(&Path::new("trace.png")).unwrap();
   match image::ImageRgb8(img).save(fout, image::PNG) {
      Ok(_) => (),
      Err(err) => {println!("{}", err)},
   }
}
