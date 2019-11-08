use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;

    let mut file = File::create("image.ppm")?;

    // Write PPM file header.
    file.write_fmt(format_args!("P3\n{}\n{}\n255\n", nx, ny));

    for y in (0..ny).rev() {
        for x in 0..nx {
            let r = x as f64 / nx as f64;
            let g = y as f64 / ny as f64;
            let b = 0.2;
            let ir = (255.99 * r) as i64;
            let ig = (255.99 * g) as i64;
            let ib = (255.99 * b) as i64;
            file.write_fmt(format_args!("{} {} {}\n", ir, ig, ib));
        }
    }

    Ok(())
}
