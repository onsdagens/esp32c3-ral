#[doc = "SPI (Serial Peripheral Interface) Controller 2"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Command control register"]
    pub CMD: crate::RWRegister<u32>,
    #[doc = "Address value register"]
    pub ADDR: crate::RWRegister<u32>,
    #[doc = "SPI control register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "SPI clock control register"]
    pub CLOCK: crate::RWRegister<u32>,
    #[doc = "SPI USER control register"]
    pub USER: crate::RWRegister<u32>,
    #[doc = "SPI USER control register 1"]
    pub USER1: crate::RWRegister<u32>,
    #[doc = "SPI USER control register 2"]
    pub USER2: crate::RWRegister<u32>,
    #[doc = "SPI data bit length control register"]
    pub MS_DLEN: crate::RWRegister<u32>,
    #[doc = "SPI misc register"]
    pub MISC: crate::RWRegister<u32>,
    #[doc = "SPI input delay mode configuration"]
    pub DIN_MODE: crate::RWRegister<u32>,
    #[doc = "SPI input delay number configuration"]
    pub DIN_NUM: crate::RWRegister<u32>,
    #[doc = "SPI output delay mode configuration"]
    pub DOUT_MODE: crate::RWRegister<u32>,
    #[doc = "SPI DMA control register"]
    pub DMA_CONF: crate::RWRegister<u32>,
    #[doc = "SPI DMA interrupt enable register"]
    pub DMA_INT_ENA: crate::RWRegister<u32>,
    #[doc = "SPI DMA interrupt clear register"]
    pub DMA_INT_CLR: crate::RWRegister<u32>,
    #[doc = "SPI DMA interrupt raw register"]
    pub DMA_INT_RAW: crate::RWRegister<u32>,
    #[doc = "SPI DMA interrupt status register"]
    pub DMA_INT_ST: crate::RWRegister<u32>,
    _reserved0: [u8; 0x54],
    #[doc = "SPI CPU-controlled buffer0"]
    pub W0: crate::RWRegister<u32>,
    #[doc = "SPI CPU-controlled buffer1"]
    pub W1: crate::RWRegister<u32>,
    #[doc = "SPI CPU-controlled buffer2"]
    pub W2: crate::RWRegister<u32>,
    #[doc = "SPI CPU-controlled buffer3"]
    pub W3: crate::RWRegister<u32>,
    #[doc = "SPI CPU-controlled buffer4"]
    pub W4: crate::RWRegister<u32>,
    #[doc = "SPI CPU-controlled buffer5"]
    pub W5: crate::RWRegister<u32>,
    #[doc = "SPI CPU-controlled buffer6"]
    pub W6: crate::RWRegister<u32>,
    #[doc = "SPI CPU-controlled buffer7"]
    pub W7: crate::RWRegister<u32>,
    #[doc = "SPI CPU-controlled buffer8"]
    pub W8: crate::RWRegister<u32>,
    #[doc = "SPI CPU-controlled buffer9"]
    pub W9: crate::RWRegister<u32>,
    #[doc = "SPI CPU-controlled buffer10"]
    pub W10: crate::RWRegister<u32>,
    #[doc = "SPI CPU-controlled buffer11"]
    pub W11: crate::RWRegister<u32>,
    #[doc = "SPI CPU-controlled buffer12"]
    pub W12: crate::RWRegister<u32>,
    #[doc = "SPI CPU-controlled buffer13"]
    pub W13: crate::RWRegister<u32>,
    #[doc = "SPI CPU-controlled buffer14"]
    pub W14: crate::RWRegister<u32>,
    #[doc = "SPI CPU-controlled buffer15"]
    pub W15: crate::RWRegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "SPI slave control register"]
    pub SLAVE: crate::RWRegister<u32>,
    #[doc = "SPI slave control register 1"]
    pub SLAVE1: crate::RWRegister<u32>,
    #[doc = "SPI module clock and register clock control"]
    pub CLK_GATE: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "Version control"]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "Command control register"]
pub mod CMD {
    #[doc = "Define the APB cycles of SPI_CONF state. Can be configured in CONF state."]
    pub mod CONF_BITLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to synchronize SPI registers from APB clock domain into SPI module clock domain, which is only used in SPI master mode."]
    pub mod UPDATE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "User define command enable. An operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable. Can not be changed by CONF_buf."]
    pub mod USR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Address value register"]
