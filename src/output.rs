use super::Colour;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn write_colour(output_buffer: &mut BufWriter<File>, colour: &Colour) {
    let r = (255.999 * colour.x()) as u8;
    let g = (255.999 * colour.y()) as u8;
    let b = (255.999 * colour.z()) as u8;

    let _bytes_written = output_buffer
        .write(format!("{r} {g} {b}\n").as_bytes())
        .unwrap();
}
