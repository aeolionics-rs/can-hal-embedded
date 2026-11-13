can-hal-embedded
================

[can-hal](https://github.com/can-hal-rs-contributors/can-hal-rs) backends for
microcontroller CAN controllers: `no_std`, no HAL dependency, each driving the
controller's registers itself. Their channels confirm transmissions
(`ConfirmedTransmit` and `ReceiveEvents`): a frame is reported sent when the
controller says it was acknowledged, in bus order with the frames received.
They use the confirmed-transmission traits on our
[fork](https://github.com/aeolionics-rs/can-hal-rs)'s `aeolionics` branch, a
git dependency that `Cargo.lock` pins to a commit.

To build against a local checkout of the fork instead, for changes not yet
pushed, put a patch in `.cargo/local.toml`, which `.cargo/config.toml`
includes if it exists and git ignores:

```toml
[patch."https://github.com/aeolionics-rs/can-hal-rs"]
can-hal-rs = { path = "../can-hal-rs/can-hal" }
```

A patched build rewrites the patched entries in `Cargo.lock` as local paths.
That does no harm: an unpatched build puts the git entries back, and the
release workflow regenerates the lockfile before it builds.

# Crates

- `can-hal-bxcan`: ST's bxCAN (STM32F0, F1, F4 and others), classic CAN.
- `can-hal-fdcan`: ST's FDCAN, the cut-down Bosch M_CAN (STM32G0, G4, H5, U5
  and others), classic CAN and CAN FD.
- `nucleo-g431kb`: bench tests of the FDCAN driver on ST's NUCLEO-G431KB
  (`cargo run-fdcan-loopback`; `cargo run-fdcan-dar` with a jumper from PA12
  to PA11).

The crates came from [ferrocan](https://github.com/ferrocan/ferrocan),
the gs_usb adapter firmware that uses them, with their history. This
repository focuses on embedded drivers, but keeps each one self-contained:
none depends on another.

# Licence

MIT OR Apache-2.0, at your option.
