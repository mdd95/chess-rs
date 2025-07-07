use crate::bitboard::BitBoard;

/// Generates white pawn pushes (single and double).
pub fn white_pawn_pushes(start: BitBoard, full_occupancy: BitBoard) -> BitBoard {
    let single_push = start.north() & !full_occupancy;
    let double_push = (single_push & BitBoard::WHITE_PAWN_START).north() & !full_occupancy;
    single_push | double_push
}

/// Generates black pawn pushes (single and double).
pub fn black_pawn_pushes(start: BitBoard, full_occupancy: BitBoard) -> BitBoard {
    let single_push = start.south() & !full_occupancy;
    let double_push = (single_push & BitBoard::BLACK_PAWN_START).south() & !full_occupancy;
    single_push | double_push
}

/// Generates white pawn attacks
pub fn white_pawn_attacks(start: BitBoard, black_occupancy: BitBoard) -> BitBoard {
    (start.northeast() | start.northwest()) & black_occupancy
}

/// Generates black pawn attacks
pub fn black_pawn_attacks(start: BitBoard, white_occupancy: BitBoard) -> BitBoard {
    (start.southeast() | start.southwest()) & white_occupancy
}
