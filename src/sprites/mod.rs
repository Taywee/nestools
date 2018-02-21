//! Tools for working with sprites.  This is primarily used to compile PNG sprite sheets into
//! nametables and generate symbols for them.
//!

pub mod sheet;
use self::sheet::SheetPatternTable;
use self::sheet::Sheet::*;
use std::error;
use std::fmt;
use std::io::Write;

type PNGError = ::lodepng::ffi::Error;

/// Global sprite error type.  Rolls up all errors that can occur loading and manipulating sprites.
/// Can also simply pass along lodepng::ffi::Error
#[derive(Debug)]
pub enum Error {
    /// If some io error occured opening or reading the image.  This just wraps lodepng::ffi::Error
    PNGError(PNGError),

    /// If the image was not big enough or too big
    DimensionsError(String),

    /// If the image is not a pallete  of exactly 4 colors
    PaletteError(String),

    /// If the image is not a pallete format
    FormatError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output: &str = match self {
            &Error::PNGError(ref err) => err.as_str(),
            &Error::DimensionsError(ref err) => &err,
            &Error::PaletteError(ref err) => &err,
            &Error::FormatError(ref err) => &err,
        };
        write!(f, "{}", output)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::PNGError(ref err) => err.as_str(),
            &Error::DimensionsError(ref err) => &err,
            &Error::PaletteError(ref err) => &err,
            &Error::FormatError(ref err) => &err,
        }
    }
}

/// A single tile, with an optional name
#[derive(Clone, Debug)]
pub struct Tile {
    /// The name of the tile.  This is the same given directly to the define for generated C and
    /// ASM headers.  If this is None, no name is output.
    pub name: Option<String>,

    /// The actual data in its raw chr form.
    pub data: [u8; 16],
}

impl Tile {
    /// Iterate over rows.  Each iteration is a row, and each row is an iterator over bytes.
    ///
    /// Example, using [nesdev sprite](https://wiki.nesdev.com/w/index.php/PPU_pattern_tables):
    ///
    /// ```
    /// use nestools::sprites::Tile;
    ///
    /// let tile = Tile {
    ///     name: None,
    ///     data: [
    ///         0x41,
    ///         0xC2,
    ///         0x44,
    ///         0x48,
    ///         0x10,
    ///         0x20,
    ///         0x40,
    ///         0x80,
    ///         0x01,
    ///         0x02,
    ///         0x04,
    ///         0x08,
    ///         0x16,
    ///         0x21,
    ///         0x42,
    ///         0x87,
    ///     ]
    /// };
    /// let pixels: Vec<Vec<u8>> = tile.iter().map(|row| row.collect()).collect();
    /// assert_eq!(pixels, [
    ///     [0, 1, 0, 0, 0, 0, 0, 3],
    ///     [1, 1, 0, 0, 0, 0, 3, 0],
    ///     [0, 1, 0, 0, 0, 3, 0, 0],
    ///     [0, 1, 0, 0, 3, 0, 0, 0],
    ///     [0, 0, 0, 3, 0, 2, 2, 0],
    ///     [0, 0, 3, 0, 0, 0, 0, 2],
    ///     [0, 3, 0, 0, 0, 0, 2, 0],
    ///     [3, 0, 0, 0, 0, 2, 2, 2],
    /// ]);
    /// ```
    pub fn iter(&self) -> TileIterator {
        TileIterator {
            row: 0,
            tile: self,
        }
    }

    /// Convert index bytes (like delivered from a png) into a Tile object
    ///
    /// Example, using [nesdev sprite](https://wiki.nesdev.com/w/index.php/PPU_pattern_tables):
    ///
    /// ```
    /// use nestools::sprites::Tile;
    ///
    /// let tile = Tile::from_bytes(&[
    ///     0, 1, 0, 0, 0, 0, 0, 3,
    ///     1, 1, 0, 0, 0, 0, 3, 0,
    ///     0, 1, 0, 0, 0, 3, 0, 0,
    ///     0, 1, 0, 0, 3, 0, 0, 0,
    ///     0, 0, 0, 3, 0, 2, 2, 0,
    ///     0, 0, 3, 0, 0, 0, 0, 2,
    ///     0, 3, 0, 0, 0, 0, 2, 0,
    ///     3, 0, 0, 0, 0, 2, 2, 2,
    /// ], None).unwrap();
    /// assert_eq!(
    ///     tile.data,
    ///     [
    ///         0x41,
    ///         0xC2,
    ///         0x44,
    ///         0x48,
    ///         0x10,
    ///         0x20,
    ///         0x40,
    ///         0x80,
    ///         0x01,
    ///         0x02,
    ///         0x04,
    ///         0x08,
    ///         0x16,
    ///         0x21,
    ///         0x42,
    ///         0x87,
    ///     ]
    /// );
    /// ```
    pub fn from_bytes(bytes: &[u8], name: Option<&str>) -> Result<Tile, Error> {
        if bytes.len() != 64 {
            return Err(Error::DimensionsError(
                format!("Need bytes array of size 64, got {}", bytes.len())
                ));
        }

        if let Some(item) = bytes.iter().find(|&&item| item > 3) {
            return Err(Error::PaletteError(
                format!("Byte out of bounds; needs to be under 4, got {}.", item)
                ));
        }

        // Parallel bytes structure, in form of [0, 8], [1, 9], [2, 10]...
        let bytepairs: Vec<[u8; 2]> = bytes.chunks(8).map(|row| {
            let byte1 = 
                (row[0] & 1) << 7
                | (row[1] & 1) << 6
                | (row[2] & 1) << 5
                | (row[3] & 1) << 4
                | (row[4] & 1) << 3
                | (row[5] & 1) << 2
                | (row[6] & 1) << 1
                | row[7] & 1;
            let byte2 = 
                (row[0] & 2) << 6
                | (row[1] & 2) << 5
                | (row[2] & 2) << 4
                | (row[3] & 2) << 3
                | (row[4] & 2) << 2
                | (row[5] & 2) << 1
                | row[6] & 2
                | (row[7] & 2) >> 1;
            [byte1, byte2]
        }).collect();
        // Invert pairs into two full lists paired properly
        let first_bytes: Vec<u8> = bytepairs.iter().map(|pair| pair.first().unwrap()).cloned().collect();
        let second_bytes: Vec<u8> = bytepairs.iter().map(|pair| pair.last().unwrap()).cloned().collect();
        let bytes: Vec<u8> = first_bytes.iter().chain(second_bytes.iter()).cloned().collect();
        let mut data: [u8; 16] = [0; 16];
        data.clone_from_slice(&bytes);

        let new_name = match name {
            Some(name) => Some(String::from(name)),
            None => None
        };
        Ok(Tile {
            name: new_name,
            data,
        })
    }
}

