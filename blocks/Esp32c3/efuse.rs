#[doc = "eFuse Controller"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Register 0 that stores data to be programmed."]
    pub PGM_DATA0: crate::RWRegister<u32>,
    #[doc = "Register 1 that stores data to be programmed."]
    pub PGM_DATA1: crate::RWRegister<u32>,
    #[doc = "Register 2 that stores data to be programmed."]
    pub PGM_DATA2: crate::RWRegister<u32>,
    #[doc = "Register 3 that stores data to be programmed."]
    pub PGM_DATA3: crate::RWRegister<u32>,
    #[doc = "Register 4 that stores data to be programmed."]
    pub PGM_DATA4: crate::RWRegister<u32>,
    #[doc = "Register 5 that stores data to be programmed."]
    pub PGM_DATA5: crate::RWRegister<u32>,
    #[doc = "Register 6 that stores data to be programmed."]
    pub PGM_DATA6: crate::RWRegister<u32>,
    #[doc = "Register 7 that stores data to be programmed."]
    pub PGM_DATA7: crate::RWRegister<u32>,
    #[doc = "Register 0 that stores the RS code to be programmed."]
    pub PGM_CHECK_VALUE0: crate::RWRegister<u32>,
    #[doc = "Register 1 that stores the RS code to be programmed."]
    pub PGM_CHECK_VALUE1: crate::RWRegister<u32>,
    #[doc = "Register 2 that stores the RS code to be programmed."]
    pub PGM_CHECK_VALUE2: crate::RWRegister<u32>,
    #[doc = "BLOCK0 data register 0."]
    pub RD_WR_DIS: crate::RWRegister<u32>,
    #[doc = "BLOCK0 data register 1."]
    pub RD_REPEAT_DATA0: crate::RWRegister<u32>,
    #[doc = "BLOCK0 data register 2."]
    pub RD_REPEAT_DATA1: crate::RWRegister<u32>,
    #[doc = "BLOCK0 data register 3."]
    pub RD_REPEAT_DATA2: crate::RWRegister<u32>,
    #[doc = "BLOCK0 data register 4."]
    pub RD_REPEAT_DATA3: crate::RWRegister<u32>,
    #[doc = "BLOCK0 data register 5."]
    pub RD_REPEAT_DATA4: crate::RWRegister<u32>,
    #[doc = "BLOCK1 data register 0."]
    pub RD_MAC_SPI_SYS_0: crate::RWRegister<u32>,
    #[doc = "BLOCK1 data register 1."]
    pub RD_MAC_SPI_SYS_1: crate::RWRegister<u32>,
    #[doc = "BLOCK1 data register 2."]
    pub RD_MAC_SPI_SYS_2: crate::RWRegister<u32>,
    #[doc = "BLOCK1 data register 3."]
    pub RD_MAC_SPI_SYS_3: crate::RWRegister<u32>,
    #[doc = "BLOCK1 data register 4."]
    pub RD_MAC_SPI_SYS_4: crate::RWRegister<u32>,
    #[doc = "BLOCK1 data register 5."]
    pub RD_MAC_SPI_SYS_5: crate::RWRegister<u32>,
    #[doc = "Register 0 of BLOCK2 (system)."]
    pub RD_SYS_PART1_DATA0: crate::RWRegister<u32>,
    #[doc = "Register 1 of BLOCK2 (system)."]
    pub RD_SYS_PART1_DATA1: crate::RWRegister<u32>,
    #[doc = "Register 2 of BLOCK2 (system)."]
    pub RD_SYS_PART1_DATA2: crate::RWRegister<u32>,
    #[doc = "Register 3 of BLOCK2 (system)."]
    pub RD_SYS_PART1_DATA3: crate::RWRegister<u32>,
    #[doc = "Register 4 of BLOCK2 (system)."]
    pub RD_SYS_PART1_DATA4: crate::RWRegister<u32>,
    #[doc = "Register 5 of BLOCK2 (system)."]
    pub RD_SYS_PART1_DATA5: crate::RWRegister<u32>,
    #[doc = "Register 6 of BLOCK2 (system)."]
    pub RD_SYS_PART1_DATA6: crate::RWRegister<u32>,
    #[doc = "Register 7 of BLOCK2 (system)."]
    pub RD_SYS_PART1_DATA7: crate::RWRegister<u32>,
    #[doc = "Register 0 of BLOCK3 (user)."]
    pub RD_USR_DATA0: crate::RWRegister<u32>,
    #[doc = "Register 1 of BLOCK3 (user)."]
    pub RD_USR_DATA1: crate::RWRegister<u32>,
    #[doc = "Register 2 of BLOCK3 (user)."]
    pub RD_USR_DATA2: crate::RWRegister<u32>,
    #[doc = "Register 3 of BLOCK3 (user)."]
    pub RD_USR_DATA3: crate::RWRegister<u32>,
    #[doc = "Register 4 of BLOCK3 (user)."]
    pub RD_USR_DATA4: crate::RWRegister<u32>,
    #[doc = "Register 5 of BLOCK3 (user)."]
    pub RD_USR_DATA5: crate::RWRegister<u32>,
    #[doc = "Register 6 of BLOCK3 (user)."]
    pub RD_USR_DATA6: crate::RWRegister<u32>,
    #[doc = "Register 7 of BLOCK3 (user)."]
    pub RD_USR_DATA7: crate::RWRegister<u32>,
    #[doc = "Register 0 of BLOCK4 (KEY0)."]
    pub RD_KEY0_DATA0: crate::RWRegister<u32>,
    #[doc = "Register 1 of BLOCK4 (KEY0)."]
    pub RD_KEY0_DATA1: crate::RWRegister<u32>,
    #[doc = "Register 2 of BLOCK4 (KEY0)."]
    pub RD_KEY0_DATA2: crate::RWRegister<u32>,
    #[doc = "Register 3 of BLOCK4 (KEY0)."]
    pub RD_KEY0_DATA3: crate::RWRegister<u32>,
    #[doc = "Register 4 of BLOCK4 (KEY0)."]
    pub RD_KEY0_DATA4: crate::RWRegister<u32>,
    #[doc = "Register 5 of BLOCK4 (KEY0)."]
    pub RD_KEY0_DATA5: crate::RWRegister<u32>,
    #[doc = "Register 6 of BLOCK4 (KEY0)."]
    pub RD_KEY0_DATA6: crate::RWRegister<u32>,
    #[doc = "Register 7 of BLOCK4 (KEY0)."]
    pub RD_KEY0_DATA7: crate::RWRegister<u32>,
    #[doc = "Register 0 of BLOCK5 (KEY1)."]
    pub RD_KEY1_DATA0: crate::RWRegister<u32>,
    #[doc = "Register 1 of BLOCK5 (KEY1)."]
    pub RD_KEY1_DATA1: crate::RWRegister<u32>,
    #[doc = "Register 2 of BLOCK5 (KEY1)."]
    pub RD_KEY1_DATA2: crate::RWRegister<u32>,
    #[doc = "Register 3 of BLOCK5 (KEY1)."]
    pub RD_KEY1_DATA3: crate::RWRegister<u32>,
    #[doc = "Register 4 of BLOCK5 (KEY1)."]
    pub RD_KEY1_DATA4: crate::RWRegister<u32>,
    #[doc = "Register 5 of BLOCK5 (KEY1)."]
    pub RD_KEY1_DATA5: crate::RWRegister<u32>,
    #[doc = "Register 6 of BLOCK5 (KEY1)."]
    pub RD_KEY1_DATA6: crate::RWRegister<u32>,
    #[doc = "Register 7 of BLOCK5 (KEY1)."]
    pub RD_KEY1_DATA7: crate::RWRegister<u32>,
    #[doc = "Register 0 of BLOCK6 (KEY2)."]
    pub RD_KEY2_DATA0: crate::RWRegister<u32>,
    #[doc = "Register 1 of BLOCK6 (KEY2)."]
    pub RD_KEY2_DATA1: crate::RWRegister<u32>,
    #[doc = "Register 2 of BLOCK6 (KEY2)."]
    pub RD_KEY2_DATA2: crate::RWRegister<u32>,
    #[doc = "Register 3 of BLOCK6 (KEY2)."]
    pub RD_KEY2_DATA3: crate::RWRegister<u32>,
    #[doc = "Register 4 of BLOCK6 (KEY2)."]
    pub RD_KEY2_DATA4: crate::RWRegister<u32>,
    #[doc = "Register 5 of BLOCK6 (KEY2)."]
    pub RD_KEY2_DATA5: crate::RWRegister<u32>,
    #[doc = "Register 6 of BLOCK6 (KEY2)."]
    pub RD_KEY2_DATA6: crate::RWRegister<u32>,
    #[doc = "Register 7 of BLOCK6 (KEY2)."]
    pub RD_KEY2_DATA7: crate::RWRegister<u32>,
    #[doc = "Register 0 of BLOCK7 (KEY3)."]
    pub RD_KEY3_DATA0: crate::RWRegister<u32>,
    #[doc = "Register 1 of BLOCK7 (KEY3)."]
    pub RD_KEY3_DATA1: crate::RWRegister<u32>,
    #[doc = "Register 2 of BLOCK7 (KEY3)."]
    pub RD_KEY3_DATA2: crate::RWRegister<u32>,
    #[doc = "Register 3 of BLOCK7 (KEY3)."]
    pub RD_KEY3_DATA3: crate::RWRegister<u32>,
    #[doc = "Register 4 of BLOCK7 (KEY3)."]
    pub RD_KEY3_DATA4: crate::RWRegister<u32>,
    #[doc = "Register 5 of BLOCK7 (KEY3)."]
    pub RD_KEY3_DATA5: crate::RWRegister<u32>,
    #[doc = "Register 6 of BLOCK7 (KEY3)."]
    pub RD_KEY3_DATA6: crate::RWRegister<u32>,
    #[doc = "Register 7 of BLOCK7 (KEY3)."]
    pub RD_KEY3_DATA7: crate::RWRegister<u32>,
    #[doc = "Register 0 of BLOCK8 (KEY4)."]
    pub RD_KEY4_DATA0: crate::RWRegister<u32>,
    #[doc = "Register 1 of BLOCK8 (KEY4)."]
    pub RD_KEY4_DATA1: crate::RWRegister<u32>,
    #[doc = "Register 2 of BLOCK8 (KEY4)."]
    pub RD_KEY4_DATA2: crate::RWRegister<u32>,
    #[doc = "Register 3 of BLOCK8 (KEY4)."]
    pub RD_KEY4_DATA3: crate::RWRegister<u32>,
    #[doc = "Register 4 of BLOCK8 (KEY4)."]
    pub RD_KEY4_DATA4: crate::RWRegister<u32>,
    #[doc = "Register 5 of BLOCK8 (KEY4)."]
    pub RD_KEY4_DATA5: crate::RWRegister<u32>,
    #[doc = "Register 6 of BLOCK8 (KEY4)."]
    pub RD_KEY4_DATA6: crate::RWRegister<u32>,
    #[doc = "Register 7 of BLOCK8 (KEY4)."]
    pub RD_KEY4_DATA7: crate::RWRegister<u32>,
    #[doc = "Register 0 of BLOCK9 (KEY5)."]
    pub RD_KEY5_DATA0: crate::RWRegister<u32>,
    #[doc = "Register 1 of BLOCK9 (KEY5)."]
    pub RD_KEY5_DATA1: crate::RWRegister<u32>,
    #[doc = "Register 2 of BLOCK9 (KEY5)."]
    pub RD_KEY5_DATA2: crate::RWRegister<u32>,
    #[doc = "Register 3 of BLOCK9 (KEY5)."]
    pub RD_KEY5_DATA3: crate::RWRegister<u32>,
    #[doc = "Register 4 of BLOCK9 (KEY5)."]
    pub RD_KEY5_DATA4: crate::RWRegister<u32>,
    #[doc = "Register 5 of BLOCK9 (KEY5)."]
    pub RD_KEY5_DATA5: crate::RWRegister<u32>,
    #[doc = "Register 6 of BLOCK9 (KEY5)."]
    pub RD_KEY5_DATA6: crate::RWRegister<u32>,
    #[doc = "Register 7 of BLOCK9 (KEY5)."]
    pub RD_KEY5_DATA7: crate::RWRegister<u32>,
    #[doc = "Register 0 of BLOCK10 (system)."]
    pub RD_SYS_PART2_DATA0: crate::RWRegister<u32>,
    #[doc = "Register 1 of BLOCK9 (KEY5)."]
    pub RD_SYS_PART2_DATA1: crate::RWRegister<u32>,
    #[doc = "Register 2 of BLOCK10 (system)."]
    pub RD_SYS_PART2_DATA2: crate::RWRegister<u32>,
    #[doc = "Register 3 of BLOCK10 (system)."]
    pub RD_SYS_PART2_DATA3: crate::RWRegister<u32>,
    #[doc = "Register 4 of BLOCK10 (system)."]
    pub RD_SYS_PART2_DATA4: crate::RWRegister<u32>,
    #[doc = "Register 5 of BLOCK10 (system)."]
    pub RD_SYS_PART2_DATA5: crate::RWRegister<u32>,
    #[doc = "Register 6 of BLOCK10 (system)."]
    pub RD_SYS_PART2_DATA6: crate::RWRegister<u32>,
    #[doc = "Register 7 of BLOCK10 (system)."]
    pub RD_SYS_PART2_DATA7: crate::RWRegister<u32>,
    #[doc = "Programming error record register 0 of BLOCK0."]
    pub RD_REPEAT_ERR0: crate::RWRegister<u32>,
    #[doc = "Programming error record register 1 of BLOCK0."]
    pub RD_REPEAT_ERR1: crate::RWRegister<u32>,
    #[doc = "Programming error record register 2 of BLOCK0."]
    pub RD_REPEAT_ERR2: crate::RWRegister<u32>,
    #[doc = "Programming error record register 3 of BLOCK0."]
    pub RD_REPEAT_ERR3: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Programming error record register 4 of BLOCK0."]
    pub RD_REPEAT_ERR4: crate::RWRegister<u32>,
    _reserved1: [u8; 0x2c],
    #[doc = "Programming error record register 0 of BLOCK1-10."]
    pub RD_RS_ERR0: crate::RWRegister<u32>,
    #[doc = "Programming error record register 1 of BLOCK1-10."]
    pub RD_RS_ERR1: crate::RWRegister<u32>,
    #[doc = "eFuse clcok configuration register."]
    pub CLK: crate::RWRegister<u32>,
    #[doc = "eFuse operation mode configuraiton register;"]
    pub CONF: crate::RWRegister<u32>,
    #[doc = "eFuse status register."]
    pub STATUS: crate::RWRegister<u32>,
    #[doc = "eFuse command register."]
    pub CMD: crate::RWRegister<u32>,
    #[doc = "eFuse raw interrupt register."]
    pub INT_RAW: crate::RWRegister<u32>,
    #[doc = "eFuse interrupt status register."]
    pub INT_ST: crate::RWRegister<u32>,
    #[doc = "eFuse interrupt enable register."]
    pub INT_ENA: crate::RWRegister<u32>,
    #[doc = "eFuse interrupt clear register."]
    pub INT_CLR: crate::RWRegister<u32>,
    #[doc = "Controls the eFuse programming voltage."]
    pub DAC_CONF: crate::RWRegister<u32>,
    #[doc = "Configures read timing parameters."]
    pub RD_TIM_CONF: crate::RWRegister<u32>,
    #[doc = "Configurarion register 1 of eFuse programming timing parameters."]
    pub WR_TIM_CONF1: crate::RWRegister<u32>,
    #[doc = "Configurarion register 2 of eFuse programming timing parameters."]
    pub WR_TIM_CONF2: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "eFuse version register."]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "Register 0 that stores data to be programmed."]
