#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CCM_ANALOG

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Analog USB1 480MHz PLL Control Register
pub mod PLL_USB1 {

    /// This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22.
    pub mod DIV_SELECT {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Powers the 9-phase PLL outputs for USBPHYn
    pub mod EN_USB_CLKS {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLL outputs for USBPHYn off.
            pub const EN_USB_CLKS_0: u32 = 0b0;

            /// 0b1: PLL outputs for USBPHYn on.
            pub const EN_USB_CLKS_1: u32 = 0b1;
        }
    }

    /// Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens.
    pub mod POWER {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable the PLL clock output.
    pub mod ENABLE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Determines the bypass source.
    pub mod BYPASS_CLK_SRC {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Select the 24MHz oscillator as source.
            pub const REF_CLK_24M: u32 = 0b00;
        }
    }

    /// Bypass the PLL.
    pub mod BYPASS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 1 - PLL is currently locked. 0 - PLL is not currently locked.
    pub mod LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Analog USB1 480MHz PLL Control Register
pub mod PLL_USB1_SET {
    pub use super::PLL_USB1::BYPASS;
    pub use super::PLL_USB1::BYPASS_CLK_SRC;
    pub use super::PLL_USB1::DIV_SELECT;
    pub use super::PLL_USB1::ENABLE;
    pub use super::PLL_USB1::EN_USB_CLKS;
    pub use super::PLL_USB1::LOCK;
    pub use super::PLL_USB1::POWER;
}

/// Analog USB1 480MHz PLL Control Register
pub mod PLL_USB1_CLR {
    pub use super::PLL_USB1::BYPASS;
    pub use super::PLL_USB1::BYPASS_CLK_SRC;
    pub use super::PLL_USB1::DIV_SELECT;
    pub use super::PLL_USB1::ENABLE;
    pub use super::PLL_USB1::EN_USB_CLKS;
    pub use super::PLL_USB1::LOCK;
    pub use super::PLL_USB1::POWER;
}

/// Analog USB1 480MHz PLL Control Register
pub mod PLL_USB1_TOG {
    pub use super::PLL_USB1::BYPASS;
    pub use super::PLL_USB1::BYPASS_CLK_SRC;
    pub use super::PLL_USB1::DIV_SELECT;
    pub use super::PLL_USB1::ENABLE;
    pub use super::PLL_USB1::EN_USB_CLKS;
    pub use super::PLL_USB1::LOCK;
    pub use super::PLL_USB1::POWER;
}

/// Analog System PLL Control Register
pub mod PLL_SYS {

    /// This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22.
    pub mod DIV_SELECT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Powers down the PLL.
    pub mod POWERDOWN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable PLL output
    pub mod ENABLE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Determines the bypass source.
    pub mod BYPASS_CLK_SRC {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Select the 24MHz oscillator as source.
            pub const REF_CLK_24M: u32 = 0b00;
        }
    }

    /// Bypass the PLL.
    pub mod BYPASS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 1 - PLL is currently locked; 0 - PLL is not currently locked.
    pub mod LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Analog System PLL Control Register
pub mod PLL_SYS_SET {
    pub use super::PLL_SYS::BYPASS;
    pub use super::PLL_SYS::BYPASS_CLK_SRC;
    pub use super::PLL_SYS::DIV_SELECT;
    pub use super::PLL_SYS::ENABLE;
    pub use super::PLL_SYS::LOCK;
    pub use super::PLL_SYS::POWERDOWN;
}

/// Analog System PLL Control Register
pub mod PLL_SYS_CLR {
    pub use super::PLL_SYS::BYPASS;
    pub use super::PLL_SYS::BYPASS_CLK_SRC;
    pub use super::PLL_SYS::DIV_SELECT;
    pub use super::PLL_SYS::ENABLE;
    pub use super::PLL_SYS::LOCK;
    pub use super::PLL_SYS::POWERDOWN;
}

/// Analog System PLL Control Register
pub mod PLL_SYS_TOG {
    pub use super::PLL_SYS::BYPASS;
    pub use super::PLL_SYS::BYPASS_CLK_SRC;
    pub use super::PLL_SYS::DIV_SELECT;
    pub use super::PLL_SYS::ENABLE;
    pub use super::PLL_SYS::LOCK;
    pub use super::PLL_SYS::POWERDOWN;
}

/// 528MHz System PLL Spread Spectrum Register
pub mod PLL_SYS_SS {

    /// Frequency change step = step/CCM_ANALOG_PLL_SYS_DENOM\[B\]*24MHz.
    pub mod STEP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (15 bits: 0x7fff << 0)
        pub const mask: u32 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable bit
    pub mod ENABLE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Spread spectrum modulation disabled
            pub const ENABLE_0: u32 = 0b0;

            /// 0b1: Soread spectrum modulation enabled
            pub const ENABLE_1: u32 = 0b1;
        }
    }

    /// Frequency change = stop/CCM_ANALOG_PLL_SYS_DENOM\[B\]*24MHz.
    pub mod STOP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Numerator of 528MHz System PLL Fractional Loop Divider Register
pub mod PLL_SYS_NUM {

    /// 30 bit numerator (A) of fractional loop divider (signed integer).
    pub mod A {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (30 bits: 0x3fffffff << 0)
        pub const mask: u32 = 0x3fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Denominator of 528MHz System PLL Fractional Loop Divider Register
pub mod PLL_SYS_DENOM {

    /// 30 bit Denominator (B) of fractional loop divider (unsigned integer).
    pub mod B {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (30 bits: 0x3fffffff << 0)
        pub const mask: u32 = 0x3fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Analog Audio PLL control Register
pub mod PLL_AUDIO {