pub mod ADDR {
    #[doc = "Address to slave. Can be configured in CONF state."]
    pub mod USR_ADDR_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI control register"]
pub mod CTRL {
    #[doc = "In the dummy phase the signal level of spi is output by the spi controller. Can be configured in CONF state."]
    pub mod DUMMY_OUT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Apply 2 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    pub mod FADDR_DUAL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Apply 4 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    pub mod FADDR_QUAD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Apply 2 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
    pub mod FCMD_DUAL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Apply 4 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
    pub mod FCMD_QUAD {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the read operations, read-data phase apply 2 signals. 1: enable 0: disable. Can be configured in CONF state."]
    pub mod FREAD_DUAL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the read operations read-data phase apply 4 signals. 1: enable 0: disable. Can be configured in CONF state."]
    pub mod FREAD_QUAD {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
    pub mod Q_POL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
    pub mod D_POL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPI_HOLD output value when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
    pub mod HOLD_POL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
    pub mod WP_POL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
    pub mod RD_BIT_ORDER {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
    pub mod WR_BIT_ORDER {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI clock control register"]
pub mod CLOCK {
    #[doc = "In the master mode it must be equal to spi_clkcnt_N. In the slave mode it must be 0. Can be configured in CONF state."]
    pub mod CLKCNT_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the master mode it must be floor((spi_clkcnt_N+1)/2-1). In the slave mode it must be 0. Can be configured in CONF state."]
    pub mod CLKCNT_H {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the master mode it is the divider of spi_clk. So spi_clk frequency is system/(spi_clkdiv_pre+1)/(spi_clkcnt_N+1). Can be configured in CONF state."]
    pub mod CLKCNT_N {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the master mode it is pre-divider of spi_clk. Can be configured in CONF state."]
    pub mod CLKDIV_PRE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the master mode 1: spi_clk is eqaul to system 0: spi_clk is divided from system clock. Can be configured in CONF state."]
    pub mod CLK_EQU_SYSCLK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI USER control register"]
pub mod USER {
    #[doc = "Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
    pub mod DOUTDIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state."]
    pub mod QPI_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
    pub mod TSCK_I_EDGE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
    pub mod CS_HOLD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
    pub mod CS_SETUP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
    pub mod RSCK_I_EDGE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state."]
    pub mod CK_OUT_EDGE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the write operations read-data phase apply 2 signals. Can be configured in CONF state."]
    pub mod FWRITE_DUAL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the write operations read-data phase apply 4 signals. Can be configured in CONF state."]
    pub mod FWRITE_QUAD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state."]
    pub mod USR_CONF_NXT {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
    pub mod SIO {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
    pub mod USR_MISO_HIGHPART {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
    pub mod USR_MOSI_HIGHPART {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
    pub mod USR_DUMMY_IDLE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the write-data phase of an operation. Can be configured in CONF state."]
    pub mod USR_MOSI {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the read-data phase of an operation. Can be configured in CONF state."]
    pub mod USR_MISO {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the dummy phase of an operation. Can be configured in CONF state."]
    pub mod USR_DUMMY {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the address phase of an operation. Can be configured in CONF state."]
    pub mod USR_ADDR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the command phase of an operation. Can be configured in CONF state."]
    pub mod USR_COMMAND {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI USER control register 1"]
pub mod USER1 {
    #[doc = "The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
    pub mod USR_DUMMY_CYCLELEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
    pub mod MST_WFULL_ERR_END_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "(cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
    pub mod CS_SETUP_TIME {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
    pub mod CS_HOLD_TIME {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
    pub mod USR_ADDR_BITLEN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI USER control register 2"]
pub mod USER2 {
    #[doc = "The value of command. Can be configured in CONF state."]
    pub mod USR_COMMAND_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: SPI transfer is ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI TX AFIFO read empty error is valid in GP-SPI master FD/HD-mode."]
    pub mod MST_REMPTY_ERR_END_EN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The length in bits of command phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
    pub mod USR_COMMAND_BITLEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI data bit length control register"]
pub mod MS_DLEN {
    #[doc = "The value of these bits is the configured SPI transmission data bit length in master mode DMA controlled transfer or CPU controlled transfer. The value is also the configured bit length in slave mode DMA RX controlled transfer. The register value shall be (bit_num-1). Can be configured in CONF state."]
    pub mod MS_DATA_BITLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI misc register"]
pub mod MISC {
    #[doc = "SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state."]
    pub mod CS0_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state."]
    pub mod CS1_DIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state."]
    pub mod CS2_DIS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPI CS3 pin enable, 1: disable CS3, 0: spi_cs3 signal is from/to CS3 pin. Can be configured in CONF state."]
    pub mod CS3_DIS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPI CS4 pin enable, 1: disable CS4, 0: spi_cs4 signal is from/to CS4 pin. Can be configured in CONF state."]
    pub mod CS4_DIS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPI CS5 pin enable, 1: disable CS5, 0: spi_cs5 signal is from/to CS5 pin. Can be configured in CONF state."]
    pub mod CS5_DIS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
    pub mod CK_DIS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
    pub mod MASTER_CS_POL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
    pub mod SLAVE_CS_POL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
    pub mod CK_IDLE_EDGE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "spi cs line keep low when the bit is set. Can be configured in CONF state."]
    pub mod CS_KEEP_ACTIVE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
    pub mod QUAD_DIN_PIN_SWAP {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI input delay mode configuration"]
pub mod DIN_MODE {
    #[doc = "the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    pub mod DIN0_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    pub mod DIN1_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    pub mod DIN2_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the input signals are delayed by SPI module clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the spi_clk. Can be configured in CONF state."]
    pub mod DIN3_MODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1:enable hclk in SPI input timing module. 0: disable it. Can be configured in CONF state."]
    pub mod TIMING_HCLK_ACTIVE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI input delay number configuration"]
pub mod DIN_NUM {
    #[doc = "the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    pub mod DIN0_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    pub mod DIN1_NUM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    pub mod DIN2_NUM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    pub mod DIN3_NUM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI output delay mode configuration"]
pub mod DOUT_MODE {
    #[doc = "The output signal 0 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    pub mod DOUT0_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The output signal 1 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    pub mod DOUT1_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The output signal 2 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    pub mod DOUT2_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The output signal 3 is delayed by the SPI module clock, 0: output without delayed, 1: output delay for a SPI module clock cycle at its negative edge. Can be configured in CONF state."]
    pub mod DOUT3_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI DMA control register"]
pub mod DMA_CONF {
    #[doc = "Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
    pub mod DMA_SLV_SEG_TRANS_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: spi_dma_infifo_full_vld is cleared by spi slave cmd 5. 0: spi_dma_infifo_full_vld is cleared by spi_trans_done."]
    pub mod SLV_RX_SEG_TRANS_CLR_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: spi_dma_outfifo_empty_vld is cleared by spi slave cmd 6. 0: spi_dma_outfifo_empty_vld is cleared by spi_trans_done."]
    pub mod SLV_TX_SEG_TRANS_CLR_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: spi_dma_inlink_eof is set when the number of dma pushed data bytes is equal to the value of spi_slv/mst_dma_rd_bytelen\\[19:0\\] in spi dma transition. 0: spi_dma_inlink_eof is set by spi_trans_done in non-seg-trans or spi_dma_seg_trans_done in seg-trans."]
    pub mod RX_EOF_EN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable SPI DMA controlled receive data mode."]
    pub mod DMA_RX_ENA {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable SPI DMA controlled send data mode."]
    pub mod DMA_TX_ENA {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to reset RX AFIFO, which is used to receive data in SPI master and slave mode transfer."]
    pub mod RX_AFIFO_RST {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to reset BUF TX AFIFO, which is used send data out in SPI slave CPU controlled mode transfer and master mode transfer."]
    pub mod BUF_AFIFO_RST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to reset DMA TX AFIFO, which is used to send data out in SPI slave DMA controlled mode transfer."]
    pub mod DMA_AFIFO_RST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI DMA interrupt enable register"]
pub mod DMA_INT_ENA {
    #[doc = "The enable bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    pub mod DMA_INFIFO_FULL_ERR_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    pub mod DMA_OUTFIFO_EMPTY_ERR_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI slave Ex_QPI interrupt."]
    pub mod SLV_EX_QPI_INT_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI slave En_QPI interrupt."]
    pub mod SLV_EN_QPI_INT_ENA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI slave CMD7 interrupt."]
    pub mod SLV_CMD7_INT_ENA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI slave CMD8 interrupt."]
    pub mod SLV_CMD8_INT_ENA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI slave CMD9 interrupt."]
    pub mod SLV_CMD9_INT_ENA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI slave CMDA interrupt."]
    pub mod SLV_CMDA_INT_ENA {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
    pub mod SLV_RD_DMA_DONE_INT_ENA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
    pub mod SLV_WR_DMA_DONE_INT_ENA {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    pub mod SLV_RD_BUF_DONE_INT_ENA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    pub mod SLV_WR_BUF_DONE_INT_ENA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_TRANS_DONE_INT interrupt."]
    pub mod TRANS_DONE_INT_ENA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    pub mod DMA_SEG_TRANS_DONE_INT_ENA {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
    pub mod SEG_MAGIC_ERR_INT_ENA {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    pub mod SLV_BUF_ADDR_ERR_INT_ENA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_SLV_CMD_ERR_INT interrupt."]
    pub mod SLV_CMD_ERR_INT_ENA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    pub mod MST_RX_AFIFO_WFULL_ERR_INT_ENA {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    pub mod MST_TX_AFIFO_REMPTY_ERR_INT_ENA {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_APP2_INT interrupt."]
    pub mod APP2_INT_ENA {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for SPI_APP1_INT interrupt."]
    pub mod APP1_INT_ENA {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI DMA interrupt clear register"]
pub mod DMA_INT_CLR {
    #[doc = "The clear bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    pub mod DMA_INFIFO_FULL_ERR_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    pub mod DMA_OUTFIFO_EMPTY_ERR_INT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI slave Ex_QPI interrupt."]
    pub mod SLV_EX_QPI_INT_CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI slave En_QPI interrupt."]
    pub mod SLV_EN_QPI_INT_CLR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI slave CMD7 interrupt."]
    pub mod SLV_CMD7_INT_CLR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI slave CMD8 interrupt."]
    pub mod SLV_CMD8_INT_CLR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI slave CMD9 interrupt."]
    pub mod SLV_CMD9_INT_CLR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI slave CMDA interrupt."]
    pub mod SLV_CMDA_INT_CLR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
    pub mod SLV_RD_DMA_DONE_INT_CLR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
    pub mod SLV_WR_DMA_DONE_INT_CLR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    pub mod SLV_RD_BUF_DONE_INT_CLR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    pub mod SLV_WR_BUF_DONE_INT_CLR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_TRANS_DONE_INT interrupt."]
    pub mod TRANS_DONE_INT_CLR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    pub mod DMA_SEG_TRANS_DONE_INT_CLR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
    pub mod SEG_MAGIC_ERR_INT_CLR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    pub mod SLV_BUF_ADDR_ERR_INT_CLR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_SLV_CMD_ERR_INT interrupt."]
    pub mod SLV_CMD_ERR_INT_CLR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    pub mod MST_RX_AFIFO_WFULL_ERR_INT_CLR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    pub mod MST_TX_AFIFO_REMPTY_ERR_INT_CLR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_APP2_INT interrupt."]
    pub mod APP2_INT_CLR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clear bit for SPI_APP1_INT interrupt."]
    pub mod APP1_INT_CLR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI DMA interrupt raw register"]
pub mod DMA_INT_RAW {
    #[doc = "1: The current data rate of DMA Rx is smaller than that of SPI, which will lose the receive data. 0: Others."]
    pub mod DMA_INFIFO_FULL_ERR_INT_RAW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: The current data rate of DMA TX is smaller than that of SPI. SPI will stop in master mode and send out all 0 in slave mode. 0: Others."]
    pub mod DMA_OUTFIFO_EMPTY_ERR_INT_RAW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI slave Ex_QPI interrupt. 1: SPI slave mode Ex_QPI transmission is ended. 0: Others."]
    pub mod SLV_EX_QPI_INT_RAW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI slave En_QPI interrupt. 1: SPI slave mode En_QPI transmission is ended. 0: Others."]
    pub mod SLV_EN_QPI_INT_RAW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI slave CMD7 interrupt. 1: SPI slave mode CMD7 transmission is ended. 0: Others."]
    pub mod SLV_CMD7_INT_RAW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI slave CMD8 interrupt. 1: SPI slave mode CMD8 transmission is ended. 0: Others."]
    pub mod SLV_CMD8_INT_RAW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI slave CMD9 interrupt. 1: SPI slave mode CMD9 transmission is ended. 0: Others."]
    pub mod SLV_CMD9_INT_RAW {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI slave CMDA interrupt. 1: SPI slave mode CMDA transmission is ended. 0: Others."]
    pub mod SLV_CMDA_INT_RAW {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_SLV_RD_DMA_DONE_INT interrupt. 1: SPI slave mode Rd_DMA transmission is ended. 0: Others."]
    pub mod SLV_RD_DMA_DONE_INT_RAW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_SLV_WR_DMA_DONE_INT interrupt. 1: SPI slave mode Wr_DMA transmission is ended. 0: Others."]
    pub mod SLV_WR_DMA_DONE_INT_RAW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_SLV_RD_BUF_DONE_INT interrupt. 1: SPI slave mode Rd_BUF transmission is ended. 0: Others."]
    pub mod SLV_RD_BUF_DONE_INT_RAW {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_SLV_WR_BUF_DONE_INT interrupt. 1: SPI slave mode Wr_BUF transmission is ended. 0: Others."]
    pub mod SLV_WR_BUF_DONE_INT_RAW {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_TRANS_DONE_INT interrupt. 1: SPI master mode transmission is ended. 0: others."]
    pub mod TRANS_DONE_INT_RAW {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt. 1: spi master DMA full-duplex/half-duplex seg-conf-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-conf-trans or seg-trans is not ended or not occurred."]
    pub mod DMA_SEG_TRANS_DONE_INT_RAW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_SEG_MAGIC_ERR_INT interrupt. 1: The magic value in CONF buffer is error in the DMA seg-conf-trans. 0: others."]
    pub mod SEG_MAGIC_ERR_INT_RAW {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt. 1: The accessing data address of the current SPI slave mode CPU controlled FD, Wr_BUF or Rd_BUF transmission is bigger than 63. 0: Others."]
    pub mod SLV_BUF_ADDR_ERR_INT_RAW {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_SLV_CMD_ERR_INT interrupt. 1: The slave command value in the current SPI slave HD mode transmission is not supported. 0: Others."]
    pub mod SLV_CMD_ERR_INT_RAW {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt. 1: There is a RX AFIFO write-full error when SPI inputs data in master mode. 0: Others."]
    pub mod MST_RX_AFIFO_WFULL_ERR_INT_RAW {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt. 1: There is a TX BUF AFIFO read-empty error when SPI outputs data in master mode. 0: Others."]
    pub mod MST_TX_AFIFO_REMPTY_ERR_INT_RAW {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_APP2_INT interrupt. The value is only controlled by application."]
    pub mod APP2_INT_RAW {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw bit for SPI_APP1_INT interrupt. The value is only controlled by application."]
    pub mod APP1_INT_RAW {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI DMA interrupt status register"]
pub mod DMA_INT_ST {
    #[doc = "The status bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    pub mod DMA_INFIFO_FULL_ERR_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    pub mod DMA_OUTFIFO_EMPTY_ERR_INT_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI slave Ex_QPI interrupt."]
    pub mod SLV_EX_QPI_INT_ST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI slave En_QPI interrupt."]
    pub mod SLV_EN_QPI_INT_ST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI slave CMD7 interrupt."]
    pub mod SLV_CMD7_INT_ST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI slave CMD8 interrupt."]
    pub mod SLV_CMD8_INT_ST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI slave CMD9 interrupt."]
    pub mod SLV_CMD9_INT_ST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI slave CMDA interrupt."]
    pub mod SLV_CMDA_INT_ST {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
    pub mod SLV_RD_DMA_DONE_INT_ST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
    pub mod SLV_WR_DMA_DONE_INT_ST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    pub mod SLV_RD_BUF_DONE_INT_ST {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    pub mod SLV_WR_BUF_DONE_INT_ST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_TRANS_DONE_INT interrupt."]
    pub mod TRANS_DONE_INT_ST {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    pub mod DMA_SEG_TRANS_DONE_INT_ST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
    pub mod SEG_MAGIC_ERR_INT_ST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    pub mod SLV_BUF_ADDR_ERR_INT_ST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_SLV_CMD_ERR_INT interrupt."]
    pub mod SLV_CMD_ERR_INT_ST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    pub mod MST_RX_AFIFO_WFULL_ERR_INT_ST {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    pub mod MST_TX_AFIFO_REMPTY_ERR_INT_ST {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_APP2_INT interrupt."]
    pub mod APP2_INT_ST {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The status bit for SPI_APP1_INT interrupt."]
    pub mod APP1_INT_ST {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI CPU-controlled buffer0"]
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
#[doc = "SPI CPU-controlled buffer1"]
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
#[doc = "SPI CPU-controlled buffer2"]
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
#[doc = "SPI CPU-controlled buffer3"]
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
#[doc = "SPI CPU-controlled buffer4"]
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
#[doc = "SPI CPU-controlled buffer5"]
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
#[doc = "SPI CPU-controlled buffer6"]
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
#[doc = "SPI CPU-controlled buffer7"]
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
#[doc = "SPI CPU-controlled buffer8"]
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
#[doc = "SPI CPU-controlled buffer9"]
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
#[doc = "SPI CPU-controlled buffer10"]
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
#[doc = "SPI CPU-controlled buffer11"]
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
#[doc = "SPI CPU-controlled buffer12"]
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
#[doc = "SPI CPU-controlled buffer13"]
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
#[doc = "SPI CPU-controlled buffer14"]
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
#[doc = "SPI CPU-controlled buffer15"]
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
#[doc = "SPI slave control register"]
pub mod SLAVE {
    #[doc = "SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on. Can be configured in CONF state."]
    pub mod CLK_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "{CPOL, CPHA},1: support spi clk mode 1 and 3, first edge output data B\\[0\\]/B\\[7\\]. 0: support spi clk mode 0 and 2, first edge output data B\\[1\\]/B\\[6\\]."]
    pub mod CLK_MODE_13 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "It saves half a cycle when tsck is the same as rsck. 1: output data at rsck posedge 0: output data at tsck posedge"]
    pub mod RSCK_DATA_OUT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in DMA controlled mode(Rd_DMA). 0: others"]
    pub mod SLV_RDDMA_BITLEN_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in DMA controlled mode(Wr_DMA). 0: others"]
    pub mod SLV_WRDMA_BITLEN_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: SPI_SLV_DATA_BITLEN stores data bit length of master-read-slave data length in CPU controlled mode(Rd_BUF). 0: others"]
    pub mod SLV_RDBUF_BITLEN_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: SPI_SLV_DATA_BITLEN stores data bit length of master-write-to-slave data length in CPU controlled mode(Wr_BUF). 0: others"]
    pub mod SLV_WRBUF_BITLEN_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The magic value of BM table in master DMA seg-trans."]
    pub mod DMA_SEG_MAGIC_VALUE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set SPI work mode. 1: slave mode 0: master mode."]
    pub mod MODE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
    pub mod SOFT_RESET {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the DMA CONF phase of current seg-trans operation, which means seg-trans will start. 0: This is not seg-trans mode."]
    pub mod USR_CONF {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI slave control register 1"]
pub mod SLAVE1 {
    #[doc = "The transferred data bit length in SPI slave FD and HD mode."]
    pub mod SLV_DATA_BITLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the slave mode it is the value of command."]
    pub mod SLV_LAST_COMMAND {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In the slave mode it is the value of address."]
    pub mod SLV_LAST_ADDR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI module clock and register clock control"]
pub mod CLK_GATE {
    #[doc = "Set this bit to enable clk gate"]
    pub mod CLK_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to power on the SPI module clock."]
    pub mod MST_CLK_ACTIVE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is used to select SPI module clock source in master mode. 1: PLL_CLK_80M. 0: XTAL CLK."]
    pub mod MST_CLK_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Version control"]
pub mod DATE {
    #[doc = "SPI register version."]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