pub mod PGM_DATA0 {
    #[doc = "The content of the 0th 32-bit data to be programmed."]
    pub mod PGM_DATA_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 1 that stores data to be programmed."]
pub mod PGM_DATA1 {
    #[doc = "The content of the 1st 32-bit data to be programmed."]
    pub mod PGM_DATA_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 2 that stores data to be programmed."]
pub mod PGM_DATA2 {
    #[doc = "The content of the 2nd 32-bit data to be programmed."]
    pub mod PGM_DATA_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 3 that stores data to be programmed."]
pub mod PGM_DATA3 {
    #[doc = "The content of the 3rd 32-bit data to be programmed."]
    pub mod PGM_DATA_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 4 that stores data to be programmed."]
pub mod PGM_DATA4 {
    #[doc = "The content of the 4th 32-bit data to be programmed."]
    pub mod PGM_DATA_4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 5 that stores data to be programmed."]
pub mod PGM_DATA5 {
    #[doc = "The content of the 5th 32-bit data to be programmed."]
    pub mod PGM_DATA_5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 6 that stores data to be programmed."]
pub mod PGM_DATA6 {
    #[doc = "The content of the 6th 32-bit data to be programmed."]
    pub mod PGM_DATA_6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 7 that stores data to be programmed."]
pub mod PGM_DATA7 {
    #[doc = "The content of the 7th 32-bit data to be programmed."]
    pub mod PGM_DATA_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 0 that stores the RS code to be programmed."]
pub mod PGM_CHECK_VALUE0 {
    #[doc = "The content of the 0th 32-bit RS code to be programmed."]
    pub mod PGM_RS_DATA_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 1 that stores the RS code to be programmed."]
pub mod PGM_CHECK_VALUE1 {
    #[doc = "The content of the 1st 32-bit RS code to be programmed."]
    pub mod PGM_RS_DATA_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 2 that stores the RS code to be programmed."]
pub mod PGM_CHECK_VALUE2 {
    #[doc = "The content of the 2nd 32-bit RS code to be programmed."]
    pub mod PGM_RS_DATA_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BLOCK0 data register 0."]
pub mod RD_WR_DIS {
    #[doc = "Disable programming of individual eFuses."]
    pub mod WR_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BLOCK0 data register 1."]
pub mod RD_REPEAT_DATA0 {
    #[doc = "Set this bit to disable reading from BlOCK4-10."]
    pub mod RD_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to disable boot from RTC RAM."]
    pub mod DIS_RTC_RAM_BOOT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to disable Icache."]
    pub mod DIS_ICACHE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to disable function of usb switch to jtag in module of usb device."]
    pub mod DIS_USB_JTAG {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to disable Icache in download mode (boot_mode\\[3:0\\] is 0, 1, 2, 3, 6, 7)."]
    pub mod DIS_DOWNLOAD_ICACHE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to disable usb device."]
    pub mod DIS_USB_DEVICE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to disable the function that forces chip into download mode."]
    pub mod DIS_FORCE_DOWNLOAD {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved (used for four backups method)."]
    pub mod RPT4_RESERVED6 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to disable CAN function."]
    pub mod DIS_CAN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable selection between usb_to_jtag and pad_to_jtag through strapping gpio10 when both reg_dis_usb_jtag and reg_dis_pad_jtag are equal to 0."]
    pub mod JTAG_SEL_ENABLE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set these bits to disable JTAG in the soft way (odd number 1 means disable ). JTAG can be enabled in HMAC module."]
    pub mod SOFT_DIS_JTAG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to disable JTAG in the hard way. JTAG is disabled permanently."]
    pub mod DIS_PAD_JTAG {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to disable flash encryption when in download boot modes."]
    pub mod DIS_DOWNLOAD_MANUAL_ENCRYPT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controls single-end input threshold vrefh, 1.76 V to 2 V with step of 80 mV, stored in eFuse."]
    pub mod USB_DREFH {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controls single-end input threshold vrefl, 0.8 V to 1.04 V with step of 80 mV, stored in eFuse."]
    pub mod USB_DREFL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to exchange USB D+ and D- pins."]
    pub mod USB_EXCHG_PINS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to vdd spi pin function as gpio."]
    pub mod VDD_SPI_AS_GPIO {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable btlc gpio."]
    pub mod BTLC_GPIO_ENABLE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable power glitch function."]
    pub mod POWERGLITCH_EN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sample delay configuration of power glitch."]
    pub mod POWER_GLITCH_DSENSE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BLOCK0 data register 2."]
pub mod RD_REPEAT_DATA1 {
    #[doc = "Reserved (used for four backups method)."]
    pub mod RPT4_RESERVED2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects RTC watchdog timeout threshold, in unit of slow clock cycle. 0: 40000. 1: 80000. 2: 160000. 3:320000."]
    pub mod WDT_DELAY_SEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable SPI boot encrypt/decrypt. Odd number of 1: enable. even number of 1: disable."]
    pub mod SPI_BOOT_CRYPT_CNT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable revoking first secure boot key."]
    pub mod SECURE_BOOT_KEY_REVOKE0 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable revoking second secure boot key."]
    pub mod SECURE_BOOT_KEY_REVOKE1 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable revoking third secure boot key."]
    pub mod SECURE_BOOT_KEY_REVOKE2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Purpose of Key0."]
    pub mod KEY_PURPOSE_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Purpose of Key1."]
    pub mod KEY_PURPOSE_1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BLOCK0 data register 3."]
pub mod RD_REPEAT_DATA2 {
    #[doc = "Purpose of Key2."]
    pub mod KEY_PURPOSE_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Purpose of Key3."]
    pub mod KEY_PURPOSE_3 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Purpose of Key4."]
    pub mod KEY_PURPOSE_4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Purpose of Key5."]
    pub mod KEY_PURPOSE_5 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved (used for four backups method)."]
    pub mod RPT4_RESERVED3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable secure boot."]
    pub mod SECURE_BOOT_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable revoking aggressive secure boot."]
    pub mod SECURE_BOOT_AGGRESSIVE_REVOKE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved (used for four backups method)."]
    pub mod RPT4_RESERVED0 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Configures flash waiting time after power-up, in unit of ms. If the value is less than 15, the waiting time is the configurable value; Otherwise, the waiting time is twice the configurable value."]
    pub mod FLASH_TPUW {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BLOCK0 data register 4."]
pub mod RD_REPEAT_DATA3 {
    #[doc = "Set this bit to disable download mode (boot_mode\\[3:0\\] = 0, 1, 2, 3, 6, 7)."]
    pub mod DIS_DOWNLOAD_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to disable Legacy SPI boot mode (boot_mode\\[3:0\\] = 4)."]
    pub mod DIS_LEGACY_SPI_BOOT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selectes the default UART print channel. 0: UART0. 1: UART1."]
    pub mod UART_PRINT_CHANNEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set ECC mode in ROM, 0: ROM would Enable Flash ECC 16to18 byte mode. 1:ROM would use 16to17 byte mode."]
    pub mod FLASH_ECC_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to disable UART download mode through USB."]
    pub mod DIS_USB_DOWNLOAD_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable secure UART download mode."]
    pub mod ENABLE_SECURITY_DOWNLOAD {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set the default UARTboot message output mode. 00: Enabled. 01: Enabled when GPIO8 is low at reset. 10: Enabled when GPIO8 is high at reset. 11:disabled."]
    pub mod UART_PRINT_CONTROL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GPIO33-GPIO37 power supply selection in ROM code. 0: VDD3P3_CPU. 1: VDD_SPI."]
    pub mod PIN_POWER_SELECTION {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set the maximum lines of SPI flash. 0: four lines. 1: eight lines."]
    pub mod FLASH_TYPE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set Flash page size."]
    pub mod FLASH_PAGE_SIZE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set 1 to enable ECC for flash boot."]
    pub mod FLASH_ECC_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to force ROM code to send a resume command during SPI boot."]
    pub mod FORCE_SEND_RESUME {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Secure version (used by ESP-IDF anti-rollback feature)."]
    pub mod SECURE_VERSION {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved (used for four backups method)."]
    pub mod RPT4_RESERVED1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BLOCK0 data register 5."]
pub mod RD_REPEAT_DATA4 {
    #[doc = "Reserved (used for four backups method)."]
    pub mod RPT4_RESERVED4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BLOCK1 data register 0."]
pub mod RD_MAC_SPI_SYS_0 {
    #[doc = "Stores the low 32 bits of MAC address."]
    pub mod MAC_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BLOCK1 data register 1."]
pub mod RD_MAC_SPI_SYS_1 {
    #[doc = "Stores the high 16 bits of MAC address."]
    pub mod MAC_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stores the zeroth part of SPI_PAD_CONF."]
    pub mod SPI_PAD_CONF_0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BLOCK1 data register 2."]
pub mod RD_MAC_SPI_SYS_2 {
    #[doc = "Stores the first part of SPI_PAD_CONF."]
    pub mod SPI_PAD_CONF_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BLOCK1 data register 3."]
pub mod RD_MAC_SPI_SYS_3 {
    #[doc = "Stores the second part of SPI_PAD_CONF."]
    pub mod SPI_PAD_CONF_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stores the fist 14 bits of the zeroth part of system data."]
    pub mod SYS_DATA_PART0_0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BLOCK1 data register 4."]
pub mod RD_MAC_SPI_SYS_4 {
    #[doc = "Stores the fist 32 bits of the zeroth part of system data."]
    pub mod SYS_DATA_PART0_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BLOCK1 data register 5."]
pub mod RD_MAC_SPI_SYS_5 {
    #[doc = "Stores the second 32 bits of the zeroth part of system data."]
    pub mod SYS_DATA_PART0_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 0 of BLOCK2 (system)."]
pub mod RD_SYS_PART1_DATA0 {
    #[doc = "Stores the zeroth 32 bits of the first part of system data."]
    pub mod SYS_DATA_PART1_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 1 of BLOCK2 (system)."]
pub mod RD_SYS_PART1_DATA1 {
    #[doc = "Stores the first 32 bits of the first part of system data."]
    pub mod SYS_DATA_PART1_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 2 of BLOCK2 (system)."]
pub mod RD_SYS_PART1_DATA2 {
    #[doc = "Stores the second 32 bits of the first part of system data."]
    pub mod SYS_DATA_PART1_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 3 of BLOCK2 (system)."]
pub mod RD_SYS_PART1_DATA3 {
    #[doc = "Stores the third 32 bits of the first part of system data."]
    pub mod SYS_DATA_PART1_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 4 of BLOCK2 (system)."]
pub mod RD_SYS_PART1_DATA4 {
    #[doc = "Stores the fourth 32 bits of the first part of system data."]
    pub mod SYS_DATA_PART1_4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 5 of BLOCK2 (system)."]
pub mod RD_SYS_PART1_DATA5 {
    #[doc = "Stores the fifth 32 bits of the first part of system data."]
    pub mod SYS_DATA_PART1_5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 6 of BLOCK2 (system)."]
pub mod RD_SYS_PART1_DATA6 {
    #[doc = "Stores the sixth 32 bits of the first part of system data."]
    pub mod SYS_DATA_PART1_6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 7 of BLOCK2 (system)."]
pub mod RD_SYS_PART1_DATA7 {
    #[doc = "Stores the seventh 32 bits of the first part of system data."]
    pub mod SYS_DATA_PART1_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 0 of BLOCK3 (user)."]
pub mod RD_USR_DATA0 {
    #[doc = "Stores the zeroth 32 bits of BLOCK3 (user)."]
    pub mod USR_DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 1 of BLOCK3 (user)."]
pub mod RD_USR_DATA1 {
    #[doc = "Stores the first 32 bits of BLOCK3 (user)."]
    pub mod USR_DATA1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 2 of BLOCK3 (user)."]
pub mod RD_USR_DATA2 {
    #[doc = "Stores the second 32 bits of BLOCK3 (user)."]
    pub mod USR_DATA2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 3 of BLOCK3 (user)."]
pub mod RD_USR_DATA3 {
    #[doc = "Stores the third 32 bits of BLOCK3 (user)."]
    pub mod USR_DATA3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 4 of BLOCK3 (user)."]
pub mod RD_USR_DATA4 {
    #[doc = "Stores the fourth 32 bits of BLOCK3 (user)."]
    pub mod USR_DATA4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 5 of BLOCK3 (user)."]
pub mod RD_USR_DATA5 {
    #[doc = "Stores the fifth 32 bits of BLOCK3 (user)."]
    pub mod USR_DATA5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 6 of BLOCK3 (user)."]
pub mod RD_USR_DATA6 {
    #[doc = "Stores the sixth 32 bits of BLOCK3 (user)."]
    pub mod USR_DATA6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 7 of BLOCK3 (user)."]
pub mod RD_USR_DATA7 {
    #[doc = "Stores the seventh 32 bits of BLOCK3 (user)."]
    pub mod USR_DATA7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 0 of BLOCK4 (KEY0)."]
pub mod RD_KEY0_DATA0 {
    #[doc = "Stores the zeroth 32 bits of KEY0."]
    pub mod KEY0_DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 1 of BLOCK4 (KEY0)."]
pub mod RD_KEY0_DATA1 {
    #[doc = "Stores the first 32 bits of KEY0."]
    pub mod KEY0_DATA1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 2 of BLOCK4 (KEY0)."]
pub mod RD_KEY0_DATA2 {
    #[doc = "Stores the second 32 bits of KEY0."]
    pub mod KEY0_DATA2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 3 of BLOCK4 (KEY0)."]
pub mod RD_KEY0_DATA3 {
    #[doc = "Stores the third 32 bits of KEY0."]
    pub mod KEY0_DATA3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 4 of BLOCK4 (KEY0)."]
pub mod RD_KEY0_DATA4 {
    #[doc = "Stores the fourth 32 bits of KEY0."]
    pub mod KEY0_DATA4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 5 of BLOCK4 (KEY0)."]
pub mod RD_KEY0_DATA5 {
    #[doc = "Stores the fifth 32 bits of KEY0."]
    pub mod KEY0_DATA5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 6 of BLOCK4 (KEY0)."]
pub mod RD_KEY0_DATA6 {
    #[doc = "Stores the sixth 32 bits of KEY0."]
    pub mod KEY0_DATA6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 7 of BLOCK4 (KEY0)."]
pub mod RD_KEY0_DATA7 {
    #[doc = "Stores the seventh 32 bits of KEY0."]
    pub mod KEY0_DATA7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 0 of BLOCK5 (KEY1)."]
pub mod RD_KEY1_DATA0 {
    #[doc = "Stores the zeroth 32 bits of KEY1."]
    pub mod KEY1_DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 1 of BLOCK5 (KEY1)."]
pub mod RD_KEY1_DATA1 {
    #[doc = "Stores the first 32 bits of KEY1."]
    pub mod KEY1_DATA1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 2 of BLOCK5 (KEY1)."]
pub mod RD_KEY1_DATA2 {
    #[doc = "Stores the second 32 bits of KEY1."]
    pub mod KEY1_DATA2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 3 of BLOCK5 (KEY1)."]
pub mod RD_KEY1_DATA3 {
    #[doc = "Stores the third 32 bits of KEY1."]
    pub mod KEY1_DATA3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 4 of BLOCK5 (KEY1)."]
pub mod RD_KEY1_DATA4 {
    #[doc = "Stores the fourth 32 bits of KEY1."]
    pub mod KEY1_DATA4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 5 of BLOCK5 (KEY1)."]
pub mod RD_KEY1_DATA5 {
    #[doc = "Stores the fifth 32 bits of KEY1."]
    pub mod KEY1_DATA5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 6 of BLOCK5 (KEY1)."]
pub mod RD_KEY1_DATA6 {
    #[doc = "Stores the sixth 32 bits of KEY1."]
    pub mod KEY1_DATA6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 7 of BLOCK5 (KEY1)."]
pub mod RD_KEY1_DATA7 {
    #[doc = "Stores the seventh 32 bits of KEY1."]
    pub mod KEY1_DATA7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 0 of BLOCK6 (KEY2)."]
pub mod RD_KEY2_DATA0 {
    #[doc = "Stores the zeroth 32 bits of KEY2."]
    pub mod KEY2_DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 1 of BLOCK6 (KEY2)."]
pub mod RD_KEY2_DATA1 {
    #[doc = "Stores the first 32 bits of KEY2."]
    pub mod KEY2_DATA1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 2 of BLOCK6 (KEY2)."]
pub mod RD_KEY2_DATA2 {
    #[doc = "Stores the second 32 bits of KEY2."]
    pub mod KEY2_DATA2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 3 of BLOCK6 (KEY2)."]
pub mod RD_KEY2_DATA3 {
    #[doc = "Stores the third 32 bits of KEY2."]
    pub mod KEY2_DATA3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 4 of BLOCK6 (KEY2)."]
pub mod RD_KEY2_DATA4 {
    #[doc = "Stores the fourth 32 bits of KEY2."]
    pub mod KEY2_DATA4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 5 of BLOCK6 (KEY2)."]
pub mod RD_KEY2_DATA5 {
    #[doc = "Stores the fifth 32 bits of KEY2."]
    pub mod KEY2_DATA5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 6 of BLOCK6 (KEY2)."]
pub mod RD_KEY2_DATA6 {
    #[doc = "Stores the sixth 32 bits of KEY2."]
    pub mod KEY2_DATA6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 7 of BLOCK6 (KEY2)."]
pub mod RD_KEY2_DATA7 {
    #[doc = "Stores the seventh 32 bits of KEY2."]
    pub mod KEY2_DATA7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 0 of BLOCK7 (KEY3)."]
pub mod RD_KEY3_DATA0 {
    #[doc = "Stores the zeroth 32 bits of KEY3."]
    pub mod KEY3_DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 1 of BLOCK7 (KEY3)."]
pub mod RD_KEY3_DATA1 {
    #[doc = "Stores the first 32 bits of KEY3."]
    pub mod KEY3_DATA1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 2 of BLOCK7 (KEY3)."]
pub mod RD_KEY3_DATA2 {
    #[doc = "Stores the second 32 bits of KEY3."]
    pub mod KEY3_DATA2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 3 of BLOCK7 (KEY3)."]
pub mod RD_KEY3_DATA3 {
    #[doc = "Stores the third 32 bits of KEY3."]
    pub mod KEY3_DATA3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 4 of BLOCK7 (KEY3)."]
pub mod RD_KEY3_DATA4 {
    #[doc = "Stores the fourth 32 bits of KEY3."]
    pub mod KEY3_DATA4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 5 of BLOCK7 (KEY3)."]
pub mod RD_KEY3_DATA5 {
    #[doc = "Stores the fifth 32 bits of KEY3."]
    pub mod KEY3_DATA5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 6 of BLOCK7 (KEY3)."]
pub mod RD_KEY3_DATA6 {
    #[doc = "Stores the sixth 32 bits of KEY3."]
    pub mod KEY3_DATA6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 7 of BLOCK7 (KEY3)."]
pub mod RD_KEY3_DATA7 {
    #[doc = "Stores the seventh 32 bits of KEY3."]
    pub mod KEY3_DATA7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 0 of BLOCK8 (KEY4)."]
pub mod RD_KEY4_DATA0 {
    #[doc = "Stores the zeroth 32 bits of KEY4."]
    pub mod KEY4_DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 1 of BLOCK8 (KEY4)."]
pub mod RD_KEY4_DATA1 {
    #[doc = "Stores the first 32 bits of KEY4."]
    pub mod KEY4_DATA1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 2 of BLOCK8 (KEY4)."]
pub mod RD_KEY4_DATA2 {
    #[doc = "Stores the second 32 bits of KEY4."]
    pub mod KEY4_DATA2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 3 of BLOCK8 (KEY4)."]
pub mod RD_KEY4_DATA3 {
    #[doc = "Stores the third 32 bits of KEY4."]
    pub mod KEY4_DATA3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 4 of BLOCK8 (KEY4)."]
pub mod RD_KEY4_DATA4 {
    #[doc = "Stores the fourth 32 bits of KEY4."]
    pub mod KEY4_DATA4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 5 of BLOCK8 (KEY4)."]
pub mod RD_KEY4_DATA5 {
    #[doc = "Stores the fifth 32 bits of KEY4."]
    pub mod KEY4_DATA5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 6 of BLOCK8 (KEY4)."]
pub mod RD_KEY4_DATA6 {
    #[doc = "Stores the sixth 32 bits of KEY4."]
    pub mod KEY4_DATA6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 7 of BLOCK8 (KEY4)."]
pub mod RD_KEY4_DATA7 {
    #[doc = "Stores the seventh 32 bits of KEY4."]
    pub mod KEY4_DATA7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 0 of BLOCK9 (KEY5)."]
pub mod RD_KEY5_DATA0 {
    #[doc = "Stores the zeroth 32 bits of KEY5."]
    pub mod KEY5_DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 1 of BLOCK9 (KEY5)."]
pub mod RD_KEY5_DATA1 {
    #[doc = "Stores the first 32 bits of KEY5."]
    pub mod KEY5_DATA1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 2 of BLOCK9 (KEY5)."]
pub mod RD_KEY5_DATA2 {
    #[doc = "Stores the second 32 bits of KEY5."]
    pub mod KEY5_DATA2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 3 of BLOCK9 (KEY5)."]
pub mod RD_KEY5_DATA3 {
    #[doc = "Stores the third 32 bits of KEY5."]
    pub mod KEY5_DATA3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 4 of BLOCK9 (KEY5)."]
pub mod RD_KEY5_DATA4 {
    #[doc = "Stores the fourth 32 bits of KEY5."]
    pub mod KEY5_DATA4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 5 of BLOCK9 (KEY5)."]
pub mod RD_KEY5_DATA5 {
    #[doc = "Stores the fifth 32 bits of KEY5."]
    pub mod KEY5_DATA5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 6 of BLOCK9 (KEY5)."]
pub mod RD_KEY5_DATA6 {
    #[doc = "Stores the sixth 32 bits of KEY5."]
    pub mod KEY5_DATA6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 7 of BLOCK9 (KEY5)."]
pub mod RD_KEY5_DATA7 {
    #[doc = "Stores the seventh 32 bits of KEY5."]
    pub mod KEY5_DATA7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 0 of BLOCK10 (system)."]
pub mod RD_SYS_PART2_DATA0 {
    #[doc = "Stores the 0th 32 bits of the 2nd part of system data."]
    pub mod SYS_DATA_PART2_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 1 of BLOCK9 (KEY5)."]
pub mod RD_SYS_PART2_DATA1 {
    #[doc = "Stores the 1st 32 bits of the 2nd part of system data."]
    pub mod SYS_DATA_PART2_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 2 of BLOCK10 (system)."]
pub mod RD_SYS_PART2_DATA2 {
    #[doc = "Stores the 2nd 32 bits of the 2nd part of system data."]
    pub mod SYS_DATA_PART2_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 3 of BLOCK10 (system)."]
pub mod RD_SYS_PART2_DATA3 {
    #[doc = "Stores the 3rd 32 bits of the 2nd part of system data."]
    pub mod SYS_DATA_PART2_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 4 of BLOCK10 (system)."]
pub mod RD_SYS_PART2_DATA4 {
    #[doc = "Stores the 4th 32 bits of the 2nd part of system data."]
    pub mod SYS_DATA_PART2_4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 5 of BLOCK10 (system)."]
pub mod RD_SYS_PART2_DATA5 {
    #[doc = "Stores the 5th 32 bits of the 2nd part of system data."]
    pub mod SYS_DATA_PART2_5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 6 of BLOCK10 (system)."]
pub mod RD_SYS_PART2_DATA6 {
    #[doc = "Stores the 6th 32 bits of the 2nd part of system data."]
    pub mod SYS_DATA_PART2_6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Register 7 of BLOCK10 (system)."]
pub mod RD_SYS_PART2_DATA7 {
    #[doc = "Stores the 7th 32 bits of the 2nd part of system data."]
    pub mod SYS_DATA_PART2_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Programming error record register 0 of BLOCK0."]
pub mod RD_REPEAT_ERR0 {
    #[doc = "If any bit in RD_DIS is 1, then it indicates a programming error."]
    pub mod RD_DIS_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If DIS_RTC_RAM_BOOT is 1, then it indicates a programming error."]
    pub mod DIS_RTC_RAM_BOOT_ERR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If DIS_ICACHE is 1, then it indicates a programming error."]
    pub mod DIS_ICACHE_ERR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If DIS_USB_JTAG is 1, then it indicates a programming error."]
    pub mod DIS_USB_JTAG_ERR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If DIS_DOWNLOAD_ICACHE is 1, then it indicates a programming error."]
    pub mod DIS_DOWNLOAD_ICACHE_ERR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If DIS_USB_DEVICE is 1, then it indicates a programming error."]
    pub mod DIS_USB_DEVICE_ERR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If DIS_FORCE_DOWNLOAD is 1, then it indicates a programming error."]
    pub mod DIS_FORCE_DOWNLOAD_ERR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RPT4_RESERVED6_ERR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If DIS_CAN is 1, then it indicates a programming error."]
    pub mod DIS_CAN_ERR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If JTAG_SEL_ENABLE is 1, then it indicates a programming error."]
    pub mod JTAG_SEL_ENABLE_ERR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If SOFT_DIS_JTAG is 1, then it indicates a programming error."]
    pub mod SOFT_DIS_JTAG_ERR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If DIS_PAD_JTAG is 1, then it indicates a programming error."]
    pub mod DIS_PAD_JTAG_ERR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If DIS_DOWNLOAD_MANUAL_ENCRYPT is 1, then it indicates a programming error."]
    pub mod DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If any bit in USB_DREFH is 1, then it indicates a programming error."]
    pub mod USB_DREFH_ERR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If any bit in USB_DREFL is 1, then it indicates a programming error."]
    pub mod USB_DREFL_ERR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If USB_EXCHG_PINS is 1, then it indicates a programming error."]
    pub mod USB_EXCHG_PINS_ERR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If VDD_SPI_AS_GPIO is 1, then it indicates a programming error."]
    pub mod VDD_SPI_AS_GPIO_ERR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If any bit in BTLC_GPIO_ENABLE is 1, then it indicates a programming error."]
    pub mod BTLC_GPIO_ENABLE_ERR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If POWERGLITCH_EN is 1, then it indicates a programming error."]
    pub mod POWERGLITCH_EN_ERR {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If any bit in POWER_GLITCH_DSENSE is 1, then it indicates a programming error."]
    pub mod POWER_GLITCH_DSENSE_ERR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Programming error record register 1 of BLOCK0."]
pub mod RD_REPEAT_ERR1 {
    #[doc = "Reserved."]
    pub mod RPT4_RESERVED2_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If any bit in WDT_DELAY_SEL is 1, then it indicates a programming error."]
    pub mod WDT_DELAY_SEL_ERR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If any bit in SPI_BOOT_CRYPT_CNT is 1, then it indicates a programming error."]
    pub mod SPI_BOOT_CRYPT_CNT_ERR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If SECURE_BOOT_KEY_REVOKE0 is 1, then it indicates a programming error."]
    pub mod SECURE_BOOT_KEY_REVOKE0_ERR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If SECURE_BOOT_KEY_REVOKE1 is 1, then it indicates a programming error."]
    pub mod SECURE_BOOT_KEY_REVOKE1_ERR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If SECURE_BOOT_KEY_REVOKE2 is 1, then it indicates a programming error."]
    pub mod SECURE_BOOT_KEY_REVOKE2_ERR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If any bit in KEY_PURPOSE_0 is 1, then it indicates a programming error."]
    pub mod KEY_PURPOSE_0_ERR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If any bit in KEY_PURPOSE_1 is 1, then it indicates a programming error."]
    pub mod KEY_PURPOSE_1_ERR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Programming error record register 2 of BLOCK0."]
pub mod RD_REPEAT_ERR2 {
    #[doc = "If any bit in KEY_PURPOSE_2 is 1, then it indicates a programming error."]
    pub mod KEY_PURPOSE_2_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If any bit in KEY_PURPOSE_3 is 1, then it indicates a programming error."]
    pub mod KEY_PURPOSE_3_ERR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If any bit in KEY_PURPOSE_4 is 1, then it indicates a programming error."]
    pub mod KEY_PURPOSE_4_ERR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If any bit in KEY_PURPOSE_5 is 1, then it indicates a programming error."]
    pub mod KEY_PURPOSE_5_ERR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RPT4_RESERVED3_ERR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If SECURE_BOOT_EN is 1, then it indicates a programming error."]
    pub mod SECURE_BOOT_EN_ERR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If SECURE_BOOT_AGGRESSIVE_REVOKE is 1, then it indicates a programming error."]
    pub mod SECURE_BOOT_AGGRESSIVE_REVOKE_ERR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RPT4_RESERVED0_ERR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If any bit in FLASH_TPUM is 1, then it indicates a programming error."]
    pub mod FLASH_TPUW_ERR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Programming error record register 3 of BLOCK0."]
pub mod RD_REPEAT_ERR3 {
    #[doc = "If DIS_DOWNLOAD_MODE is 1, then it indicates a programming error."]
    pub mod DIS_DOWNLOAD_MODE_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If DIS_LEGACY_SPI_BOOT is 1, then it indicates a programming error."]
    pub mod DIS_LEGACY_SPI_BOOT_ERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If UART_PRINT_CHANNEL is 1, then it indicates a programming error."]
    pub mod UART_PRINT_CHANNEL_ERR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If FLASH_ECC_MODE is 1, then it indicates a programming error."]
    pub mod FLASH_ECC_MODE_ERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If DIS_USB_DOWNLOAD_MODE is 1, then it indicates a programming error."]
    pub mod DIS_USB_DOWNLOAD_MODE_ERR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If ENABLE_SECURITY_DOWNLOAD is 1, then it indicates a programming error."]
    pub mod ENABLE_SECURITY_DOWNLOAD_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If any bit in UART_PRINT_CONTROL is 1, then it indicates a programming error."]
    pub mod UART_PRINT_CONTROL_ERR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If PIN_POWER_SELECTION is 1, then it indicates a programming error."]
    pub mod PIN_POWER_SELECTION_ERR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If FLASH_TYPE is 1, then it indicates a programming error."]
    pub mod FLASH_TYPE_ERR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If any bits in FLASH_PAGE_SIZE is 1, then it indicates a programming error."]
    pub mod FLASH_PAGE_SIZE_ERR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If FLASH_ECC_EN_ERR is 1, then it indicates a programming error."]
    pub mod FLASH_ECC_EN_ERR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If FORCE_SEND_RESUME is 1, then it indicates a programming error."]
    pub mod FORCE_SEND_RESUME_ERR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If any bit in SECURE_VERSION is 1, then it indicates a programming error."]
    pub mod SECURE_VERSION_ERR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RPT4_RESERVED1_ERR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Programming error record register 4 of BLOCK0."]
pub mod RD_REPEAT_ERR4 {
    #[doc = "Reserved."]
    pub mod RPT4_RESERVED4_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Programming error record register 0 of BLOCK1-10."]
pub mod RD_RS_ERR0 {
    #[doc = "The value of this signal means the number of error bytes."]
    pub mod MAC_SPI_8M_ERR_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: Means no failure and that the data of MAC_SPI_8M is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
    pub mod MAC_SPI_8M_FAIL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The value of this signal means the number of error bytes."]
    pub mod SYS_PART1_NUM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: Means no failure and that the data of system part1 is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
    pub mod SYS_PART1_FAIL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The value of this signal means the number of error bytes."]
    pub mod USR_DATA_ERR_NUM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: Means no failure and that the user data is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
    pub mod USR_DATA_FAIL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The value of this signal means the number of error bytes."]
    pub mod KEY0_ERR_NUM {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: Means no failure and that the data of key0 is reliable 1: Means that programming key0 failed and the number of error bytes is over 6."]
    pub mod KEY0_FAIL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The value of this signal means the number of error bytes."]
    pub mod KEY1_ERR_NUM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: Means no failure and that the data of key1 is reliable 1: Means that programming key1 failed and the number of error bytes is over 6."]
    pub mod KEY1_FAIL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The value of this signal means the number of error bytes."]
    pub mod KEY2_ERR_NUM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: Means no failure and that the data of key2 is reliable 1: Means that programming key2 failed and the number of error bytes is over 6."]
    pub mod KEY2_FAIL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The value of this signal means the number of error bytes."]
    pub mod KEY3_ERR_NUM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: Means no failure and that the data of key3 is reliable 1: Means that programming key3 failed and the number of error bytes is over 6."]
    pub mod KEY3_FAIL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The value of this signal means the number of error bytes."]
    pub mod KEY4_ERR_NUM {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: Means no failure and that the data of key4 is reliable 1: Means that programming key4 failed and the number of error bytes is over 6."]
    pub mod KEY4_FAIL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Programming error record register 1 of BLOCK1-10."]
pub mod RD_RS_ERR1 {
    #[doc = "The value of this signal means the number of error bytes."]
    pub mod KEY5_ERR_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: Means no failure and that the data of KEY5 is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
    pub mod KEY5_FAIL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The value of this signal means the number of error bytes."]
    pub mod SYS_PART2_ERR_NUM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: Means no failure and that the data of system part2 is reliable 1: Means that programming user data failed and the number of error bytes is over 6."]
    pub mod SYS_PART2_FAIL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "eFuse clcok configuration register."]
pub mod CLK {
    #[doc = "Set this bit to force eFuse SRAM into power-saving mode."]
    pub mod EFUSE_MEM_FORCE_PD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit and force to activate clock signal of eFuse SRAM."]
    pub mod MEM_CLK_FORCE_ON {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to force eFuse SRAM into working mode."]
    pub mod EFUSE_MEM_FORCE_PU {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit and force to enable clock signal of eFuse memory."]
    pub mod EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "eFuse operation mode configuraiton register;"]
pub mod CONF {
    #[doc = "0x5A5A: Operate programming command 0x5AA5: Operate read command."]
    pub mod OP_CODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "eFuse status register."]
pub mod STATUS {
    #[doc = "Indicates the state of the eFuse state machine."]
    pub mod STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The value of OTP_LOAD_SW."]
    pub mod OTP_LOAD_SW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The value of OTP_VDDQ_C_SYNC2."]
    pub mod OTP_VDDQ_C_SYNC2 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The value of OTP_STROBE_SW."]
    pub mod OTP_STROBE_SW {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The value of OTP_CSB_SW."]
    pub mod OTP_CSB_SW {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The value of OTP_PGENB_SW."]
    pub mod OTP_PGENB_SW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The value of OTP_VDDQ_IS_SW."]
    pub mod OTP_VDDQ_IS_SW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the number of error bits during programming BLOCK0."]
    pub mod REPEAT_ERR_CNT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "eFuse command register."]
pub mod CMD {
    #[doc = "Set this bit to send read command."]
    pub mod READ_CMD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to send programming command."]
    pub mod PGM_CMD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The serial number of the block to be programmed. Value 0-10 corresponds to block number 0-10, respectively."]
    pub mod BLK_NUM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "eFuse raw interrupt register."]
pub mod INT_RAW {
    #[doc = "The raw bit signal for read_done interrupt."]
    pub mod READ_DONE_INT_RAW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit signal for pgm_done interrupt."]
    pub mod PGM_DONE_INT_RAW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "eFuse interrupt status register."]
pub mod INT_ST {
    #[doc = "The status signal for read_done interrupt."]
    pub mod READ_DONE_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status signal for pgm_done interrupt."]
    pub mod PGM_DONE_INT_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "eFuse interrupt enable register."]
pub mod INT_ENA {
    #[doc = "The enable signal for read_done interrupt."]
    pub mod READ_DONE_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable signal for pgm_done interrupt."]
    pub mod PGM_DONE_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "eFuse interrupt clear register."]
pub mod INT_CLR {
    #[doc = "The clear signal for read_done interrupt."]
    pub mod READ_DONE_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear signal for pgm_done interrupt."]
    pub mod PGM_DONE_INT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controls the eFuse programming voltage."]
pub mod DAC_CONF {
    #[doc = "Controls the division factor of the rising clock of the programming voltage."]
    pub mod DAC_CLK_DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Don't care."]
    pub mod DAC_CLK_PAD_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controls the rising period of the programming voltage."]
    pub mod DAC_NUM {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reduces the power supply of the programming voltage."]
    pub mod OE_CLR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configures read timing parameters."]
pub mod RD_TIM_CONF {
    #[doc = "Configures the initial read time of eFuse."]
    pub mod READ_INIT_NUM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configurarion register 1 of eFuse programming timing parameters."]
pub mod WR_TIM_CONF1 {
    #[doc = "Configures the power up time for VDDQ."]
    pub mod PWR_ON_NUM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configurarion register 2 of eFuse programming timing parameters."]
pub mod WR_TIM_CONF2 {
    #[doc = "Configures the power outage time for VDDQ."]
    pub mod PWR_OFF_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "eFuse version register."]
pub mod DATE {
    #[doc = "Stores eFuse version."]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
