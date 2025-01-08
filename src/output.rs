use crate::{Colour,Interval};
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn write_colour(output_buffer: &mut BufWriter<File>, colour: &Colour) {
    let intensity = Interval::build(0.0, 0.999);
    let r = (256.0 * intensity.clamp(*colour.r())) as u32;
    let g = (256.0 * intensity.clamp(*colour.g())) as u32;
    let b = (256.0 * intensity.clamp(*colour.b())) as u32;

    let _bytes_written = output_buffer
        .write(format!("{r} {g} {b}\n").as_bytes())
        .unwrap();
}
