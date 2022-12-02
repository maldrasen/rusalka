
# Rusalka
A 3D Perlin Noise Generator adopted from the Javascript package by Jorge Moreno and translated into Rust.
[https://github.com/alterebro/perlin-noise-3d]

His version was based on the previous adapting done by Rune Madsen on rune.noise.js, which was an adaptation from P5.js done by Daniel Shiffman, which was an adaptation from Processing / PApplet.java done by Ben Fry, which was an adaptation from the contributions by Karsten Schmidt aka toxi adapting the code created by the German demo scene group Farbrausch on their demo production "art" ( fr010src.zip@files.scene.org ) adapted from the original work by Ken Perlin. 

Just keeping this ball rollin'

---

## Usage
This implementation is dead simple. You first create the NoiseGenerator, seeding it with a string. Then get any point in the noise volume with three floats. The example in the main.rs file scales the noise up and prints out a few slices.

```rust
  let generator = NoiseGenerator::new("Rusalka!");
      generator.get(x,y,z);
```
