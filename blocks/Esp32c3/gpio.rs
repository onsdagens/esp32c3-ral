#[doc = "General Purpose Input/Output"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "GPIO bit select register"]
    pub BT_SELECT: crate::RWRegister<u32>,
    #[doc = "GPIO output register"]
    pub OUT: crate::RWRegister<u32>,
    #[doc = "GPIO output set register"]
    pub OUT_W1TS: crate::RWRegister<u32>,
    #[doc = "GPIO output clear register"]
    pub OUT_W1TC: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "GPIO sdio select register"]
    pub SDIO_SELECT: crate::RWRegister<u32>,
    #[doc = "GPIO output enable register"]
    pub ENABLE: crate::RWRegister<u32>,
    #[doc = "GPIO output enable set register"]
    pub ENABLE_W1TS: crate::RWRegister<u32>,
    #[doc = "GPIO output enable clear register"]
    pub ENABLE_W1TC: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "pad strapping register"]
    pub STRAP: crate::RWRegister<u32>,
    #[doc = "GPIO input register"]
    pub IN: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "GPIO interrupt status register"]
    pub STATUS: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt status set register"]
    pub STATUS_W1TS: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt status clear register"]
    pub STATUS_W1TC: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "GPIO PRO_CPU interrupt status register"]
    pub PCPU_INT: crate::RWRegister<u32>,
    #[doc = "GPIO PRO_CPU(not shielded) interrupt status register"]
    pub PCPU_NMI_INT: crate::RWRegister<u32>,
    #[doc = "GPIO CPUSDIO interrupt status register"]
    pub CPUSDIO_INT: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "GPIO pin configuration register"]
    pub PIN: [crate::RWRegister<u32>; 26usize],
    _reserved5: [u8; 0x70],
    #[doc = "GPIO interrupt source register"]
    pub STATUS_NEXT: crate::RWRegister<u32>,
    _reserved6: [u8; 0x04],
    #[doc = "GPIO input function configuration register"]
    pub FUNC_IN_SEL_CFG: [crate::RWRegister<u32>; 128usize],
    _reserved7: [u8; 0x0200],
    #[doc = "GPIO output function select register"]
    pub FUNC_OUT_SEL_CFG: [crate::RWRegister<u32>; 26usize],
    _reserved8: [u8; 0x70],
    #[doc = "GPIO clock gate register"]
    pub CLOCK_GATE: crate::RWRegister<u32>,
    _reserved9: [u8; 0xcc],
    #[doc = "GPIO version register"]
    pub REG_DATE: crate::RWRegister<u32>,
}
#[doc = "GPIO bit select register"]
pub mod BT_SELECT {
    #[doc = "GPIO bit select register"]
    pub mod BT_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output register"]
pub mod OUT {
    #[doc = "GPIO output register for GPIO0-25"]
    pub mod DATA_ORIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output set register"]
pub mod OUT_W1TS {
    #[doc = "GPIO output set register for GPIO0-25"]
    pub mod OUT_W1TS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output clear register"]
pub mod OUT_W1TC {
    #[doc = "GPIO output clear register for GPIO0-25"]
    pub mod OUT_W1TC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO sdio select register"]
pub mod SDIO_SELECT {
    #[doc = "GPIO sdio select register"]
    pub mod SDIO_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output enable register"]
pub mod ENABLE {
    #[doc = "GPIO output enable register for GPIO0-25"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output enable set register"]
pub mod ENABLE_W1TS {
    #[doc = "GPIO output enable set register for GPIO0-25"]
    pub mod ENABLE_W1TS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output enable clear register"]
pub mod ENABLE_W1TC {
    #[doc = "GPIO output enable clear register for GPIO0-25"]
    pub mod ENABLE_W1TC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "pad strapping register"]
pub mod STRAP {
    #[doc = "pad strapping register"]
    pub mod STRAPPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input register"]
pub mod IN {
    #[doc = "GPIO input register for GPIO0-25"]
    pub mod DATA_NEXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt status register"]
pub mod STATUS {
    #[doc = "GPIO interrupt status register for GPIO0-25"]
    pub mod INTERRUPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt status set register"]
pub mod STATUS_W1TS {
    #[doc = "GPIO interrupt status set register for GPIO0-25"]
    pub mod STATUS_W1TS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt status clear register"]
pub mod STATUS_W1TC {
    #[doc = "GPIO interrupt status clear register for GPIO0-25"]
    pub mod STATUS_W1TC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO PRO_CPU interrupt status register"]
pub mod PCPU_INT {
    #[doc = "GPIO PRO_CPU interrupt status register for GPIO0-25"]
    pub mod PROCPU_INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO PRO_CPU(not shielded) interrupt status register"]
pub mod PCPU_NMI_INT {
    #[doc = "GPIO PRO_CPU(not shielded) interrupt status register for GPIO0-25"]
    pub mod PROCPU_NMI_INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO CPUSDIO interrupt status register"]
pub mod CPUSDIO_INT {
    #[doc = "GPIO CPUSDIO interrupt status register for GPIO0-25"]
    pub mod SDIO_INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO pin configuration register"]
pub mod PIN {
    #[doc = "set GPIO input_sync2 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    pub mod SYNC2_BYPASS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set this bit to select pad driver. 1:open-drain. 0:normal."]
    pub mod PAD_DRIVER {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set GPIO input_sync1 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    pub mod SYNC1_BYPASS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set this value to choose interrupt mode. 0:disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
    pub mod INT_TYPE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
    pub mod WAKEUP_ENABLE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod CONFIG {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
    pub mod INT_ENA {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt source register"]
pub mod STATUS_NEXT {
    #[doc = "GPIO interrupt source register for GPIO0-25"]
    pub mod STATUS_INTERRUPT_NEXT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input function configuration register"]
pub mod FUNC_IN_SEL_CFG {
    #[doc = "set this value: s=0-53: connect GPIO\\[s\\] to this port. s=0x38: set this port always high level. s=0x3C: set this port always low level."]
    pub mod IN_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set this bit to invert input signal. 1:invert. 0:not invert."]
    pub mod IN_INV_SEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    pub mod SEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output function select register"]
pub mod FUNC_OUT_SEL_CFG {
    #[doc = "The value of the bits: 0<=s<=256. Set the value to select output signal. s=0-255: output of GPIO\\[n\\] equals input of peripheral\\[s\\]. s=256: output of GPIO\\[n\\] equals GPIO_OUT_REG\\[n\\]."]
    pub mod OUT_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set this bit to invert output signal.1:invert.0:not invert."]
    pub mod INV_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set this bit to select output enable signal.1:use GPIO_ENABLE_REG\\[n\\] as output enable signal.0:use peripheral output enable signal."]
    pub mod OEN_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set this bit to invert output enable signal.1:invert.0:not invert."]
    pub mod OEN_INV_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO clock gate register"]
pub mod CLOCK_GATE {
    #[doc = "set this bit to enable GPIO clock gate"]
    pub mod CLK_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO version register"]
pub mod REG_DATE {
    #[doc = "version register"]
    pub mod REG_DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