    /// This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54.
    pub mod DIV_SELECT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Powers down the PLL.
    pub mod POWERDOWN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable PLL output
    pub mod ENABLE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Determines the bypass source.
    pub mod BYPASS_CLK_SRC {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Select the 24MHz oscillator as source.
            pub const REF_CLK_24M: u32 = 0b00;
        }
    }

    /// Bypass the PLL.
    pub mod BYPASS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// These bits implement a divider after the PLL, but before the enable and bypass mux.
    pub mod POST_DIV_SELECT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (2 bits: 0b11 << 19)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Divide by 4.
            pub const POST_DIV_SELECT_0: u32 = 0b00;

            /// 0b01: Divide by 2.
            pub const POST_DIV_SELECT_1: u32 = 0b01;

            /// 0b10: Divide by 1.
            pub const POST_DIV_SELECT_2: u32 = 0b10;
        }
    }

    /// 1 - PLL is currently locked. 0 - PLL is not currently locked.
    pub mod LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Analog Audio PLL control Register
pub mod PLL_AUDIO_SET {
    pub use super::PLL_AUDIO::BYPASS;
    pub use super::PLL_AUDIO::BYPASS_CLK_SRC;
    pub use super::PLL_AUDIO::DIV_SELECT;
    pub use super::PLL_AUDIO::ENABLE;
    pub use super::PLL_AUDIO::LOCK;
    pub use super::PLL_AUDIO::POST_DIV_SELECT;
    pub use super::PLL_AUDIO::POWERDOWN;
}

/// Analog Audio PLL control Register
pub mod PLL_AUDIO_CLR {
    pub use super::PLL_AUDIO::BYPASS;
    pub use super::PLL_AUDIO::BYPASS_CLK_SRC;
    pub use super::PLL_AUDIO::DIV_SELECT;
    pub use super::PLL_AUDIO::ENABLE;
    pub use super::PLL_AUDIO::LOCK;
    pub use super::PLL_AUDIO::POST_DIV_SELECT;
    pub use super::PLL_AUDIO::POWERDOWN;
}

/// Analog Audio PLL control Register
pub mod PLL_AUDIO_TOG {
    pub use super::PLL_AUDIO::BYPASS;
    pub use super::PLL_AUDIO::BYPASS_CLK_SRC;
    pub use super::PLL_AUDIO::DIV_SELECT;
    pub use super::PLL_AUDIO::ENABLE;
    pub use super::PLL_AUDIO::LOCK;
    pub use super::PLL_AUDIO::POST_DIV_SELECT;
    pub use super::PLL_AUDIO::POWERDOWN;
}

/// Numerator of Audio PLL Fractional Loop Divider Register
pub mod PLL_AUDIO_NUM {
    pub use super::PLL_SYS_NUM::A;
}

/// Denominator of Audio PLL Fractional Loop Divider Register
pub mod PLL_AUDIO_DENOM {
    pub use super::PLL_SYS_DENOM::B;
}

/// Analog ENET PLL Control Register
pub mod PLL_ENET {

    /// Controls the frequency of the ethernet reference clock
    pub mod DIV_SELECT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Powers down the PLL.
    pub mod POWERDOWN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable the ethernet clock output.
    pub mod ENABLE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Determines the bypass source.
    pub mod BYPASS_CLK_SRC {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Select the 24MHz oscillator as source.
            pub const REF_CLK_24M: u32 = 0b00;
        }
    }

    /// Bypass the PLL.
    pub mod BYPASS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable the PLL providing ENET 25 MHz reference clock
    pub mod ENET_25M_REF_EN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable the PLL providing ENET 500 MHz reference clock
    pub mod ENET_500M_REF_EN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// 1 - PLL is currently locked; 0 - PLL is not currently locked.
    pub mod LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Analog ENET PLL Control Register
pub mod PLL_ENET_SET {
    pub use super::PLL_ENET::BYPASS;
    pub use super::PLL_ENET::BYPASS_CLK_SRC;
    pub use super::PLL_ENET::DIV_SELECT;
    pub use super::PLL_ENET::ENABLE;
    pub use super::PLL_ENET::ENET_25M_REF_EN;
    pub use super::PLL_ENET::ENET_500M_REF_EN;
    pub use super::PLL_ENET::LOCK;
    pub use super::PLL_ENET::POWERDOWN;
}

/// Analog ENET PLL Control Register
pub mod PLL_ENET_CLR {
    pub use super::PLL_ENET::BYPASS;
    pub use super::PLL_ENET::BYPASS_CLK_SRC;
    pub use super::PLL_ENET::DIV_SELECT;
    pub use super::PLL_ENET::ENABLE;
    pub use super::PLL_ENET::ENET_25M_REF_EN;
    pub use super::PLL_ENET::ENET_500M_REF_EN;
    pub use super::PLL_ENET::LOCK;
    pub use super::PLL_ENET::POWERDOWN;
}

/// Analog ENET PLL Control Register
pub mod PLL_ENET_TOG {
    pub use super::PLL_ENET::BYPASS;
    pub use super::PLL_ENET::BYPASS_CLK_SRC;
    pub use super::PLL_ENET::DIV_SELECT;
    pub use super::PLL_ENET::ENABLE;
    pub use super::PLL_ENET::ENET_25M_REF_EN;
    pub use super::PLL_ENET::ENET_500M_REF_EN;
    pub use super::PLL_ENET::LOCK;
    pub use super::PLL_ENET::POWERDOWN;
}

/// 480MHz Clock (PLL3) Phase Fractional Divider Control Register
pub mod PFD_480 {

