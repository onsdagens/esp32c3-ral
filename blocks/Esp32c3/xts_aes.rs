#[doc = "XTS-AES-128 Flash Encryption"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "The memory that stores plaintext"]
    pub PLAIN_MEM: [crate::RWRegister<u8>; 16usize],
    _reserved0: [u8; 0x30],
    #[doc = "XTS-AES line-size register"]
    pub LINESIZE: crate::RWRegister<u32>,
    #[doc = "XTS-AES destination register"]
    pub DESTINATION: crate::RWRegister<u32>,
    #[doc = "XTS-AES physical address register"]
    pub PHYSICAL_ADDRESS: crate::RWRegister<u32>,
    #[doc = "XTS-AES trigger register"]
    pub TRIGGER: crate::RWRegister<u32>,
    #[doc = "XTS-AES release register"]
    pub RELEASE: crate::RWRegister<u32>,
    #[doc = "XTS-AES destroy register"]
    pub DESTROY: crate::RWRegister<u32>,
    #[doc = "XTS-AES status register"]
    pub STATE: crate::RWRegister<u32>,
    #[doc = "XTS-AES version control register"]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "XTS-AES line-size register"]
pub mod LINESIZE {
    #[doc = "This bit stores the line size parameter. 0: 16Byte, 1: 32Byte."]
    pub mod LINESIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTS-AES destination register"]
pub mod DESTINATION {
    #[doc = "This bit stores the destination. 0: flash(default). 1: reserved."]
    pub mod DESTINATION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTS-AES physical address register"]
pub mod PHYSICAL_ADDRESS {
    #[doc = "Those bits stores the physical address. If linesize is 16-byte, the physical address should be aligned of 16 bytes. If linesize is 32-byte, the physical address should be aligned of 32 bytes."]
    pub mod PHYSICAL_ADDRESS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTS-AES trigger register"]
pub mod TRIGGER {
    #[doc = "Set this bit to start manual encryption calculation"]
    pub mod TRIGGER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTS-AES release register"]
pub mod RELEASE {
    #[doc = "Set this bit to release the manual encrypted result, after that the result will be visible to spi"]
    pub mod RELEASE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTS-AES destroy register"]
pub mod DESTROY {
    #[doc = "Set this bit to destroy XTS-AES result."]
    pub mod DESTROY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTS-AES status register"]
pub mod STATE {
    #[doc = "Those bits shows XTS-AES status. 0=IDLE, 1=WORK, 2=RELEASE, 3=USE. IDLE means that XTS-AES is idle. WORK means that XTS-AES is busy with calculation. RELEASE means the encrypted result is generated but not visible to mspi. USE means that the encrypted result is visible to mspi."]
    pub mod STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTS-AES version control register"]
pub mod DATE {
    #[doc = "Those bits stores the version information of XTS-AES."]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
