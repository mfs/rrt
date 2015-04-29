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
use std::f32::consts;

pub trait BRDF {
    fn f(&self, sr: &ShadeRec, wi: &Vector, wo: &Vector) -> Color;
    fn rho(&self, sr: &ShadeRec, wo: &Vector) -> Color;
}

#[derive(Debug,Copy,Clone)]
pub struct Lambertian {
    kd: f32,
    cd: Color,
}

impl Lambertian {
    pub fn new(kd: f32, cd: Color) -> Lambertian {
        Lambertian { kd: kd, cd: cd }
    }

    pub fn from_vec(v: Vec<f32>) -> Lambertian {
        Lambertian { kd: v[0], cd: Color::new(v[1], v[2], v[3]) }
    }

    pub fn import(obj: &Value) -> Result<Lambertian, String> {
        let elements = vec!["kd", "cd.0", "cd.1", "cd.2"];
        let mut values = Vec::new();

        for e in elements {
            let value = try!(obj.lookup(e).ok_or("Missing element."));
            values.push(try!(value.as_float().ok_or("Invalid float.")) as f32);
        }

        Ok(Lambertian::from_vec(values))
    }
}

impl BRDF for Lambertian {
    fn f(&self, sr: &ShadeRec, wi: &Vector, wo: &Vector) -> Color {
        self.cd * self.kd * consts::FRAC_1_PI
    }

    fn rho(&self, sr: &ShadeRec, wo: &Vector) -> Color {
        self.cd * self.kd
    }
}
