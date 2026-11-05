#![no_std]
#![no_main]

use gb::ppu::{self, map, bg, tile, palette};
use gb::mmio::{Palette, Shade};

pub type Tile = [u8; 16];

const CHECKER: Tile = [
    0b01010101, 0b00000000,
    0b10101010, 0b00000000,
    0b01010101, 0b00000000,
    0b10101010, 0b00000000,
    0b01010101, 0b00000000,
    0b10101010, 0b00000000,
    0b01010101, 0b00000000,
    0b10101010, 0b00000000,
];

#[gb::rt::entry]
fn main() -> ! {
   
    ppu::with_lcd_off(|access| {
        tile::write(access, 0, &CHECKER);
        tile::set_addressing(tile::Addressing::Base8000);
        map::set(access, bg::map(), 0, 0, 0);

        palette::set_background(Palette::new().with_id0(Shade::White).with_id1(Shade::Black));
        bg::set_enabled(true);
    });

    
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