/// An iterator for iterating through rows of a tile.  This is created through a call of
/// the [`iter`] method on [`Tile`].
///
/// [`iter`]: struct.Tile.html#method.iter
/// [`Tile`]: struct.Tile.html
pub struct TileIterator<'a> {
    row: u8,
    tile: &'a Tile,
}

impl<'a> Iterator for TileIterator<'a> {
    type Item = TileRowIterator;

    fn next(&mut self) -> Option<TileRowIterator> {
        let row = self.row as usize;
        self.row += 1;
        if row > 7 {
            return None;
        }
        let byte1 = self.tile.data[row];
        let byte2 = self.tile.data[row + 8];
        Some(TileRowIterator {
            column: 0,
            bytes: (byte1, byte2),
        })
    }
}

/// An iterator for iterating through a tile row.  This is yielded from iterations on
/// [`TileIterator`], which is created through a call of the [`iter`] method on [`Tile`].
///
/// [`TileIterator`]: struct.TileIterator.html
/// [`iter`]: struct.Tile.html#method.iter
/// [`Tile`]: struct.Tile.html

pub struct TileRowIterator {
    column: u8,
    bytes: (u8, u8),
}

impl Iterator for TileRowIterator {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let column = self.column;
        self.column += 1;
        if column > 7 {
            return None;
        }
        let andmask: u8 = 1 << (7 - column);
        let first = (andmask & self.bytes.0) >> (7 - column);
        // Shift one fewer for combination.  we do this instead of (6 - column) to avoid negatives
        let second = (andmask & self.bytes.1) >> (7 - column) << 1;

        Some(first | second)
    }
}

/// A pattern table of tiles, in two pages.
pub struct PatternTable {
    pub left: Vec<Tile>,
    pub right: Vec<Tile>,
}

impl PatternTable {
    /// Loads in a SheetPatternTable, and uses it to create a PatternTable.
    pub fn from_sheet_pattern_table(sheet_table: SheetPatternTable) -> Result<PatternTable, Error> {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for sheet in sheet_table.left {
            let tiles = match sheet {
                Animation(sprite) => sprite.pull_tiles(),
                Slice(sprite) => sprite.pull_tiles(),
                Simple(sprite) => sprite.pull_tiles(),
            }?;
            left.extend(tiles.into_iter());
        }
        for sheet in sheet_table.right {
            let tiles = match sheet {
                Animation(sprite) => sprite.pull_tiles(),
                Slice(sprite) => sprite.pull_tiles(),
                Simple(sprite) => sprite.pull_tiles(),
            }?;
            right.extend(tiles.into_iter());
        }

        if left.len() > 256 {
            return Err(Error::DimensionsError(format!(
                "left table contained too many tiles. Can not exceed 256, but has {}",
                left.len())));
        }
        if right.len() > 256 {
            return Err(Error::DimensionsError(format!(
                "right table contained too many tiles. Can not exceed 256, but has {}",
                right.len())));
        }
        let blank = Tile {name: None, data: [0u8; 16]};

        // Pattern table must be tightly packed
        while left.len() < 256 {
            left.push(blank.clone());
        }
        while right.len() < 256 {
            right.push(blank.clone());
        }

        Ok(PatternTable {
            left,
            right
        })
    }

    pub fn write<T: Write>(&self, writer: &mut T) {
        for ref tile in &self.left {
            writer.write(&tile.data);
        }
        for ref tile in &self.right {
            writer.write(&tile.data);
        }
    }
}
