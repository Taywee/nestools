//! Compiles spritesheets, using a meta file, into sets of headers and a chr file.

extern crate nestools;
use nestools::sprites::Tile;

fn main() {
    let tile = Tile {
        name: None,
        data: [
            0x41,
            0xC2,
            0x44,
            0x48,
            0x10,
            0x20,
            0x40,
            0x80,
            0x01,
            0x02,
            0x04,
            0x08,
            0x16,
            0x21,
            0x42,
            0x87,
        ]
    };
    let pixels: Vec<Vec<u8>> = tile.iter().map(|row| row.collect()).collect();
    for row in pixels {
        for pixel in row {
            print!("{}", pixel);
        }
        println!("");
    }
}
