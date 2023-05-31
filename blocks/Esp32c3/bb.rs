#[doc = "BB Peripheral"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x54],
    #[doc = "Baseband control register"]
    pub BBPD_CTRL: crate::RWRegister<u32>,
}
#[doc = "Baseband control register"]
pub mod BBPD_CTRL {
    pub mod DC_EST_FORCE_PD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    pub mod DC_EST_FORCE_PU {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    pub mod FFT_FORCE_PD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    pub mod FFT_FORCE_PU {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
