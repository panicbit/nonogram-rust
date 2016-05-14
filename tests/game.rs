extern crate nonogram;
use nonogram::*;

#[test]
fn cell_from_char() {
    assert_eq!(Cell::from('X'), Cell::new(true, Mode::Empty));
    assert_eq!(Cell::from('_'), Cell::new(false, Mode::Empty));
    assert_eq!(Cell::from('Ä'), Cell::new(false, Mode::Empty));
}

#[test]
fn game_from_file() {
    let game = Game::from_file("data").expect("data file");
    let field = game.field();

    let test_field = Field::from_shape_vec((9, 8), vec![
        '_'.into(), 'X'.into(), '_'.into(), 'X'.into(), '_'.into(), '_'.into(), '_'.into(), '_'.into(),
        '_'.into(), 'X'.into(), 'X'.into(), 'X'.into(), '_'.into(), '_'.into(), 'A'.into(), '_'.into(),
        'X'.into(), 'X'.into(), 'X'.into(), 'X'.into(), 'X'.into(), '_'.into(), 'ß'.into(), '_'.into(),
        'X'.into(), 'X'.into(), 'X'.into(), 'X'.into(), 'X'.into(), '_'.into(), '_'.into(), '_'.into(),
        '_'.into(), '_'.into(), 'X'.into(), '_'.into(), ' '.into(), '_'.into(), '_'.into(), 'X'.into(),
        'ö'.into(), '_'.into(), 'X'.into(), '_'.into(), 'û'.into(), '_'.into(), '_'.into(), 'X'.into(),
        '_'.into(), '_'.into(), 'X'.into(), 'X'.into(), '_'.into(), '_'.into(), 'X'.into(), 'X'.into(),
        '_'.into(), '_'.into(), 'X'.into(), 'X'.into(), 'X'.into(), '_'.into(), 'X'.into(), '_'.into(),
        '_'.into(), '_'.into(), 'X'.into(), 'X'.into(), 'X'.into(), 'X'.into(), 'X'.into(), '_'.into(),
    ]).expect("valid ndarray");

    assert_eq!(field, &test_field);
}

#[test]
fn game_won() {
    let mut game = Game::from_file("tests/small_test").expect("data file");

    game.mark(1, 0);
    game.mark(3, 0);
    game.mark(1, 1);
    game.mark(2, 1);
    game.mark(3, 1);
    game.mark(0, 2);
    game.mark(1, 2);
    game.mark(2, 2);
    game.mark(3, 2);

    assert!(game.won());
}
