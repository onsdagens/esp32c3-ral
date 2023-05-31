#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - WIFI_MAC"]
    WIFI_MAC = 0,
    #[doc = "1 - WIFI_MAC_NMI"]
    WIFI_MAC_NMI = 1,
    #[doc = "2 - WIFI_PWR"]
    WIFI_PWR = 2,
    #[doc = "3 - WIFI_BB"]
    WIFI_BB = 3,
    #[doc = "4 - BT_MAC"]
    BT_MAC = 4,
    #[doc = "5 - BT_BB"]
    BT_BB = 5,
    #[doc = "6 - BT_BB_NMI"]
    BT_BB_NMI = 6,
    #[doc = "7 - RWBT"]
    RWBT = 7,
    #[doc = "8 - RWBLE"]
    RWBLE = 8,
    #[doc = "9 - RWBT_NMI"]
    RWBT_NMI = 9,
    #[doc = "10 - RWBLE_NMI"]
    RWBLE_NMI = 10,
    #[doc = "11 - I2C_MASTER"]
    I2C_MASTER = 11,
    #[doc = "12 - SLC0"]
    SLC0 = 12,
    #[doc = "13 - SLC1"]
    SLC1 = 13,
    #[doc = "14 - APB_CTRL"]
    APB_CTRL = 14,
    #[doc = "15 - UHCI0"]
    UHCI0 = 15,
    #[doc = "16 - GPIO"]
    GPIO = 16,
    #[doc = "17 - GPIO_NMI"]
    GPIO_NMI = 17,
    #[doc = "18 - SPI1"]
    SPI1 = 18,
    #[doc = "19 - SPI2"]
    SPI2 = 19,
    #[doc = "20 - I2S0"]
    I2S0 = 20,
    #[doc = "21 - UART0"]
    UART0 = 21,
    #[doc = "22 - UART1"]
    UART1 = 22,
    #[doc = "23 - LEDC"]
    LEDC = 23,
    #[doc = "24 - EFUSE"]
    EFUSE = 24,
    #[doc = "25 - TWAI0"]
    TWAI0 = 25,
    #[doc = "26 - USB_SERIAL_JTAG"]
    USB_SERIAL_JTAG = 26,
    #[doc = "27 - RTC_CORE"]
    RTC_CORE = 27,
    #[doc = "28 - RMT"]
    RMT = 28,
    #[doc = "29 - I2C_EXT0"]
    I2C_EXT0 = 29,
    #[doc = "30 - TIMER1"]
    TIMER1 = 30,
    #[doc = "31 - TIMER2"]
    TIMER2 = 31,
    #[doc = "32 - TG0_T0_LEVEL"]
    TG0_T0_LEVEL = 32,
    #[doc = "33 - TG0_WDT_LEVEL"]
    TG0_WDT_LEVEL = 33,
    #[doc = "34 - TG1_T0_LEVEL"]
    TG1_T0_LEVEL = 34,
    #[doc = "35 - TG1_WDT_LEVEL"]
    TG1_WDT_LEVEL = 35,
    #[doc = "36 - CACHE_IA"]
    CACHE_IA = 36,
    #[doc = "37 - SYSTIMER_TARGET0"]
    SYSTIMER_TARGET0 = 37,
    #[doc = "38 - SYSTIMER_TARGET1"]
    SYSTIMER_TARGET1 = 38,
    #[doc = "39 - SYSTIMER_TARGET2"]
    SYSTIMER_TARGET2 = 39,
    #[doc = "40 - SPI_MEM_REJECT_CACHE"]
    SPI_MEM_REJECT_CACHE = 40,
    #[doc = "41 - ICACHE_PRELOAD0"]
    ICACHE_PRELOAD0 = 41,
    #[doc = "42 - ICACHE_SYNC0"]
    ICACHE_SYNC0 = 42,
    #[doc = "43 - APB_ADC"]
    APB_ADC = 43,
    #[doc = "44 - DMA_CH0"]
    DMA_CH0 = 44,
    #[doc = "45 - DMA_CH1"]
    DMA_CH1 = 45,
    #[doc = "46 - DMA_CH2"]
    DMA_CH2 = 46,
    #[doc = "47 - RSA"]
    RSA = 47,
    #[doc = "48 - AES"]
    AES = 48,
    #[doc = "49 - SHA"]
    SHA = 49,
    #[doc = "50 - FROM_CPU_INTR0"]
    FROM_CPU_INTR0 = 50,
    #[doc = "51 - FROM_CPU_INTR1"]
    FROM_CPU_INTR1 = 51,
    #[doc = "52 - FROM_CPU_INTR2"]
    FROM_CPU_INTR2 = 52,
    #[doc = "53 - FROM_CPU_INTR3"]
    FROM_CPU_INTR3 = 53,
    #[doc = "54 - ASSIST_DEBUG"]
    ASSIST_DEBUG = 54,
    #[doc = "55 - DMA_APBPERI_PMS"]
    DMA_APBPERI_PMS = 55,
    #[doc = "56 - CORE0_IRAM0_PMS"]
    CORE0_IRAM0_PMS = 56,
    #[doc = "57 - CORE0_DRAM0_PMS"]
    CORE0_DRAM0_PMS = 57,
    #[doc = "58 - CORE0_PIF_PMS"]
    CORE0_PIF_PMS = 58,
    #[doc = "59 - CORE0_PIF_PMS_SIZE"]
    CORE0_PIF_PMS_SIZE = 59,
    #[doc = "60 - BAK_PMS_VIOLATE"]
    BAK_PMS_VIOLATE = 60,
    #[doc = "61 - CACHE_CORE0_ACS"]
    CACHE_CORE0_ACS = 61,
}
pub type interrupt = Interrupt;
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    extern "C" {
        fn WIFI_MAC();
        fn WIFI_MAC_NMI();
        fn WIFI_PWR();
        fn WIFI_BB();
        fn BT_MAC();
        fn BT_BB();
        fn BT_BB_NMI();
        fn RWBT();
        fn RWBLE();
        fn RWBT_NMI();
        fn RWBLE_NMI();
        fn I2C_MASTER();
        fn SLC0();
        fn SLC1();
        fn APB_CTRL();
        fn UHCI0();
        fn GPIO();
        fn GPIO_NMI();
        fn SPI1();
        fn SPI2();
        fn I2S0();
        fn UART0();
        fn UART1();
        fn LEDC();
        fn EFUSE();
        fn TWAI0();
        fn USB_SERIAL_JTAG();
        fn RTC_CORE();
        fn RMT();
        fn I2C_EXT0();
        fn TIMER1();
        fn TIMER2();
        fn TG0_T0_LEVEL();
        fn TG0_WDT_LEVEL();
        fn TG1_T0_LEVEL();
        fn TG1_WDT_LEVEL();
        fn CACHE_IA();
        fn SYSTIMER_TARGET0();
        fn SYSTIMER_TARGET1();
        fn SYSTIMER_TARGET2();
        fn SPI_MEM_REJECT_CACHE();
        fn ICACHE_PRELOAD0();
        fn ICACHE_SYNC0();
        fn APB_ADC();
        fn DMA_CH0();
        fn DMA_CH1();
        fn DMA_CH2();
        fn RSA();
        fn AES();
        fn SHA();
        fn FROM_CPU_INTR0();
        fn FROM_CPU_INTR1();
        fn FROM_CPU_INTR2();
        fn FROM_CPU_INTR3();
        fn ASSIST_DEBUG();
        fn DMA_APBPERI_PMS();
        fn CORE0_IRAM0_PMS();
        fn CORE0_DRAM0_PMS();
        fn CORE0_PIF_PMS();
        fn CORE0_PIF_PMS_SIZE();
        fn BAK_PMS_VIOLATE();
        fn CACHE_CORE0_ACS();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[cfg_attr(target_os = "none", link_section = ".vector_table.interrupts")]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 62] = [
        Vector { _handler: WIFI_MAC },
        Vector {
            _handler: WIFI_MAC_NMI,
        },
        Vector { _handler: WIFI_PWR },
        Vector { _handler: WIFI_BB },
        Vector { _handler: BT_MAC },
        Vector { _handler: BT_BB },
        Vector {
            _handler: BT_BB_NMI,
        },
        Vector { _handler: RWBT },
        Vector { _handler: RWBLE },
        Vector { _handler: RWBT_NMI },
        Vector {
            _handler: RWBLE_NMI,
        },
        Vector {
            _handler: I2C_MASTER,
        },
        Vector { _handler: SLC0 },
        Vector { _handler: SLC1 },
        Vector { _handler: APB_CTRL },
        Vector { _handler: UHCI0 },
        Vector { _handler: GPIO },
        Vector { _handler: GPIO_NMI },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: I2S0 },
        Vector { _handler: UART0 },
        Vector { _handler: UART1 },
        Vector { _handler: LEDC },
        Vector { _handler: EFUSE },
        Vector { _handler: TWAI0 },
        Vector {
            _handler: USB_SERIAL_JTAG,
        },
        Vector { _handler: RTC_CORE },
        Vector { _handler: RMT },
        Vector { _handler: I2C_EXT0 },
        Vector { _handler: TIMER1 },
        Vector { _handler: TIMER2 },
        Vector {
            _handler: TG0_T0_LEVEL,
        },
        Vector {
            _handler: TG0_WDT_LEVEL,
        },
        Vector {
            _handler: TG1_T0_LEVEL,
        },
        Vector {
            _handler: TG1_WDT_LEVEL,
        },
        Vector { _handler: CACHE_IA },
        Vector {
            _handler: SYSTIMER_TARGET0,
        },
        Vector {
            _handler: SYSTIMER_TARGET1,
        },
        Vector {
            _handler: SYSTIMER_TARGET2,
        },
        Vector {
            _handler: SPI_MEM_REJECT_CACHE,
        },
        Vector {
            _handler: ICACHE_PRELOAD0,
        },
        Vector {
            _handler: ICACHE_SYNC0,
        },
        Vector { _handler: APB_ADC },
        Vector { _handler: DMA_CH0 },
        Vector { _handler: DMA_CH1 },
        Vector { _handler: DMA_CH2 },
        Vector { _handler: RSA },
        Vector { _handler: AES },
        Vector { _handler: SHA },
        Vector {
            _handler: FROM_CPU_INTR0,
        },
        Vector {
            _handler: FROM_CPU_INTR1,
        },
        Vector {
            _handler: FROM_CPU_INTR2,
        },
        Vector {
            _handler: FROM_CPU_INTR3,
        },
        Vector {
            _handler: ASSIST_DEBUG,
        },
        Vector {
            _handler: DMA_APBPERI_PMS,
        },
        Vector {
            _handler: CORE0_IRAM0_PMS,
        },
        Vector {
            _handler: CORE0_DRAM0_PMS,
        },
        Vector {
            _handler: CORE0_PIF_PMS,
        },
        Vector {
            _handler: CORE0_PIF_PMS_SIZE,
        },
        Vector {
            _handler: BAK_PMS_VIOLATE,
        },
        Vector {
            _handler: CACHE_CORE0_ACS,
        },
    ];
}
#[path = "."]
pub mod aes {
    #[doc = "AES (Advanced Encryption Standard) Accelerator"]
    pub const AES: *const RegisterBlock = 0x6003_a000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/aes.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type AES = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for AES {}
    impl crate::Valid for AES {}
    impl AES {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(AES)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, AES).then_some(0)
    }
}
#[path = "."]
pub mod apb_ctrl {
    #[doc = "APB (Advanced Peripheral Bus) Controller"]
    pub const APB_CTRL: *const RegisterBlock = 0x6002_6000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/apb_ctrl.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type APB_CTRL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for APB_CTRL {}
    impl crate::Valid for APB_CTRL {}
    impl APB_CTRL {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(APB_CTRL)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, APB_CTRL).then_some(0)
    }
}
#[path = "."]
pub mod apb_saradc {
    #[doc = "SAR (Successive Approximation Register) Analog-to-Digital Converter"]
    pub const APB_SARADC: *const RegisterBlock = 0x6004_0000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/apb_saradc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type APB_SARADC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for APB_SARADC {}
    impl crate::Valid for APB_SARADC {}
    impl APB_SARADC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(APB_SARADC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, APB_SARADC).then_some(0)
    }
}
#[path = "."]
pub mod assist_debug {
    #[doc = "Debug Assist"]
    pub const ASSIST_DEBUG: *const RegisterBlock = 0x600c_e000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/assist_debug.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ASSIST_DEBUG = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ASSIST_DEBUG {}
    impl crate::Valid for ASSIST_DEBUG {}
    impl ASSIST_DEBUG {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ASSIST_DEBUG)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ASSIST_DEBUG).then_some(0)
    }
}
#[path = "."]
pub mod bb {
    #[doc = "BB Peripheral"]
    pub const BB: *const RegisterBlock = 0x6001_d000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/bb.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type BB = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for BB {}
    impl crate::Valid for BB {}
    impl BB {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BB)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BB).then_some(0)
    }
}
#[path = "."]
pub mod dma {
    #[doc = "DMA (Direct Memory Access) Controller"]
    pub const DMA: *const RegisterBlock = 0x6003_f000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/dma.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DMA = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DMA {}
    impl crate::Valid for DMA {}
    impl DMA {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DMA)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DMA).then_some(0)
    }
}
#[path = "."]
pub mod ds {
    #[doc = "Digital Signature"]
    pub const DS: *const RegisterBlock = 0x6003_d000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/ds.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DS = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DS {}
    impl crate::Valid for DS {}
    impl DS {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DS)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DS).then_some(0)
    }
}
#[path = "."]
pub mod efuse {
    #[doc = "eFuse Controller"]
    pub const EFUSE: *const RegisterBlock = 0x6000_8800 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/efuse.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type EFUSE = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for EFUSE {}
    impl crate::Valid for EFUSE {}
    impl EFUSE {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EFUSE)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, EFUSE).then_some(0)
    }
}
#[path = "."]
pub mod extmem {
    #[doc = "External Memory"]
    pub const EXTMEM: *const RegisterBlock = 0x600c_4000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/extmem.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type EXTMEM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for EXTMEM {}
    impl crate::Valid for EXTMEM {}
    impl EXTMEM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EXTMEM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, EXTMEM).then_some(0)
    }
}
#[path = "."]
pub mod gpio {
    #[doc = "General Purpose Input/Output"]
    pub const GPIO: *const RegisterBlock = 0x6000_4000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/gpio.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type GPIO = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for GPIO {}
    impl crate::Valid for GPIO {}
    impl GPIO {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, GPIO).then_some(0)
    }
}
#[path = "."]
pub mod gpio_sd {
    #[doc = "Sigma-Delta Modulation"]
    pub const GPIO_SD: *const RegisterBlock = 0x6000_4f00 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/gpio_sd.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type GPIO_SD = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for GPIO_SD {}
    impl crate::Valid for GPIO_SD {}
    impl GPIO_SD {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO_SD)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, GPIO_SD).then_some(0)
    }
}
#[path = "."]
pub mod hmac {
    #[doc = "HMAC (Hash-based Message Authentication Code) Accelerator"]
    pub const HMAC: *const RegisterBlock = 0x6003_e000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/hmac.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type HMAC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for HMAC {}
    impl crate::Valid for HMAC {}
    impl HMAC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(HMAC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, HMAC).then_some(0)
    }
}
#[path = "."]
pub mod i2c0 {
    #[doc = "I2C (Inter-Integrated Circuit) Controller 0"]
    pub const I2C0: *const RegisterBlock = 0x6001_3000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/i2c0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type I2C0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for I2C0 {}
    impl crate::Valid for I2C0 {}
    impl I2C0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I2C0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, I2C0).then_some(0)
    }
}
#[path = "."]
pub mod i2s0 {
    #[doc = "I2S (Inter-IC Sound) Controller 0"]
    pub const I2S0: *const RegisterBlock = 0x6002_d000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/i2s0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type I2S0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for I2S0 {}
    impl crate::Valid for I2S0 {}
    impl I2S0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I2S0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, I2S0).then_some(0)
    }
}
#[path = "."]
pub mod interrupt_core0 {
    #[doc = "Interrupt Controller (Core 0)"]
    pub const INTERRUPT_CORE0: *const RegisterBlock = 0x600c_2000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/interrupt_core0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type INTERRUPT_CORE0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for INTERRUPT_CORE0 {}
    impl crate::Valid for INTERRUPT_CORE0 {}
    impl INTERRUPT_CORE0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(INTERRUPT_CORE0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, INTERRUPT_CORE0).then_some(0)
    }
}
#[path = "."]
pub mod io_mux {
    #[doc = "Input/Output Multiplexer"]
    pub const IO_MUX: *const RegisterBlock = 0x6000_9000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/io_mux.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type IO_MUX = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for IO_MUX {}
    impl crate::Valid for IO_MUX {}
    impl IO_MUX {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(IO_MUX)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, IO_MUX).then_some(0)
    }
}
#[path = "."]
pub mod ledc {
    #[doc = "LED Control PWM (Pulse Width Modulation)"]
    pub const LEDC: *const RegisterBlock = 0x6001_9000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/ledc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type LEDC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for LEDC {}
    impl crate::Valid for LEDC {}
    impl LEDC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LEDC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, LEDC).then_some(0)
    }
}
#[path = "."]
pub mod rmt {
    #[doc = "Remote Control"]
    pub const RMT: *const RegisterBlock = 0x6001_6000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/rmt.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type RMT = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for RMT {}
    impl crate::Valid for RMT {}
    impl RMT {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RMT)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, RMT).then_some(0)
    }
}
#[path = "."]
pub mod rng {
    #[doc = "Hardware Random Number Generator"]
    pub const RNG: *const RegisterBlock = 0x6002_6000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/rng.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type RNG = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for RNG {}
    impl crate::Valid for RNG {}
    impl RNG {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RNG)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, RNG).then_some(0)
    }
}
#[path = "."]
pub mod rsa {
    #[doc = "RSA (Rivest Shamir Adleman) Accelerator"]
    pub const RSA: *const RegisterBlock = 0x6003_c000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/rsa.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type RSA = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for RSA {}
    impl crate::Valid for RSA {}
    impl RSA {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RSA)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, RSA).then_some(0)
    }
}
#[path = "."]
pub mod rtc_cntl {
    #[doc = "Real-Time Clock Control"]
    pub const RTC_CNTL: *const RegisterBlock = 0x6000_8000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/rtc_cntl.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type RTC_CNTL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for RTC_CNTL {}
    impl crate::Valid for RTC_CNTL {}
    impl RTC_CNTL {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RTC_CNTL)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, RTC_CNTL).then_some(0)
    }
}
#[path = "."]
pub mod sensitive {
    #[doc = "SENSITIVE Peripheral"]
    pub const SENSITIVE: *const RegisterBlock = 0x600c_1000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/sensitive.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SENSITIVE = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SENSITIVE {}
    impl crate::Valid for SENSITIVE {}
    impl SENSITIVE {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SENSITIVE)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SENSITIVE).then_some(0)
    }
}
#[path = "."]
pub mod sha {
    #[doc = "SHA (Secure Hash Algorithm) Accelerator"]
    pub const SHA: *const RegisterBlock = 0x6003_b000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/sha.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SHA = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SHA {}
    impl crate::Valid for SHA {}
    impl SHA {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SHA)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SHA).then_some(0)
    }
}
#[path = "."]
pub mod spi0 {
    #[doc = "SPI (Serial Peripheral Interface) Controller 0"]
    pub const SPI0: *const RegisterBlock = 0x6000_3000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/spi0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SPI0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SPI0 {}
    impl crate::Valid for SPI0 {}
    impl SPI0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SPI0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SPI0).then_some(0)
    }
}
#[path = "."]
pub mod spi1 {
    #[doc = "SPI (Serial Peripheral Interface) Controller 1"]
    pub const SPI1: *const RegisterBlock = 0x6000_2000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/spi1.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SPI1 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SPI1 {}
    impl crate::Valid for SPI1 {}
    impl SPI1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SPI1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SPI1).then_some(0)
    }
}
#[path = "."]
pub mod spi2 {
    #[doc = "SPI (Serial Peripheral Interface) Controller 2"]
    pub const SPI2: *const RegisterBlock = 0x6002_4000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/spi2.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SPI2 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SPI2 {}
    impl crate::Valid for SPI2 {}
    impl SPI2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SPI2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SPI2).then_some(0)
    }
}
#[path = "."]
pub mod system {
    #[doc = "System Configuration Registers"]
    pub const SYSTEM: *const RegisterBlock = 0x600c_0000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/system.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SYSTEM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SYSTEM {}
    impl crate::Valid for SYSTEM {}
    impl SYSTEM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SYSTEM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SYSTEM).then_some(0)
    }
}
#[path = "."]
pub mod systimer {
    #[doc = "System Timer"]
    pub const SYSTIMER: *const RegisterBlock = 0x6002_3000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/systimer.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SYSTIMER = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SYSTIMER {}
    impl crate::Valid for SYSTIMER {}
    impl SYSTIMER {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SYSTIMER)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SYSTIMER).then_some(0)
    }
}
#[path = "."]
pub mod timg {
    #[doc = "Timer Group 0"]
    pub const TIMG0: *const RegisterBlock = 0x6001_f000 as *const RegisterBlock;
    #[doc = "Timer Group 1"]
    pub const TIMG1: *const RegisterBlock = 0x6002_0000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/timg.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type TIMG0 = Instance<0>;
    impl crate::private::Sealed for TIMG0 {}
    impl crate::Valid for TIMG0 {}
    impl TIMG0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TIMG0)
        }
    }
    pub type TIMG1 = Instance<1>;
    impl crate::private::Sealed for TIMG1 {}
    impl crate::Valid for TIMG1 {}
    impl TIMG1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TIMG1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(TIMG0, 0), (TIMG1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod twai0 {
    #[doc = "Two-Wire Automotive Interface"]
    pub const TWAI0: *const RegisterBlock = 0x6002_b000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/twai0.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type TWAI0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for TWAI0 {}
    impl crate::Valid for TWAI0 {}
    impl TWAI0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TWAI0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TWAI0).then_some(0)
    }
}
#[path = "."]
pub mod uart {
    #[doc = "UART (Universal Asynchronous Receiver-Transmitter) Controller 0"]
    pub const UART0: *const RegisterBlock = 0x6000_0000 as *const RegisterBlock;
    #[doc = "UART (Universal Asynchronous Receiver-Transmitter) Controller 1"]
    pub const UART1: *const RegisterBlock = 0x6001_0000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/uart.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type UART0 = Instance<0>;
    impl crate::private::Sealed for UART0 {}
    impl crate::Valid for UART0 {}
    impl UART0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART0)
        }
    }
    pub type UART1 = Instance<1>;
    impl crate::private::Sealed for UART1 {}
    impl crate::Valid for UART1 {}
    impl UART1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(UART0, 0), (UART1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod uhci {
    #[doc = "Universal Host Controller Interface 0"]
    pub const UHCI0: *const RegisterBlock = 0x6001_4000 as *const RegisterBlock;
    #[doc = "Universal Host Controller Interface 1"]
    pub const UHCI1: *const RegisterBlock = 0x6000_c000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/uhci.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type UHCI0 = Instance<0>;
    impl crate::private::Sealed for UHCI0 {}
    impl crate::Valid for UHCI0 {}
    impl UHCI0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UHCI0)
        }
    }
    pub type UHCI1 = Instance<1>;
    impl crate::private::Sealed for UHCI1 {}
    impl crate::Valid for UHCI1 {}
    impl UHCI1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UHCI1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(UHCI0, 0), (UHCI1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod usb_device {
    #[doc = "Full-speed USB Serial/JTAG Controller"]
    pub const USB_DEVICE: *const RegisterBlock = 0x6004_3000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/usb_device.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type USB_DEVICE = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for USB_DEVICE {}
    impl crate::Valid for USB_DEVICE {}
    impl USB_DEVICE {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USB_DEVICE)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USB_DEVICE).then_some(0)
    }
}
#[path = "."]
pub mod xts_aes {
    #[doc = "XTS-AES-128 Flash Encryption"]
    pub const XTS_AES: *const RegisterBlock = 0x600c_c000 as *const RegisterBlock;
    #[path = "blocks/Esp32c3/xts_aes.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type XTS_AES = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for XTS_AES {}
    impl crate::Valid for XTS_AES {}
    impl XTS_AES {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XTS_AES)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, XTS_AES).then_some(0)
    }
}
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[doc = r" Instances for all of this device's peripherals."]
#[doc = r""]
#[doc = r" Use this if you want a single way to acquire *all* instances"]
#[doc = r" for your device."]
pub struct Instances {
    pub AES: aes::AES,
    pub APB_CTRL: apb_ctrl::APB_CTRL,
    pub APB_SARADC: apb_saradc::APB_SARADC,
    pub ASSIST_DEBUG: assist_debug::ASSIST_DEBUG,
    pub BB: bb::BB,
    pub DMA: dma::DMA,
    pub DS: ds::DS,
    pub EFUSE: efuse::EFUSE,
    pub EXTMEM: extmem::EXTMEM,
    pub GPIO: gpio::GPIO,
    pub GPIO_SD: gpio_sd::GPIO_SD,
    pub HMAC: hmac::HMAC,
    pub I2C0: i2c0::I2C0,
    pub I2S0: i2s0::I2S0,
    pub INTERRUPT_CORE0: interrupt_core0::INTERRUPT_CORE0,
    pub IO_MUX: io_mux::IO_MUX,
    pub LEDC: ledc::LEDC,
    pub RMT: rmt::RMT,
    pub RNG: rng::RNG,
    pub RSA: rsa::RSA,
    pub RTC_CNTL: rtc_cntl::RTC_CNTL,
    pub SENSITIVE: sensitive::SENSITIVE,
    pub SHA: sha::SHA,
    pub SPI0: spi0::SPI0,
    pub SPI1: spi1::SPI1,
    pub SPI2: spi2::SPI2,
    pub SYSTEM: system::SYSTEM,
    pub SYSTIMER: systimer::SYSTIMER,
    pub TIMG0: timg::TIMG0,
    pub TIMG1: timg::TIMG1,
    pub TWAI0: twai0::TWAI0,
    pub UART0: uart::UART0,
    pub UART1: uart::UART1,
    pub UHCI0: uhci::UHCI0,
    pub UHCI1: uhci::UHCI1,
    pub USB_DEVICE: usb_device::USB_DEVICE,
    pub XTS_AES: xts_aes::XTS_AES,
}
impl Instances {
    #[doc = r" Acquire all peripheral instances."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Since this calls `instance()` to initialize each of its members,"]
    #[doc = r" the `instance()` safety contract applies. See [the `Instance` safety"]
    #[doc = r" documentation](crate::Instance) for more information."]
    #[inline]
    pub const unsafe fn instances() -> Self {
        Self {
            AES: aes::AES::instance(),
            APB_CTRL: apb_ctrl::APB_CTRL::instance(),
            APB_SARADC: apb_saradc::APB_SARADC::instance(),
            ASSIST_DEBUG: assist_debug::ASSIST_DEBUG::instance(),
            BB: bb::BB::instance(),
            DMA: dma::DMA::instance(),
            DS: ds::DS::instance(),
            EFUSE: efuse::EFUSE::instance(),
            EXTMEM: extmem::EXTMEM::instance(),
            GPIO: gpio::GPIO::instance(),
            GPIO_SD: gpio_sd::GPIO_SD::instance(),
            HMAC: hmac::HMAC::instance(),
            I2C0: i2c0::I2C0::instance(),
            I2S0: i2s0::I2S0::instance(),
            INTERRUPT_CORE0: interrupt_core0::INTERRUPT_CORE0::instance(),
            IO_MUX: io_mux::IO_MUX::instance(),
            LEDC: ledc::LEDC::instance(),
            RMT: rmt::RMT::instance(),
            RNG: rng::RNG::instance(),
            RSA: rsa::RSA::instance(),
            RTC_CNTL: rtc_cntl::RTC_CNTL::instance(),
            SENSITIVE: sensitive::SENSITIVE::instance(),
            SHA: sha::SHA::instance(),
            SPI0: spi0::SPI0::instance(),
            SPI1: spi1::SPI1::instance(),
            SPI2: spi2::SPI2::instance(),
            SYSTEM: system::SYSTEM::instance(),
            SYSTIMER: systimer::SYSTIMER::instance(),
            TIMG0: timg::TIMG0::instance(),
            TIMG1: timg::TIMG1::instance(),
            TWAI0: twai0::TWAI0::instance(),
            UART0: uart::UART0::instance(),
            UART1: uart::UART1::instance(),
            UHCI0: uhci::UHCI0::instance(),
            UHCI1: uhci::UHCI1::instance(),
            USB_DEVICE: usb_device::USB_DEVICE::instance(),
            XTS_AES: xts_aes::XTS_AES::instance(),
        }
    }
}
