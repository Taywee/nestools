//! This module works with the serialization of the stage format, and allows reading stage source
//! files for export as compressed binary stages.

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
pub struct Stage {
    /// Whether the stage is serialized to be a horizontal stage or a vertical one
    #[serde(default)]
    pub orientation: Orientation,

    /// Metatiles, for specifying the sprites used to compose a tile
    pub metatiles: HashMap<String, Vec<String>>,

    /// The number of tiles to generate
    pub data: String,
}

