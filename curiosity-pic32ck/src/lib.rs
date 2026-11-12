// SPDX-FileCopyrightText: 2026-2026. Aeolionics, LLC
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! What every bench program on the PIC32CK SG01 Curiosity Ultra needs:
//! [`boot!`].

#![no_std]

/// The two words the boot ROM starts from, and the SRAM initialisation.
/// Invoke it once, at the top level of each program.
///
/// - The boot ROM loads the stack pointer and PC from the first two words of
///   boot flash (0x08000000); the program and its vector table are in
///   program flash, so boot flash holds just the stack top and cortex-m-rt's
///   `Reset` (`.bfm` in `memory.x`), and `Reset` (`set-vtor`) points VTOR at
///   the vector table.
/// - SRAM has ECC, and its content starts random: a word must be written
///   whole before it is read, or written in part (DS60001795 13.4.2). The
///   `__pre_init` that cortex-m-rt calls before it sets up `.data` and `.bss`
///   writes zero to every word from the start of SRAM up to the stack
///   pointer. In assembly: no Rust may touch RAM yet.
#[macro_export]
macro_rules! boot {
    () => {
        mod boot {
            unsafe extern "C" {
                /// cortex-m-rt: the top of the stack, and the reset handler.
                static _stack_start: u32;
                fn Reset() -> !;
            }

            #[repr(C)]
            pub struct BootVector([*const (); 2]);

            // SAFETY: two addresses, read only by the boot ROM.
            unsafe impl Sync for BootVector {}

            #[unsafe(link_section = ".bfm")]
            #[used]
            pub static BOOT: BootVector = BootVector([&raw const _stack_start as *const (), Reset as *const ()]);

            core::arch::global_asm!(
                ".section .text.__pre_init, \"ax\"",
                ".global __pre_init",
                ".type __pre_init, %function",
                ".thumb_func",
                "__pre_init:",
                "    ldr r0, =0x20000000",
                "    mov r1, sp",
                "    movs r2, #0",
                "1:  cmp r0, r1",
                "    bhs 2f",
                "    str r2, [r0]",
                "    adds r0, #4",
                "    b 1b",
                "2:  bx lr",
                ".ltorg",
            );
        }
    };
}
