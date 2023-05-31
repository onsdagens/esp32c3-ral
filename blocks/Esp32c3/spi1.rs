#[doc = "SPI (Serial Peripheral Interface) Controller 1"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "SPI1 memory command register"]
    pub CMD: crate::RWRegister<u32>,
    #[doc = "SPI1 address register"]
    pub ADDR: crate::RWRegister<u32>,
    #[doc = "SPI1 control register."]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "SPI1 control1 register."]
    pub CTRL1: crate::RWRegister<u32>,
    #[doc = "SPI1 control2 register."]
    pub CTRL2: crate::RWRegister<u32>,
    #[doc = "SPI1 clock division control register."]
    pub CLOCK: crate::RWRegister<u32>,
    #[doc = "SPI1 user register."]
    pub USER: crate::RWRegister<u32>,
    #[doc = "SPI1 user1 register."]
    pub USER1: crate::RWRegister<u32>,
    #[doc = "SPI1 user2 register."]
    pub USER2: crate::RWRegister<u32>,
    #[doc = "SPI1 send data bit length control register."]
    pub MOSI_DLEN: crate::RWRegister<u32>,
    #[doc = "SPI1 receive data bit length control register."]
    pub MISO_DLEN: crate::RWRegister<u32>,
    #[doc = "SPI1 status register."]
    pub RD_STATUS: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "SPI1 misc register"]
    pub MISC: crate::RWRegister<u32>,
    #[doc = "SPI1 TX CRC data register."]
    pub TX_CRC: crate::RWRegister<u32>,
    #[doc = "SPI1 bit mode control register."]
    pub CACHE_FCTRL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x18],
    #[doc = "SPI1 memory data buffer0"]
    pub W0: crate::RWRegister<u32>,
    #[doc = "SPI1 memory data buffer1"]
    pub W1: crate::RWRegister<u32>,
    #[doc = "SPI1 memory data buffer2"]
    pub W2: crate::RWRegister<u32>,
    #[doc = "SPI1 memory data buffer3"]
    pub W3: crate::RWRegister<u32>,
    #[doc = "SPI1 memory data buffer4"]
    pub W4: crate::RWRegister<u32>,
    #[doc = "SPI1 memory data buffer5"]
    pub W5: crate::RWRegister<u32>,
    #[doc = "SPI1 memory data buffer6"]
    pub W6: crate::RWRegister<u32>,
    #[doc = "SPI1 memory data buffer7"]
    pub W7: crate::RWRegister<u32>,
    #[doc = "SPI1 memory data buffer8"]
    pub W8: crate::RWRegister<u32>,
    #[doc = "SPI1 memory data buffer9"]
    pub W9: crate::RWRegister<u32>,
    #[doc = "SPI1 memory data buffer10"]
    pub W10: crate::RWRegister<u32>,
    #[doc = "SPI1 memory data buffer11"]
    pub W11: crate::RWRegister<u32>,
    #[doc = "SPI1 memory data buffer12"]
    pub W12: crate::RWRegister<u32>,
    #[doc = "SPI1 memory data buffer13"]
    pub W13: crate::RWRegister<u32>,
    #[doc = "SPI1 memory data buffer14"]
    pub W14: crate::RWRegister<u32>,
    #[doc = "SPI1 memory data buffer15"]
    pub W15: crate::RWRegister<u32>,
    #[doc = "SPI1 wait idle control register"]
    pub FLASH_WAITI_CTRL: crate::RWRegister<u32>,
    #[doc = "SPI1 flash suspend control register"]
    pub FLASH_SUS_CTRL: crate::RWRegister<u32>,
    #[doc = "SPI1 flash suspend command register"]
    pub FLASH_SUS_CMD: crate::RWRegister<u32>,
    #[doc = "SPI1 flash suspend status register"]
    pub SUS_STATUS: crate::RWRegister<u32>,
    #[doc = "SPI1 timing control register"]
    pub TIMING_CALI: crate::RWRegister<u32>,
    _reserved2: [u8; 0x14],
    #[doc = "SPI1 interrupt enable register"]
    pub INT_ENA: crate::RWRegister<u32>,
    #[doc = "SPI1 interrupt clear register"]
    pub INT_CLR: crate::RWRegister<u32>,
    #[doc = "SPI1 interrupt raw register"]
    pub INT_RAW: crate::RWRegister<u32>,
    #[doc = "SPI1 interrupt status register"]
    pub INT_ST: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "SPI1 clk_gate register"]
    pub CLOCK_GATE: crate::RWRegister<u32>,
    _reserved4: [u8; 0x031c],
    #[doc = "Version control register"]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "SPI1 memory command register"]
