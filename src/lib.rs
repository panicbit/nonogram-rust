#![warn(missing_debug_implementations)]
extern crate ndarray;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use ndarray::OwnedArray;
use std::iter::repeat;

pub mod cell;
pub use cell::{Cell, Mode};

pub type Ix = ndarray::Ix;
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

    pub fn mark(&mut self, x: Ix, y: Ix) {
        if let Some(cell) = self.field.get_mut((y, x)) {
            match cell.mode() {
                Mode::Marked | Mode::Crossed => cell.set_mode(Mode::Empty),
                Mode::Empty => cell.set_mode(Mode::Marked)
            }
        }
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
