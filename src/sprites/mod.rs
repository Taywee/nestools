//! Tools for working with sprites.  This is primarily used to compile PNG sprite sheets into
//! nametables and generate symbols for them.
//!

pub mod sheet;

/// A single tile, with an optional name
pub struct Tile {
    /// The name of the tile.  This is the same given directly to the define for generated C and
    /// ASM headers.  If this is None, no name is output.
    pub name: Option<String>,

    /// The actual data in its raw chr form.
    pub data: [u8; 16],
}

impl Tile {
    pub fn iter(&self) -> TileIterator {
        TileIterator {
            row: 0,
            tile: self,
        }
    }
}

/// An iterator for iterating through rows of a tile.
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

/// An iterator for iterating through a tile row.
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

/// A pattern table of tiles, in two pages
pub struct PatternTable {
    pub left: Vec<Tile>,
    pub right: Vec<Tile>,
}