pub mod CMD {
    #[doc = "The current status of SPI1 master FSM."]
    pub mod SPI1_MST_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The current status of SPI1 slave FSM: mspi_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
    pub mod MSPI_ST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In user mode, it is set to indicate that program/erase operation will be triggered. The bit is combined with spi_mem_usr bit. The bit will be cleared once the operation done.1: enable 0: disable."]
    pub mod FLASH_PE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    pub mod USR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Drive Flash into high performance mode. The bit will be cleared once the operation done.1: enable 0: disable."]
    pub mod FLASH_HPM {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit combined with reg_resandres bit releases Flash from the power-down state or high performance mode and obtains the devices ID. The bit will be cleared once the operation done.1: enable 0: disable."]
    pub mod FLASH_RES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Drive Flash into power down. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    pub mod FLASH_DP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Chip erase enable. Chip erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    pub mod FLASH_CE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Block erase enable(32KB) . Block erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    pub mod FLASH_BE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sector erase enable(4KB). Sector erase operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    pub mod FLASH_SE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Page program enable(1 byte ~256 bytes data to be programmed). Page program operation will be triggered when the bit is set. The bit will be cleared once the operation done .1: enable 0: disable."]
    pub mod FLASH_PP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write status register enable. Write status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    pub mod FLASH_WRSR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read status register-1. Read status operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    pub mod FLASH_RDSR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read JEDEC ID . Read ID command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    pub mod FLASH_RDID {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write flash disable. Write disable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    pub mod FLASH_WRDI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write flash enable. Write enable command will be sent when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    pub mod FLASH_WREN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read flash enable. Read flash operation will be triggered when the bit is set. The bit will be cleared once the operation done. 1: enable 0: disable."]
    pub mod FLASH_READ {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 address register"]
pub mod ADDR {
    #[doc = "In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer."]
    pub mod USR_ADDR_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 control register."]
pub mod CTRL {
    #[doc = "In the dummy phase the signal level of spi is output by the spi controller."]
    pub mod FDUMMY_OUT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Apply 2 signals during command phase 1:enable 0: disable"]
    pub mod FCMD_DUAL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Apply 4 signals during command phase 1:enable 0: disable"]
    pub mod FCMD_QUAD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI1, initialize crc32 module before writing encrypted data to flash. Active low."]
    pub mod FCS_CRC_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
    pub mod TX_CRC_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
    pub mod FASTRD_MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
    pub mod FREAD_DUAL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
    pub mod RESANDRES {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to set MISO line polarity, 1: high 0, low"]
    pub mod Q_POL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to set MOSI line polarity, 1: high 0, low"]
    pub mod D_POL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
    pub mod FREAD_QUAD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write protect signal output when SPI is idle. 1: output high, 0: output low."]
    pub mod WP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "two bytes data will be written to status register when it is set. 1: enable 0: disable."]
    pub mod WRSR_2B {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
    pub mod FREAD_DIO {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
    pub mod FREAD_QIO {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 control1 register."]
pub mod CTRL1 {
    #[doc = "SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    pub mod CLK_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 512) SPI_CLK cycles."]
    pub mod CS_HOLD_DLY_RES {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 control2 register."]
pub mod CTRL2 {
    #[doc = "The FSM will be reset."]
    pub mod SYNC_RESET {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 clock division control register."]
pub mod CLOCK {
    #[doc = "In the master mode it must be equal to spi_mem_clkcnt_N."]
    pub mod CLKCNT_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the master mode it must be floor((spi_mem_clkcnt_N+1)/2-1)."]
    pub mod CLKCNT_H {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(spi_mem_clkcnt_N+1)"]
    pub mod CLKCNT_N {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod CLK_EQU_SYSCLK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 user register."]
pub mod USER {
    #[doc = "the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode."]
    pub mod CK_OUT_EDGE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the write operations read-data phase apply 2 signals"]
    pub mod FWRITE_DUAL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the write operations read-data phase apply 4 signals"]
    pub mod FWRITE_QUAD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the write operations address phase and read-data phase apply 2 signals."]
    pub mod FWRITE_DIO {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the write operations address phase and read-data phase apply 4 signals."]
    pub mod FWRITE_QIO {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "read-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable."]
    pub mod USR_MISO_HIGHPART {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "write-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable."]
    pub mod USR_MOSI_HIGHPART {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPI clock is disable in dummy phase when the bit is enable."]
    pub mod USR_DUMMY_IDLE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the write-data phase of an operation."]
    pub mod USR_MOSI {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the read-data phase of an operation."]
    pub mod USR_MISO {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the dummy phase of an operation."]
    pub mod USR_DUMMY {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the address phase of an operation."]
    pub mod USR_ADDR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the command phase of an operation."]
    pub mod USR_COMMAND {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 user1 register."]
pub mod USER1 {
    #[doc = "The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
    pub mod USR_DUMMY_CYCLELEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The length in bits of address phase. The register value shall be (bit_num-1)."]
    pub mod USR_ADDR_BITLEN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 user2 register."]
pub mod USER2 {
    #[doc = "The value of command."]
    pub mod USR_COMMAND_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The length in bits of command phase. The register value shall be (bit_num-1)"]
    pub mod USR_COMMAND_BITLEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 send data bit length control register."]
pub mod MOSI_DLEN {
    #[doc = "The length in bits of write-data. The register value shall be (bit_num-1)."]
    pub mod USR_MOSI_DBITLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 receive data bit length control register."]
pub mod MISO_DLEN {
    #[doc = "The length in bits of read-data. The register value shall be (bit_num-1)."]
    pub mod USR_MISO_DBITLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 status register."]
pub mod RD_STATUS {
    #[doc = "The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit."]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
    pub mod WB_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 misc register"]
pub mod MISC {
    #[doc = "SPI_CS0 pin enable, 1: disable SPI_CS0, 0: SPI_CS0 pin is active to select SPI device, such as flash, external RAM and so on."]
    pub mod CS0_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPI_CS1 pin enable, 1: disable SPI_CS1, 0: SPI_CS1 pin is active to select SPI device, such as flash, external RAM and so on."]
    pub mod CS1_DIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: spi clk line is high when idle 0: spi clk line is low when idle"]
    pub mod CK_IDLE_EDGE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "spi cs line keep low when the bit is set."]
    pub mod CS_KEEP_ACTIVE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 TX CRC data register."]
pub mod TX_CRC {
    #[doc = "For SPI1, the value of crc32."]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 bit mode control register."]
pub mod CACHE_FCTRL {
    #[doc = "For SPI1, cache read flash with 4 bytes address, 1: enable, 0:disable."]
    pub mod CACHE_USR_ADDR_4BYTE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI1, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    pub mod FDIN_DUAL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI1, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    pub mod FDOUT_DUAL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI1, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    pub mod FADDR_DUAL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI1, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    pub mod FDIN_QUAD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI1, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    pub mod FDOUT_QUAD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI1, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    pub mod FADDR_QUAD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer0"]
pub mod W0 {
    #[doc = "data buffer"]
    pub mod BUF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer1"]
pub mod W1 {
    #[doc = "data buffer"]
    pub mod BUF1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer2"]
pub mod W2 {
    #[doc = "data buffer"]
    pub mod BUF2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer3"]
pub mod W3 {
    #[doc = "data buffer"]
    pub mod BUF3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer4"]
pub mod W4 {
    #[doc = "data buffer"]
    pub mod BUF4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer5"]
pub mod W5 {
    #[doc = "data buffer"]
    pub mod BUF5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer6"]
pub mod W6 {
    #[doc = "data buffer"]
    pub mod BUF6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer7"]
pub mod W7 {
    #[doc = "data buffer"]
    pub mod BUF7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer8"]
pub mod W8 {
    #[doc = "data buffer"]
    pub mod BUF8 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer9"]
pub mod W9 {
    #[doc = "data buffer"]
    pub mod BUF9 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer10"]
pub mod W10 {
    #[doc = "data buffer"]
    pub mod BUF10 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer11"]
pub mod W11 {
    #[doc = "data buffer"]
    pub mod BUF11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer12"]
pub mod W12 {
    #[doc = "data buffer"]
    pub mod BUF12 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer13"]
pub mod W13 {
    #[doc = "data buffer"]
    pub mod BUF13 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer14"]
pub mod W14 {
    #[doc = "data buffer"]
    pub mod BUF14 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 memory data buffer15"]
pub mod W15 {
    #[doc = "data buffer"]
    pub mod BUF15 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 wait idle control register"]
pub mod FLASH_WAITI_CTRL {
    #[doc = "The dummy phase enable when wait flash idle (RDSR)"]
    pub mod WAITI_DUMMY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The command to wait flash idle(RDSR)."]
    pub mod WAITI_CMD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The dummy cycle length when wait flash idle(RDSR)."]
    pub mod WAITI_DUMMY_CYCLELEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 flash suspend control register"]
pub mod FLASH_SUS_CTRL {
    #[doc = "program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    pub mod FLASH_PER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    pub mod FLASH_PES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent."]
    pub mod FLASH_PER_WAIT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent."]
    pub mod FLASH_PES_WAIT_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done."]
    pub mod PES_PER_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable Auto-suspending function."]
    pub mod FLASH_PES_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in\\[15:0\\](only status_in\\[7:0\\] is valid when only one byte of data is read out, status_in\\[15:0\\] is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in\\[15:0\\]^ SPI_MEM_PESR_END_MSK\\[15:0\\]."]
    pub mod PESR_END_MSK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit"]
    pub mod RD_SUS_2B {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0."]
    pub mod PER_END_EN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0."]
    pub mod PES_END_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\] times, it will be treated as check pass."]
    pub mod SUS_TIMEOUT_CNT {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 flash suspend command register"]
pub mod FLASH_SUS_CMD {
    #[doc = "Program/Erase resume command."]
    pub mod FLASH_PER_COMMAND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Program/Erase suspend command."]
    pub mod FLASH_PES_COMMAND {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash SUS/SUS1/SUS2 status bit read command. The command should be sent when SUS/SUS1/SUS2 bit should be checked to insure the suspend or resume status of flash."]
    pub mod WAIT_PESR_COMMAND {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 flash suspend status register"]
pub mod SUS_STATUS {
    #[doc = "The status of flash suspend, only used in SPI1."]
    pub mod FLASH_SUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[15:0\\] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\\[7:0\\] to check SUS/SUS1/SUS2 bit."]
    pub mod WAIT_PESR_CMD_2B {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after HPM command is sent."]
    pub mod FLASH_HPM_DLY_128 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after RES command is sent."]
    pub mod FLASH_RES_DLY_128 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after DP command is sent."]
    pub mod FLASH_DP_DLY_128 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PER command is sent."]
    pub mod FLASH_PER_DLY_128 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4) SPI_CLK cycles after PES command is sent."]
    pub mod FLASH_PES_DLY_128 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it."]
    pub mod SPI0_LOCK_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 timing control register"]
pub mod TIMING_CALI {
    #[doc = "The bit is used to enable timing auto-calibration for all reading operations."]
    pub mod TIMING_CALI {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "add extra dummy spi clock cycle length for spi clock calibration."]
    pub mod EXTRA_DUMMY_CYCLELEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 interrupt enable register"]
pub mod INT_ENA {
    #[doc = "The enable bit for SPI_MEM_PER_END_INT interrupt."]
    pub mod PER_END_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_MEM_PES_END_INT interrupt."]
    pub mod PES_END_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_MEM_WPE_END_INT interrupt."]
    pub mod WPE_END_INT_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    pub mod SLV_ST_END_INT_ENA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
    pub mod MST_ST_END_INT_ENA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 interrupt clear register"]
pub mod INT_CLR {
    #[doc = "The clear bit for SPI_MEM_PER_END_INT interrupt."]
    pub mod PER_END_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_MEM_PES_END_INT interrupt."]
    pub mod PES_END_INT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_MEM_WPE_END_INT interrupt."]
    pub mod WPE_END_INT_CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    pub mod SLV_ST_END_INT_CLR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
    pub mod MST_ST_END_INT_CLR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 interrupt raw register"]
pub mod INT_RAW {
    #[doc = "The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed. 0: Others."]
    pub mod PER_END_INT_RAW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended. 0: Others."]
    pub mod PES_END_INT_RAW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
    pub mod WPE_END_INT_RAW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    pub mod SLV_ST_END_INT_RAW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
    pub mod MST_ST_END_INT_RAW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 interrupt status register"]
pub mod INT_ST {
    #[doc = "The status bit for SPI_MEM_PER_END_INT interrupt."]
    pub mod PER_END_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_MEM_PES_END_INT interrupt."]
    pub mod PES_END_INT_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_MEM_WPE_END_INT interrupt."]
    pub mod WPE_END_INT_ST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    pub mod SLV_ST_END_INT_ST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_MEM_MST_ST_END_INT interrupt."]
    pub mod MST_ST_END_INT_ST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI1 clk_gate register"]
pub mod CLOCK_GATE {
    #[doc = "Register clock gate enable signal. 1: Enable. 0: Disable."]
    pub mod CLK_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Version control register"]
pub mod DATE {
    #[doc = "Version control register"]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
