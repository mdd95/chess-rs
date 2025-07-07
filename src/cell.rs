use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cell {
    pub rank: u8,
    pub file: u8,
}

impl Cell {
    pub fn new(rank: u8, file: u8) -> Option<Self> {
        if rank < 8 && file < 8 {
            Some(Cell { rank, file })
        } else {
            None
        }
    }

    pub fn from_index(index: u8) -> Option<Self> {
        if index < 64 {
            Some(Cell {
                rank: index / 8,
                file: index % 8,
            })
        } else {
            None
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let bytes = s.as_bytes();
        if bytes.len() != 2 {
            return None;
        }
        let file = match bytes[0] {
            b'a'..=b'h' => bytes[0] - b'a',
            _ => return None,
        };
        let rank = match bytes[1] {
            b'1'..=b'8' => bytes[1] - b'1',
            _ => return None,
        };
        Cell::new(rank, file)
    }

    #[inline]
    pub fn index(&self) -> u8 {
        self.rank * 8 + self.file
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file = (b'a' + self.file) as char;
        let rank = (b'1' + self.rank) as char;
        write!(f, "{}{}", file, rank)
    }
}
