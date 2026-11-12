# can-hal-mcan: plan

A can-hal backend for the full Bosch M_CAN, with the same public shape as
`can-hal-fdcan` (confirmed transmission, receive events, bus status,
recovery, statistics), and frames stamped with the controller's own
timestamps: `Sent(at)` from the Tx event FIFO's TXTS, received frames from
RXTS. First target: the PIC32CK SG01 Curiosity Ultra. Later: the PIC32CM GC
(the same M_CAN behind a Microchip wrapper, plus a 32-bit timestamping unit),
the STM32H7 and the SAM E5x.

## The board: EV33A17A, PIC32CK SG01 Curiosity Ultra (DS70005529)

- PIC32CK2051SG01144: Cortex-M33, 120 MHz, 2 MB flash, 512 KB SRAM.
- On-board debugger: PKoB4 (MPLAB mode now; the nmea-mux board was switched
  to CMSIS-DAP with `pycmsisdapswitcher`). Virtual COM port on the Mac:
  `/dev/cu.usbmodem1132102`.
- CAN0: PB16 (TX) and PB17 (RX) to U701, an ATA6561 (CAN FD up to 5 Mbit/s,
  DS20005991B), its STBY tied to ground, so always in normal mode. Bus on
  J704: 1 CANL, 2 GND, 3 CANH. SW700 looks like the termination switch (two
  62 Ω, split); to confirm before connecting.
- CAN1: PB23/PB24 to U702, on J705.
- On the FD bus with the SH-C31A and SH-C31G (both ends terminated), the
  CK's termination stays off.

## The controller (DS60001795, chapter 39)

- CAN0 at `0x4501_E000`, CAN1 at `0x4502_0000`.
- Bosch's register layout from offset 0, with `MRCFG` at 0x08 (message RAM
  base: `OFFSET`, bits 23:16, plus bus QoS) where Bosch has `CUST`. No
  Microchip wrapper here (the PIC32CM GC puts the same registers at 0x100,
  behind interrupt-flag, `SYNCBUSY` and write-protect registers).
- Message RAM in system SRAM, up to 4352 words, its sections placed by start
  address (`SIDFC`, `XIDFC`, `RXF0C`, `RXF1C`, `RXBC`, `TXEFC`, `TXBC`) with
  element sizes in `RXESC`/`TXESC`. Elements are the same format as ST's.
- Interrupts in Bosch's numbering (TC 9, TCF 10, TEFN 12, TSW 16, EP 23,
  BO 25, PEA 27, PED 28, …), on two lines (`ILS`/`ILE`). CAN0's NVIC number
  looks like 79, CAN1's 80: to confirm.
- Clocks: kernel clock from GCLK peripheral channel 39 (CAN0) / 40 (CAN1),
  bus clock enabled by `MCLK.CLKMSK0[13]` (CAN0). An 80 MHz kernel clock
  gives 16 time quanta per bit at 5 Mbit/s and the same timings as the
  G431.
- Timestamps: the 16-bit counter (`TSCC`/`TSCV`), counting bit times with a
  prescaler of 1 to 16, captured at start of frame into RXTS and TXTS. It
  wraps every 65,536 counts (131 ms of bit times at 500 kbit/s), which the
  driver has to extend (below). No timestamping unit on the CK.
- Transmitter delay compensation (`TDCR`) as on ST's FDCAN.

## The driver

- Same structure as `can-hal-fdcan`: `Can<C, N>`, a `static State`,
  `on_interrupt` from the line 0 handler, `Config` and `BitTiming`, the
  ordering of reports (`order.rs`), frames to and from elements
  (`frame.rs`), and the single-shot failure watch on the timeout counter.
  Copied first; a shared crate later, once both drivers settle.
- Registers: Bosch's layout and interrupt bits, in its own `registers.rs`.
- Message RAM: a layout chosen at build time (filter, FIFO and buffer counts
  and element sizes, through const generics or a const layout) in a buffer
  the application provides as a `static`, whose address goes to `MRCFG` and
  the start-address registers. To start: no filters (accept all into FIFO 0,
  as `can-hal-fdcan` does), a Tx queue of a few buffers, a Tx event FIFO of
  the same depth.
- Timestamps: at each interrupt the driver reads `TSCV` with `C::now()` as an
  anchor and converts each RXTS/TXTS to `C::Instant` by its distance back from
  that anchor, in bit times. With TSW enabled the anchor is never more than
  one wrap old; a frame older than one wrap at handling would be ambiguous,
  so the interrupt latency budget is one wrap (131 ms at 500 kbit/s). The
  same would work for `can-hal-fdcan`, whose `Sent` uses the interrupt time
  today.
- Filters, FIFO 1, dedicated Rx buffers and the GC's timestamping unit
  later, each with its own API.

## The bench crate (`curiosity-pic32ck`, in this repository)

- Target `thumbv8m.main-none-eabihf`; `cortex-m-rt`; `memory.x` from the
  data sheet's memory map (flash and SRAM addresses to confirm, and any
  boot ROM handshake, as the PIC32CM SG00 has).
- PAC: svd2rust 0.37 from the `Microchip.PIC32CK-SG_DFP` 1.11.288 pack's
  SVD, the way nmea-mux's `fw-pic32cm-sg/pac/generate.sh` does it (with its
  `fix_svd.py`, if the CK's SVD needs the same fixes). Check the pack's
  licence before committing the SVD or the generated PAC.
- Clocks ourselves: 120 MHz CPU, a GCLK generator at 80 MHz for CAN0, from
  the PAC.
- Debug and flashing: CMSIS-DAP (switch the PKoB4 as for nmea-mux), then
  probe-rs with a `target-gen` target from the pack (as long_sentences did
  for the PL10) for defmt over RTT; pyOCD with the pack as the fallback.
- `Clock` for the driver from the DWT cycle counter, extended to 64 bits;
  an executor only if the tests need one (`embassy-executor` with no time
  driver, timeouts from the same counter).

## Steps

1. Tooling: download the pack, generate the PAC, switch the PKoB4 to
   CMSIS-DAP, get a target for probe-rs (or pyOCD), and blink an LED with a
   defmt line.
2. Clocks: 120 MHz CPU, 80 MHz for CAN0, checked with a timer or the counter.
3. `can-hal-mcan`: registers, message RAM layout, configuration, interrupt
   handling, internal loopback. The bench's loopback tests are
   `nucleo-g431kb`'s: every classic and FD frame, three at once with an
   abort, classic only.
4. Timestamps: anchor and conversion; `Sent` and `Received` in bus order
   with monotonic stamps; the wrap.
5. On the FD bus (J704 to the SH-C31A/G bus, termination off): classic and
   FD both ways at 500k/2M and 500k/5M, then the error-handling set
   (single-shot, bus-off on an H-L short, bus error frames, listen-only).
6. Later: filters, the PIC32CM GC (wrapper, timestamping unit), a shared
   core for `can-hal-fdcan` and `can-hal-mcan`.

## To confirm

- CAN0's NVIC number, and the pin function letter for CAN0 on PB16/PB17.
- The flash and SRAM addresses, and whether the CK needs a boot ROM
  handshake for flashing.
- `MRCFG.OFFSET`'s exact meaning (which address bits it supplies) and which
  SRAM the message RAM may use.
- SW700's position for termination off.
- That the PKoB4 on this board switches to CMSIS-DAP like the SG00 board's.
