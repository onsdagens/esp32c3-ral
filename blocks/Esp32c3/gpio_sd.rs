#[doc = "Sigma-Delta Modulation"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Duty Cycle Configure Register of SDM%s"]
    pub SIGMADELTA: [crate::RWRegister<u32>; 4usize],
    _reserved0: [u8; 0x10],
    #[doc = "Clock Gating Configure Register"]
    pub SIGMADELTA_CG: crate::RWRegister<u32>,
    #[doc = "MISC Register"]
    pub SIGMADELTA_MISC: crate::RWRegister<u32>,
    #[doc = "Version Control Register"]
    pub SIGMADELTA_VERSION: crate::RWRegister<u32>,
}
#[doc = "Duty Cycle Configure Register of SDM%s"]
pub mod SIGMADELTA {
    #[doc = "This field is used to configure the duty cycle of sigma delta modulation output."]
    pub mod SD0_IN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field is used to set a divider value to divide APB clock."]
    pub mod SD0_PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock Gating Configure Register"]
pub mod SIGMADELTA_CG {
    #[doc = "Clock enable bit of configuration registers for sigma delta modulation."]
    pub mod CLK_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MISC Register"]
pub mod SIGMADELTA_MISC {
    #[doc = "Clock enable bit of sigma delta modulation."]
    pub mod FUNCTION_CLK_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod SPI_SWAP {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Version Control Register"]
pub mod SIGMADELTA_VERSION {
    #[doc = "Version control register."]
    pub mod GPIO_SD_DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