    /// This field controls the fractional divide value
    pub mod PFD0_FRAC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code
    pub mod PFD0_STABLE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// If set to 1, the IO fractional divider clock (reference ref_pfd0) is off (power savings)
    pub mod PFD0_CLKGATE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This field controls the fractional divide value
    pub mod PFD1_FRAC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code
    pub mod PFD1_STABLE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IO Clock Gate
    pub mod PFD1_CLKGATE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This field controls the fractional divide value
    pub mod PFD2_FRAC {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code
    pub mod PFD2_STABLE {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IO Clock Gate
    pub mod PFD2_CLKGATE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This field controls the fractional divide value
    pub mod PFD3_FRAC {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (6 bits: 0x3f << 24)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This read-only bitfield is for DIAGNOSTIC PURPOSES ONLY since the fractional divider should become stable quickly enough that this field will never need to be used by either device driver or application code
    pub mod PFD3_STABLE {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IO Clock Gate
    pub mod PFD3_CLKGATE {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// 480MHz Clock (PLL3) Phase Fractional Divider Control Register
pub mod PFD_480_SET {
    pub use super::PFD_480::PFD0_CLKGATE;
    pub use super::PFD_480::PFD0_FRAC;
    pub use super::PFD_480::PFD0_STABLE;
    pub use super::PFD_480::PFD1_CLKGATE;
    pub use super::PFD_480::PFD1_FRAC;
    pub use super::PFD_480::PFD1_STABLE;
    pub use super::PFD_480::PFD2_CLKGATE;
    pub use super::PFD_480::PFD2_FRAC;
    pub use super::PFD_480::PFD2_STABLE;
    pub use super::PFD_480::PFD3_CLKGATE;
    pub use super::PFD_480::PFD3_FRAC;
    pub use super::PFD_480::PFD3_STABLE;
}

/// 480MHz Clock (PLL3) Phase Fractional Divider Control Register
pub mod PFD_480_CLR {
    pub use super::PFD_480::PFD0_CLKGATE;
    pub use super::PFD_480::PFD0_FRAC;
    pub use super::PFD_480::PFD0_STABLE;
    pub use super::PFD_480::PFD1_CLKGATE;
    pub use super::PFD_480::PFD1_FRAC;
    pub use super::PFD_480::PFD1_STABLE;
    pub use super::PFD_480::PFD2_CLKGATE;
    pub use super::PFD_480::PFD2_FRAC;
    pub use super::PFD_480::PFD2_STABLE;
    pub use super::PFD_480::PFD3_CLKGATE;
    pub use super::PFD_480::PFD3_FRAC;
    pub use super::PFD_480::PFD3_STABLE;
}

/// 480MHz Clock (PLL3) Phase Fractional Divider Control Register
pub mod PFD_480_TOG {
    pub use super::PFD_480::PFD0_CLKGATE;
    pub use super::PFD_480::PFD0_FRAC;
    pub use super::PFD_480::PFD0_STABLE;
    pub use super::PFD_480::PFD1_CLKGATE;
    pub use super::PFD_480::PFD1_FRAC;
    pub use super::PFD_480::PFD1_STABLE;
    pub use super::PFD_480::PFD2_CLKGATE;
    pub use super::PFD_480::PFD2_FRAC;
    pub use super::PFD_480::PFD2_STABLE;
    pub use super::PFD_480::PFD3_CLKGATE;
    pub use super::PFD_480::PFD3_FRAC;
    pub use super::PFD_480::PFD3_STABLE;
}

/// 528MHz Clock (PLL2) Phase Fractional Divider Control Register
pub mod PFD_528 {
    pub use super::PFD_480::PFD0_CLKGATE;
    pub use super::PFD_480::PFD0_FRAC;
    pub use super::PFD_480::PFD0_STABLE;
    pub use super::PFD_480::PFD1_CLKGATE;
    pub use super::PFD_480::PFD1_FRAC;
    pub use super::PFD_480::PFD1_STABLE;
    pub use super::PFD_480::PFD2_CLKGATE;
    pub use super::PFD_480::PFD2_FRAC;
    pub use super::PFD_480::PFD2_STABLE;
    pub use super::PFD_480::PFD3_CLKGATE;
    pub use super::PFD_480::PFD3_FRAC;
    pub use super::PFD_480::PFD3_STABLE;
}

/// 528MHz Clock (PLL2) Phase Fractional Divider Control Register
pub mod PFD_528_SET {
    pub use super::PFD_480::PFD0_CLKGATE;
    pub use super::PFD_480::PFD0_FRAC;
    pub use super::PFD_480::PFD0_STABLE;
    pub use super::PFD_480::PFD1_CLKGATE;
    pub use super::PFD_480::PFD1_FRAC;
    pub use super::PFD_480::PFD1_STABLE;
    pub use super::PFD_480::PFD2_CLKGATE;
    pub use super::PFD_480::PFD2_FRAC;
    pub use super::PFD_480::PFD2_STABLE;
    pub use super::PFD_480::PFD3_CLKGATE;
    pub use super::PFD_480::PFD3_FRAC;
    pub use super::PFD_480::PFD3_STABLE;
}

/// 528MHz Clock (PLL2) Phase Fractional Divider Control Register
pub mod PFD_528_CLR {
    pub use super::PFD_480::PFD0_CLKGATE;
    pub use super::PFD_480::PFD0_FRAC;
    pub use super::PFD_480::PFD0_STABLE;
    pub use super::PFD_480::PFD1_CLKGATE;
    pub use super::PFD_480::PFD1_FRAC;
    pub use super::PFD_480::PFD1_STABLE;
    pub use super::PFD_480::PFD2_CLKGATE;
    pub use super::PFD_480::PFD2_FRAC;
    pub use super::PFD_480::PFD2_STABLE;
    pub use super::PFD_480::PFD3_CLKGATE;
    pub use super::PFD_480::PFD3_FRAC;
    pub use super::PFD_480::PFD3_STABLE;
}

/// 528MHz Clock (PLL2) Phase Fractional Divider Control Register
pub mod PFD_528_TOG {
    pub use super::PFD_480::PFD0_CLKGATE;
    pub use super::PFD_480::PFD0_FRAC;
    pub use super::PFD_480::PFD0_STABLE;
    pub use super::PFD_480::PFD1_CLKGATE;
    pub use super::PFD_480::PFD1_FRAC;
    pub use super::PFD_480::PFD1_STABLE;
    pub use super::PFD_480::PFD2_CLKGATE;
    pub use super::PFD_480::PFD2_FRAC;
    pub use super::PFD_480::PFD2_STABLE;
    pub use super::PFD_480::PFD3_CLKGATE;
    pub use super::PFD_480::PFD3_FRAC;
    pub use super::PFD_480::PFD3_STABLE;
}

/// Miscellaneous Register 0
pub mod MISC0 {

