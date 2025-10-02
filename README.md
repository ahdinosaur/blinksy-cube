# LED Cube with [Blinksy][blinksy] 🧊🌈

Make an LED cube with 3D animations using the [Blinksy Rust LED library][blinksy], a [Gledopto GL-C-016WL-D controller](https://www.aliexpress.com/item/1005008707989546.html), 6x LED panels, and a custom 3D-printed cube.

https://github.com/user-attachments/assets/36a2c6ad-7ae6-4498-85b3-ed76d0b62264

(For other Blinksy project examples: see [`blinksy-quickstart-1d-rope`](https://github.com/ahdinosaur/blinksy-quickstart-1d-rope) and [`blinksy-quickstart-3d-grid`](https://github.com/ahdinosaur/blinksy-quickstart-3d-grid).)

## How To Build

TODO video

### Bill of Materials

To get started now, buy:

- Gledopto GL-C-016WL-D: [AliExpress](https://www.aliexpress.com/item/1005008707989546.html), [Amazon](https://www.amazon.com/Controller-Dynamic-Lighting-Download-Addressable/dp/B0DT9QM25R)
- 6x BTF-LIGHTING WS2812B ECO 16x16 LED Panel: [AliExpress](https://www.aliexpress.com/item/32944813367.html)
- A BTF-LIGHTING 5V 10A power supply: [AliExpress](https://www.aliexpress.com/item/32810906485.html)
- 72x 3x1mm magnets

You will need to print this design: [16x16 WS2812 WLED Cube](https://makerworld.com/en/models/1085530-16x16-ws2812-wled-cube):

- 3x panel tops
- 3x panel bottoms
- 6x or 12x panel clamps
- 2x magnet press helpers

You might want an [adhesive](https://hackaday.com/2025/01/30/comparing-adhesives-for-gluing-petg-prints/) or [solvent](https://assemblyadhesives.com/products/acrylics/weld-on-3/) to glue or weld the 3x cube faces into 2x half-cubes.

## Development

### Pre-requisites

- Install Rust with [`rustup`][rustup]
- Install [`just`][just]

```shell
cargo install just
```

- Install [`espup`][espup]

```shell
cargo install espup@'^0.15' --locked
espup install
```

- Install [`espflash`][espflash]

```shell
cargo install espflash@'^4' --locked
```

- On Linux, add user to `dialout` group (or `uucp` on Arch)

```shell
sudo usermod -a -G dialout $USER
```

- Source the [ESP environment variables][esp-env-vars]

```shell
. $HOME/export-esp.sh
```

[rustup]: https://rustup.rs/
[just]: https://github.com/casey/just
[espup]: https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html
[espflash]: https://docs.esp-rs.org/book/tooling/espflash.html
[esp-env-vars]: https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html#3-set-up-the-environment-variables

#### CH341 Driver

The Gledopto board has a CH341 USB-to-serial converter, to help your computer talk to the board.

This should work out-of-the-box with Linux, but for MacOS and Windows you will need to install a driver.

##### MacOS

On MacOS, install [CH341 driver](https://www.wch-ic.com/downloads/CH341SER_MAC_ZIP.html)

See ["MacOS" section of Sparkfun guide](https://learn.sparkfun.com/tutorials/how-to-install-ch340-drivers/all#mac-osx) for more details.

##### Windows

On Windows, install [CH341 driver](https://www.wch-ic.com/downloads/CH341SER_EXE.html)

See ["Windows 7/10" section of Sparkfun guide](https://learn.sparkfun.com/tutorials/how-to-install-ch340-drivers/all#windows-710) for more details.

#### USB Cable

For some reason, there's been issues with people using a USB-C to USB-C cable to plug into the Gledopto.

If you only have USB-C outputs, try a USB-C-to-USB-A dongle, then a USB-A to USB-C cable.

🤷

### Run on microcontroller

```shell
just mcu
```

### Simulate on desktop

```shell
just desktop
```

## Resources

- [`blinksy`][blinksy]: Rust _no-std_ _no-alloc_  LED control library for 1D, 2D, and 3D LED setups
- [`gledopto`][gledopto]: Board support crate for Gledopto ESP32 Digital LED controllers

[blinksy]: https://github.com/ahdinosaur/blinksy
[gledopto]: https://github.com/ahdinosaur/blinksy/tree/main/esp/gledopto

As the Gledopto controller is an ESP32, here are some more ESP resources to help:

- [The Rust on ESP Book](https://docs.esp-rs.org/book/introduction.html): An overall guide on ESP32 on Rust
- [esp-hal](https://docs.espressif.com/projects/rust/esp-hal/1.0.0-beta.0/esp32/esp_hal/index.html): The Hardware Abstraction Layer for an ESP32 on Rust
- [espup](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html): How to install the Xtensa target for Rust, required for ESP32
- [esp-generate](https://docs.esp-rs.org/book/writing-your-own-application/generate-project/esp-generate.html): A template to help you kickstart your project

And in case they are helpful:

- [ESP no-std book](https://docs.esp-rs.org/no_std-training)
- [ESP no-std examples](https://github.com/esp-rs/no_std-training)
- [Gledopto GL-C-016WL-D page](https://www.gledopto.eu/gledopto-esp32-wled-uart_1)
- [Gledopto GL-C-016WL-D user instructions](https://www.gledopto.eu/mediafiles/anleitungen/7002-gl-c-016wl-d-eng.pdf)

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
