extern crate nonogram;
extern crate rustbox;
extern crate itertools;

use std::time::Duration;
use std::thread::sleep;
use std::fmt::Write;
use nonogram::{Game,Mode,Field,Ix};
use rustbox::{RustBox,Style,Color,Event,Mouse,InputMode,Key};
use itertools::Itertools;

fn main() {
    let mut game = Game::from_file("data").expect("game data");
    let rb = RustBox::init(Default::default()).expect("rustbox");
    rb.set_input_mode(InputMode::EscMouse);

    // Initial render
    draw(&rb, &game);

    loop {
        if game.won() {
            break;
        }

        match rb.poll_event(false).expect("input") {
            Event::MouseEvent(Mouse::Left, x, y) => {
                game.mark(x as Ix, y as Ix);
            },
            Event::MouseEvent(Mouse::Right, x, y) => {
                game.cross(x as Ix, y as Ix);
            },
            Event::KeyEvent(Key::Char('q')) => return,
            _ => {}
        };

        rb.clear();
        draw(&rb, &game);
    }

    let message = r"Yay, you win \o/";
    rb.print(0, game.height() + 2, Style::empty(), Color::Default, Color::Default, message);
    rb.present();

    sleep(Duration::from_millis(3000));
}

fn draw(rb: &RustBox, game: &Game) {
    draw_field(&rb, game);
    draw_row_labels(&rb, game);
    draw_col_labels(&rb, game);
    rb.present();
}

fn draw_field(rb: &RustBox, game: &Game) {
    for ((y, x), cell) in game.field().indexed_iter() {
        let ch = match cell.mode() {
            Mode::Marked => '▉',
            Mode::Crossed => 'X',
            _ => '_'
        };
        rb.print_char(x, y, Style::empty(), Color::Default, Color::Default, ch)
    }
}

fn draw_row_labels(rb: &RustBox, game: &Game) {
        let mut output = String::new();
        for (row_index, labels) in game.row_labels().iter().enumerate() {
            for label in labels {
                write!(&mut output, "{} ", label);
            }
            rb.print(game.width() + 1, row_index, Style::empty(), Color::Default, Color::Default, &output);
            output.clear();
        }
}

fn draw_col_labels(rb: &RustBox, game: &Game) {
        for (col_index, labels) in game.col_labels().iter().enumerate() {
            for (label_index, label) in labels.iter().enumerate() {
                rb.print(col_index , game.height() + 1 + label_index, Style::empty(), Color::Default, Color::Default, &format!("{}", label));
            }
        }
}
