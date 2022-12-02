use rusalka::NoiseGenerator;

/// Generating a scaled 3 x 10 x 30 volume of perlin noise.
fn main() {
  
  let scale = 0.2;
  let generator = NoiseGenerator::new("Seed can be any string.");
  
  println!("\n=== 3D Perlin Noise ===\n");

  for z in 0..3 {
    for y in -5..5 {
      for x in -15..15 {
        let x_scaled: f32 = scale * x as f32;
        let y_scaled: f32 = scale * y as f32;
        let z_scaled: f32 = scale * z as f32;
        print!("{:.1} ", generator.get(x_scaled, y_scaled, z_scaled));
      }
      print!("\n")
    }
    print!("\n");
  }
}
