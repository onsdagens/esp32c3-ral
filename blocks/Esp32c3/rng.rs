#[doc = "Hardware Random Number Generator"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0xb0],
    #[doc = "Random number data"]
    pub DATA: crate::RORegister<u32>,
}
