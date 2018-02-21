//! This is a set of relatively simple tools used to assist with the building of NES games.
//! Currently, its only functionality is in managing sprite sheets.
//!
//! # binaries
//! 
//! ## spritesheetc
//! 
//! Compiles spritesheets, using a meta file, into sets of headers and a chr file.
//! 
//! The usage can be retrieved exactly how you'd expect:
//! 
//! ```sh
//! $ spritesheetc -h
//! Usage: spritesheetc [options]
//! 
//! Options:
//!     -i, --input INPUT   input json description file. Defaults to stdin.
//!     -o, --char INPUT    output NES char file name. Defaults to stdout.
//!     -c, --header INPUT  output C header file name
//!     -a, --asm INPUT     output asm define file name
//!     -h, --help          print this help menu
//! ```
//!
//! The format of the input file should be a JSON file that is deserializable by serde_json.  An
//! example input might look something like this: 
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
//!
//! The sheet types are described in in
//! [`nestools::sprites::sheet`](sprites/sheet/enum.Sheet.html), but should be largely
//! self-explanatory.  The sprites are loaded into their relevant section of the PatternTable in
//! order, tightly-packed, and the remaining tiles are filled with blanks.  All file paths are
//! processed relative to your current working directory (a chdir flag may be added in the future).

#[macro_use]
extern crate serde_derive;
extern crate lodepng;

pub mod sprites;
