use crate::bitboard::BitBoard;

pub struct BoardState {
    pub piece_bitboards: [BitBoard; 12],
    pub side_occupancy: [BitBoard; 2],
    pub full_occupancy: BitBoard,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            piece_bitboards: [
                BitBoard(0x0000_0000_0000_FF00),
                BitBoard(0x0000_0000_0000_0081),
                BitBoard(0x0000_0000_0000_0042),
                BitBoard(0x0000_0000_0000_0024),
                BitBoard(0x0000_0000_0000_0008),
                BitBoard(0x0000_0000_0000_0010),
                BitBoard(0x00FF_0000_0000_0000),
                BitBoard(0x8100_0000_0000_0000),
                BitBoard(0x4200_0000_0000_0000),
                BitBoard(0x2400_0000_0000_0000),
                BitBoard(0x0800_0000_0000_0000),
                BitBoard(0x1000_0000_0000_0000),
            ],
            side_occupancy: [
                BitBoard(0x0000_0000_0000_FFFF),
                BitBoard(0xFFFF_0000_0000_0000),
            ],
            full_occupancy: BitBoard(0xFFFF_0000_0000_FFFF),
        }
    }
}
