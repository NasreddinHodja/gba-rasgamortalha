#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

use agb::display::tiled::VRAM_MANAGER;
use agb::display::{
    Priority,
    tiled::{RegularBackground, RegularBackgroundSize, TileFormat},
};
use agb::include_background_gfx;

include_background_gfx!(
    mod background,
    PLAY_FIELD => "gfx/bg.png",
);

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    VRAM_MANAGER.set_background_palettes(background::PALETTES);

    let mut bg = RegularBackground::new(
        Priority::P3,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    bg.fill_with(&background::PLAY_FIELD);

    let mut gfx = gba.graphics.get();

    loop {
        let mut frame = gfx.frame();

        bg.show(&mut frame);

        frame.commit();
    }
}
