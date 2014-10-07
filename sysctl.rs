use core::iter::range;

/* Ported from tivaware's sysctl.c */
pub fn clock_en() {
    let mut rcc  = ::io::read32(::map::sysctl::RCC);
    let mut rcc2 = ::io::read32(::map::sysctl::RCC2);

    /* bypass PLL and clock dividers */
    rcc  |= ::map::sysctl::rcc::BYPASS;
    rcc  &= !::map::sysctl::rcc::USESYSDIV;
    rcc2 |= ::map::sysctl::rcc2::BYPASS2;

    ::io::write32(rcc,  ::map::sysctl::RCC);
    ::io::write32(rcc2, ::map::sysctl::RCC2);

    /* Check if the main oscillator is disabled */
    if rcc & ::map::sysctl::rcc::MOSCDIS != 0 {

        /* Clear the main oscillator power up interrupt */
        ::io::write32(::map::sysctl::misc::MOSCPUPMIS,
                      ::map::sysctl::MISC);

        /* Enable the main oscillator */
        rcc &= !::map::sysctl::rcc::MOSCDIS;
        ::io::write32(rcc, ::map::sysctl::RCC);

        /* Poll the RIS until the oscillator is powered up. The
         * timeout value is directly lifted from the tivaware code, no
         * idea where it comes from. */
        for _ in range(0, 0x80000u) {
            let ris = ::io::read32(::map::sysctl::RIS);

            if ris & ::map::sysctl::ris::MOSCPUPMIS != 0 {
                /* Oscillator powered up */
                break;
            }
        }

        /* XXX: what should we do when the timeout is reached and the
         * oscillator did not power up? The tivaware code just
         * silently returns here, but it seems a bit... abrupt. I
         * suppose it's pretty unlikely anyway. */
        
        /* Set the new crystal valiue and oscillator source. */
        rcc &= !(::map::sysctl::rcc::XTAL_M | ::map::sysctl::rcc::OSCSRC_M);
        rcc |= 0x540;

        rcc2 &= !(::map::sysctl::rcc2::USERCC2 |
                  ::map::sysctl::rcc2::OSCSRC2_M);

        ::io::write32(rcc,  ::map::sysctl::RCC);
        ::io::write32(rcc2, ::map::sysctl::RCC2);

        /* PLL configuration */
        rcc  &= ::map::sysctl::rcc::PWRDN;
        rcc2 &= ::map::sysctl::rcc2::PWRDN2;

        /* Clear PLL lock interrupt */
        ::io::write32(::map::sysctl::misc::PLLLMIS, ::map::sysctl::MISC);

        ::io::write32(rcc,  ::map::sysctl::RCC);
        ::io::write32(rcc2, ::map::sysctl::RCC2);

        /* Set the system divider and disable the appropriate
         * oscillators */
        rcc &= !(::map::sysctl::rcc::SYSDIV_M  |
                 ::map::sysctl::rcc::USESYSDIV |
                 ::map::sysctl::rcc::MOSCDIS);
        rcc |= 0x1c00000;

        rcc2 &= !::map::sysctl::rcc2::SYSDIV2_M;
        rcc2 |= 0x1800000;
        rcc2 &= !::map::sysctl::rcc2::DIV400;

        /* Wait for pll lock */
        for _ in range(0, 0x8000u) {
            let pllstat = ::io::read32(::map::sysctl::PLLSTAT);
            if pllstat & ::map::sysctl::pllstat::LOCK != 0 {
                break;
            }
        }

        /* Enable PLL */
        rcc  &= !::map::sysctl::rcc::BYPASS;
        rcc2 &= !::map::sysctl::rcc2::BYPASS2;

        ::io::write32(rcc,  ::map::sysctl::RCC);
        ::io::write32(rcc2, ::map::sysctl::RCC2);

        /* XXX implement delay*/
        //delay(16);
    }
}

pub mod periph {
    pub static GPIOF : u32 = 0xf0000805;


    #[inline]
    fn set_enable(p: u32, enable: bool) {
        ::io::write_bit(enable,
                        p & 0xff,
                        ::map::sysctl::RCGCBASE + ((p & 0xff00) >> 8));
    }

    pub fn enable(p: u32) {
        set_enable(p, true);
    }
}
