//! This module works with the serialization of the stage format, and allows reading stage source
//! files for export as compressed binary stages.

use std::collections::HashMap;

/// Orientation enum for setting orientation
#[derive(Serialize, Deserialize, Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// Top level stage sheet type.  Holds all the data necessary to compile the stage file.
#[derive(Serialize, Deserialize, Debug)]
pub struct Stage {
    /// Whether the stage is serialized to be a horizontal stage or a vertical one
    pub orientation: Orientation,

    /// Metatiles, for specifying the sprites used to compose a tile
    pub metatiles: HashMap<String, Vec<String>>,

    /// The number of tiles to generate
    pub data: String,
}
