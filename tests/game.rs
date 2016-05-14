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