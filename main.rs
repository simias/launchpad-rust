#![no_std]
#![crate_type = "staticlib"]
#![feature(lang_items)]

extern crate core;

pub mod vector;

mod io;
mod map;
mod gpio;
pub mod sysctl;

static RED_LED   : u8 = gpio::PIN_1;
static BLUE_LED  : u8 = gpio::PIN_2;
static GREEN_LED : u8 = gpio::PIN_3;

#[no_split_stack]
#[no_mangle]
pub fn main() {

    sysctl::periph::enable(sysctl::periph::GPIOF);
    gpio::set_output(map::gpio::PORTF_BASE, RED_LED | BLUE_LED | GREEN_LED);

    gpio::write_pins(map::gpio::PORTF_BASE,
                     RED_LED | BLUE_LED | GREEN_LED,
                     GREEN_LED);
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {
}

#[lang = "stack_exhausted"]
extern fn stack_exhausted() {}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "fail_fmt"]
fn fail_fmt() -> ! { loop {} }
