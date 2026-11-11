can-hal-embedded
================

[can-hal](https://github.com/can-hal-rs-contributors/can-hal-rs) backends for
microcontroller CAN controllers: `no_std`, no HAL dependency, each driving the
controller's registers itself. Their channels confirm transmissions
(`ConfirmedTransmit` and `ReceiveEvents`): a frame is reported sent when the
controller says it was acknowledged, in bus order with the frames received.
They use the confirmed-transmission traits on our fork's `aeolionics` branch,
checked out beside this repository as `../can-hal-rs`.

# Crates

- `can-hal-bxcan`: ST's bxCAN (STM32F0, F1, F4 and others), classic CAN.
- `can-hal-mcan`: ST's FDCAN (STM32G0, G4, H5, U5 and others), classic CAN
  and CAN FD.
- `nucleo-g431kb`: bench tests of the FDCAN driver on ST's NUCLEO-G431KB
  (`cargo run-mcan-loopback`; `cargo run-mcan-dar` with a jumper from PA12
  to PA11).

The crates came from [ferrocan](https://github.com/aeolionics-rs/ferrocan),
the gs_usb adapter firmware that uses them, with their history. Each is
independent of the others, so any could move to can-hal-rs itself if its
maintainers want embedded backends there.

# Licence

MIT OR Apache-2.0, at your option.
