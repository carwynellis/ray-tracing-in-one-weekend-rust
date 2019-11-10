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
            let v = Vec3 {
                x:  f64::from(x) / f64::from(nx),
                y:  f64::from(y) / f64::from(ny),
                z:  0.2,
            };
            let max = 255.99;
            file.write_fmt(format_args!("{} {} {}\n",
                (max * v.r()) as i64,
                (max * v.g()) as i64,
                (max * v.b()) as i64,
            ))?;
        }
    }

    Ok(())
}