    /// Control bit to power-down the analog bandgap reference circuitry
    pub mod REFTOP_PWD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Control bit to disable the self-bias circuit in the analog bandgap
    pub mod REFTOP_SELFBIASOFF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Uses coarse bias currents for startup
            pub const REFTOP_SELFBIASOFF_0: u32 = 0b0;

            /// 0b1: Uses bandgap-based bias currents for best performance.
            pub const REFTOP_SELFBIASOFF_1: u32 = 0b1;
        }
    }

    /// Not related to CCM. See Power Management Unit (PMU)
    pub mod REFTOP_VBGADJ {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Nominal VBG
            pub const REFTOP_VBGADJ_0: u32 = 0b000;

            /// 0b001: VBG+0.78%
            pub const REFTOP_VBGADJ_1: u32 = 0b001;

            /// 0b010: VBG+1.56%
            pub const REFTOP_VBGADJ_2: u32 = 0b010;

            /// 0b011: VBG+2.34%
            pub const REFTOP_VBGADJ_3: u32 = 0b011;

            /// 0b100: VBG-0.78%
            pub const REFTOP_VBGADJ_4: u32 = 0b100;

            /// 0b101: VBG-1.56%
            pub const REFTOP_VBGADJ_5: u32 = 0b101;

            /// 0b110: VBG-2.34%
            pub const REFTOP_VBGADJ_6: u32 = 0b110;

            /// 0b111: VBG-3.12%
            pub const REFTOP_VBGADJ_7: u32 = 0b111;
        }
    }

    /// Status bit that signals the analog bandgap voltage is up and stable
    pub mod REFTOP_VBGUP {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Configure the analog behavior in stop mode.
    pub mod STOP_MODE_CONFIG {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: All analog except rtc powered down on stop mode assertion. XtalOsc=on, RCOsc=off;
            pub const STOP_MODE_CONFIG_0: u32 = 0b00;

            /// 0b01: Certain analog functions such as certain regulators left up. XtalOsc=on, RCOsc=off;
            pub const STOP_MODE_CONFIG_1: u32 = 0b01;

            /// 0b10: XtalOsc=off, RCOsc=on, Old BG=on, New BG=off.
            pub const STOP_MODE_CONFIG_2: u32 = 0b10;

            /// 0b11: XtalOsc=off, RCOsc=on, Old BG=off, New BG=on.
            pub const STOP_MODE_CONFIG_3: u32 = 0b11;
        }
    }

    /// This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN.
    pub mod DISCON_HIGH_SNVS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Turn on the switch
            pub const DISCON_HIGH_SNVS_0: u32 = 0b0;

            /// 0b1: Turn off the switch
            pub const DISCON_HIGH_SNVS_1: u32 = 0b1;
        }
    }

    /// This field determines the bias current in the 24MHz oscillator
    pub mod OSC_I {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Nominal
            pub const NOMINAL: u32 = 0b00;

            /// 0b01: Decrease current by 12.5%
            pub const MINUS_12_5_PERCENT: u32 = 0b01;

            /// 0b10: Decrease current by 25.0%
            pub const MINUS_25_PERCENT: u32 = 0b10;

            /// 0b11: Decrease current by 37.5%
            pub const MINUS_37_5_PERCENT: u32 = 0b11;
        }
    }

    /// Status bit that signals that the output of the 24-MHz crystal oscillator is stable
    pub mod OSC_XTALOK {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit enables the detector that signals when the 24MHz crystal oscillator is stable
    pub mod OSC_XTALOK_EN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block
    pub mod CLKGATE_CTRL {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Allow the logic to automatically gate the clock when the XTAL is powered down.
            pub const ALLOW_AUTO_GATE: u32 = 0b0;

            /// 0b1: Prevent the logic from ever gating off the clock.
            pub const NO_AUTO_GATE: u32 = 0b1;
        }
    }

    /// This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block
    pub mod CLKGATE_DELAY {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (3 bits: 0b111 << 26)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: 0.5ms
            pub const CLKGATE_DELAY_0: u32 = 0b000;

            /// 0b001: 1.0ms
            pub const CLKGATE_DELAY_1: u32 = 0b001;

            /// 0b010: 2.0ms
            pub const CLKGATE_DELAY_2: u32 = 0b010;

            /// 0b011: 3.0ms
            pub const CLKGATE_DELAY_3: u32 = 0b011;

            /// 0b100: 4.0ms
            pub const CLKGATE_DELAY_4: u32 = 0b100;

