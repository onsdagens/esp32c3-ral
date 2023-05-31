#[doc = "Digital Signature"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "memory that stores Y"]
    pub Y_MEM: [crate::RWRegister<u8>; 512usize],
    #[doc = "memory that stores M"]
    pub M_MEM: [crate::RWRegister<u8>; 512usize],
    #[doc = "memory that stores Rb"]
    pub RB_MEM: [crate::RWRegister<u8>; 512usize],
    #[doc = "memory that stores BOX"]
    pub BOX_MEM: [crate::RWRegister<u8>; 48usize],
    _reserved0: [u8; 0x01d0],
    #[doc = "memory that stores X"]
    pub X_MEM: [crate::RWRegister<u8>; 512usize],
    #[doc = "memory that stores Z"]
    pub Z_MEM: [crate::RWRegister<u8>; 512usize],
    _reserved1: [u8; 0x0200],
    #[doc = "DS start control register"]
    pub SET_START: crate::RWRegister<u32>,
    #[doc = "DS continue control register"]
    pub SET_CONTINUE: crate::RWRegister<u32>,
    #[doc = "DS finish control register"]
    pub SET_FINISH: crate::RWRegister<u32>,
    #[doc = "DS query busy register"]
    pub QUERY_BUSY: crate::RWRegister<u32>,
    #[doc = "DS query key-wrong counter register"]
    pub QUERY_KEY_WRONG: crate::RWRegister<u32>,
    #[doc = "DS query check result register"]
    pub QUERY_CHECK: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "DS version control register"]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "DS start control register"]
pub mod SET_START {
    #[doc = "set this bit to start DS operation."]
    pub mod SET_START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DS continue control register"]
pub mod SET_CONTINUE {
    #[doc = "set this bit to continue DS operation."]
    pub mod SET_CONTINUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DS finish control register"]
pub mod SET_FINISH {
    #[doc = "Set this bit to finish DS process."]
    pub mod SET_FINISH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DS query busy register"]
pub mod QUERY_BUSY {
    #[doc = "digital signature state. 1'b0: idle, 1'b1: busy"]
    pub mod QUERY_BUSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DS query key-wrong counter register"]
pub mod QUERY_KEY_WRONG {
    #[doc = "digital signature key wrong counter"]
    pub mod QUERY_KEY_WRONG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DS query check result register"]
pub mod QUERY_CHECK {
    #[doc = "MD checkout result. 1'b0: MD check pass, 1'b1: MD check fail"]
    pub mod MD_ERROR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "padding checkout result. 1'b0: a good padding, 1'b1: a bad padding"]
    pub mod PADDING_BAD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DS version control register"]
pub mod DATE {
    #[doc = "ds version information"]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
