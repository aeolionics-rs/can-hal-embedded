// SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The bench's first light: LED0 (PD20) blinks, and a line goes out over RTT.
//! Runs on the boot clocks, whatever they are; nothing is set up but the pin.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use pic32ck_sg01_pac as pac;
use rtt_target::{rprintln, rtt_init_print};

curiosity_pic32ck::boot!();

/// PORT group D, LED0's pin.
const GROUP_D: usize = 3;
const LED0: u32 = 1 << 20;

/// Breadcrumbs for a probe to read after a reset (SRAM keeps them; the boot
/// ROM only clears the first 2 KiB): a marker on reaching `main`, group D's
/// DIR as read back after setting it, the loop count, TT on this code, and
/// group D's NONSEC.
const CRUMBS: usize = 0x2004_0000;

fn crumb(index: usize, value: u32) {
    // SAFETY: SRAM well above anything this program uses.
    unsafe { core::ptr::write_volatile((CRUMBS + 4 * index) as *mut u32, value) }
}

#[entry]
fn main() -> ! {
    crumb(0, 0xB007_0001);
    // The security state: TT on this code's own address (its S bit, 22, is
    // set only for Secure code running in the Secure state).
    let tt: u32;
    // SAFETY: TT only reads the attribution of an address.
    unsafe { core::arch::asm!("tt {0}, {1}", out(reg) tt, in(reg) crumb as *const () as usize) };
    crumb(3, tt);
    // And on PORT's address: a Non-secure attribution there would make even
    // Secure code's accesses Non-secure.
    let tt_port: u32;
    // SAFETY: as above.
    unsafe { core::arch::asm!("tt {0}, {1}", out(reg) tt_port, in(reg) 0x4480_0000_usize) };
    crumb(10, tt_port);
    // SAU_CTRL, readable in the Secure state.
    // SAFETY: a read of a core register.
    crumb(11, unsafe { core::ptr::read_volatile(0xE000_EDD0 as *const u32) });
    // EXPERIMENT: the boot ROM leaves SAU_CTRL.ALLNS set, which makes the
    // SAU call all memory Non-secure; clear it, so that what the IDAU leaves
    // to the SAU is Secure.
    // SAFETY: SAU_CTRL, in the Secure state, before any Non-secure code.
    unsafe { core::ptr::write_volatile(0xE000_EDD0 as *mut u32, 0) };
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
    rtt_init_print!();
    rprintln!("curiosity-pic32ck: blink");
    // SAFETY: the only user of the peripherals.
    let p = unsafe { pac::Peripherals::steal() };
    let port = p.port.group(GROUP_D);
    // SAFETY: setting LED0's direction and toggling it touches nothing else.
    port.dirset().write(|w| unsafe { w.bits(LED0) });
    crumb(1, port.dir().read().bits());
    crumb(4, port.nonsec().read().bits());
    // What the PAC saw of that write: bridge interrupt flags, bridge B's
    // peripheral flags, write protection and non-secure status, and MCLK's
    // clock mask for bridge B.
    for (index, address) in [(5, 0x4401_C010), (6, 0x4401_C018), (7, 0x4401_C038), (8, 0x4401_C058), (9, 0x4401_2044)] {
        // SAFETY: status registers, read only.
        crumb(index, unsafe { core::ptr::read_volatile(address as *const u32) });
    }
    let mut count = 0u32;
    loop {
        port.outtgl().write(|w| unsafe { w.bits(LED0) });
        count += 1;
        crumb(2, count);
        rprintln!("toggle {}", count);
        cortex_m::asm::delay(4_000_000);
    }
}
