#![cfg_attr(feature = "mcu", no_std)]
#![cfg_attr(feature = "mcu", no_main)]

#[cfg(all(feature = "mcu", feature = "desktop"))]
compile_error!("features \"mcu\" and \"desktop\" are mutually exclusive");

use blinksy::{
    markers::{Blocking, Dim3d},
    patterns::noise::{noise_fns, Noise3d, NoiseParams},
    ControlBuilder,
};

#[cfg(feature = "mcu")]
gledopto::bootloader!();

mod layout;

use crate::layout::Layout;

type Pattern = Noise3d<noise_fns::Perlin>;
type PatternParams = NoiseParams;

fn setup_control() -> ControlBuilder<Dim3d, Blocking, Layout, Pattern, ()> {
    ControlBuilder::new_3d()
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
    blinksy_desktop::driver::Desktop::new_3d::<Layout>().start(|driver| {
        let mut control = setup_control().with_driver(driver).build();

        loop {
            let elapsed_in_ms = blinksy_desktop::time::elapsed_in_ms();
            control.tick(elapsed_in_ms).unwrap();
        }
    });
}
