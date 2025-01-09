use crate::{Colour,Interval};
use std::fs::File;
use std::io::{BufWriter, Write};

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return f64::sqrt(linear_component);
    }

    0.0
}

pub fn write_colour(output_buffer: &mut BufWriter<File>, colour: &Colour) {
    let r = linear_to_gamma(*colour.r());
    let g = linear_to_gamma(*colour.g());
    let b = linear_to_gamma(*colour.b());

    let intensity = Interval::build(0.0, 0.999);
    let r = (256.0 * intensity.clamp(r)) as u32;
    let g = (256.0 * intensity.clamp(g)) as u32;
    let b = (256.0 * intensity.clamp(b)) as u32;

    let _bytes_written = output_buffer
        .write(format!("{r} {g} {b}\n").as_bytes())
        .unwrap();
}
