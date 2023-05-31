#[doc = "HMAC (Hash-based Message Authentication Code) Accelerator"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    #[doc = "Process control register 0."]
    pub SET_START: crate::RWRegister<u32>,
    #[doc = "Configure purpose."]
    pub SET_PARA_PURPOSE: crate::RWRegister<u32>,
    #[doc = "Configure key."]
    pub SET_PARA_KEY: crate::RWRegister<u32>,
    #[doc = "Finish initial configuration."]
    pub SET_PARA_FINISH: crate::RWRegister<u32>,
    #[doc = "Process control register 1."]
    pub SET_MESSAGE_ONE: crate::RWRegister<u32>,
    #[doc = "Process control register 2."]
    pub SET_MESSAGE_ING: crate::RWRegister<u32>,
    #[doc = "Process control register 3."]
    pub SET_MESSAGE_END: crate::RWRegister<u32>,
    #[doc = "Process control register 4."]
    pub SET_RESULT_FINISH: crate::RWRegister<u32>,
    #[doc = "Invalidate register 0."]
    pub SET_INVALIDATE_JTAG: crate::RWRegister<u32>,
    #[doc = "Invalidate register 1."]
    pub SET_INVALIDATE_DS: crate::RWRegister<u32>,
    #[doc = "Error register."]
    pub QUERY_ERROR: crate::RWRegister<u32>,
    #[doc = "Busy register."]
    pub QUERY_BUSY: crate::RWRegister<u32>,
    _reserved1: [u8; 0x10],
    #[doc = "Message block memory."]
    pub WR_MESSAGE_MEM: [crate::RWRegister<u8>; 64usize],
    #[doc = "Result from upstream."]
    pub RD_RESULT_MEM: [crate::RWRegister<u8>; 32usize],
    _reserved2: [u8; 0x10],
    #[doc = "Process control register 5."]
    pub SET_MESSAGE_PAD: crate::RWRegister<u32>,
    #[doc = "Process control register 6."]
    pub ONE_BLOCK: crate::RWRegister<u32>,
    #[doc = "Jtag register 0."]
    pub SOFT_JTAG_CTRL: crate::RWRegister<u32>,
    #[doc = "Jtag register 1."]
    pub WR_JTAG: crate::RWRegister<u32>,
}
#[doc = "Process control register 0."]
pub mod SET_START {
    #[doc = "Start hmac operation."]
    pub mod SET_START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configure purpose."]
pub mod SET_PARA_PURPOSE {
    #[doc = "Set hmac parameter purpose."]
    pub mod PURPOSE_SET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configure key."]
pub mod SET_PARA_KEY {
    #[doc = "Set hmac parameter key."]
    pub mod KEY_SET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Finish initial configuration."]
pub mod SET_PARA_FINISH {
    #[doc = "Finish hmac configuration."]
    pub mod SET_PARA_END {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Process control register 1."]
pub mod SET_MESSAGE_ONE {
    #[doc = "Call SHA to calculate one message block."]
    pub mod SET_TEXT_ONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Process control register 2."]
pub mod SET_MESSAGE_ING {
    #[doc = "Continue typical hmac."]
    pub mod SET_TEXT_ING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Process control register 3."]
pub mod SET_MESSAGE_END {
    #[doc = "Start hardware padding."]
    pub mod SET_TEXT_END {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Process control register 4."]
pub mod SET_RESULT_FINISH {
    #[doc = "After read result from upstream, then let hmac back to idle."]
    pub mod SET_RESULT_END {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Invalidate register 0."]
pub mod SET_INVALIDATE_JTAG {
    #[doc = "Clear result from hmac downstream JTAG."]
    pub mod SET_INVALIDATE_JTAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Invalidate register 1."]
pub mod SET_INVALIDATE_DS {
    #[doc = "Clear result from hmac downstream DS."]
    pub mod SET_INVALIDATE_DS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error register."]
pub mod QUERY_ERROR {
    #[doc = "Hmac configuration state. 0: key are agree with purpose. 1: error"]
    pub mod QUREY_CHECK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Busy register."]
pub mod QUERY_BUSY {
    #[doc = "Hmac state. 1'b0: idle. 1'b1: busy"]
    pub mod BUSY_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Process control register 5."]
pub mod SET_MESSAGE_PAD {
    #[doc = "Start software padding."]
    pub mod SET_TEXT_PAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Process control register 6."]
pub mod ONE_BLOCK {
    #[doc = "Don't have to do padding."]
    pub mod SET_ONE_BLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Jtag register 0."]
pub mod SOFT_JTAG_CTRL {
    #[doc = "Turn on JTAG verification."]
    pub mod SOFT_JTAG_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Jtag register 1."]
pub mod WR_JTAG {
    #[doc = "32-bit of key to be compared."]
    pub mod WR_JTAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
