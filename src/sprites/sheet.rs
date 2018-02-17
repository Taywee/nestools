//! This module assists in compiling manageable png sprite sheets into NES-friendly CHR files and
//! sets of headers that work with them.
//!
//! An example input might look something like this: 
//!
//! ```json
//! {
//!   "left": [
//!     {
//!       "Animation": {
//!         "file": "first.png",
//!         "frame_height": 2,
//!         "frame_width": 2,
//!         "frames": 4,
//!         "name": "first"
//!       }
//!     },
//!     {
//!       "Slice": {
//!         "file": "second.png",
//!         "height": 4,
//!         "name": "second",
//!         "slices": [
//!           [15, 12, 7],
//!           [0, 4, 5]
//!         ],
//!         "width": 4
//!       }
//!     }
//!   ],
//!   "right": [
//!     {
//!       "Simple": {
//!         "file": "third.png",
//!         "height": 8,
//!         "name": "third",
//!         "width": 8
//!       }
//!     },
//!     {
//!       "Animation": {
//!         "file": "fourth.png",
//!         "frame_height": 4,
//!         "frame_width": 3,
//!         "frames": 3,
//!         "name": "fourth"
//!       }
//!     }
//!   ]
//! }
//! ```

/// A very simple sprite type.  Simply pulls in the tiles and numbers them.
///
/// Generates defines named $name_$tilenumber.  The tilenumber is relative to the entire image.
#[derive(Serialize, Deserialize, Debug)]
pub struct Simple {
    /// The png filename to pull in
    pub file: String,

    /// The name, used for generation of the C and ASM definition headers
    pub name: String,

    /// The width of the whole image, in 8x8 tiles
    pub width: u8,

    /// The height of the whole image, in 8x8 tiles
    pub height: u8,
}

/// An animation sprite type.  Used for creating sprites that are composed of multiple equal-sized
/// frames.
///
/// Frames will be layed out in memory one frame after another.  Because of this, each frame is
/// composed of frame_width * frame_height tiles, with each tile separated from the same file in
/// the next or previous frame by that width.  Generates defines named
/// $name_$framenumber_$tilenumber.  Each tilenumber is relative to its frame.
#[derive(Serialize, Deserialize, Debug)]
pub struct Animation {
    /// The png filename to pull in
    pub file: String,

    /// The name, used for generation of the C and ASM definition headers
    pub name: String,

    /// The width of a single frame, in 8x8 tiles
    pub frame_width: u8,

    /// The height of a single frame, in 8x8 tiles
    pub frame_height: u8,

    /// The number of frames in this animation
    pub frames: u8,
}

/// A sprite composed of "slices", for more fine-tuned separation of sprites (especially for
/// partial animation) without having to drop into raw numbering.
///
/// Generates defines named $name_$slicenumber_$tilenumber.  Each tilenumber is specific to its
/// slice, and is ordered in the order given in the slices.
#[derive(Serialize, Deserialize, Debug)]
pub struct Slice {
    /// The png filename to pull in
    pub file: String,

    /// The name, used for generation of the C and ASM definition headers
    pub name: String,

    /// The width of the whole image, in 8x8 tiles
    pub width: u8,

    /// The height of the whole image, in 8x8 tiles
    pub height: u8,

    /// Slices, indexed in row-major order
    pub slices: Vec<Vec<u16>>,
}

/// An enum used for differentiating sheets by type
#[derive(Serialize, Deserialize, Debug)]
pub enum Sheet {
    Animation(Animation),
    Slice(Slice),
    Simple(Simple),
}

/// A sheet pattern table, for organizing sprite sheets by order into their appropriate table
/// section.
#[derive(Serialize, Deserialize, Debug)]
pub struct SheetPatternTable {
    pub left: Vec<Sheet>,
    pub right: Vec<Sheet>,
}

use super::{Tile, Error};

pub trait LoadTiles {
    /// width of the sheet in tiles
    fn sheet_width(&self) -> u8;

    /// height of the sheet in tiles
    fn sheet_height(&self) -> u8;

    /// path to the image
    fn image_path<'a>(&'a self) -> &'a str;

    /// name of the image
    fn name<'a>(&'a self) -> &'a str;

    fn load_tiles(&self) -> Result<Vec<Tile>, Error> {
        let width = self.sheet_width();
        let height = self.sheet_height();

        let mut image_result = ::lodepng::decode_file(self.image_path(), ::lodepng::ffi::ColorType::PALETTE, 8);
        match image_result {
            Ok(image) => {
                if let ::lodepng::Image::RawData(bitmap) = image {
                    if bitmap.width < width as usize * 8 {
                        return Err(Error::DimensionsError(
                                format!("Image too thin, need {}, got {}.", width * 8, bitmap.width)
                                ));
                    } else if bitmap.height < height as usize * 8 {
                        return Err(Error::DimensionsError(
                                format!("Image too short, need {}, got {}.", height * 8, bitmap.height)
                                ));
                    }
                    // Test for out of bound pixels
                    if let Some(item) = bitmap.buffer.iter().find(|&&item| item > 3) {
                        return Err(Error::PaletteError(
                                format!("Image has a byte out of bounds; needs to be under 4, got {}.", item)
                                ));
                    }

                    let mut tiles = Vec::new();

                    for row in 0..height {
                        let x_offset = bitmap.width * row as usize;
                        for column in 0..width {
                            let offset = x_offset + column as usize * 64;
                            let slice = &bitmap.buffer[offset..(offset + 64)];
                            tiles.push(Tile::from_bytes(slice, Some(self.name()))?);
                        }
                    }
                    Ok(tiles)
                } else {
                    return Err(Error::FormatError(String::from("Image format was incorrect")));
                }
            },
            Err(err) => {
                Err(Error::PNGError(err))
            },
        }
    }
}

impl LoadTiles for Simple {
    fn sheet_width(&self) -> u8 {
        self.width
    }
    fn sheet_height(&self) -> u8 {
        self.height
    }
    fn image_path<'a>(&'a self) -> &'a str {
        &self.file
    }
    fn name<'a>(&'a self) -> &'a str {
        &self.name
    }
}

impl LoadTiles for Animation {
    fn sheet_width(&self) -> u8 {
        self.frame_width * self.frames
    }
    fn sheet_height(&self) -> u8 {
        self.frame_height
    }
    fn image_path<'a>(&'a self) -> &'a str {
        &self.file
    }
    fn name<'a>(&'a self) -> &'a str {
        &self.name
    }
}

impl LoadTiles for Slice {
    fn sheet_width(&self) -> u8 {
        self.width
    }
    fn sheet_height(&self) -> u8 {
        self.height
    }
    fn image_path<'a>(&'a self) -> &'a str {
        &self.file
    }
    fn name<'a>(&'a self) -> &'a str {
        &self.name
    }
}
