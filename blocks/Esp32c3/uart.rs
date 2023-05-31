#[doc = "UART (Universal Asynchronous Receiver-Transmitter) Controller 0"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "FIFO data register"]
    pub FIFO: crate::RWRegister<u32>,
    #[doc = "Raw interrupt status"]
    pub INT_RAW: crate::RWRegister<u32>,
    #[doc = "Masked interrupt status"]
    pub INT_ST: crate::RWRegister<u32>,
    #[doc = "Interrupt enable bits"]
    pub INT_ENA: crate::RWRegister<u32>,
    #[doc = "Interrupt clear bits"]
    pub INT_CLR: crate::RWRegister<u32>,
    #[doc = "Clock divider configuration"]
    pub CLKDIV: crate::RWRegister<u32>,
    #[doc = "Rx Filter configuration"]
    pub RX_FILT: crate::RWRegister<u32>,
    #[doc = "UART status register"]
    pub STATUS: crate::RWRegister<u32>,
    #[doc = "a"]
    pub CONF0: crate::RWRegister<u32>,
    #[doc = "Configuration register 1"]
    pub CONF1: crate::RWRegister<u32>,
    #[doc = "Autobaud minimum low pulse duration register"]
    pub LOWPULSE: crate::RWRegister<u32>,
    #[doc = "Autobaud minimum high pulse duration register"]
    pub HIGHPULSE: crate::RWRegister<u32>,
    #[doc = "Autobaud edge change count register"]
    pub RXD_CNT: crate::RWRegister<u32>,
    #[doc = "Software flow-control configuration"]
    pub FLOW_CONF: crate::RWRegister<u32>,
    #[doc = "Sleep-mode configuration"]
    pub SLEEP_CONF: crate::RWRegister<u32>,
    #[doc = "Software flow-control character configuration"]
    pub SWFC_CONF0: crate::RWRegister<u32>,
    #[doc = "Software flow-control character configuration"]
    pub SWFC_CONF1: crate::RWRegister<u32>,
    #[doc = "Tx Break character configuration"]
    pub TXBRK_CONF: crate::RWRegister<u32>,
    #[doc = "Frame-end idle configuration"]
    pub IDLE_CONF: crate::RWRegister<u32>,
    #[doc = "RS485 mode configuration"]
    pub RS485_CONF: crate::RWRegister<u32>,
    #[doc = "Pre-sequence timing configuration"]
    pub AT_CMD_PRECNT: crate::RWRegister<u32>,
    #[doc = "Post-sequence timing configuration"]
    pub AT_CMD_POSTCNT: crate::RWRegister<u32>,
    #[doc = "Timeout configuration"]
    pub AT_CMD_GAPTOUT: crate::RWRegister<u32>,
    #[doc = "AT escape sequence detection configuration"]
    pub AT_CMD_CHAR: crate::RWRegister<u32>,
    #[doc = "UART threshold and allocation configuration"]
    pub MEM_CONF: crate::RWRegister<u32>,
    #[doc = "Tx-FIFO write and read offset address."]
    pub MEM_TX_STATUS: crate::RWRegister<u32>,
    #[doc = "Rx-FIFO write and read offset address."]
    pub MEM_RX_STATUS: crate::RWRegister<u32>,
    #[doc = "UART transmit and receive status."]
    pub FSM_STATUS: crate::RWRegister<u32>,
    #[doc = "Autobaud high pulse register"]
    pub POSPULSE: crate::RWRegister<u32>,
    #[doc = "Autobaud low pulse register"]
    pub NEGPULSE: crate::RWRegister<u32>,
    #[doc = "UART core clock configuration"]
    pub CLK_CONF: crate::RWRegister<u32>,
    #[doc = "UART Version register"]
    pub DATE: crate::RWRegister<u32>,
    #[doc = "UART ID register"]
    pub ID: crate::RWRegister<u32>,
}
#[doc = "FIFO data register"]
pub mod FIFO {
    #[doc = "UART 0 accesses FIFO via this register."]
    pub mod RXFIFO_RD_BYTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Raw interrupt status"]
pub mod INT_RAW {
    #[doc = "This interrupt raw bit turns to high level when receiver receives more data than what rxfifo_full_thrhd specifies."]
    pub mod RXFIFO_FULL_INT_RAW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when the amount of data in Tx-FIFO is less than what txfifo_empty_thrhd specifies ."]
    pub mod TXFIFO_EMPTY_INT_RAW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when receiver detects a parity error in the data."]
    pub mod PARITY_ERR_INT_RAW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when receiver detects a data frame error ."]
    pub mod FRM_ERR_INT_RAW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when receiver receives more data than the FIFO can store."]
    pub mod RXFIFO_OVF_INT_RAW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when receiver detects the edge change of DSRn signal."]
    pub mod DSR_CHG_INT_RAW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when receiver detects the edge change of CTSn signal."]
    pub mod CTS_CHG_INT_RAW {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when receiver detects a 0 after the stop bit."]
    pub mod BRK_DET_INT_RAW {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when receiver takes more time than rx_tout_thrhd to receive a byte."]
    pub mod RXFIFO_TOUT_INT_RAW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when receiver recevies Xon char when uart_sw_flow_con_en is set to 1."]
    pub mod SW_XON_INT_RAW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when receiver receives Xoff char when uart_sw_flow_con_en is set to 1."]
    pub mod SW_XOFF_INT_RAW {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when receiver detects a glitch in the middle of a start bit."]
    pub mod GLITCH_DET_INT_RAW {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when transmitter completes sending NULL characters, after all data in Tx-FIFO are sent."]
    pub mod TX_BRK_DONE_INT_RAW {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when transmitter has kept the shortest duration after sending the last data."]
    pub mod TX_BRK_IDLE_DONE_INT_RAW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when transmitter has send out all data in FIFO."]
    pub mod TX_DONE_INT_RAW {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when receiver detects a parity error from the echo of transmitter in rs485 mode."]
    pub mod RS485_PARITY_ERR_INT_RAW {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when receiver detects a data frame error from the echo of transmitter in rs485 mode."]
    pub mod RS485_FRM_ERR_INT_RAW {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when detects a clash between transmitter and receiver in rs485 mode."]
    pub mod RS485_CLASH_INT_RAW {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when receiver detects the configured at_cmd char."]
    pub mod AT_CMD_CHAR_DET_INT_RAW {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This interrupt raw bit turns to high level when input rxd edge changes more times than what reg_active_threshold specifies in light sleeping mode."]
    pub mod WAKEUP_INT_RAW {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Masked interrupt status"]
pub mod INT_ST {
    #[doc = "This is the status bit for rxfifo_full_int_raw when rxfifo_full_int_ena is set to 1."]
    pub mod RXFIFO_FULL_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for txfifo_empty_int_raw when txfifo_empty_int_ena is set to 1."]
    pub mod TXFIFO_EMPTY_INT_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for parity_err_int_raw when parity_err_int_ena is set to 1."]
    pub mod PARITY_ERR_INT_ST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for frm_err_int_raw when frm_err_int_ena is set to 1."]
    pub mod FRM_ERR_INT_ST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for rxfifo_ovf_int_raw when rxfifo_ovf_int_ena is set to 1."]
    pub mod RXFIFO_OVF_INT_ST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for dsr_chg_int_raw when dsr_chg_int_ena is set to 1."]
    pub mod DSR_CHG_INT_ST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for cts_chg_int_raw when cts_chg_int_ena is set to 1."]
    pub mod CTS_CHG_INT_ST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for brk_det_int_raw when brk_det_int_ena is set to 1."]
    pub mod BRK_DET_INT_ST {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for rxfifo_tout_int_raw when rxfifo_tout_int_ena is set to 1."]
    pub mod RXFIFO_TOUT_INT_ST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for sw_xon_int_raw when sw_xon_int_ena is set to 1."]
    pub mod SW_XON_INT_ST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for sw_xoff_int_raw when sw_xoff_int_ena is set to 1."]
    pub mod SW_XOFF_INT_ST {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for glitch_det_int_raw when glitch_det_int_ena is set to 1."]
    pub mod GLITCH_DET_INT_ST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for tx_brk_done_int_raw when tx_brk_done_int_ena is set to 1."]
    pub mod TX_BRK_DONE_INT_ST {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the stauts bit for tx_brk_idle_done_int_raw when tx_brk_idle_done_int_ena is set to 1."]
    pub mod TX_BRK_IDLE_DONE_INT_ST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for tx_done_int_raw when tx_done_int_ena is set to 1."]
    pub mod TX_DONE_INT_ST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for rs485_parity_err_int_raw when rs485_parity_int_ena is set to 1."]
    pub mod RS485_PARITY_ERR_INT_ST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for rs485_frm_err_int_raw when rs485_fm_err_int_ena is set to 1."]
    pub mod RS485_FRM_ERR_INT_ST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for rs485_clash_int_raw when rs485_clash_int_ena is set to 1."]
    pub mod RS485_CLASH_INT_ST {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for at_cmd_det_int_raw when at_cmd_char_det_int_ena is set to 1."]
    pub mod AT_CMD_CHAR_DET_INT_ST {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status bit for uart_wakeup_int_raw when uart_wakeup_int_ena is set to 1."]
    pub mod WAKEUP_INT_ST {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt enable bits"]
pub mod INT_ENA {
    #[doc = "This is the enable bit for rxfifo_full_int_st register."]
    pub mod RXFIFO_FULL_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for txfifo_empty_int_st register."]
    pub mod TXFIFO_EMPTY_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for parity_err_int_st register."]
    pub mod PARITY_ERR_INT_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for frm_err_int_st register."]
    pub mod FRM_ERR_INT_ENA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for rxfifo_ovf_int_st register."]
    pub mod RXFIFO_OVF_INT_ENA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for dsr_chg_int_st register."]
    pub mod DSR_CHG_INT_ENA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for cts_chg_int_st register."]
    pub mod CTS_CHG_INT_ENA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for brk_det_int_st register."]
    pub mod BRK_DET_INT_ENA {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for rxfifo_tout_int_st register."]
    pub mod RXFIFO_TOUT_INT_ENA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for sw_xon_int_st register."]
    pub mod SW_XON_INT_ENA {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for sw_xoff_int_st register."]
    pub mod SW_XOFF_INT_ENA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for glitch_det_int_st register."]
    pub mod GLITCH_DET_INT_ENA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for tx_brk_done_int_st register."]
    pub mod TX_BRK_DONE_INT_ENA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for tx_brk_idle_done_int_st register."]
    pub mod TX_BRK_IDLE_DONE_INT_ENA {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for tx_done_int_st register."]
    pub mod TX_DONE_INT_ENA {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for rs485_parity_err_int_st register."]
    pub mod RS485_PARITY_ERR_INT_ENA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for rs485_parity_err_int_st register."]
    pub mod RS485_FRM_ERR_INT_ENA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for rs485_clash_int_st register."]
    pub mod RS485_CLASH_INT_ENA {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for at_cmd_char_det_int_st register."]
    pub mod AT_CMD_CHAR_DET_INT_ENA {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for uart_wakeup_int_st register."]
    pub mod WAKEUP_INT_ENA {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt clear bits"]
pub mod INT_CLR {
    #[doc = "Set this bit to clear the rxfifo_full_int_raw interrupt."]
    pub mod RXFIFO_FULL_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear txfifo_empty_int_raw interrupt."]
    pub mod TXFIFO_EMPTY_INT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear parity_err_int_raw interrupt."]
    pub mod PARITY_ERR_INT_CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear frm_err_int_raw interrupt."]
    pub mod FRM_ERR_INT_CLR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear rxfifo_ovf_int_raw interrupt."]
    pub mod RXFIFO_OVF_INT_CLR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the dsr_chg_int_raw interrupt."]
    pub mod DSR_CHG_INT_CLR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the cts_chg_int_raw interrupt."]
    pub mod CTS_CHG_INT_CLR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the brk_det_int_raw interrupt."]
    pub mod BRK_DET_INT_CLR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the rxfifo_tout_int_raw interrupt."]
    pub mod RXFIFO_TOUT_INT_CLR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the sw_xon_int_raw interrupt."]
    pub mod SW_XON_INT_CLR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the sw_xoff_int_raw interrupt."]
    pub mod SW_XOFF_INT_CLR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the glitch_det_int_raw interrupt."]
    pub mod GLITCH_DET_INT_CLR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the tx_brk_done_int_raw interrupt.."]
    pub mod TX_BRK_DONE_INT_CLR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the tx_brk_idle_done_int_raw interrupt."]
    pub mod TX_BRK_IDLE_DONE_INT_CLR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the tx_done_int_raw interrupt."]
    pub mod TX_DONE_INT_CLR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the rs485_parity_err_int_raw interrupt."]
    pub mod RS485_PARITY_ERR_INT_CLR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the rs485_frm_err_int_raw interrupt."]
    pub mod RS485_FRM_ERR_INT_CLR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the rs485_clash_int_raw interrupt."]
    pub mod RS485_CLASH_INT_CLR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the at_cmd_char_det_int_raw interrupt."]
    pub mod AT_CMD_CHAR_DET_INT_CLR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the uart_wakeup_int_raw interrupt."]
    pub mod WAKEUP_INT_CLR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock divider configuration"]
pub mod CLKDIV {
    #[doc = "The integral part of the frequency divider factor."]
    pub mod CLKDIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The decimal part of the frequency divider factor."]
    pub mod FRAG {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Filter configuration"]
pub mod RX_FILT {
    #[doc = "when input pulse width is lower than this value, the pulse is ignored."]
    pub mod GLITCH_FILT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable Rx signal filter."]
    pub mod GLITCH_FILT_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UART status register"]
pub mod STATUS {
    #[doc = "Stores the byte number of valid data in Rx-FIFO."]
    pub mod RXFIFO_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The register represent the level value of the internal uart dsr signal."]
    pub mod DSRN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register represent the level value of the internal uart cts signal."]
    pub mod CTSN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register represent the level value of the internal uart rxd signal."]
    pub mod RXD {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stores the byte number of data in Tx-FIFO."]
    pub mod TXFIFO_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit represents the level of the internal uart dtr signal."]
    pub mod DTRN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit represents the level of the internal uart rts signal."]
    pub mod RTSN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit represents the level of the internal uart txd signal."]
    pub mod TXD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod CONF0 {
    #[doc = "This register is used to configure the parity check mode."]
    pub mod PARITY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable uart parity check."]
    pub mod PARITY_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register is used to set the length of data."]
    pub mod BIT_NUM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register is used to set the length of stop bit."]
    pub mod STOP_BIT_NUM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register is used to configure the software rts signal which is used in software flow control."]
    pub mod SW_RTS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register is used to configure the software dtr signal which is used in software flow control."]
    pub mod SW_DTR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enbale transmitter to send NULL when the process of sending data is done."]
    pub mod TXD_BRK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable IrDA loopback mode."]
    pub mod IRDA_DPLX {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the start enable bit for IrDA transmitter."]
    pub mod IRDA_TX_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1'h1: The IrDA transmitter's 11th bit is the same as 10th bit. 1'h0: Set IrDA transmitter's 11th bit to 0."]
    pub mod IRDA_WCTL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to invert the level of IrDA transmitter."]
    pub mod IRDA_TX_INV {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to invert the level of IrDA receiver."]
    pub mod IRDA_RX_INV {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable uart loopback test mode."]
    pub mod LOOPBACK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable flow control function for transmitter."]
    pub mod TX_FLOW_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable IrDA protocol."]
    pub mod IRDA_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to reset the uart receive-FIFO."]
    pub mod RXFIFO_RST {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to reset the uart transmit-FIFO."]
    pub mod TXFIFO_RST {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to inverse the level value of uart rxd signal."]
    pub mod RXD_INV {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to inverse the level value of uart cts signal."]
    pub mod CTS_INV {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to inverse the level value of uart dsr signal."]
    pub mod DSR_INV {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to inverse the level value of uart txd signal."]
    pub mod TXD_INV {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to inverse the level value of uart rts signal."]
    pub mod RTS_INV {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to inverse the level value of uart dtr signal."]
    pub mod DTR_INV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    pub mod CLK_EN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong."]
    pub mod ERR_WR_MASK {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enable bit for detecting baudrate."]
    pub mod AUTOBAUD_EN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UART memory clock gate enable signal."]
    pub mod MEM_CLK_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configuration register 1"]
pub mod CONF1 {
    #[doc = "It will produce rxfifo_full_int interrupt when receiver receives more data than this register value."]
    pub mod RXFIFO_FULL_THRHD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value."]
    pub mod TXFIFO_EMPTY_THRHD {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable UART Rx data overflow detect."]
    pub mod DIS_RX_DAT_OVF {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to stop accumulating idle_cnt when hardware flow control works."]
    pub mod RX_TOUT_FLOW_DIS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the flow enable bit for UART receiver."]
    pub mod RX_FLOW_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the enble bit for uart receiver's timeout function."]
    pub mod RX_TOUT_EN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Autobaud minimum low pulse duration register"]
pub mod LOWPULSE {
    #[doc = "This register stores the value of the minimum duration time of the low level pulse. It is used in baud rate-detect process."]
    pub mod MIN_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Autobaud minimum high pulse duration register"]
pub mod HIGHPULSE {
    #[doc = "This register stores the value of the maxinum duration time for the high level pulse. It is used in baud rate-detect process."]
    pub mod MIN_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Autobaud edge change count register"]
pub mod RXD_CNT {
    #[doc = "This register stores the count of rxd edge change. It is used in baud rate-detect process."]
    pub mod RXD_EDGE_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Software flow-control configuration"]
pub mod FLOW_CONF {
    #[doc = "Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff."]
    pub mod SW_FLOW_CON_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to remove flow control char from the received data."]
    pub mod XONOFF_DEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable the transmitter to go on sending data."]
    pub mod FORCE_XON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to stop the transmitter from sending data."]
    pub mod FORCE_XOFF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to send Xon char. It is cleared by hardware automatically."]
    pub mod SEND_XON {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to send Xoff char. It is cleared by hardware automatically."]
    pub mod SEND_XOFF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Sleep-mode configuration"]
pub mod SLEEP_CONF {
    #[doc = "The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value."]
    pub mod ACTIVE_THRESHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Software flow-control character configuration"]
pub mod SWFC_CONF0 {
    #[doc = "When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1, it will send a Xoff char."]
    pub mod XOFF_THRESHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register stores the Xoff flow control char."]
    pub mod XOFF_CHAR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Software flow-control character configuration"]
pub mod SWFC_CONF1 {
    #[doc = "When the data amount in Rx-FIFO is less than this register value with uart_sw_flow_con_en set to 1, it will send a Xon char."]
    pub mod XON_THRESHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register stores the Xon flow control char."]
    pub mod XON_CHAR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Break character configuration"]
pub mod TXBRK_CONF {
    #[doc = "This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1."]
    pub mod TX_BRK_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frame-end idle configuration"]
pub mod IDLE_CONF {
    #[doc = "It will produce frame end signal when receiver takes more time to receive one byte data than this register value."]
    pub mod RX_IDLE_THRHD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register is used to configure the duration time between transfers."]
    pub mod TX_IDLE_NUM {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RS485 mode configuration"]
pub mod RS485_CONF {
    #[doc = "Set this bit to choose the rs485 mode."]
    pub mod RS485_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to delay the stop bit by 1 bit."]
    pub mod DL0_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to delay the stop bit by 1 bit."]
    pub mod DL1_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable receiver could receive data when the transmitter is transmitting data in rs485 mode."]
    pub mod RS485TX_RX_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1'h1: enable rs485 transmitter to send data when rs485 receiver line is busy."]
    pub mod RS485RXBY_TX_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register is used to delay the receiver's internal data signal."]
    pub mod RS485_RX_DLY_NUM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register is used to delay the transmitter's internal data signal."]
    pub mod RS485_TX_DLY_NUM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pre-sequence timing configuration"]
pub mod AT_CMD_PRECNT {
    #[doc = "This register is used to configure the idle duration time before the first at_cmd is received by receiver."]
    pub mod PRE_IDLE_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Post-sequence timing configuration"]
pub mod AT_CMD_POSTCNT {
    #[doc = "This register is used to configure the duration time between the last at_cmd and the next data."]
    pub mod POST_IDLE_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timeout configuration"]
pub mod AT_CMD_GAPTOUT {
    #[doc = "This register is used to configure the duration time between the at_cmd chars."]
    pub mod RX_GAP_TOUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AT escape sequence detection configuration"]
pub mod AT_CMD_CHAR {
    #[doc = "This register is used to configure the content of at_cmd char."]
    pub mod AT_CMD_CHAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register is used to configure the num of continuous at_cmd chars received by receiver."]
    pub mod CHAR_NUM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UART threshold and allocation configuration"]
pub mod MEM_CONF {
    #[doc = "This register is used to configure the amount of mem allocated for receive-FIFO. The default number is 128 bytes."]
    pub mod RX_SIZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes."]
    pub mod TX_SIZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
    pub mod RX_FLOW_THRHD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1."]
    pub mod RX_TOUT_THRHD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to force power down UART memory."]
    pub mod MEM_FORCE_PD {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to force power up UART memory."]
    pub mod MEM_FORCE_PU {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx-FIFO write and read offset address."]
pub mod MEM_TX_STATUS {
    #[doc = "This register stores the offset address in Tx-FIFO when software writes Tx-FIFO via APB."]
    pub mod APB_TX_WADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register stores the offset address in Tx-FIFO when Tx-FSM reads data via Tx-FIFO_Ctrl."]
    pub mod TX_RADDR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx-FIFO write and read offset address."]
pub mod MEM_RX_STATUS {
    #[doc = "This register stores the offset address in RX-FIFO when software reads data from Rx-FIFO via APB. UART0 is 10'h100. UART1 is 10'h180."]
    pub mod APB_RX_RADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register stores the offset address in Rx-FIFO when Rx-FIFO_Ctrl writes Rx-FIFO. UART0 is 10'h100. UART1 is 10'h180."]
    pub mod RX_WADDR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UART transmit and receive status."]
pub mod FSM_STATUS {
    #[doc = "This is the status register of receiver."]
    pub mod ST_URX_OUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the status register of transmitter."]
    pub mod ST_UTX_OUT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Autobaud high pulse register"]
pub mod POSPULSE {
    #[doc = "This register stores the minimal input clock count between two positive edges. It is used in boudrate-detect process."]
    pub mod POSEDGE_MIN_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Autobaud low pulse register"]
pub mod NEGPULSE {
    #[doc = "This register stores the minimal input clock count between two negative edges. It is used in boudrate-detect process."]
    pub mod NEGEDGE_MIN_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UART core clock configuration"]
pub mod CLK_CONF {
    #[doc = "The denominator of the frequency divider factor."]
    pub mod SCLK_DIV_B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The numerator of the frequency divider factor."]
    pub mod SCLK_DIV_A {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The integral part of the frequency divider factor."]
    pub mod SCLK_DIV_NUM {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UART clock source select. 1: 80Mhz, 2: 8Mhz, 3: XTAL."]
    pub mod SCLK_SEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable UART Tx/Rx clock."]
    pub mod SCLK_EN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write 1 then write 0 to this bit, reset UART Tx/Rx."]
    pub mod RST_CORE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable UART Tx clock."]
    pub mod TX_SCLK_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable UART Rx clock."]
    pub mod RX_SCLK_EN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write 1 then write 0 to this bit, reset UART Tx."]
    pub mod TX_RST_CORE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write 1 then write 0 to this bit, reset UART Rx."]
    pub mod RX_RST_CORE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UART Version register"]
pub mod DATE {
    #[doc = "This is the version register."]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UART ID register"]
pub mod ID {
    #[doc = "This register is used to configure the uart_id."]
    pub mod ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit used to select synchronize mode. 1: Registers are auto synchronized into UART Core clock and UART core should be keep the same with APB clock. 0: After configure registers, software needs to write 1 to UART_REG_UPDATE to synchronize registers."]
    pub mod HIGH_SPEED {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software write 1 would synchronize registers into UART Core clock domain and would be cleared by hardware after synchronization is done."]
    pub mod REG_UPDATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
