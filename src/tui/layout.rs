use crate::tui::{X_CELL_COUNT, Y_CELL_COUNT};

#[rustfmt::skip]
pub const LAYOUT: [[Cell; X_CELL_COUNT]; Y_CELL_COUNT] = [
    [Cell::Glyph('-'); X_CELL_COUNT],
    [
        Cell::Glyph('|'), Cell::Square(0), Cell::Square(1), Cell::Square(2),
        Cell::Glyph('|'), Cell::Square(3), Cell::Square(4), Cell::Square(5),
        Cell::Glyph('|'), Cell::Square(6), Cell::Square(7), Cell::Square(8),
        Cell::Glyph('|'),
    ],
    [
        Cell::Glyph('|'), Cell::Square(9), Cell::Square(10), Cell::Square(11),
        Cell::Glyph('|'), Cell::Square(12), Cell::Square(13), Cell::Square(14),
        Cell::Glyph('|'), Cell::Square(15), Cell::Square(16), Cell::Square(17),
        Cell::Glyph('|'),
    ],
    [
        Cell::Glyph('|'), Cell::Square(18), Cell::Square(19), Cell::Square(20),
        Cell::Glyph('|'), Cell::Square(21), Cell::Square(22), Cell::Square(23),
        Cell::Glyph('|'), Cell::Square(24), Cell::Square(25), Cell::Square(26),
        Cell::Glyph('|'),
    ],
    [Cell::Glyph('-'); X_CELL_COUNT],
    [
        Cell::Glyph('|'), Cell::Square(27), Cell::Square(28), Cell::Square(29),
        Cell::Glyph('|'), Cell::Square(30), Cell::Square(31), Cell::Square(32),
        Cell::Glyph('|'), Cell::Square(33), Cell::Square(34), Cell::Square(35),
        Cell::Glyph('|'),
    ],
    [
        Cell::Glyph('|'), Cell::Square(36), Cell::Square(37), Cell::Square(38),
        Cell::Glyph('|'), Cell::Square(39), Cell::Square(40), Cell::Square(41),
        Cell::Glyph('|'), Cell::Square(42), Cell::Square(43), Cell::Square(44),
        Cell::Glyph('|'),
    ],
    [
        Cell::Glyph('|'), Cell::Square(45), Cell::Square(46), Cell::Square(47),
        Cell::Glyph('|'), Cell::Square(48), Cell::Square(49), Cell::Square(50),
        Cell::Glyph('|'), Cell::Square(51), Cell::Square(52), Cell::Square(53),
        Cell::Glyph('|'),
    ],
    [Cell::Glyph('-'); X_CELL_COUNT],
    [
        Cell::Glyph('|'), Cell::Square(54), Cell::Square(55), Cell::Square(56),
        Cell::Glyph('|'), Cell::Square(57), Cell::Square(58), Cell::Square(59),
        Cell::Glyph('|'), Cell::Square(60), Cell::Square(61), Cell::Square(62),
        Cell::Glyph('|'),
    ],
    [
        Cell::Glyph('|'), Cell::Square(63), Cell::Square(64), Cell::Square(65),
        Cell::Glyph('|'), Cell::Square(66), Cell::Square(67), Cell::Square(68),
        Cell::Glyph('|'), Cell::Square(69), Cell::Square(70), Cell::Square(71),
        Cell::Glyph('|'),
    ],
    [
        Cell::Glyph('|'), Cell::Square(72), Cell::Square(73), Cell::Square(74),
        Cell::Glyph('|'), Cell::Square(75), Cell::Square(76), Cell::Square(77),
        Cell::Glyph('|'), Cell::Square(78), Cell::Square(79), Cell::Square(80),
        Cell::Glyph('|'),
    ],
    [Cell::Glyph('-'); X_CELL_COUNT],
];

#[derive(Copy, Clone)]
pub enum Cell {
    /// Render this glyph.
    Glyph(char),
    /// Render the value of the square at this index.
    Square(usize),
}