            /// 0b101: 5.0ms
            pub const CLKGATE_DELAY_5: u32 = 0b101;

            /// 0b110: 6.0ms
            pub const CLKGATE_DELAY_6: u32 = 0b110;

            /// 0b111: 7.0ms
            pub const CLKGATE_DELAY_7: u32 = 0b111;
        }
    }

    /// This field indicates which chip source is being used for the rtc clock
    pub mod RTC_XTAL_SOURCE {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Internal ring oscillator
            pub const RTC_XTAL_SOURCE_0: u32 = 0b0;

            /// 0b1: RTC_XTAL
            pub const RTC_XTAL_SOURCE_1: u32 = 0b1;
        }
    }

    /// This field powers down the 24M crystal oscillator if set true
    pub mod XTAL_24M_PWD {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Miscellaneous Register 0
pub mod MISC0_SET {
    pub use super::MISC0::CLKGATE_CTRL;
    pub use super::MISC0::CLKGATE_DELAY;
    pub use super::MISC0::DISCON_HIGH_SNVS;
    pub use super::MISC0::OSC_I;
    pub use super::MISC0::OSC_XTALOK;
    pub use super::MISC0::OSC_XTALOK_EN;
    pub use super::MISC0::REFTOP_PWD;
    pub use super::MISC0::REFTOP_SELFBIASOFF;
    pub use super::MISC0::REFTOP_VBGADJ;
    pub use super::MISC0::REFTOP_VBGUP;
    pub use super::MISC0::RTC_XTAL_SOURCE;
    pub use super::MISC0::STOP_MODE_CONFIG;
    pub use super::MISC0::XTAL_24M_PWD;
}

/// Miscellaneous Register 0
pub mod MISC0_CLR {
    pub use super::MISC0::CLKGATE_CTRL;
    pub use super::MISC0::CLKGATE_DELAY;
    pub use super::MISC0::DISCON_HIGH_SNVS;
    pub use super::MISC0::OSC_I;
    pub use super::MISC0::OSC_XTALOK;
    pub use super::MISC0::OSC_XTALOK_EN;
    pub use super::MISC0::REFTOP_PWD;
    pub use super::MISC0::REFTOP_SELFBIASOFF;
    pub use super::MISC0::REFTOP_VBGADJ;
    pub use super::MISC0::REFTOP_VBGUP;
    pub use super::MISC0::RTC_XTAL_SOURCE;
    pub use super::MISC0::STOP_MODE_CONFIG;
    pub use super::MISC0::XTAL_24M_PWD;
}

/// Miscellaneous Register 0
pub mod MISC0_TOG {
    pub use super::MISC0::CLKGATE_CTRL;
    pub use super::MISC0::CLKGATE_DELAY;
    pub use super::MISC0::DISCON_HIGH_SNVS;
    pub use super::MISC0::OSC_I;
    pub use super::MISC0::OSC_XTALOK;
    pub use super::MISC0::OSC_XTALOK_EN;
    pub use super::MISC0::REFTOP_PWD;
    pub use super::MISC0::REFTOP_SELFBIASOFF;
    pub use super::MISC0::REFTOP_VBGADJ;
    pub use super::MISC0::REFTOP_VBGUP;
    pub use super::MISC0::RTC_XTAL_SOURCE;
    pub use super::MISC0::STOP_MODE_CONFIG;
    pub use super::MISC0::XTAL_24M_PWD;
}

/// Miscellaneous Register 1
pub mod MISC1 {

    /// This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off
    pub mod PFD_480_AUTOGATE_EN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off
    pub mod PFD_528_AUTOGATE_EN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature
    pub mod IRQ_TEMPPANIC {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This status bit is set to one when the temperature sensor low interrupt asserts for low temperature
    pub mod IRQ_TEMPLOW {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This status bit is set to one when the temperature sensor high interrupt asserts for high temperature
    pub mod IRQ_TEMPHIGH {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This status bit is set to one when when any of the analog regulator brownout interrupts assert
    pub mod IRQ_ANA_BO {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// This status bit is set to one when when any of the digital regulator brownout interrupts assert
    pub mod IRQ_DIG_BO {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Miscellaneous Register 1
pub mod MISC1_SET {
    pub use super::MISC1::IRQ_ANA_BO;
    pub use super::MISC1::IRQ_DIG_BO;
    pub use super::MISC1::IRQ_TEMPHIGH;
    pub use super::MISC1::IRQ_TEMPLOW;
    pub use super::MISC1::IRQ_TEMPPANIC;
    pub use super::MISC1::PFD_480_AUTOGATE_EN;
    pub use super::MISC1::PFD_528_AUTOGATE_EN;
}

/// Miscellaneous Register 1
pub mod MISC1_CLR {
    pub use super::MISC1::IRQ_ANA_BO;
    pub use super::MISC1::IRQ_DIG_BO;
    pub use super::MISC1::IRQ_TEMPHIGH;
    pub use super::MISC1::IRQ_TEMPLOW;
    pub use super::MISC1::IRQ_TEMPPANIC;
    pub use super::MISC1::PFD_480_AUTOGATE_EN;
    pub use super::MISC1::PFD_528_AUTOGATE_EN;
}

/// Miscellaneous Register 1
pub mod MISC1_TOG {
    pub use super::MISC1::IRQ_ANA_BO;
    pub use super::MISC1::IRQ_DIG_BO;
    pub use super::MISC1::IRQ_TEMPHIGH;
    pub use super::MISC1::IRQ_TEMPLOW;
    pub use super::MISC1::IRQ_TEMPPANIC;
    pub use super::MISC1::PFD_480_AUTOGATE_EN;
    pub use super::MISC1::PFD_528_AUTOGATE_EN;
}

/// Miscellaneous Register 2
pub mod MISC2 {

