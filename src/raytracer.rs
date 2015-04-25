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

use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use camera::Camera;
use light::{PointLight, Light};
use geometry::{Geometry, ShadeRec, Sphere};
use color::Color;
use vector::dot;
use byteorder::{LittleEndian, WriteBytesExt};
use toml::Parser;

pub struct RayTracer {
    camera: Camera,
    image: Vec<(u8, u8, u8)>,
    lights: Vec<Box<Light>>,
    scene: Vec<Box<Geometry>>,
}

impl RayTracer {

    pub fn new(camera: Camera) -> RayTracer {
        RayTracer {
            camera: camera,
            image: vec![(0, 0, 0); camera.screen_width() * camera.screen_height()],
            lights: Vec::new(),
            scene: Vec::new(),
        }
    }

    pub fn import_scene(&mut self, filename: String) {

        let path = Path::new(&filename);
        let mut fin = File::open(&path).unwrap();
        let mut scene = String::new();
        fin.read_to_string(&mut scene).unwrap();

        let mut p = Parser::new(&scene);
        let toml = p.parse().unwrap();
        let objects = toml.get("object").unwrap().as_slice().unwrap();

        for obj in objects {
            let t = obj.lookup("type").unwrap().as_str().unwrap();
            match t {
                "sphere" => {
                    match Sphere::import(obj) {
                        Ok(s) => self.scene.push(Box::new(s)),
                        Err(e) => {println!("Error parsing sphere - {}", e)},
                    }
                }
                "point_light" => {
                    match PointLight::import(obj) {
                        Ok(pl) => self.lights.push(Box::new(pl)),
                        Err(e) => {println!("Error parsing sphere - {}", e)},
                    }
                }

                _ => { println!("Unknown object {} ignored.", t); }
           }
        }
    }

    pub fn render(self)  {
        // Write file out as a 24 bit uncompressed TGA.
        // http://en.wikipedia.org/wiki/Truevision_TGA
        let ref mut fout = File::create(&Path::new("trace.tga")).unwrap();
        // field 1,2,3,4
        fout.write(&[0, 0, 2, 0, 0, 0, 0, 0]).unwrap();
        // field 5
        fout.write_u16::<LittleEndian>(0 as u16).unwrap();
        fout.write_u16::<LittleEndian>(0 as u16).unwrap();
        fout.write_u16::<LittleEndian>(self.camera.screen_width() as u16).unwrap();
        fout.write_u16::<LittleEndian>(self.camera.screen_height() as u16).unwrap();
        fout.write(&[24, 32]).unwrap();
        // image data
        for pix in self.image {
            let (p0, p1, p2) = pix;
            fout.write(&[p2, p1, p0]).unwrap();
        }
    }

    pub fn trace(&mut self) {
        for y in (0 .. self.camera.screen_height()) {
            for x in (0 .. self.camera.screen_width()) {
                let mut tmax = 10000.0;
                let mut hit: Option<ShadeRec> = None;

                let r = self.camera.ray(x as f32, y as f32);

                // find nearest intersection
                for sh in self.scene.iter() {
                    match sh.intersect(r, 0.00001, tmax) {
                        None => { },
                        Some(hr) => { tmax = hr.t; hit = Some(hr); },
                    }
                }

                // calc simple shading for now
                let c = match hit {
                    Some(h) => h.color * dot(-r.direction, h.normal),
                    None => Color::new(0.0, 0.4, 0.8),
                };

                let color = c.to_bytes();

                self.image[x + self.camera.screen_width() * (self.camera.screen_height() - y - 1)] = color;
            }
        }
    }
}




