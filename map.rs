/* Register map */



pub mod sysctl {
    // Run-Mode Clock Configuration
    pub static RCC : u32 = 0x400FE060;

    pub mod rcc {
        pub static MOSCDIS   : u32 = 0x00000001; // Main Oscillator Disable
        pub static OSCSRC_M  : u32 = 0x00000030; // Oscillator Source
        pub static XTAL_M    : u32 = 0x000007C0; // Crystal Value
        pub static BYPASS    : u32 = 0x00000800; // PLL Bypass
        pub static PWRDN     : u32 = 0x00002000; // PLL Power Down
        pub static USESYSDIV : u32 = 0x00400000; // Enable System Clock Divider
        pub static SYSDIV_M  : u32 = 0x07800000; // System Clock Divisor
    }

    // Run-Mode Clock Configuration 2
    pub static RCC2 : u32 = 0x400FE070;

    pub mod rcc2 {
        pub static OSCSRC2_M : u32 = 0x00000070; // Oscillator Source 2
        pub static BYPASS2   : u32 = 0x00000800; // PLL Bypass 2
        pub static PWRDN2    : u32 = 0x00002000; // Power-Down PLL 2
        pub static SYSDIV2_M : u32 = 0x1F800000; // System Clock Divisor 2
        pub static USERCC2   : u32 = 0x80000000; // Use RCC2
        pub static DIV400    : u32 = 0x40000000; // Divide PLL as 400 MHz
    }

    // Masked Interrupt Status and Clear
    pub static MISC : u32 = 0x400FE058; 

    pub mod misc {
        pub static PLLLMIS    : u32 = 0x00000040; // PLL Lock
        pub static MOSCPUPMIS : u32 = 0x00000100; // MOSC Power Up
    }

    // Raw Interrupt Status
    pub static RIS : u32 = 0x400FE050;

    pub mod ris {
        pub static MOSCPUPMIS : u32 = 0x00000100; // MOSC Power Up
    }

    // PLL Status
    pub static PLLSTAT : u32 = 0x400FE168;

    pub mod pllstat {
        pub static LOCK : u32 = 0x00000001; // PLL Lock
    }

    // Run mode clock gating control base for all peripherals
    pub static RCGCBASE : u32 = 0x400fe600;
}

pub mod gpio {
    // GPIO Port F
    pub static PORTF_BASE : u32 = 0x40025000;

    // GPIO Peripheral Configuration
    pub static O_PC : u32 = 0x00000FC4;

    // GPIO 2-mA Drive Select
    pub static O_DR2R : u32 = 0x00000500;

    // GPIO Digital Enable
    pub static O_DEN : u32 = 0x0000051C;

    // GPIO Direction
    pub static O_DIR : u32 = 0x00000400;

    // GPIO Data
    pub static O_DATA : u32 = 0x00000000;
}
