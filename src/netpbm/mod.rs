use std::fs::File;
use std::io::prelude::*;

pub fn create_ppm(name: &str, pixels: &Vec<u8>, width: u32, height: u32) -> std::io::Result<()> {
    let header = format!("{}\n{} {}\n{}\n", "P6", width, height, 255);

    let mut file = File::create(name)?;
    file.write_all(header.as_bytes())?;
    file.write_all(pixels)?;

    Ok(())
}