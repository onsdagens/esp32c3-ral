#[doc = "SPI (Serial Peripheral Interface) Controller 0"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "SPI0 control register."]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "SPI0 control1 register."]
    pub CTRL1: crate::RWRegister<u32>,
    #[doc = "SPI0 control2 register."]
    pub CTRL2: crate::RWRegister<u32>,
    #[doc = "SPI clock division control register."]
    pub CLOCK: crate::RWRegister<u32>,
    #[doc = "SPI0 user register."]
    pub USER: crate::RWRegister<u32>,
    #[doc = "SPI0 user1 register."]
    pub USER1: crate::RWRegister<u32>,
    #[doc = "SPI0 user2 register."]
    pub USER2: crate::RWRegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "SPI0 read control register."]
    pub RD_STATUS: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "SPI0 misc register"]
    pub MISC: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "SPI0 bit mode control register."]
    pub CACHE_FCTRL: crate::RWRegister<u32>,
    _reserved4: [u8; 0x14],
    #[doc = "SPI0 FSM status register"]
    pub FSM: crate::RWRegister<u32>,
    _reserved5: [u8; 0x50],
    #[doc = "SPI0 timing calibration register"]
    pub TIMING_CALI: crate::RWRegister<u32>,
    #[doc = "SPI0 input delay mode control register"]
    pub DIN_MODE: crate::RWRegister<u32>,
    #[doc = "SPI0 input delay number control register"]
    pub DIN_NUM: crate::RWRegister<u32>,
    #[doc = "SPI0 output delay mode control register"]
    pub DOUT_MODE: crate::RWRegister<u32>,
    _reserved6: [u8; 0x24],
    #[doc = "SPI0 clk_gate register"]
    pub CLOCK_GATE: crate::RWRegister<u32>,
    #[doc = "SPI0 module clock select register"]
    pub CORE_CLK_SEL: crate::RWRegister<u32>,
    _reserved7: [u8; 0x0318],
    #[doc = "Version control register"]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "SPI0 control register."]
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
#[doc = "SPI0 control1 register."]
pub mod CTRL1 {
    #[doc = "SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    pub mod CLK_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPI0 RX FIFO reset signal."]
    pub mod RXFIFO_RST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI0 control2 register."]
pub mod CTRL2 {
    #[doc = "(cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
    pub mod CS_SETUP_TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
    pub mod CS_HOLD_TIME {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    pub mod CS_HOLD_DELAY {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The FSM will be reset."]
    pub mod SYNC_RESET {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI clock division control register."]
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
    #[doc = "Set this bit in 1-division mode."]
    pub mod CLK_EQU_SYSCLK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI0 user register."]
pub mod USER {
    #[doc = "spi cs keep low when spi is in done phase. 1: enable 0: disable."]
    pub mod CS_HOLD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
    pub mod CS_SETUP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode."]
    pub mod CK_OUT_EDGE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "spi clock is disable in dummy phase when the bit is enable."]
    pub mod USR_DUMMY_IDLE {
        pub const offset: u32 = 26;
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
}
#[doc = "SPI0 user1 register."]
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
#[doc = "SPI0 user2 register."]
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
#[doc = "SPI0 read control register."]
pub mod RD_STATUS {
    #[doc = "Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
    pub mod WB_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI0 misc register"]
pub mod MISC {
    #[doc = "The bit is used to indicate the spi0_mst_st controlled transmitting is done."]
    pub mod TRANS_END {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable the interrupt of spi0_mst_st controlled transmitting is done."]
    pub mod TRANS_END_INT_ENA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
    pub mod CSPI_ST_TRANS_END {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable the interrupt of spi0_slv_st controlled transmitting is done."]
    pub mod CSPI_ST_TRANS_END_INT_ENA {
        pub const offset: u32 = 6;
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
#[doc = "SPI0 bit mode control register."]
pub mod CACHE_FCTRL {
    #[doc = "For SPI0, Cache access enable, 1: enable, 0:disable."]
    pub mod CACHE_REQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
    pub mod CACHE_USR_ADDR_4BYTE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
    pub mod CACHE_FLASH_USR_CMD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    pub mod FDIN_DUAL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    pub mod FDOUT_DUAL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    pub mod FADDR_DUAL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    pub mod FDIN_QUAD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    pub mod FDOUT_QUAD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    pub mod FADDR_QUAD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI0 FSM status register"]
pub mod FSM {
    #[doc = "The current status of SPI0 slave FSM: spi0_slv_st. 0: idle state, 1: preparation state, 2: send command state, 3: send address state, 4: wait state, 5: read data state, 6:write data state, 7: done state, 8: read data end state."]
    pub mod CSPI_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The current status of SPI0 master FSM: spi0_mst_st. 0: idle state, 1:EM_CACHE_GRANT , 2: program/erase suspend state, 3: SPI0 read data state, 4: wait cache/EDMA sent data is stored in SPI0 TX FIFO, 5: SPI0 write data state."]
    pub mod EM_ST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
    pub mod CSPI_LOCK_DELAY_TIME {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI0 timing calibration register"]
pub mod TIMING_CALI {
    #[doc = "The bit is used to enable timing adjust clock for all reading operations."]
    pub mod TIMING_CLK_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
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
#[doc = "SPI0 input delay mode control register"]
pub mod DIN_MODE {
    #[doc = "the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    pub mod DIN0_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    pub mod DIN1_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    pub mod DIN2_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the input signals are delayed by system clock cycles, 0: input without delayed, 1: input with the posedge of clk_apb,2 input with the negedge of clk_apb, 3: input with the posedge of clk_160, 4 input with the negedge of clk_160, 5: input with the spi_clk high edge, 6: input with the spi_clk low edge"]
    pub mod DIN3_MODE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI0 input delay number control register"]
pub mod DIN_NUM {
    #[doc = "the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    pub mod DIN0_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    pub mod DIN1_NUM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    pub mod DIN2_NUM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    pub mod DIN3_NUM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI0 output delay mode control register"]
pub mod DOUT_MODE {
    #[doc = "the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    pub mod DOUT0_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    pub mod DOUT1_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    pub mod DOUT2_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the posedge of clk_160,4 output with the negedge of clk_160,5: output with the spi_clk high edge ,6: output with the spi_clk low edge"]
    pub mod DOUT3_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPI0 clk_gate register"]
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
#[doc = "SPI0 module clock select register"]
pub mod CORE_CLK_SEL {
    #[doc = "When the digital system clock selects PLL clock and the frequency of PLL clock is 480MHz, the value of reg_spi01_clk_sel: 0: SPI0/1 module clock (clk) is 80MHz. 1: SPI0/1 module clock (clk) is 120MHz. 2: SPI0/1 module clock (clk) 160MHz. 3: Not used. When the digital system clock selects PLL clock and the frequency of PLL clock is 320MHz, the value of reg_spi01_clk_sel: 0: SPI0/1 module clock (clk) is 80MHz. 1: SPI0/1 module clock (clk) is 80MHz. 2: SPI0/1 module clock (clk) 160MHz. 3: Not used."]
    pub mod SPI01_CLK_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Version control register"]
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
