use core::option::{Option, Some, None};

extern {
    /* Symbols provided by the linker  */
    fn _stack_top();

    static     _etext : u32;
    static mut _data  : u32;
    static mut _edata : u32;
    static mut _bss   : u32;
    static mut _ebss  : u32;
}

#[no_mangle]
#[link_section=".isr_vector"]
pub static ISR_VECTOR: [Option<unsafe extern fn()>, .. 3] = [
  Some(_stack_top),
  Some(reset),            // Reset
  None,
  /* Leave the other entries out as long as we don't implement them,
   * the linker script will pad them with 0 */
];

#[no_mangle]
#[no_split_stack]
unsafe extern fn reset() {
    /* memset the .bss to 0 */
    let mut bss  = &mut _bss  as *mut u32;
    let     ebss = &mut _ebss as *mut u32;

    while bss < ebss {
        *bss = 0;
        bss = ((bss as u32) + 4) as *mut u32;
    }

    /* Copy .data section to SRAM. Initially it's stored in flash just
     * after the .text section */
    let mut data  = &mut _data  as *mut   u32;
    let     edata = &mut _edata as *mut   u32;
    let mut etext = &    _etext as *const u32;

    while data < edata {
        *data = *etext;
        data  = ((data as u32)  + 4) as *mut   u32;
        etext = ((etext as u32) + 4) as *const u32;
    }


    ::main();
}
