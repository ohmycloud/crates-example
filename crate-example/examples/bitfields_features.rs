use bitfields::bitfield;

/// All fields in the bitfield must sum up to the number of bits of the bitfield type.
#[bitfield(u64)]
pub struct Bitfield {
    /// Fields without bits
    u8init: u8,
}

fn main() {}