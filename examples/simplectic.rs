// Copyright 2015 the simplectic-noise developers. For a full listing of the
// authors, refer to the AUTHORS file at the top-level directory of this
// distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of using simplectic noise

#![allow(unstable)]

extern crate image;
extern crate simplectic_noise;

use simplectic_noise::{simplectic2, simplectic3, simplectic4, Seed, Point2};

use std::num::{self, Float, NumCast};

fn main() {
    render_png("simplectic2.png", &Seed::new(0), 256, 256, scaled_simplectic2);
    render_png("simplectic3.png", &Seed::new(0), 256, 256, scaled_simplectic3);
    render_png("simplectic4.png", &Seed::new(0), 256, 256, scaled_simplectic4);

    println!("\nGenerated simplectic2.png, simplectic3.png and simplectic4.png");
}

fn scaled_simplectic2(seed: &Seed, point: &Point2<f32>) -> f32 {
    simplectic2(seed, &[point[0] / 64.0, point[1] / 64.0])
}

fn scaled_simplectic3(seed: &Seed, point: &Point2<f32>) -> f32 {
    simplectic3(seed, &[point[0] / 64.0, point[1] / 64.0, 0.0])
}

fn scaled_simplectic4(seed: &Seed, point: &Point2<f32>) -> f32 {
    simplectic4(seed, &[point[0] / 64.0, 0.0, 0.0, point[1] / 64.0])
}

fn clamp<F: Float>(val: F, min: F, max: F) -> F {
    match () {
        _ if val < min => min,
        _ if val > max => max,
        _ => val,
    }
}

pub fn cast<T: NumCast, U: NumCast>(x: T) -> U {
    num::cast(x).unwrap()
}

pub fn render_png<T, F>(filename: &str, seed: &Seed, width: u32, height: u32, func: F) where
    T: Float + NumCast,
    F: Fn(&Seed, &Point2<T>) -> T,
{
    let mut pixels = Vec::with_capacity((width * height) as usize);

    for y in (0..height) {
        for x in (0..width) {
            let value: f32 = cast(func(seed, &[cast(x), cast(y)]));
            pixels.push(cast(clamp(value * 0.5 + 0.5, 0.0, 1.0) * 255.0));
        }
    }

    let _ = image::save_buffer(&Path::new(filename), &*pixels, width, height, image::ColorType::Grey(8));
}
