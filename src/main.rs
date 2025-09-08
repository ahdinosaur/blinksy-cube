#![cfg_attr(feature = "mcu", no_std)]
#![cfg_attr(feature = "mcu", no_main)]

#[cfg(all(feature = "mcu", feature = "desktop"))]
compile_error!("features \"mcu\" and \"desktop\" are mutually exclusive");

use blinksy::{
    layout1d,
    markers::{Blocking, Dim1d},
    ControlBuilder,
};

#[cfg(feature = "mcu")]
gledopto::bootloader!();

mod patterns;

use crate::patterns::rainbow::{Rainbow, RainbowParams};

layout1d!(Layout, 50);

type Pattern = Rainbow;
type PatternParams = RainbowParams;

fn setup_control() -> ControlBuilder<Dim1d, Blocking, Layout, Pattern, ()> {
    ControlBuilder::new_1d()
        .with_layout::<Layout>()
        .with_pattern::<Pattern>(PatternParams::default())
}

#[cfg(feature = "mcu")]
#[gledopto::main]
fn main() -> ! {
    let p = gledopto::board!();

    let mut control = setup_control()
        .with_driver(gledopto::ws2812!(p, Layout::PIXEL_COUNT))
        .build();

    control.set_brightness(0.2);

    loop {
        let elapsed_in_ms = gledopto::elapsed().as_millis();
        control.tick(elapsed_in_ms).unwrap();
    }
}

#[cfg(feature = "desktop")]
fn main() {
    blinksy_desktop::driver::Desktop::new_1d::<Layout>().start(|driver| {
        let mut control = setup_control().with_driver(driver).build();

        loop {
            let elapsed_in_ms = blinksy_desktop::time::elapsed_in_ms();
            control.tick(elapsed_in_ms).unwrap();
        }
    });
}
