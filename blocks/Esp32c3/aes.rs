#[doc = "AES (Advanced Encryption Standard) Accelerator"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Key material key_0 configure register"]
    pub KEY_0: crate::RWRegister<u32>,
    #[doc = "Key material key_1 configure register"]
    pub KEY_1: crate::RWRegister<u32>,
    #[doc = "Key material key_2 configure register"]
    pub KEY_2: crate::RWRegister<u32>,
    #[doc = "Key material key_3 configure register"]
    pub KEY_3: crate::RWRegister<u32>,
    #[doc = "Key material key_4 configure register"]
    pub KEY_4: crate::RWRegister<u32>,
    #[doc = "Key material key_5 configure register"]
    pub KEY_5: crate::RWRegister<u32>,
    #[doc = "Key material key_6 configure register"]
    pub KEY_6: crate::RWRegister<u32>,
    #[doc = "Key material key_7 configure register"]
    pub KEY_7: crate::RWRegister<u32>,
    #[doc = "source text material text_in_0 configure register"]
    pub TEXT_IN_0: crate::RWRegister<u32>,
    #[doc = "source text material text_in_1 configure register"]
    pub TEXT_IN_1: crate::RWRegister<u32>,
    #[doc = "source text material text_in_2 configure register"]
    pub TEXT_IN_2: crate::RWRegister<u32>,
    #[doc = "source text material text_in_3 configure register"]
    pub TEXT_IN_3: crate::RWRegister<u32>,
    #[doc = "result text material text_out_0 configure register"]
    pub TEXT_OUT_0: crate::RWRegister<u32>,
    #[doc = "result text material text_out_1 configure register"]
    pub TEXT_OUT_1: crate::RWRegister<u32>,
    #[doc = "result text material text_out_2 configure register"]
    pub TEXT_OUT_2: crate::RWRegister<u32>,
    #[doc = "result text material text_out_3 configure register"]
    pub TEXT_OUT_3: crate::RWRegister<u32>,
    #[doc = "AES Mode register"]
    pub MODE: crate::RWRegister<u32>,
    #[doc = "AES Endian configure register"]
    pub ENDIAN: crate::RWRegister<u32>,
    #[doc = "AES trigger register"]
    pub TRIGGER: crate::RWRegister<u32>,
    #[doc = "AES state register"]
    pub STATE: crate::RWRegister<u32>,
    #[doc = "The memory that stores initialization vector"]
    pub IV_MEM: [crate::RWRegister<u8>; 16usize],
    #[doc = "The memory that stores GCM hash subkey"]
    pub H_MEM: [crate::RWRegister<u8>; 16usize],
    #[doc = "The memory that stores J0"]
    pub J0_MEM: [crate::RWRegister<u8>; 16usize],
    #[doc = "The memory that stores T0"]
    pub T0_MEM: [crate::RWRegister<u8>; 16usize],
    #[doc = "DMA-AES working mode register"]
    pub DMA_ENABLE: crate::RWRegister<u32>,
    #[doc = "AES cipher block mode register"]
    pub BLOCK_MODE: crate::RWRegister<u32>,
    #[doc = "AES block number register"]
    pub BLOCK_NUM: crate::RWRegister<u32>,
    #[doc = "Standard incrementing function configure register"]
    pub INC_SEL: crate::RWRegister<u32>,
    #[doc = "Additional Authential Data block number register"]
    pub AAD_BLOCK_NUM: crate::RWRegister<u32>,
    #[doc = "AES remainder bit number register"]
    pub REMAINDER_BIT_NUM: crate::RWRegister<u32>,
    #[doc = "AES continue register"]
    pub CONTINUE: crate::RWRegister<u32>,
    #[doc = "AES Interrupt clear register"]
    pub INT_CLEAR: crate::RWRegister<u32>,
    #[doc = "AES Interrupt enable register"]
    pub INT_ENA: crate::RWRegister<u32>,
    #[doc = "AES version control register"]
    pub DATE: crate::RWRegister<u32>,
    #[doc = "AES-DMA exit config"]
    pub DMA_EXIT: crate::RWRegister<u32>,
}
#[doc = "Key material key_0 configure register"]
pub mod KEY_0 {
    #[doc = "This bits stores key_0 that is a part of key material."]
    pub mod KEY_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key material key_1 configure register"]
pub mod KEY_1 {
    #[doc = "This bits stores key_1 that is a part of key material."]
    pub mod KEY_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key material key_2 configure register"]
pub mod KEY_2 {
    #[doc = "This bits stores key_2 that is a part of key material."]
    pub mod KEY_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key material key_3 configure register"]
pub mod KEY_3 {
    #[doc = "This bits stores key_3 that is a part of key material."]
    pub mod KEY_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key material key_4 configure register"]
pub mod KEY_4 {
    #[doc = "This bits stores key_4 that is a part of key material."]
    pub mod KEY_4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key material key_5 configure register"]
pub mod KEY_5 {
    #[doc = "This bits stores key_5 that is a part of key material."]
    pub mod KEY_5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key material key_6 configure register"]
pub mod KEY_6 {
    #[doc = "This bits stores key_6 that is a part of key material."]
    pub mod KEY_6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key material key_7 configure register"]
pub mod KEY_7 {
    #[doc = "This bits stores key_7 that is a part of key material."]
    pub mod KEY_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "source text material text_in_0 configure register"]
pub mod TEXT_IN_0 {
    #[doc = "This bits stores text_in_0 that is a part of source text material."]
    pub mod TEXT_IN_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "source text material text_in_1 configure register"]
pub mod TEXT_IN_1 {
    #[doc = "This bits stores text_in_1 that is a part of source text material."]
    pub mod TEXT_IN_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "source text material text_in_2 configure register"]
pub mod TEXT_IN_2 {
    #[doc = "This bits stores text_in_2 that is a part of source text material."]
    pub mod TEXT_IN_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "source text material text_in_3 configure register"]
pub mod TEXT_IN_3 {
    #[doc = "This bits stores text_in_3 that is a part of source text material."]
    pub mod TEXT_IN_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "result text material text_out_0 configure register"]
pub mod TEXT_OUT_0 {
    #[doc = "This bits stores text_out_0 that is a part of result text material."]
    pub mod TEXT_OUT_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "result text material text_out_1 configure register"]
pub mod TEXT_OUT_1 {
    #[doc = "This bits stores text_out_1 that is a part of result text material."]
    pub mod TEXT_OUT_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "result text material text_out_2 configure register"]
pub mod TEXT_OUT_2 {
    #[doc = "This bits stores text_out_2 that is a part of result text material."]
    pub mod TEXT_OUT_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "result text material text_out_3 configure register"]
pub mod TEXT_OUT_3 {
    #[doc = "This bits stores text_out_3 that is a part of result text material."]
    pub mod TEXT_OUT_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AES Mode register"]
pub mod MODE {
    #[doc = "This bits decides which one operation mode will be used. 3'd0: AES-EN-128, 3'd1: AES-EN-192, 3'd2: AES-EN-256, 3'd4: AES-DE-128, 3'd5: AES-DE-192, 3'd6: AES-DE-256."]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AES Endian configure register"]
pub mod ENDIAN {
    #[doc = "endian. \\[1:0\\] key endian, \\[3:2\\] text_in endian or in_stream endian, \\[5:4\\] text_out endian or out_stream endian"]
    pub mod ENDIAN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AES trigger register"]
pub mod TRIGGER {
    #[doc = "Set this bit to start AES calculation."]
    pub mod TRIGGER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AES state register"]
pub mod STATE {
    #[doc = "Those bits shows AES status. For typical AES, 0: idle, 1: busy. For DMA-AES, 0: idle, 1: busy, 2: calculation_done."]
    pub mod STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA-AES working mode register"]
pub mod DMA_ENABLE {
    #[doc = "1'b0: typical AES working mode, 1'b1: DMA-AES working mode."]
    pub mod DMA_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AES cipher block mode register"]
pub mod BLOCK_MODE {
    #[doc = "Those bits decides which block mode will be used. 0x0: ECB, 0x1: CBC, 0x2: OFB, 0x3: CTR, 0x4: CFB-8, 0x5: CFB-128, 0x6: GCM, 0x7: reserved."]
    pub mod BLOCK_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AES block number register"]
pub mod BLOCK_NUM {
    #[doc = "Those bits stores the number of Plaintext/ciphertext block."]
    pub mod BLOCK_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Standard incrementing function configure register"]
pub mod INC_SEL {
    #[doc = "This bit decides the standard incrementing function. 0: INC32. 1: INC128."]
    pub mod INC_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Additional Authential Data block number register"]
pub mod AAD_BLOCK_NUM {
    #[doc = "Those bits stores the number of AAD block."]
    pub mod AAD_BLOCK_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AES remainder bit number register"]
pub mod REMAINDER_BIT_NUM {
    #[doc = "Those bits stores the number of remainder bit."]
    pub mod REMAINDER_BIT_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AES continue register"]
pub mod CONTINUE {
    #[doc = "Set this bit to continue GCM operation."]
    pub mod CONTINUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AES Interrupt clear register"]
pub mod INT_CLEAR {
    #[doc = "Set this bit to clear the AES interrupt."]
    pub mod INT_CLEAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AES Interrupt enable register"]
pub mod INT_ENA {
    #[doc = "Set this bit to enable interrupt that occurs when DMA-AES calculation is done."]
    pub mod INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AES version control register"]
pub mod DATE {
    #[doc = "This bits stores the version information of AES."]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AES-DMA exit config"]
pub mod DMA_EXIT {
    #[doc = "Set this register to leave calculation done stage. Recommend to use it after software finishes reading DMA's output buffer."]
    pub mod DMA_EXIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
