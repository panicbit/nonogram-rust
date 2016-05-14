#![warn(missing_debug_implementations)]
extern crate rustbox;
extern crate ndarray;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use ndarray::{OwnedArray, Ix};
use std::iter::repeat;

pub mod cell;
use cell::Cell;

pub type Field = OwnedArray<Cell, (Ix, Ix)>;

#[derive(Debug)]
pub struct Game {
    field: Field,
    width: usize,
    height: usize
}

impl Game {
    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Game> {
        let mut file = try!(File::open(path));
        let mut input = String::new();

        try!(file.read_to_string(&mut input));

        let height = input.lines().count();
        let width = input.lines().map(str::len).max().unwrap_or(0);
        let mut field = Vec::new();

        for line in input.lines() {
            field.extend(line.chars().map(Cell::from));
            field.extend(repeat(Cell::default()).take(width-line.len()));
        }

        let field = Field::from_shape_vec((height, width), field).expect("invalid shape");

        Ok(Game {
            field: field,
            width: width,
            height: height
        })
    }

}
