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

use agb::sound::mixer::Frequency;
use agb_tracker::{Track, Tracker};

use agb::include_background_gfx;
use agb_tracker::include_xm;

include_background_gfx!(
    mod background,
    PLAY_FIELD => "gfx/bg.png",
);

static BGM: Track = include_xm!("sfx/bgm.xm");

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

    let mut mixer = gba.mixer.mixer(Frequency::Hz32768);
    let mut tracker = Tracker::new(&BGM);

    loop {
        let mut frame = gfx.frame();

        bg.show(&mut frame);

        tracker.step(&mut mixer);

        mixer.frame();
        frame.commit();
    }
}
