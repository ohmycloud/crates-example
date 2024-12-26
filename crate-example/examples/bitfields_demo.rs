use bitfields::bitfield;

struct DisplayMode {
    bg0_on: bool,
    bg1_on: bool,
}

// Create a struct annotated with the #[bitfield] attribute.
// By default, the field order is the least significant bit
// to most significant bit
#[bitfield(u8)]
#[derive(Copy, Clone)]
struct DisplayControl {
    /// We specify gg mode occupies the first 2 bits (0-1) of the bitfield
    /// using the `#[bits]` attribute.
    #[bits(2)]
    bg_mode: u8,
    /// Custom type fields must implement the `from_bits` and `into_bits`
    /// functions and declare its size using the `#[bits]` attribute.
    #[bits(2)]
    display_mode: DisplayMode,
    /// We can omit the `#[bits]` attribute for non-custom types, the macro
    /// will assume the number of bits is the size of the field type. Here,
    /// its 1 bit for a `bool` type.
    obj_char_vram_mapping: bool,
    /// Prefixing a field with "_" makes it as a padding field which
    /// is inaccessible. Padding fields are 0 by default, unless a default value
    /// is provided.
    #[bits(3, default = 0x3)]
    _always_0x3_padding: u8,
}

/// Implement the `from_bits` and `into_bits` funcs for the custom type.
impl DisplayMode {
    const fn from_bits(bits: u8) -> Self {
        Self {
            bg0_on: bits & 0b001 != 0,
            bg1_on: bits & 0b010 != 0,
        }
    }

    // Convert the custom type into bytes
    const fn into_bits(self) -> u8 {
        (self.bg0_on as u8) | (self.bg1_on as u8) << 1
    }
}

fn main() {
    // Creating the display mode custom type.
    let display_mode = DisplayMode {
      bg0_on: true,
      bg1_on: false
    };

    // Building the display control
    let display_control = DisplayControlBuilder::new()
        .with_bg_mode(0b1)
        .with_display_mode(display_mode)
        .with_obj_char_vram_mapping(true)
        .build();

    let val = display_control.into_bits();
    println!("{}", val);
}