    /// This field defines the brown out voltage offset for the CORE power domain
    pub mod REG0_BO_OFFSET {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b100: Brownout offset = 0.100V
            pub const REG0_BO_OFFSET_4: u32 = 0b100;

            /// 0b111: Brownout offset = 0.175V
            pub const REG0_BO_OFFSET_7: u32 = 0b111;
        }
    }

    /// Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)
    pub mod REG0_BO_STATUS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: Brownout, supply is below target minus brownout offset.
            pub const REG0_BO_STATUS_1: u32 = 0b1;
        }
    }

    /// Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)
    pub mod REG0_ENABLE_BO {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ARM supply Not related to CCM. See Power Management Unit (PMU)
    pub mod REG0_OK {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode
    pub mod PLL3_disable {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PLL3 is being used by peripherals and is enabled when SoC is not in any low power mode
            pub const PLL3_disable_0: u32 = 0b0;

            /// 0b1: PLL3 can be disabled when the SoC is not in any low power mode
            pub const PLL3_disable_1: u32 = 0b1;
        }
    }

    /// This field defines the brown out voltage offset for the xPU power domain
    pub mod REG1_BO_OFFSET {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b100: Brownout offset = 0.100V
            pub const REG1_BO_OFFSET_4: u32 = 0b100;

            /// 0b111: Brownout offset = 0.175V
            pub const REG1_BO_OFFSET_7: u32 = 0b111;
        }
    }

    /// Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)
    pub mod REG1_BO_STATUS {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: Brownout, supply is below target minus brownout offset.
            pub const REG1_BO_STATUS_1: u32 = 0b1;
        }
    }

    /// Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)
    pub mod REG1_ENABLE_BO {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)
    pub mod REG1_OK {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSB of Post-divider for Audio PLL
    pub mod AUDIO_DIV_LSB {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: divide by 1 (Default)
            pub const AUDIO_DIV_LSB_0: u32 = 0b0;

            /// 0b1: divide by 2
            pub const AUDIO_DIV_LSB_1: u32 = 0b1;
        }
    }

    /// This field defines the brown out voltage offset for the xPU power domain
    pub mod REG2_BO_OFFSET {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b100: Brownout offset = 0.100V
            pub const REG2_BO_OFFSET_4: u32 = 0b100;

            /// 0b111: Brownout offset = 0.175V
            pub const REG2_BO_OFFSET_7: u32 = 0b111;
        }
    }

    /// Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)
    pub mod REG2_BO_STATUS {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)
    pub mod REG2_ENABLE_BO {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Signals that the voltage is above the brownout level for the SOC supply
    pub mod REG2_OK {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MSB of Post-divider for Audio PLL
    pub mod AUDIO_DIV_MSB {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: divide by 1 (Default)
            pub const AUDIO_DIV_MSB_0: u32 = 0b0;

            /// 0b1: divide by 2
            pub const AUDIO_DIV_MSB_1: u32 = 0b1;
        }
    }

    /// Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)
    pub mod REG0_STEP_TIME {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 64
            pub const _64_CLOCKS: u32 = 0b00;

            /// 0b01: 128
            pub const _128_CLOCKS: u32 = 0b01;

            /// 0b10: 256
            pub const _256_CLOCKS: u32 = 0b10;

            /// 0b11: 512
            pub const _512_CLOCKS: u32 = 0b11;
        }
    }

    /// Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)
    pub mod REG1_STEP_TIME {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::REG0_STEP_TIME::RW;
    }

    /// Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)
    pub mod REG2_STEP_TIME {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::REG0_STEP_TIME::RW;
    }

    /// Post-divider for video
    pub mod VIDEO_DIV {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: divide by 1 (Default)
            pub const VIDEO_DIV_0: u32 = 0b00;

            /// 0b01: divide by 2
            pub const VIDEO_DIV_1: u32 = 0b01;

            /// 0b10: divide by 1
            pub const VIDEO_DIV_2: u32 = 0b10;

