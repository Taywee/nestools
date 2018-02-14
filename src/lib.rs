//! This is a set of relatively simple tools used to assist with the building of NES games.
//! Currently, its only functionality is in managing sprite sheets.

#[macro_use]
extern crate serde_derive;

pub mod sprites;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
