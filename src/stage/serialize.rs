//! This module works with the serialization of the stage format, and allows reading stage source
//! files for export as compressed binary stages.

use std::io::Write;
use std::io::Result;
use std::default::Default;
use std::collections::HashMap;

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
    pub tiles: [u8; 4],

    /// Attribute table pallete section
    pub palette: u8,
}

/// Top level stage sheet type.  Holds all the data necessary to compile the stage file.
#[derive(Serialize, Deserialize, Debug)]
pub struct Stage {
    /// Whether the stage is serialized to be a horizontal stage or a vertical one
    #[serde(default)]
    pub orientation: Orientation,

    /// Background Palette definition
    pub background_palette: [u8; 16],

    /// Sprite Palette definition
    pub sprite_palette: [u8; 16],

    /// Metatiles, for specifying the sprites used to compose a tile
    pub metatiles: Vec<Metatile>,

    /// The number of tiles to generate
    pub data: String,
}

// Take a slice of chars and return a RLE-compressed version of it
fn run_length_encode(chars: &[char], limit: u16) -> Vec<(char, u16)> {
    let mut output = Vec::new();
    if chars.len() > 0 {
        let mut current = chars[0];
        let mut count = 1;
        for &c in &chars[1..] {
            if c == current {
                count += 1;
                if count == limit {
                    output.push((current, count));
                    // Reset this character to ensure we restart at 1 for the next character
                    current = '\0';
                    count = 0;
                }
            } else {
                if count > 0 {
                    output.push((current, count));
                }
                current = c;
                count = 1;
            }
        }
        if count > 0 {
            output.push((current, count));
        }
    }
    output
}

impl Stage {
    pub fn write_binary(&self, write: &mut dyn Write) -> Result<()> {
        let metatiles: HashMap<char, u8> = self.metatiles.iter()
            .enumerate()
            .map(|(i, metatile)| (metatile.symbol, i as u8))
            .collect(); 

        // Write out the palettes literally
        // This could be eventually optimized a bit.  The NES only has 64 colors, so it's possible
        // to make each palette item take 6 bits instead of 8, so the full set of 32 colors to take
        // 24 bytes instead of 32.  Even fewer when you consider sprite palettes and the universal
        // background color, which is just 25 colors total for a maximum necessary 19 bytes (1 for
        // universal background and 12 colors for background and foreground palettes at 9 bytes a
        // piece).
        write.write(&self.background_palette)?;
        write.write(&self.sprite_palette)?;

        // Write count of metatiles, should not exceed 16
        write.write(&[self.metatiles.len() as u8])?;

        // Simple metatile information
        for metatile in &self.metatiles {
            // TODO: compress this more.  Palette only needs 2 bits.  This may become a general
            // attribute byte
            write.write(&[metatile.palette])?;
            write.write(&metatile.tiles)?;
        }
        // Build list of chars for RLE
        let mut iterators: Vec<_> = self.data.lines().map(|line| line.chars()).collect();
        let mut chars = Vec::new();
        match &self.orientation {
            // Build a vector of chars by transposing characters.
            Orientation::Horizontal => 'outer: loop {
                for iterator in &mut iterators {
                    if let Some(c) = iterator.next() {
                        chars.push(c);
                    } else {
                        break 'outer;
                    }
                }
            },
            // Do straight composition
            Orientation::Vertical => for iterator in iterators {
                for c in iterator {
                    chars.push(c);
                }
            },
        }

        // Run-length encode
        let encoded = run_length_encode(&chars, 16);
        let mut outbytes = Vec::new();
        for (c, count) in encoded {
            if let Some(&index) = metatiles.get(&c) {
                // Stage bytes are written in two nibbles.  The first is length - 1, and the second
                // is the index of the metatile in the stage header
                let outbyte: u8 = (count as u8) - 1 << 4 | 0x0F & index as u8;
                outbytes.push(outbyte);
            }
        }

        // Write stage body compressed length in bytes (to allow entering the map from the other side) 
        write.write(&[outbytes.len() as u8])?;

        // Write stage body
        write.write(&outbytes)?;
        Ok(())
    }
}
