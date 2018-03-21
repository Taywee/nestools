//! This module assists in compiling manageable png sprite sheets into NES-friendly CHR files and
//! sets of headers that work with them.

use super::{Tile, Error};

use std::iter;

/// The simplest sprite type.  Fills the tile with its value
///
/// Generates defines named $name.
#[derive(Serialize, Deserialize, Debug)]
pub struct Fill {
    /// The value to fill in; must be 0-3, inclusive
    pub value: u8,

    /// The name, used for generation of the C and ASM definition headers
    pub name: String,

    /// The number of tiles to generate
    pub count: usize,
}

impl Fill {
    /// Pulls the named tiles out of this simple sprite
    pub fn pull_tiles(&self) -> Result<Vec<Tile>, Error> {
        if self.value > 3 {
            return Err(Error::FormatError(
                format!("Value must be between 0 and 3, but was {}", self.value)
            ));
        }
        let data: Vec<u8> = iter::repeat(self.value).take(64).collect();
        let tile = Tile::from_bytes(&data, Some(&self.name))?;
        let output: Vec<Tile> = iter::repeat(tile).take(self.count).collect();

        Ok(output)
    }
}

/// A very simple sprite type.  Simply pulls in the tiles and numbers them.
///
/// Generates defines named $name_$x_$y.  The tilenumber is relative to the top left of the image.
#[derive(Serialize, Deserialize, Debug)]
pub struct Simple {
    /// The png filename to pull in
    pub file: String,

    /// The name, used for generation of the C and ASM definition headers
    pub name: String,

    /// The width of the whole image, in 8x8 tiles
    pub width: usize,

    /// The height of the whole image, in 8x8 tiles
    pub height: usize,
}

impl Simple {
    /// Pulls the named tiles out of this simple sprite
    pub fn pull_tiles(&self) -> Result<Vec<Tile>, Error> {
        let tiles = self.load_tiles()?;

        let mut output = Vec::new();

        let width = self.width;
        let height = self.height;

        for y in 0..height {
            for x in 0..width {
                let mut tile_number = (y * width) + x;
                let mut tile = tiles[tile_number].clone();
                tile.name = Some(format!("{name}_{x}_{y}",
                     name = self.name,
                     x = x,
                     y = y,
                     ));
                output.push(tile);
            }
        }
        Ok(output)
    }
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
    pub frame_width: usize,

    /// The height of a single frame, in 8x8 tiles
    pub frame_height: usize,

    /// The number of frames in this animation
    pub frames: usize,
}

impl Animation {
    /// Pulls the named tiles out of this animation
    pub fn pull_tiles(&self) -> Result<Vec<Tile>, Error> {
        let tiles = self.load_tiles()?;

        let mut output = Vec::new();
        let sheet_width = self.frame_width * self.frames;

        for frame in 0..self.frames {
            // The frame offset is a simple x shift
            let frame_offset = frame * self.frame_width;
            for y in 0..self.frame_height {
                // Y offset is a shift in the total width of the image
                let y_offset = y * sheet_width;
                for x in 0..self.frame_width {
                    // X offset is another simple x shift
                    let mut tile_number = frame_offset + y_offset + x;
                    let mut tile = tiles[tile_number].clone();
                    // Number of tile in this frame
                    let frame_tile_number = y * self.frame_width + x;
                    tile.name = Some(format!("{name}_{frame}_{tile}",
                        name = self.name,
                        frame = frame,
                        tile = frame_tile_number,
                        ));
                    output.push(tile);
                }
            }
        }
        Ok(output)
    }
}

/// A sprite composed of "slices", for more fine-tuned separation of sprites (especially for
/// partial animation) without having to drop into raw numbering.  It's basically identical to
/// Simple, but allowing you to organize it better yourself.
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
    pub width: usize,

    /// The height of the whole image, in 8x8 tiles
    pub height: usize,

    /// Slices, indexed in row-major order
    pub slices: Vec<Vec<usize>>,
}

impl Slice {
    /// Pulls the named tiles out of this slice
    pub fn pull_tiles(&self) -> Result<Vec<Tile>, Error> {
        let tiles = self.load_tiles()?;

        let mut output = Vec::new();

        for (slice_number, slice) in self.slices.iter().enumerate() {
            for (tile_number, slice_tile) in slice.iter().enumerate() {
                let mut tile = tiles[*slice_tile].clone();
                tile.name = Some(format!("{name}_{slicenumber}_{tilenumber}",
                     name = self.name,
                     slicenumber = slice_number,
                     tilenumber = tile_number,
                     ));
                output.push(tile);
            }
        }
        Ok(output)
    }
}

/// An enum used for differentiating sheets by type
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Sheet {
    Animation(Animation),
    Slice(Slice),
    Simple(Simple),
    Fill(Fill),
}

/// A sheet pattern table, for organizing sprite sheets by order into their appropriate table
/// section.
#[derive(Serialize, Deserialize, Debug)]
pub struct SheetPatternTable {
    pub left: Vec<Sheet>,
    pub right: Vec<Sheet>,
}

pub trait LoadTiles {
    /// width of the sheet in tiles
    fn sheet_width(&self) -> usize;

    /// height of the sheet in tiles
    fn sheet_height(&self) -> usize;

    /// path to the image
    fn image_path<'a>(&'a self) -> &'a str;

    /// name of the image
    fn name<'a>(&'a self) -> &'a str;

    /// Simply loads the image and pulls in the PNG data as tiles.
    ///
    /// No reordering or manipulation is done; they are returned in row-major order, and any
    /// specific details in respect to things like animations or slices are taken into
    /// consideration.  Those details are managed at a different level.
    fn load_tiles(&self) -> Result<Vec<Tile>, Error> {
        let width = self.sheet_width();
        let height = self.sheet_height();

        let image_result = ::lodepng::decode_file(self.image_path(), ::lodepng::ffi::ColorType::PALETTE, 8);
        match image_result {
            Ok(image) => {
                if let ::lodepng::Image::RawData(bitmap) = image {
                    if bitmap.width < width * 8 {
                        return Err(Error::DimensionsError(
                                format!("Image too thin, need {}, got {}.", width * 8, bitmap.width)
                                ));
                    } else if bitmap.height < height * 8 {
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
                        let y_offset = bitmap.width * row * 8;
                        for column in 0..width {
                            let x_offset = column * 8;

                            let bytes: Vec<u8> = (0..8).flat_map(|line| {
                                let offset = y_offset + x_offset + (line * bitmap.width);
                                &bitmap.buffer[offset..(offset + 8)]
                            }).cloned().collect();

                            tiles.push(Tile::from_bytes(&bytes, Some(self.name()))?);
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
    fn sheet_width(&self) -> usize {
        self.width
    }
    fn sheet_height(&self) -> usize {
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
    fn sheet_width(&self) -> usize {
        self.frame_width * self.frames
    }
    fn sheet_height(&self) -> usize {
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
    fn sheet_width(&self) -> usize {
        self.width
    }
    fn sheet_height(&self) -> usize {
        self.height
    }
    fn image_path<'a>(&'a self) -> &'a str {
        &self.file
    }
    fn name<'a>(&'a self) -> &'a str {
        &self.name
    }
}
