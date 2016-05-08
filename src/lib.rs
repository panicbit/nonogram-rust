#![warn(missing_debug_implementations)]
extern crate rustbox;
extern crate ndarray;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use ndarray::{OwnedArray, Ix};

pub mod cell;
use cell::Cell;

pub type Field = OwnedArray<Cell, (Ix, Ix)>;

#[derive(Debug)]
pub struct Game {
    field: Field,
    rows: usize,
    cols: usize
}

impl Game {
    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Game> {
        let mut file = try!(File::open(path));
        let mut input = String::new();

        try!(file.read_to_string(&mut input));

        let (cols, rows, mut field) = input.lines().fold((0, 0, Vec::new()), |(cols, rows, mut field), line| {
            let (cols_inner, row) = line.chars().fold((0, Vec::new()), |(cols, mut row), ch| {
                row.push(Cell::from(ch));
                (cols + 1, row)
            });
            
            field.push(row);

            (
                if cols_inner > cols { cols_inner } else { cols },
                rows + 1,
                field
            )
        });

        // Pad the rows
        for row in field.iter_mut() {
            if row.len() < cols {
                for _ in 0..(rows-row.len()-1) {
                    row.push(Cell::default());
                }
            }
        }

        let field = field.into_iter().flat_map(|row| row.into_iter()).collect();

        let field = Field::from_shape_vec((rows, cols), field).expect("foo");

        Ok(Game {
            field: field,
            rows: rows,
            cols: cols
        })
    }

}
