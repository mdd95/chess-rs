use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

#[derive(Clone, Copy, Debug)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const EMPTY: BitBoard = BitBoard(0);

    // Precomputed masks for files and ranks
    pub const RANK_1: BitBoard = BitBoard(0x0000_0000_0000_00FF);
    pub const RANK_2: BitBoard = BitBoard(0x0000_0000_0000_FF00);
    pub const RANK_3: BitBoard = BitBoard(0x0000_0000_00FF_0000);
    pub const RANK_4: BitBoard = BitBoard(0x0000_0000_FF00_0000);
    pub const RANK_5: BitBoard = BitBoard(0x0000_00FF_0000_0000);
    pub const RANK_6: BitBoard = BitBoard(0x0000_FF00_0000_0000);
    pub const RANK_7: BitBoard = BitBoard(0x00FF_0000_0000_0000);
    pub const RANK_8: BitBoard = BitBoard(0xFF00_0000_0000_0000);

    pub const FILE_A: BitBoard = BitBoard(0x0101_0101_0101_0101);
    pub const FILE_B: BitBoard = BitBoard(0x0202_0202_0202_0202);
    pub const FILE_C: BitBoard = BitBoard(0x0404_0404_0404_0404);
    pub const FILE_D: BitBoard = BitBoard(0x0808_0808_0808_0808);
    pub const FILE_E: BitBoard = BitBoard(0x1010_1010_1010_1010);
    pub const FILE_F: BitBoard = BitBoard(0x2020_2020_2020_2020);
    pub const FILE_G: BitBoard = BitBoard(0x4040_4040_4040_4040);
    pub const FILE_H: BitBoard = BitBoard(0x8080_8080_8080_8080);

    // Precomputed pawn start ranks
    pub const WHITE_PAWN_START: BitBoard = BitBoard::RANK_2;
    pub const BLACK_PAWN_START: BitBoard = BitBoard::RANK_7;

    /// Creates a `BitBoard` from index
    #[inline]
    pub fn from_index(index: u8) -> BitBoard {
        BitBoard(1 << index)
    }

    /// Counts the number of set bits (population count).
    #[inline]
    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    /// Returns whether the bitboard is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Returns whether the bitboard has any set bits.
    #[inline]
    pub fn is_any(&self) -> bool {
        self.0 != 0
    }

    /// Shifts the bitboard north (toward rank 8).
    #[inline]
    pub fn north(&self) -> BitBoard {
        BitBoard(self.0 << 8)
    }

    /// Shifts the bitboard south (toward rank 1).
    #[inline]
    pub fn south(&self) -> BitBoard {
        BitBoard(self.0 >> 8)
    }

    /// Shifts the bitboard east (toward file H, no wrap).
    #[inline]
    pub fn east(&self) -> BitBoard {
        BitBoard(self.0 << 1) & !BitBoard::FILE_A
    }

    /// Shifts the bitboard west (toward file A, no wrap).
    #[inline]
    pub fn west(&self) -> BitBoard {
        BitBoard(self.0 >> 1) & !BitBoard::FILE_H
    }

    /// Shifts the bitboard northeast.
    #[inline]
    pub fn northeast(&self) -> BitBoard {
        self.north().east()
    }

    /// Shifts the bitboard northwest.
    #[inline]
    pub fn northwest(&self) -> BitBoard {
        self.north().west()
    }

    /// Shifts the bitboard southeast.
    #[inline]
    pub fn southeast(&self) -> BitBoard {
        self.south().east()
    }

    /// Shifts the bitboard southwest.
    #[inline]
    pub fn southwest(&self) -> BitBoard {
        self.south().west()
    }
}

impl Not for BitBoard {
    type Output = BitBoard;
    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;
    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;
    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}