            /// 0b11: divide by 4
            pub const VIDEO_DIV_3: u32 = 0b11;
        }
    }
}

/// Miscellaneous Register 2
pub mod MISC2_SET {
    pub use super::MISC2::PLL3_disable;
    pub use super::MISC2::AUDIO_DIV_LSB;
    pub use super::MISC2::AUDIO_DIV_MSB;
    pub use super::MISC2::REG0_BO_OFFSET;
    pub use super::MISC2::REG0_BO_STATUS;
    pub use super::MISC2::REG0_ENABLE_BO;
    pub use super::MISC2::REG0_OK;
    pub use super::MISC2::REG0_STEP_TIME;
    pub use super::MISC2::REG1_BO_OFFSET;
    pub use super::MISC2::REG1_BO_STATUS;
    pub use super::MISC2::REG1_ENABLE_BO;
    pub use super::MISC2::REG1_OK;
    pub use super::MISC2::REG1_STEP_TIME;
    pub use super::MISC2::REG2_BO_OFFSET;
    pub use super::MISC2::REG2_BO_STATUS;
    pub use super::MISC2::REG2_ENABLE_BO;
    pub use super::MISC2::REG2_OK;
    pub use super::MISC2::REG2_STEP_TIME;
    pub use super::MISC2::VIDEO_DIV;
}

/// Miscellaneous Register 2
pub mod MISC2_CLR {
    pub use super::MISC2::PLL3_disable;
    pub use super::MISC2::AUDIO_DIV_LSB;
    pub use super::MISC2::AUDIO_DIV_MSB;
    pub use super::MISC2::REG0_BO_OFFSET;
    pub use super::MISC2::REG0_BO_STATUS;
    pub use super::MISC2::REG0_ENABLE_BO;
    pub use super::MISC2::REG0_OK;
    pub use super::MISC2::REG0_STEP_TIME;
    pub use super::MISC2::REG1_BO_OFFSET;
    pub use super::MISC2::REG1_BO_STATUS;
    pub use super::MISC2::REG1_ENABLE_BO;
    pub use super::MISC2::REG1_OK;
    pub use super::MISC2::REG1_STEP_TIME;
    pub use super::MISC2::REG2_BO_OFFSET;
    pub use super::MISC2::REG2_BO_STATUS;
    pub use super::MISC2::REG2_ENABLE_BO;
    pub use super::MISC2::REG2_OK;
    pub use super::MISC2::REG2_STEP_TIME;
    pub use super::MISC2::VIDEO_DIV;
}

/// Miscellaneous Register 2
pub mod MISC2_TOG {
    pub use super::MISC2::PLL3_disable;
    pub use super::MISC2::AUDIO_DIV_LSB;
    pub use super::MISC2::AUDIO_DIV_MSB;
    pub use super::MISC2::REG0_BO_OFFSET;
    pub use super::MISC2::REG0_BO_STATUS;
    pub use super::MISC2::REG0_ENABLE_BO;
    pub use super::MISC2::REG0_OK;
    pub use super::MISC2::REG0_STEP_TIME;
    pub use super::MISC2::REG1_BO_OFFSET;
    pub use super::MISC2::REG1_BO_STATUS;
    pub use super::MISC2::REG1_ENABLE_BO;
    pub use super::MISC2::REG1_OK;
    pub use super::MISC2::REG1_STEP_TIME;
    pub use super::MISC2::REG2_BO_OFFSET;
    pub use super::MISC2::REG2_BO_STATUS;
    pub use super::MISC2::REG2_ENABLE_BO;
    pub use super::MISC2::REG2_OK;
    pub use super::MISC2::REG2_STEP_TIME;
    pub use super::MISC2::VIDEO_DIV;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 4],

    /// Analog USB1 480MHz PLL Control Register
    pub PLL_USB1: RWRegister<u32>,

    /// Analog USB1 480MHz PLL Control Register
    pub PLL_USB1_SET: RWRegister<u32>,

    /// Analog USB1 480MHz PLL Control Register
    pub PLL_USB1_CLR: RWRegister<u32>,

    /// Analog USB1 480MHz PLL Control Register
    pub PLL_USB1_TOG: RWRegister<u32>,

    _reserved2: [u32; 4],

    /// Analog System PLL Control Register
    pub PLL_SYS: RWRegister<u32>,

    /// Analog System PLL Control Register
    pub PLL_SYS_SET: RWRegister<u32>,

    /// Analog System PLL Control Register
    pub PLL_SYS_CLR: RWRegister<u32>,

    /// Analog System PLL Control Register
    pub PLL_SYS_TOG: RWRegister<u32>,

    /// 528MHz System PLL Spread Spectrum Register
    pub PLL_SYS_SS: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// Numerator of 528MHz System PLL Fractional Loop Divider Register
    pub PLL_SYS_NUM: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// Denominator of 528MHz System PLL Fractional Loop Divider Register
    pub PLL_SYS_DENOM: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// Analog Audio PLL control Register
    pub PLL_AUDIO: RWRegister<u32>,

    /// Analog Audio PLL control Register
    pub PLL_AUDIO_SET: RWRegister<u32>,

    /// Analog Audio PLL control Register
    pub PLL_AUDIO_CLR: RWRegister<u32>,

    /// Analog Audio PLL control Register
    pub PLL_AUDIO_TOG: RWRegister<u32>,

    /// Numerator of Audio PLL Fractional Loop Divider Register
    pub PLL_AUDIO_NUM: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// Denominator of Audio PLL Fractional Loop Divider Register
    pub PLL_AUDIO_DENOM: RWRegister<u32>,

    _reserved7: [u32; 19],

    /// Analog ENET PLL Control Register
    pub PLL_ENET: RWRegister<u32>,

    /// Analog ENET PLL Control Register
    pub PLL_ENET_SET: RWRegister<u32>,

    /// Analog ENET PLL Control Register
    pub PLL_ENET_CLR: RWRegister<u32>,

    /// Analog ENET PLL Control Register
    pub PLL_ENET_TOG: RWRegister<u32>,

    /// 480MHz Clock (PLL3) Phase Fractional Divider Control Register
    pub PFD_480: RWRegister<u32>,

    /// 480MHz Clock (PLL3) Phase Fractional Divider Control Register
    pub PFD_480_SET: RWRegister<u32>,

    /// 480MHz Clock (PLL3) Phase Fractional Divider Control Register
    pub PFD_480_CLR: RWRegister<u32>,

    /// 480MHz Clock (PLL3) Phase Fractional Divider Control Register
    pub PFD_480_TOG: RWRegister<u32>,

    /// 528MHz Clock (PLL2) Phase Fractional Divider Control Register
    pub PFD_528: RWRegister<u32>,

    /// 528MHz Clock (PLL2) Phase Fractional Divider Control Register
    pub PFD_528_SET: RWRegister<u32>,

    /// 528MHz Clock (PLL2) Phase Fractional Divider Control Register
    pub PFD_528_CLR: RWRegister<u32>,

    /// 528MHz Clock (PLL2) Phase Fractional Divider Control Register
    pub PFD_528_TOG: RWRegister<u32>,

