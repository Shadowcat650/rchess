use super::BitBoard;

/// The [`Magic`] struct contains a magic number and relevant data to perform a magic lookup.
#[derive(Debug)]
pub struct Magic {
    mask: BitBoard,
    magic: u64,
    shift: u8,
    offset: usize,
}

impl Magic {
    /// Creates a new [`Magic`] struct.
    pub const fn new(mask: BitBoard, magic: u64, shift: u8, offset: usize) -> Self {
        Self {
            mask,
            magic,
            shift,
            offset,
        }
    }

    /// Gets the key of the [Magic`] for the given occupancy.
    pub fn key(&self, occupancy: BitBoard) -> usize {
        Self::calculate_key(occupancy, self.mask, self.magic, self.shift) + self.offset
    }

    /// Calculates a magic key for the given occupancy with the given magic data.
    pub fn calculate_key(occupancy: BitBoard, mask: BitBoard, magic: u64, shift: u8) -> usize {
        let masked = occupancy & mask;
        let hash = masked.to_u64().wrapping_mul(magic);
        let key = hash >> shift;
        key as usize
    }

    /// Gets the occupancy mask.
    pub fn mask(&self) -> BitBoard {
        self.mask
    }

    /// Gets the magic number.
    pub fn magic(&self) -> u64 {
        self.magic
    }

    /// Gets the shift.
    pub fn shift(&self) -> u8 {
        self.shift
    }

    /// Gets the offset.
    pub fn offset(&self) -> usize {
        self.offset
    }
}
