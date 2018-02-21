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
//!     -i, --input FILE    input json description file. Defaults to stdin.
//!     -o, --char FILE     output NES char file name. Defaults to stdout.
//!     -c, --header FILE   output C header file name.  Not generated if not specified.
//!     -a, --asm FILE      output asm header file name.  Not generated if not specified.
//!     -p, --prefix PREFIX the prefix for the header defines.  Defaults to blank.
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
//!       "type": "Animation",
//!       "file": "first.png",
//!       "frame_height": 2,
//!       "frame_width": 2,
//!       "frames": 4,
//!       "name": "first"
//!     },
//!     {
//!       "type": "Slice",
//!       "file": "second.png",
//!       "height": 4,
//!       "name": "second",
//!       "slices": [
//!         [15, 12, 7],
//!         [0, 4, 5]
//!       ],
//!       "width": 4
//!     }
//!   ],
//!   "right": [
//!     {
//!       "type": "Simple",
//!       "file": "third.png",
//!       "height": 8,
//!       "name": "third",
//!       "width": 8
//!     },
//!     {
//!       "type": "Animation",
//!       "file": "fourth.png",
//!       "frame_height": 4,
//!       "frame_width": 3,
//!       "frames": 3,
//!       "name": "fourth"
//!     }
//!   ]
//! }
//! ```
//!
//! The sheet types are described in in
//! [`nestools::sprites::sheet`](sprites/sheet/enum.Sheet.html), but should be largely
//! self-explanatory.  The tiles are loaded into their relevant section of the PatternTable in
//! order, tightly-packed, and the remaining tiles are filled with blanks.  All file paths are
//! processed relative to your current working directory.
//!
//! ### Types
//!
//! Each type is specified in the individual sprite's `type` attribute.  All types have a `file`
//! field and a `name` field.  The `file` field is the png file that the sprite is extracted from.
//! The `name` field is used as the `NAME` part of the generated C/ASM header field, as described
//! in the individual type below.  `PREFIX` is set by the program in a flag. `SHEET` is either LEFT
//! or RIGHT
//!
//! In this readme, "tile" refers to a single 8x8 pixel NES tile, "sprite" refers to one or more of
//! these tiles as intended to be rendered to form something on the screen, and "sheet" or "sprite
//! sheet" refers to the sprites as laid out before being pulled in.  In this way, these types are
//! "sprites", and they pull in "sheets", in order to generate "tiles", and header files with names
//! referring to the indices of the "tiles" in their respective pattern table.
//!
//! #### Simple
//!
//! The simplest sprite type, typically useful for individual 8x8 tiles that are related and may be
//! useful to be contiguous, such as numbers or alphabet tiles, or sets of scenery tiles for laying
//! out.  The sprite sheet is used as a whole, and numbered from left-to-right, top-to-bottom.
//!
//! ##### Attributes
//!
//! * `height`
//!     * The height of the sheet in tiles (that is, in `pixels / 8`)
//! * `width`
//!     * The width of the sheet in tiles (that is, in `pixels / 8`)
//!
//! ##### Name
//!
//! This sprite's parts in the files generated will be
//! `{PREFIX}{SHEET}_{NAME}_{TILE}`, where `TILE` ranges from `0` up to but not including `hight *
//! width`.
//!
//! #### Animation
//!
//! A type that is used for animated sprites of a solid size.  This isn't ideal for things like
//! Mega Man, which use partially-animated sprites to save space.  It works fine for things like
//! Mario, which use a fully animated sprite that has every tile change with every animation frame.
//! Animations are expected to have a horizontal layout, with each frame to the right of its
//! previous one.  This can also be useful simply for a set of related sprites of the same size
//! that can easily share a name, like a set of 16x16 number tiles.  The sheet is pulled in and
//! divided into `frames` frames each `frame_width` in width, which are then divided into
//! `frame_width * frame_height` tiles each.
//!
//! ##### Attributes
//!
//! * `frame_height`
//!     * The height of an individual frame in tiles (that is, in `pixels / 8`)
//! * `frame_width`
//!     * The width of an individual frame in tiles (that is, in `pixels / 8`)
//! * `frames`
//!     * The actual amount of frames in the sheet
//!
//! The total width of the sheet in pixels is therefore `8 * frame_width * frames`
//!
//! ##### Name
//!
//! This sprite's parts in the files generated will be
//! `{PREFIX}{SHEET}_{NAME}_{FRAME}_{FRAMETILE}`, where `FRAME` ranges from `0` up to but not
//! including `frames`, arranged left-to-right top-to-bottom, and `FRAMETILE` ranges from `0` up to
//! but not including `frame_width * frame_height`, arranged within the frame left-to-right and
//! top-to-bottom.
//!
//! #### Slice
//!
//! A sprite type for frames that should be divided into chunks, like `Animation`, but not chunks
//! that can easily be laid out in the same arrangement or aren't all necessarily the same size.
//! This can be useful for partially-animated sprites like in MegaMan, but in a way that doesn't
//! force you to use two different sprites (an `Animation` bottom and a `Simple` top, for
//! instance, or having to do the whole thing with a `Simple`).  This is also useful for pieces of
//! scenery and things that should share a name but not necessarily have the same size.  The
//! `Slice` format also allows you to throw away unused tiles, so you can arrange them however you
//! like in your sheet and not worry about blank space, which won't take up space in your pattern
//! table.
//!
//! ##### Attributes
//!
//! * `height`
//!     * The height of the sheet in tiles (that is, in `pixels / 8`)
//! * `width`
//!     * The width of the sheet in tiles (that is, in `pixels / 8`)
//! * `slices`
//!     * An array of arrays.  Each inner array is a list of numbers each specifying an individual
//!         tile in the sheet (indexed in the usual left-to-right, top-to-bottom manner)
//!
//! The total width of the sheet in pixels is therefore `8 * frame_width * frames`
//!
//! ##### Name
//!
//! This sprite's parts in the files generated will be
//! `{PREFIX}{SHEET}_{NAME}_{SLICE}_{SLICETILE}`, where `SLICE` ranges from `0` up to but not
//! including the number of sub-arrays in `slices`, in the order specified, and `SLICETILE` ranges
//! from `0` up to but not including the size of the slice in question, also in the order
//! specified.

#[macro_use]
extern crate serde_derive;
extern crate lodepng;

pub mod sprites;

// Binary entrypoints
pub mod spritesheetc;
