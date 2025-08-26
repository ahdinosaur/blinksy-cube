#![cfg_attr(feature = "mcu", no_std)]
#![cfg_attr(feature = "mcu", no_main)]

#[cfg(all(feature = "mcu", feature = "desktop"))]
compile_error!("features \"mcu\" and \"desktop\" are mutually exclusive");

use blinksy::{layout1d, ControlBuilder};
use cfg_iif::cfg_iif;

#[cfg(feature = "mcu")]
use blinksy::layout::Layout1d;

#[cfg(feature = "mcu")]
gledopto::bootloader!();

mod patterns;

use crate::patterns::rainbow::{Rainbow, RainbowParams};


#[cfg_attr(feature = "mcu", gledopto::main)]
fn main() -> ! {
    #[cfg(feature = "mcu")]
    let p = gledopto::board!();

    layout1d!(Layout, 50);

    let mut control = ControlBuilder::new_1d()
        .with_layout::<Layout>()
        .with_pattern::<Rainbow>(RainbowParams {
            ..Default::default()
        })
        .with_driver(cfg_iif!(
            #[cfg(feature = "mcu")] {
                gledopto::ws2812!(p, Layout::PIXEL_COUNT)
            } else {
                blinksy_desktop::driver::Desktop::new_1d::<Layout>()
            }
        ))
        .build();

    cfg_iif!(
        #[cfg(feature = "mcu")]
        {
            control.set_brightness(0.2);
        }
    );

    loop {
        let elapsed_in_ms = cfg_iif!(
            #[cfg(feature = "mcu")] { gledopto::elapsed().as_millis() }
            else { blinksy_desktop::time::elapsed_in_ms() }
        );
        control.tick(elapsed_in_ms).unwrap();
    }
}
