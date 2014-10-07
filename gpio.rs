use core::iter::range;

pub static PIN_1 : u8 = 0x02;
pub static PIN_2 : u8 = 0x04;
pub static PIN_3 : u8 = 0x08;

fn set_config(port: u32, pins: u8) {
    for bit in range(0, 8) {
        if pins & (1 << bit) != 0 {
            let pc_addr = port + ::map::gpio::O_PC;

            let mut pc = ::io::read32(pc_addr);
            pc &= !(0x3 << (2 * bit));
            ::io::write32(pc, pc_addr);
        }

        let dr2r_addr = port + ::map::gpio::O_DR2R;

        let mut dr2r = ::io::read32(dr2r_addr);
        dr2r |= pins as u32;
        ::io::write32(dr2r, dr2r_addr);

        let den_addr = port + ::map::gpio::O_DEN;

        let mut den = ::io::read32(den_addr);
        den |= pins as u32;
        ::io::write32(den, den_addr);
    }
}

pub fn set_output(port: u32, pins: u8) {
    set_config(port, pins);

    let dir_addr = port + ::map::gpio::O_DIR;

    let mut dir = ::io::read32(dir_addr);
    dir |= pins as u32;
    ::io::write32(dir, dir_addr);
}


pub fn write_pins(port: u32, pins: u8, val: u8) {
    let data_addr = port + ::map::gpio::O_DATA + ((pins as u32) << 2);

    ::io::write32(val as u32, data_addr);
}
