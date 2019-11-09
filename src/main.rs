use std::fs::File;
use std::io::prelude::*;

mod vec3;
use vec3::Vec3;

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;

    let file_name = "image.ppm";

    let mut file = File::create(file_name)?;

    // Write PPM file header.
    file.write_fmt(format_args!("P3\n{}\n{}\n255\n", nx, ny))?;

    for y in (0..ny).rev() {
        for x in 0..nx {
            let r = f64::from(x) / f64::from(nx);
            let g = f64::from(y) / f64::from(ny);
            let b = 0.2;
            let pixel = Vec3 {
                x: (255.99 * r),
                y: (255.99 * g),
                z: (255.99 * b)
            };
            file.write_fmt(format_args!("{} {} {}\n",
                pixel.r() as i64,
                pixel.g() as i64,
                pixel.b() as i64,
            ))?;
        }
    }

    Ok(())
}
