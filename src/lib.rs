
/// 3D Perlin Noise Generator
///
/// Based on http://mrl.nyu.edu/~perlin/noise/
///
/// Adopting from alterebro/perlin-noise-3d
/// Which was adapted from runemadsen/rune.noise.js
/// Which was adapted from P5.js
/// Which was adapted from PApplet.java
/// which was adapted from toxi
/// which was adapted from the german demo group farbrausch as used in their demo "art": http://www.farb-rausch.de/fr010src.zip

use std::f32::consts::PI;
use rand::prelude::*;
use rand_seeder::{Seeder};
use rand_pcg::Pcg64;

const PERLIN_OCTAVES: i32 = 4; // default to medium smooth
const PERLIN_AMP_FALLOFF: f32 = 0.5; // 50% reduction/octave

const PERLIN_YWRAPB: i32 = 4;
const PERLIN_YWRAP: i32 = 1<<PERLIN_YWRAPB; // 8
const PERLIN_ZWRAPB: i32 = 8;
const PERLIN_ZWRAP: i32 = 1<<PERLIN_ZWRAPB; // 256
const PERLIN_SIZE: i32 = 4095;
const DEG_TO_RAD: f32 = PI/180.0;

pub struct NoiseGenerator {
  sincos_length: i32,
  perlin_pi: i32,
  coslut: Vec<f32>,
  perlin: Vec<f32>,
}

impl NoiseGenerator {

  /// The constructor builds the lookup tables and the random data field.
  /// All of the heavy lifting is done here where the generator is built.
  pub fn new(seed: &str) -> Self {
    let precision: f32 = 0.5;
    let length: i32 = (360.0 / precision).floor() as i32;
    let mut coslut: Vec<f32> = vec![0.0; length as usize];

    let mut perlin_pi = length;
    perlin_pi >>= 1;

    (0..length).for_each(|i| {
      coslut[i as usize] = ((i as f32) * DEG_TO_RAD * precision).cos();
    });

    let mut perlin = vec![0.0; (PERLIN_SIZE+1) as usize];
    let mut random_generator: Pcg64 = Seeder::from(seed).make_rng();
    random_generator.try_fill(&mut perlin[..]).expect("Could not create random array for some reason.");

    Self {
      sincos_length: length,
      coslut: coslut,
      perlin_pi: perlin_pi,
      perlin: perlin,
    }
  }

  /// Get the perlin noise value at (x,y,z)
  pub fn get(&self, x:f32, y:f32, z:f32) -> f32 {
    let mut xi: i32 = x.floor() as i32;
    let mut yi: i32 = y.floor() as i32;
    let mut zi: i32 = z.floor() as i32;

    let mut xf = x - xi as f32;
    let mut yf = y - yi as f32;
    let mut zf = z - zi as f32;

    let mut rxf: f32;
    let mut ryf: f32;
    let mut r = 0.0;
    let mut ampl = 0.5;

    let mut n1: f32;
    let mut n2: f32;
    let mut n3: f32;

    for _o in 0..PERLIN_OCTAVES {
      let mut octive_f = xi+(yi<<PERLIN_YWRAPB)+(zi<<PERLIN_ZWRAPB);

      rxf = self.noise_fsc(xf);
      ryf = self.noise_fsc(yf);

      n1 = self.perlin[(octive_f & PERLIN_SIZE) as usize];
      n1 += rxf * (self.perlin[((octive_f + 1) & PERLIN_SIZE) as usize] - n1 );
      n2 = self.perlin[((octive_f + PERLIN_YWRAP) & PERLIN_SIZE) as usize];
      n2 += rxf * (self.perlin[((octive_f + PERLIN_YWRAP + 1) & PERLIN_SIZE) as usize] - n2);
      n1 += ryf * (n2-n1);

      octive_f += PERLIN_ZWRAP;
      n2 = self.perlin[(octive_f & PERLIN_SIZE) as usize];
      n2 += rxf * (self.perlin[((octive_f + 1) & PERLIN_SIZE) as usize] - n2);
      n3 = self.perlin[((octive_f + PERLIN_YWRAP) & PERLIN_SIZE) as usize];
      n3 += rxf * (self.perlin[((octive_f + PERLIN_YWRAP + 1) & PERLIN_SIZE) as usize] - n3);
      n2 += ryf * (n3-n2);

      n1 += self.noise_fsc(zf) * (n2-n1);
      r += n1 * ampl;
      ampl *= PERLIN_AMP_FALLOFF;

      xi <<= 1;
      xf *= 2.0;
      yi <<= 1;
      yf *= 2.0;
      zi <<= 1;
      zf *= 2.0;

      if xf >= 1.0 { xi += 1; xf -= 1.0; }
      if yf >= 1.0 { yi += 1; yf -= 1.0; }
      if zf >= 1.0 { zi += 1; zf -= 1.0; }
    }

    return r;
  }

  // Use the lookup table to get a fast cosine.
  fn noise_fsc(&self, i: f32) -> f32 {
    let pi = self.perlin_pi as f32;
    let length = self.sincos_length as f32;
    let index: usize = ((i * pi) % length) as usize;

    0.5 * (1.0 - self.coslut[index])
  }
}
