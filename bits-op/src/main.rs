use anyhow::{anyhow, bail};
use bit_struct::*;

enums! {
    // 2 bits, i.e. 0b00, 0b01, 0b10
    pub HouseKind { Urban, Suburan, Rural }
}

bit_struct! {
    // u8 is the base storage type. This can be any multiple of 8
    pub struct HouseConfig(u8) {
        // 2 bits
        kind: HouseKind,
        // two's compliment  3-bit signed number
        lowest_floor: i3,
        // 2 bit unsigned number
        highest_floor: u2,
    }
}

fn main() -> anyhow::Result<()> {
    // We can create a new `HouseConfig` like such:
    // where all numbers are statically checked to be in bounds.
    let config = HouseConfig::new(
        HouseKind::Suburan,
        i3!(-2),
        u2!(1)
    );

    // We can get the raw `u8` which represents `config`:
    let raw: u8 = config.raw();
    println!("raw: {}", raw);

    // or we can get a `HouseConfig` from a `u8` like:
    let mut config: HouseConfig = HouseConfig::try_from(114_u8)
        .map_err(|_| anyhow!("can't convert u8 into HouseConfig"))?;
    assert_eq!(config, HouseConfig::new(
        HouseKind::Suburan,
        i3!(-2),
        u2!(1)
    ));
    // We need to unwrap because `HouseConfig` is not valid for all numbers.
    // For instance, if the most significant bits are `0b11`, it encodes an invalid
    // `HouseKind`. However, if all elements of a struct are always valid (support we removed the `kind` field),
    // the struct will auto implement a trait which allows calling the non-panicking:
    // let config: HouseConfig = HouseConfig::exact_from(123_u8);

    // We can access values of `config` like so:
    let kind: HouseKind = config.kind().get();

    // And we can set values like so:
    config.lowest_floor().set(i3!(0));

    // We can also convert the new numeric types for alternate bit-widths into the
    // numeric types provided by the standard library:
    let lowest_floor: i3 = config.lowest_floor().get();
    let lowest_floor_std: i8 = lowest_floor.value();
    assert_eq!(lowest_floor_std, 0_i8);
    Ok(())
}
