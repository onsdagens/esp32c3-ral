#[doc = "Input/Output Multiplexer"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Clock Output Configuration Register"]
    pub PIN_CTRL: crate::RWRegister<u32>,
    #[doc = "IO MUX Configure Register for pad XTAL_32K_P"]
    pub GPIO: [crate::RWRegister<u32>; 22usize],
    _reserved0: [u8; 0xa0],
    #[doc = "IO MUX Version Control Register"]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "Clock Output Configuration Register"]
pub mod PIN_CTRL {
    #[doc = "If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals."]
    pub mod CLK_OUT1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
    pub mod CLK_OUT2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
    pub mod CLK_OUT3 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IO MUX Configure Register for pad XTAL_32K_P"]
pub mod GPIO {
    #[doc = "Output enable of the pad in sleep mode. 1: output enabled; 0: output disabled."]
    pub mod MCU_OE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sleep mode selection of this pad. Set to 1 to put the pad in pad mode."]
    pub mod SLP_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pull-down enable of the pad in sleep mode. 1: internal pull-down enabled; 0: internal pull-down disabled."]
    pub mod MCU_WPD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pull-up enable of the pad during sleep mode. 1: internal pull-up enabled; 0: internal pull-up disabled."]
    pub mod MCU_WPU {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input enable of the pad during sleep mode. 1: input enabled; 0: input disabled."]
    pub mod MCU_IE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pull-down enable of the pad. 1: internal pull-down enabled; 0: internal pull-down disabled."]
    pub mod FUN_WPD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pull-up enable of the pad. 1: internal pull-up enabled; 0: internal pull-up disabled."]
    pub mod FUN_WPU {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input enable of the pad. 1: input enabled; 0: input disabled."]
    pub mod FUN_IE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select the drive strength of the pad. 0: ~5 mA; 1: ~10mA; 2: ~20mA; 3: ~40mA."]
    pub mod FUN_DRV {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select IO MUX function for this signal. 0: Select Function 1; 1: Select Function 2; etc."]
    pub mod MCU_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable filter for pin input signals. 1: Filter enabled; 2: Filter disabled."]
    pub mod FILTER_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IO MUX Version Control Register"]
pub mod DATE {
    #[doc = "Version control register"]
    pub mod REG_DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
