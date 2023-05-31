#[doc = "SHA (Secure Hash Algorithm) Accelerator"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Initial configuration register."]
    pub MODE: crate::RWRegister<u32>,
    #[doc = "SHA 512/t configuration register 0."]
    pub T_STRING: crate::RWRegister<u32>,
    #[doc = "SHA 512/t configuration register 1."]
    pub T_LENGTH: crate::RWRegister<u32>,
    #[doc = "DMA configuration register 0."]
    pub DMA_BLOCK_NUM: crate::RWRegister<u32>,
    #[doc = "Typical SHA configuration register 0."]
    pub START: crate::RWRegister<u32>,
    #[doc = "Typical SHA configuration register 1."]
    pub CONTINUE: crate::RWRegister<u32>,
    #[doc = "Busy register."]
    pub BUSY: crate::RWRegister<u32>,
    #[doc = "DMA configuration register 1."]
    pub DMA_START: crate::RWRegister<u32>,
    #[doc = "DMA configuration register 2."]
    pub DMA_CONTINUE: crate::RWRegister<u32>,
    #[doc = "Interrupt clear register."]
    pub CLEAR_IRQ: crate::RWRegister<u32>,
    #[doc = "Interrupt enable register."]
    pub IRQ_ENA: crate::RWRegister<u32>,
    #[doc = "Date register."]
    pub DATE: crate::RWRegister<u32>,
    _reserved0: [u8; 0x10],
    #[doc = "Sha H memory which contains intermediate hash or finial hash."]
    pub H_MEM: [crate::RWRegister<u8>; 64usize],
    #[doc = "Sha M memory which contains message."]
    pub M_MEM: [crate::RWRegister<u8>; 64usize],
}
#[doc = "Initial configuration register."]
pub mod MODE {
    #[doc = "Sha mode."]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SHA 512/t configuration register 0."]
pub mod T_STRING {
    #[doc = "Sha t_string (used if and only if mode == SHA_512/t)."]
    pub mod T_STRING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SHA 512/t configuration register 1."]
pub mod T_LENGTH {
    #[doc = "Sha t_length (used if and only if mode == SHA_512/t)."]
    pub mod T_LENGTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA configuration register 0."]
pub mod DMA_BLOCK_NUM {
    #[doc = "Dma-sha block number."]
    pub mod DMA_BLOCK_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Typical SHA configuration register 0."]
pub mod START {
    #[doc = "Reserved."]
    pub mod START {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Typical SHA configuration register 1."]
pub mod CONTINUE {
    #[doc = "Reserved."]
    pub mod CONTINUE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Busy register."]
pub mod BUSY {
    #[doc = "Sha busy state. 1'b0: idle. 1'b1: busy."]
    pub mod STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA configuration register 1."]
pub mod DMA_START {
    #[doc = "Start dma-sha."]
    pub mod DMA_START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA configuration register 2."]
pub mod DMA_CONTINUE {
    #[doc = "Continue dma-sha."]
    pub mod DMA_CONTINUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt clear register."]
pub mod CLEAR_IRQ {
    #[doc = "Clear sha interrupt."]
    pub mod CLEAR_INTERRUPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt enable register."]
pub mod IRQ_ENA {
    #[doc = "Sha interrupt enable register. 1'b0: disable(default). 1'b1: enable."]
    pub mod INTERRUPT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Date register."]
pub mod DATE {
    #[doc = "Sha date information/ sha version information."]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
