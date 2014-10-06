use core::intrinsics::{volatile_load, volatile_store};

#[inline]
pub fn read32(reg: u32) -> u32 {
    unsafe {
        let r = reg as *const u32;
        volatile_load(r)
    }
}

#[inline]
pub fn write32(val: u32, reg: u32) {
    unsafe {
        let r = reg as *mut u32;
        volatile_store(r, val);
    }
}

#[inline]
/* Write a single bit in a 32bit word using hardware bitbanding */
pub fn write_bit(set: bool, bit: u32, reg: u32) {
    /* bitband offset */
    let mut bitband = reg & 0xf0000000 | 0x02000000;

    /* register offset */
    bitband |= (reg & 0x00fffff) << 5;
    /* bit offset */
    bitband |= bit << 2;

    write32(set as u32, bitband);
}
