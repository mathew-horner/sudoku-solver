pub const X_CELL_COUNT: usize = 37;
pub const Y_CELL_COUNT: usize = 19;

#[rustfmt::skip]
pub const LAYOUT: [[Cell; X_CELL_COUNT]; Y_CELL_COUNT] = [
    [
        Cell::Glyph('┏'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┯'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┯'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'),
        Cell::Glyph('┳'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┯'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┯'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'),
        Cell::Glyph('┳'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┯'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┯'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'),
        Cell::Glyph('┓')
    ],
    [
        Cell::Glyph('┃'), Cell::Space, Cell::Square(0), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(1), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(2), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(3), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(4), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(5), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(6), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(7), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(8), Cell::Space,
        Cell::Glyph('┃'),
    ],
    [
        Cell::Glyph('┠'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('╂'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('╂'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('┨')
    ],
    [
        Cell::Glyph('┃'), Cell::Space, Cell::Square(9), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(10), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(11), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(12), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(13), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(14), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(15), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(16), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(17), Cell::Space,
        Cell::Glyph('┃'),
    ],
    [
        Cell::Glyph('┠'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('╂'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('╂'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('┨')
    ],
    [
        Cell::Glyph('┃'), Cell::Space, Cell::Square(18), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(19), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(20), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(21), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(22), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(23), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(24), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(25), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(26), Cell::Space,
        Cell::Glyph('┃'),
    ],
    [
        Cell::Glyph('┣'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┿'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┿'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'),
        Cell::Glyph('╋'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┿'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┿'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'),
        Cell::Glyph('╋'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┿'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┿'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'),
        Cell::Glyph('┫')
    ],
    [
        Cell::Glyph('┃'), Cell::Space, Cell::Square(27), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(28), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(29), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(30), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(31), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(32), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(33), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(34), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(35), Cell::Space,
        Cell::Glyph('┃'),
    ],
    [
        Cell::Glyph('┠'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('╂'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('╂'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('┨')
    ],
    [
        Cell::Glyph('┃'), Cell::Space, Cell::Square(36), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(37), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(38), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(39), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(40), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(41), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(42), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(43), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(44), Cell::Space,
        Cell::Glyph('┃'),
    ],
    [
        Cell::Glyph('┠'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('╂'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('╂'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('┨')
    ],
    [
        Cell::Glyph('┃'), Cell::Space, Cell::Square(45), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(46), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(47), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(48), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(49), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(50), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(51), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(52), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(53), Cell::Space,
        Cell::Glyph('┃'),
    ],
    [
        Cell::Glyph('┣'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┿'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┿'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'),
        Cell::Glyph('╋'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┿'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┿'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'),
        Cell::Glyph('╋'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┿'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┿'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'),
        Cell::Glyph('┫')
    ],
    [
        Cell::Glyph('┃'), Cell::Space, Cell::Square(54), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(55), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(56), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(57), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(58), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(59), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(60), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(61), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(62), Cell::Space,
        Cell::Glyph('┃'),
    ],
    [
        Cell::Glyph('┠'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('╂'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('╂'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('┨')
    ],
    [
        Cell::Glyph('┃'), Cell::Space, Cell::Square(63), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(64), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(65), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(66), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(67), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(68), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(69), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(70), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(71), Cell::Space,
        Cell::Glyph('┃'),
    ],
    [
        Cell::Glyph('┠'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('╂'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('╂'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('┼'), Cell::Glyph('─'), Cell::Glyph('─'), Cell::Glyph('─'),
        Cell::Glyph('┨')
    ],
    [
        Cell::Glyph('┃'), Cell::Space, Cell::Square(72), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(73), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(74), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(75), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(76), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(77), Cell::Space,
        Cell::Glyph('┃'), Cell::Space, Cell::Square(78), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(79), Cell::Space, Cell::Glyph('│'), Cell::Space, Cell::Square(80), Cell::Space,
        Cell::Glyph('┃'),
    ],
    [
        Cell::Glyph('┗'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┷'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┷'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'),
        Cell::Glyph('┻'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┷'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┷'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'),
        Cell::Glyph('┻'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┷'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('┷'), Cell::Glyph('━'), Cell::Glyph('━'), Cell::Glyph('━'),
        Cell::Glyph('┛')
    ],
];

#[derive(Copy, Clone)]
pub enum Cell {
    /// Render this glyph.
    Glyph(char),
    /// Render whitespace.
    Space,
    /// Render the value of the square at this index.
    Square(usize),
}
