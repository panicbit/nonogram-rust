#![warn(missing_debug_implementations)]
extern crate ndarray;
extern crate itertools;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use ndarray::{OwnedArray,Axis};
use std::iter::repeat;
use itertools::Itertools;

pub mod cell;
pub use cell::{Cell, Mode};

pub type Ix = ndarray::Ix;
pub type Field = OwnedArray<Cell, (Ix, Ix)>;

const ROW_AXIS: Axis = Axis(0);
const COL_AXIS: Axis = Axis(1);

#[derive(Debug)]
pub struct Game {
    field: Field,
    width: usize,
    height: usize,
    row_labels: Vec<Vec<usize>>,
    col_labels: Vec<Vec<usize>>
}

impl Game {
    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn mark(&mut self, x: Ix, y: Ix) {
        if let Some(cell) = self.field.get_mut((y, x)) {
            match cell.mode() {
                Mode::Marked => cell.set_mode(Mode::Empty),
                Mode::Empty | Mode::Crossed => cell.set_mode(Mode::Marked)
            }
        }
    }

    pub fn cross(&mut self, x: Ix, y: Ix) {
        if let Some(cell) = self.field.get_mut((y, x)) {
            match cell.mode() {
                Mode::Marked | Mode::Empty => cell.set_mode(Mode::Crossed),
                Mode::Crossed => cell.set_mode(Mode::Empty)
            }
        }
    }

    pub fn won(&self) -> bool {
        self.field.iter().all(|cell| match cell.mode() {
            Mode::Marked => cell.required,
            _ => !cell.required
        })
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
            row_labels: generate_labels(&field, ROW_AXIS),
            col_labels: generate_labels(&field, COL_AXIS),
            field: field,
            width: width,
            height: height,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }


    pub fn height(&self) -> usize {
        self.height
    }

    pub fn row_labels(&self) -> &Vec<Vec<usize>> {
        &self.row_labels
    }

    pub fn col_labels(&self) -> &Vec<Vec<usize>> {
        &self.col_labels
    }
}

fn generate_labels(field: &Field, axis: Axis) -> Vec<Vec<usize>> {
    field.axis_iter(axis).map(|row| {
        row
            .iter()
            .group_by(|cell| cell.required)
            .filter(|&(required,_)| required)
            .map(|(_,cells)| cells.len())
            .collect()
    }).collect()
}
