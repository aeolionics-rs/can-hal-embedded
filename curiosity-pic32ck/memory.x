/* PIC32CK2051SG01144 (DS60001795; the DFP's linker script).
 *
 * The boot ROM hands over to the start of boot flash (BFM, 0x08000000) by
 * loading the stack pointer and PC from its first two words. The program
 * lives in program flash (PFM, 0x0C000000), so BFM holds only those two
 * words (`.bfm`, from `boot!`): the stack top and cortex-m-rt's Reset, which
 * (with `set-vtor`) points VTOR at the vector table in PFM. */
MEMORY
{
  BFM   : ORIGIN = 0x08000000, LENGTH = 128K
  FLASH : ORIGIN = 0x0C000000, LENGTH = 2048K
  RAM   : ORIGIN = 0x20000000, LENGTH = 512K
}

SECTIONS
{
  .bfm ORIGIN(BFM) :
  {
    KEEP(*(.bfm));
  } > BFM
}
INSERT AFTER .vector_table;
