#[doc = "RSA (Rivest Shamir Adleman) Accelerator"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "The memory that stores M"]
    pub M_MEM: [crate::RWRegister<u8>; 384usize],
    _reserved0: [u8; 0x80],
    #[doc = "The memory that stores Z"]
    pub Z_MEM: [crate::RWRegister<u8>; 384usize],
    _reserved1: [u8; 0x80],
    #[doc = "The memory that stores Y"]
    pub Y_MEM: [crate::RWRegister<u8>; 384usize],
    _reserved2: [u8; 0x80],
    #[doc = "The memory that stores X"]
    pub X_MEM: [crate::RWRegister<u8>; 384usize],
    _reserved3: [u8; 0x80],
    #[doc = "RSA M_prime register"]
    pub M_PRIME: crate::RWRegister<u32>,
    #[doc = "RSA mode register"]
    pub MODE: crate::RWRegister<u32>,
    #[doc = "RSA query clean register"]
    pub QUERY_CLEAN: crate::RWRegister<u32>,
    #[doc = "RSA modular exponentiation trigger register."]
    pub SET_START_MODEXP: crate::RWRegister<u32>,
    #[doc = "RSA modular multiplication trigger register."]
    pub SET_START_MODMULT: crate::RWRegister<u32>,
    #[doc = "RSA normal multiplication trigger register."]
    pub SET_START_MULT: crate::RWRegister<u32>,
    #[doc = "RSA query idle register"]
    pub QUERY_IDLE: crate::RWRegister<u32>,
    #[doc = "RSA interrupt clear register"]
    pub INT_CLR: crate::RWRegister<u32>,
    #[doc = "RSA constant time option register"]
    pub CONSTANT_TIME: crate::RWRegister<u32>,
    #[doc = "RSA search option"]
    pub SEARCH_ENABLE: crate::RWRegister<u32>,
    #[doc = "RSA search position configure register"]
    pub SEARCH_POS: crate::RWRegister<u32>,
    #[doc = "RSA interrupt enable register"]
    pub INT_ENA: crate::RWRegister<u32>,
    #[doc = "RSA version control register"]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "RSA M_prime register"]
pub mod M_PRIME {
    #[doc = "Those bits stores m'"]
    pub mod M_PRIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RSA mode register"]
pub mod MODE {
    #[doc = "rsa mode (rsa length)."]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RSA query clean register"]
pub mod QUERY_CLEAN {
    #[doc = "query clean"]
    pub mod QUERY_CLEAN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RSA modular exponentiation trigger register."]
pub mod SET_START_MODEXP {
    #[doc = "start modular exponentiation"]
    pub mod SET_START_MODEXP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RSA modular multiplication trigger register."]
pub mod SET_START_MODMULT {
    #[doc = "start modular multiplication"]
    pub mod SET_START_MODMULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RSA normal multiplication trigger register."]
pub mod SET_START_MULT {
    #[doc = "start multiplicaiton"]
    pub mod SET_START_MULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RSA query idle register"]
pub mod QUERY_IDLE {
    #[doc = "query rsa idle. 1'b0: busy, 1'b1: idle"]
    pub mod QUERY_IDLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RSA interrupt clear register"]
pub mod INT_CLR {
    #[doc = "set this bit to clear RSA interrupt."]
    pub mod CLEAR_INTERRUPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RSA constant time option register"]
pub mod CONSTANT_TIME {
    #[doc = "Configure this bit to 0 for acceleration. 0: with acceleration, 1: without acceleration(defalut)."]
    pub mod CONSTANT_TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RSA search option"]
pub mod SEARCH_ENABLE {
    #[doc = "Configure this bit to 1 for acceleration. 1: with acceleration, 0: without acceleration(default). This option should be used together with RSA_SEARCH_POS."]
    pub mod SEARCH_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RSA search position configure register"]
pub mod SEARCH_POS {
    #[doc = "Configure this field to set search position. This field should be used together with RSA_SEARCH_ENABLE. The field is only meaningful when RSA_SEARCH_ENABLE is high."]
    pub mod SEARCH_POS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RSA interrupt enable register"]
pub mod INT_ENA {
    #[doc = "Set this bit to enable interrupt that occurs when rsa calculation is done. 1'b0: disable, 1'b1: enable(default)."]
    pub mod INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RSA version control register"]
pub mod DATE {
    #[doc = "rsa version information"]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
