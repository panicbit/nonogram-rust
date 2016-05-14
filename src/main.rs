extern crate nonogram;
extern crate rustbox;

use nonogram::{Game,Cell,Mode,Field,Ix};
use rustbox::{RustBox,Style,Color,Event,Mouse,InputMode,Key};

fn main() {
    let mut game = Game::from_file("data").expect("game data");
    let rb = RustBox::init(Default::default()).expect("rustbox");
    rb.set_input_mode(InputMode::EscMouse);

    loop {
        draw_field(&rb, game.field());
        rb.present();

        match rb.poll_event(false).expect("input") {
            Event::MouseEvent(Mouse::Left, x, y) => {
                game.mark(x as Ix, y as Ix);

                // rb.print(0, 15, Style::empty(), Color::Default, Color::Default, &format!("x: {}, y: {}", x, y));
            },
            Event::KeyEvent(Key::Char('q')) => return,
            _ => {}
        };

        rb.clear();
    }
}

fn draw_field(rb: &RustBox, field: &Field) {
    for ((y, x), cell) in field.indexed_iter() {
        let ch = match cell.mode() {
            Mode::Marked => '▉',
            _ => '_'
        };
        rb.print_char(x, y, Style::empty(), Color::Default, Color::Default, ch)
    }
}
