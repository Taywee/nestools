//! Tools for working with sprites.  This is primarily used to compile PNG sprite sheets into
//! nametables and generate symbols for them.
//!

pub mod sheet;

/// A single tile, with an optional name
pub struct Tile {
    /// The name of the tile.  This is the same given directly to the define for generated C and
    /// ASM headers.  If this is None, no name is output.
    pub name: Option<String>,

    /// The actual data.  Each pixel is 0-4.  This is in row-major order.
    pub data: [[u8; 8]; 8],
}

/// A pattern table of tiles, in two pages
pub struct PatternTable {
    pub left: Vec<Tile>,
    pub right: Vec<Tile>,
}