    _reserved8: [u32; 16],

    /// Miscellaneous Register 0
    pub MISC0: RWRegister<u32>,

    /// Miscellaneous Register 0
    pub MISC0_SET: RWRegister<u32>,

    /// Miscellaneous Register 0
    pub MISC0_CLR: RWRegister<u32>,

    /// Miscellaneous Register 0
    pub MISC0_TOG: RWRegister<u32>,

    /// Miscellaneous Register 1
    pub MISC1: RWRegister<u32>,

    /// Miscellaneous Register 1
    pub MISC1_SET: RWRegister<u32>,

    /// Miscellaneous Register 1
    pub MISC1_CLR: RWRegister<u32>,

    /// Miscellaneous Register 1
    pub MISC1_TOG: RWRegister<u32>,

    /// Miscellaneous Register 2
    pub MISC2: RWRegister<u32>,

    /// Miscellaneous Register 2
    pub MISC2_SET: RWRegister<u32>,

    /// Miscellaneous Register 2
    pub MISC2_CLR: RWRegister<u32>,

    /// Miscellaneous Register 2
    pub MISC2_TOG: RWRegister<u32>,
}
pub struct ResetValues {
    pub PLL_USB1: u32,
    pub PLL_USB1_SET: u32,
    pub PLL_USB1_CLR: u32,
    pub PLL_USB1_TOG: u32,
    pub PLL_SYS: u32,
    pub PLL_SYS_SET: u32,
    pub PLL_SYS_CLR: u32,
    pub PLL_SYS_TOG: u32,
    pub PLL_SYS_SS: u32,
    pub PLL_SYS_NUM: u32,
    pub PLL_SYS_DENOM: u32,
    pub PLL_AUDIO: u32,
    pub PLL_AUDIO_SET: u32,
    pub PLL_AUDIO_CLR: u32,
    pub PLL_AUDIO_TOG: u32,
    pub PLL_AUDIO_NUM: u32,
    pub PLL_AUDIO_DENOM: u32,
    pub PLL_ENET: u32,
    pub PLL_ENET_SET: u32,
    pub PLL_ENET_CLR: u32,
    pub PLL_ENET_TOG: u32,
    pub PFD_480: u32,
    pub PFD_480_SET: u32,
    pub PFD_480_CLR: u32,
    pub PFD_480_TOG: u32,
    pub PFD_528: u32,
    pub PFD_528_SET: u32,
    pub PFD_528_CLR: u32,
    pub PFD_528_TOG: u32,
    pub MISC0: u32,
    pub MISC0_SET: u32,
    pub MISC0_CLR: u32,
    pub MISC0_TOG: u32,
    pub MISC1: u32,
    pub MISC1_SET: u32,
    pub MISC1_CLR: u32,
    pub MISC1_TOG: u32,
    pub MISC2: u32,
    pub MISC2_SET: u32,
    pub MISC2_CLR: u32,
    pub MISC2_TOG: u32,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}

unsafe impl Send for Instance {}

/// Access functions for the CCM_ANALOG peripheral instance
pub mod CCM_ANALOG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x400d8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CCM_ANALOG
    pub const reset: ResetValues = ResetValues {
        PLL_USB1: 0x00012000,
        PLL_USB1_SET: 0x00012000,
        PLL_USB1_CLR: 0x00012000,
        PLL_USB1_TOG: 0x00012000,
        PLL_SYS: 0x00013001,
        PLL_SYS_SET: 0x00013001,
        PLL_SYS_CLR: 0x00013001,
        PLL_SYS_TOG: 0x00013001,
        PLL_SYS_SS: 0x00000000,
        PLL_SYS_NUM: 0x00000000,
        PLL_SYS_DENOM: 0x00000012,
        PLL_AUDIO: 0x00011006,
        PLL_AUDIO_SET: 0x00011006,
        PLL_AUDIO_CLR: 0x00011006,
        PLL_AUDIO_TOG: 0x00011006,
        PLL_AUDIO_NUM: 0x05F5E100,
        PLL_AUDIO_DENOM: 0x2964619C,
        PLL_ENET: 0x00011001,
        PLL_ENET_SET: 0x00011001,
        PLL_ENET_CLR: 0x00011001,
        PLL_ENET_TOG: 0x00011001,
        PFD_480: 0x1311100C,
        PFD_480_SET: 0x1311100C,
        PFD_480_CLR: 0x1311100C,
        PFD_480_TOG: 0x1311100C,
        PFD_528: 0x1018101B,
        PFD_528_SET: 0x1018101B,
        PFD_528_CLR: 0x1018101B,
        PFD_528_TOG: 0x1018101B,
        MISC0: 0x04000000,
        MISC0_SET: 0x04000000,
        MISC0_CLR: 0x04000000,
        MISC0_TOG: 0x04000000,
        MISC1: 0x00000000,
        MISC1_SET: 0x00000000,
        MISC1_CLR: 0x00000000,
        MISC1_TOG: 0x00000000,
        MISC2: 0x00272727,
        MISC2_SET: 0x00272727,
        MISC2_CLR: 0x00272727,
        MISC2_TOG: 0x00272727,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CCM_ANALOG_TAKEN: bool = false;

    /// Safe access to CCM_ANALOG
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CCM_ANALOG_TAKEN {
                None
            } else {
                CCM_ANALOG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CCM_ANALOG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CCM_ANALOG_TAKEN && inst.addr == INSTANCE.addr {
                CCM_ANALOG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CCM_ANALOG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CCM_ANALOG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CCM_ANALOG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CCM_ANALOG: *const RegisterBlock = 0x400d8000 as *const _;
