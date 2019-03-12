//! This module works with the serialization of the stage format, and allows reading stage source
//! files for export as compressed binary stages.

use std::io::Write;
use std::io::Result;
use std::collections::HashMap;
use std::default::Default;

/// Orientation enum for setting orientation
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum Orientation {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
}

impl Default for Orientation {
    fn default() -> Orientation {
        Orientation::Horizontal
    }
}

/// Top level stage sheet type.  Holds all the data necessary to compile the stage file.
#[derive(Serialize, Deserialize, Debug)]
pub struct Metatile {
    /// Just used to help with some debugging.  Not compiled in at all in any way.
    pub name: String,

    /// Used symbol for data section.
    pub symbol: char,

    /// Actual tiles used
    pub tiles: Vec<u8>,
}

/// Top level stage sheet type.  Holds all the data necessary to compile the stage file.
#[derive(Serialize, Deserialize, Debug)]
pub struct Stage {
    /// Whether the stage is serialized to be a horizontal stage or a vertical one
    #[serde(default)]
    pub orientation: Orientation,

    /// Metatiles, for specifying the sprites used to compose a tile
    pub metatiles: Vec<Metatile>,

    /// The number of tiles to generate
    pub data: String,
}

impl Stage {
    pub fn write_binary(&self, write: &mut Write) -> Result<()> {
        // This stuff so far is building a vector of chars by transposing characters.
        let mut iterators: Vec<_> = self.data.lines().map(|line| line.chars()).collect();
        let mut chars = Vec::new();
        'outer: loop {
            for mut iterator in &mut iterators {
                if let Some(c) = iterator.next() {
                    chars.push(c);
                } else {
                    break 'outer;
                }
            }
        }
        println!("{:?}", chars);
        Ok(())
    }
}
