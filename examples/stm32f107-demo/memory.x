MEMORY
{
    /* STM32F107 user code start addr */
    FLASH : ORIGIN = 0x08000000, LENGTH = 64K
    /* STM32F107 has up to 64 Kbytes of SRAM. */
    RAM : ORIGIN = 0x20000000, LENGTH = 32K
}
