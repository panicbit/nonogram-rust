#![warn(missing_debug_implementations)]
extern crate rustbox;
use std::fs::File;
use std::io::{self, Read};

#[derive(Copy, Clone, Debug)]
struct Cell {
    required: bool,
    mode: Mode
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(false, Mode::Empty)
    }
}

impl Cell {
    fn new(required: bool, mode: Mode) -> Cell {
        Cell {
            required: required,
            mode: mode
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Mode {
    Marked,
    Crossed,
    Empty
}

#[derive(Debug)]
struct Game {
    field: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize
}

impl Game {
    fn from_file() -> io::Result<Game> {
        let mut file = try!(File::open("data"));
        let mut input = String::new();

        try!(file.read_to_string(&mut input));

        let (cols, rows, mut field) = input.lines().fold((0, 0, Vec::new()), |(cols, rows, mut field), line| {
            let (cols_inner, row) = line.chars().fold((0, Vec::new()), |(cols, mut row), ch| {
                if ch == 'X' {
                    row.push(Cell::new(true, Mode::Empty));
                } else {
                    row.push(Cell::default());
                }
                (cols + 1, row)
            });
            
            field.push(row);

            (
                if cols_inner > cols { cols_inner } else { cols },
                rows + 1,
                field
            )
        });

        for row in field.iter_mut() {
            if row.len() < cols {
                for _ in 0..(rows-row.len()) {
                    row.push(Cell::default());
                }
            }
        }

        Ok(Game {
            field: field,
            rows: rows,
            cols: cols
        })
    }

}

fn main() {

}
