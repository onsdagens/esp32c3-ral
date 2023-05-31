#[doc = "I2C (Inter-Integrated Circuit) Controller 0"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "I2C_SCL_LOW_PERIOD_REG"]
    pub SCL_LOW_PERIOD: crate::RWRegister<u32>,
    #[doc = "I2C_CTR_REG"]
    pub CTR: crate::RWRegister<u32>,
    #[doc = "I2C_SR_REG"]
    pub SR: crate::RWRegister<u32>,
    #[doc = "I2C_TO_REG"]
    pub TO: crate::RWRegister<u32>,
    #[doc = "I2C_SLAVE_ADDR_REG"]
    pub SLAVE_ADDR: crate::RWRegister<u32>,
    #[doc = "I2C_FIFO_ST_REG"]
    pub FIFO_ST: crate::RWRegister<u32>,
    #[doc = "I2C_FIFO_CONF_REG"]
    pub FIFO_CONF: crate::RWRegister<u32>,
    #[doc = "I2C_FIFO_DATA_REG"]
    pub DATA: crate::RWRegister<u32>,
    #[doc = "I2C_INT_RAW_REG"]
    pub INT_RAW: crate::RWRegister<u32>,
    #[doc = "I2C_INT_CLR_REG"]
    pub INT_CLR: crate::RWRegister<u32>,
    #[doc = "I2C_INT_ENA_REG"]
    pub INT_ENA: crate::RWRegister<u32>,
    #[doc = "I2C_INT_STATUS_REG"]
    pub INT_STATUS: crate::RWRegister<u32>,
    #[doc = "I2C_SDA_HOLD_REG"]
    pub SDA_HOLD: crate::RWRegister<u32>,
    #[doc = "I2C_SDA_SAMPLE_REG"]
    pub SDA_SAMPLE: crate::RWRegister<u32>,
    #[doc = "I2C_SCL_HIGH_PERIOD_REG"]
    pub SCL_HIGH_PERIOD: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "I2C_SCL_START_HOLD_REG"]
    pub SCL_START_HOLD: crate::RWRegister<u32>,
    #[doc = "I2C_SCL_RSTART_SETUP_REG"]
    pub SCL_RSTART_SETUP: crate::RWRegister<u32>,
    #[doc = "I2C_SCL_STOP_HOLD_REG"]
    pub SCL_STOP_HOLD: crate::RWRegister<u32>,
    #[doc = "I2C_SCL_STOP_SETUP_REG"]
    pub SCL_STOP_SETUP: crate::RWRegister<u32>,
    #[doc = "I2C_FILTER_CFG_REG"]
    pub FILTER_CFG: crate::RWRegister<u32>,
    #[doc = "I2C_CLK_CONF_REG"]
    pub CLK_CONF: crate::RWRegister<u32>,
    #[doc = "I2C_COMD%s_REG"]
    pub COMD: [crate::RWRegister<u32>; 8usize],
    #[doc = "I2C_SCL_ST_TIME_OUT_REG"]
    pub SCL_ST_TIME_OUT: crate::RWRegister<u32>,
    #[doc = "I2C_SCL_MAIN_ST_TIME_OUT_REG"]
    pub SCL_MAIN_ST_TIME_OUT: crate::RWRegister<u32>,
    #[doc = "I2C_SCL_SP_CONF_REG"]
    pub SCL_SP_CONF: crate::RWRegister<u32>,
    #[doc = "I2C_SCL_STRETCH_CONF_REG"]
    pub SCL_STRETCH_CONF: crate::RWRegister<u32>,
    _reserved1: [u8; 0x70],
    #[doc = "I2C_DATE_REG"]
    pub DATE: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "I2C_TXFIFO_START_ADDR_REG"]
    pub TXFIFO_START_ADDR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x7c],
    #[doc = "I2C_RXFIFO_START_ADDR_REG"]
    pub RXFIFO_START_ADDR: crate::RWRegister<u32>,
}
#[doc = "I2C_SCL_LOW_PERIOD_REG"]
pub mod SCL_LOW_PERIOD {
    #[doc = "reg_scl_low_period"]
    pub mod SCL_LOW_PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_CTR_REG"]
pub mod CTR {
    #[doc = "reg_sda_force_out"]
    pub mod SDA_FORCE_OUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_scl_force_out"]
    pub mod SCL_FORCE_OUT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sample_scl_level"]
    pub mod SAMPLE_SCL_LEVEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rx_full_ack_level"]
    pub mod RX_FULL_ACK_LEVEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ms_mode"]
    pub mod MS_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_trans_start"]
    pub mod TRANS_START {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_tx_lsb_first"]
    pub mod TX_LSB_FIRST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rx_lsb_first"]
    pub mod RX_LSB_FIRST {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_en"]
    pub mod CLK_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_arbitration_en"]
    pub mod ARBITRATION_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_fsm_rst"]
    pub mod FSM_RST {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_conf_upgate"]
    pub mod CONF_UPGATE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_slv_tx_auto_start_en"]
    pub mod SLV_TX_AUTO_START_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_addr_10bit_rw_check_en"]
    pub mod ADDR_10BIT_RW_CHECK_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_addr_broadcasting_en"]
    pub mod ADDR_BROADCASTING_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_SR_REG"]
pub mod SR {
    #[doc = "reg_resp_rec"]
    pub mod RESP_REC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_slave_rw"]
    pub mod SLAVE_RW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_arb_lost"]
    pub mod ARB_LOST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_bus_busy"]
    pub mod BUS_BUSY {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_slave_addressed"]
    pub mod SLAVE_ADDRESSED {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rxfifo_cnt"]
    pub mod RXFIFO_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_stretch_cause"]
    pub mod STRETCH_CAUSE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_txfifo_cnt"]
    pub mod TXFIFO_CNT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_scl_main_state_last"]
    pub mod SCL_MAIN_STATE_LAST {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_scl_state_last"]
    pub mod SCL_STATE_LAST {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_TO_REG"]
pub mod TO {
    #[doc = "reg_time_out_value"]
    pub mod TIME_OUT_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_time_out_en"]
    pub mod TIME_OUT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_SLAVE_ADDR_REG"]
pub mod SLAVE_ADDR {
    #[doc = "reg_slave_addr"]
    pub mod SLAVE_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_addr_10bit_en"]
    pub mod ADDR_10BIT_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_FIFO_ST_REG"]
pub mod FIFO_ST {
    #[doc = "reg_rxfifo_raddr"]
    pub mod RXFIFO_RADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rxfifo_waddr"]
    pub mod RXFIFO_WADDR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_txfifo_raddr"]
    pub mod TXFIFO_RADDR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_txfifo_waddr"]
    pub mod TXFIFO_WADDR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_slave_rw_point"]
    pub mod SLAVE_RW_POINT {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_FIFO_CONF_REG"]
pub mod FIFO_CONF {
    #[doc = "reg_rxfifo_wm_thrhd"]
    pub mod RXFIFO_WM_THRHD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_txfifo_wm_thrhd"]
    pub mod TXFIFO_WM_THRHD {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_nonfifo_en"]
    pub mod NONFIFO_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_fifo_addr_cfg_en"]
    pub mod FIFO_ADDR_CFG_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rx_fifo_rst"]
    pub mod RX_FIFO_RST {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_tx_fifo_rst"]
    pub mod TX_FIFO_RST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_fifo_prt_en"]
    pub mod FIFO_PRT_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_FIFO_DATA_REG"]
pub mod DATA {
    #[doc = "reg_fifo_rdata"]
    pub mod FIFO_RDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_INT_RAW_REG"]
pub mod INT_RAW {
    #[doc = "reg_rxfifo_wm_int_raw"]
    pub mod RXFIFO_WM_INT_RAW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_txfifo_wm_int_raw"]
    pub mod TXFIFO_WM_INT_RAW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rxfifo_ovf_int_raw"]
    pub mod RXFIFO_OVF_INT_RAW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_end_detect_int_raw"]
    pub mod END_DETECT_INT_RAW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_byte_trans_done_int_raw"]
    pub mod BYTE_TRANS_DONE_INT_RAW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_arbitration_lost_int_raw"]
    pub mod ARBITRATION_LOST_INT_RAW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mst_txfifo_udf_int_raw"]
    pub mod MST_TXFIFO_UDF_INT_RAW {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_trans_complete_int_raw"]
    pub mod TRANS_COMPLETE_INT_RAW {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_time_out_int_raw"]
    pub mod TIME_OUT_INT_RAW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_trans_start_int_raw"]
    pub mod TRANS_START_INT_RAW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_nack_int_raw"]
    pub mod NACK_INT_RAW {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_txfifo_ovf_int_raw"]
    pub mod TXFIFO_OVF_INT_RAW {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rxfifo_udf_int_raw"]
    pub mod RXFIFO_UDF_INT_RAW {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_scl_st_to_int_raw"]
    pub mod SCL_ST_TO_INT_RAW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_scl_main_st_to_int_raw"]
    pub mod SCL_MAIN_ST_TO_INT_RAW {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_det_start_int_raw"]
    pub mod DET_START_INT_RAW {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_slave_stretch_int_raw"]
    pub mod SLAVE_STRETCH_INT_RAW {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_general_call_int_raw"]
    pub mod GENERAL_CALL_INT_RAW {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_INT_CLR_REG"]
pub mod INT_CLR {
    #[doc = "reg_rxfifo_wm_int_clr"]
    pub mod RXFIFO_WM_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_txfifo_wm_int_clr"]
    pub mod TXFIFO_WM_INT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rxfifo_ovf_int_clr"]
    pub mod RXFIFO_OVF_INT_CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_end_detect_int_clr"]
    pub mod END_DETECT_INT_CLR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_byte_trans_done_int_clr"]
    pub mod BYTE_TRANS_DONE_INT_CLR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_arbitration_lost_int_clr"]
    pub mod ARBITRATION_LOST_INT_CLR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mst_txfifo_udf_int_clr"]
    pub mod MST_TXFIFO_UDF_INT_CLR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_trans_complete_int_clr"]
    pub mod TRANS_COMPLETE_INT_CLR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_time_out_int_clr"]
    pub mod TIME_OUT_INT_CLR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_trans_start_int_clr"]
    pub mod TRANS_START_INT_CLR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_nack_int_clr"]
    pub mod NACK_INT_CLR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_txfifo_ovf_int_clr"]
    pub mod TXFIFO_OVF_INT_CLR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rxfifo_udf_int_clr"]
    pub mod RXFIFO_UDF_INT_CLR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_scl_st_to_int_clr"]
    pub mod SCL_ST_TO_INT_CLR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_scl_main_st_to_int_clr"]
    pub mod SCL_MAIN_ST_TO_INT_CLR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_det_start_int_clr"]
    pub mod DET_START_INT_CLR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_slave_stretch_int_clr"]
    pub mod SLAVE_STRETCH_INT_CLR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_general_call_int_clr"]
    pub mod GENERAL_CALL_INT_CLR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_INT_ENA_REG"]
pub mod INT_ENA {
    #[doc = "reg_rxfifo_wm_int_ena"]
    pub mod RXFIFO_WM_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_txfifo_wm_int_ena"]
    pub mod TXFIFO_WM_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rxfifo_ovf_int_ena"]
    pub mod RXFIFO_OVF_INT_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_end_detect_int_ena"]
    pub mod END_DETECT_INT_ENA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_byte_trans_done_int_ena"]
    pub mod BYTE_TRANS_DONE_INT_ENA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_arbitration_lost_int_ena"]
    pub mod ARBITRATION_LOST_INT_ENA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mst_txfifo_udf_int_ena"]
    pub mod MST_TXFIFO_UDF_INT_ENA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_trans_complete_int_ena"]
    pub mod TRANS_COMPLETE_INT_ENA {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_time_out_int_ena"]
    pub mod TIME_OUT_INT_ENA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_trans_start_int_ena"]
    pub mod TRANS_START_INT_ENA {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_nack_int_ena"]
    pub mod NACK_INT_ENA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_txfifo_ovf_int_ena"]
    pub mod TXFIFO_OVF_INT_ENA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rxfifo_udf_int_ena"]
    pub mod RXFIFO_UDF_INT_ENA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_scl_st_to_int_ena"]
    pub mod SCL_ST_TO_INT_ENA {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_scl_main_st_to_int_ena"]
    pub mod SCL_MAIN_ST_TO_INT_ENA {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_det_start_int_ena"]
    pub mod DET_START_INT_ENA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_slave_stretch_int_ena"]
    pub mod SLAVE_STRETCH_INT_ENA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_general_call_int_ena"]
    pub mod GENERAL_CALL_INT_ENA {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_INT_STATUS_REG"]
pub mod INT_STATUS {
    #[doc = "reg_rxfifo_wm_int_st"]
    pub mod RXFIFO_WM_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_txfifo_wm_int_st"]
    pub mod TXFIFO_WM_INT_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rxfifo_ovf_int_st"]
    pub mod RXFIFO_OVF_INT_ST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_end_detect_int_st"]
    pub mod END_DETECT_INT_ST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_byte_trans_done_int_st"]
    pub mod BYTE_TRANS_DONE_INT_ST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_arbitration_lost_int_st"]
    pub mod ARBITRATION_LOST_INT_ST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mst_txfifo_udf_int_st"]
    pub mod MST_TXFIFO_UDF_INT_ST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_trans_complete_int_st"]
    pub mod TRANS_COMPLETE_INT_ST {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_time_out_int_st"]
    pub mod TIME_OUT_INT_ST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_trans_start_int_st"]
    pub mod TRANS_START_INT_ST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_nack_int_st"]
    pub mod NACK_INT_ST {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_txfifo_ovf_int_st"]
    pub mod TXFIFO_OVF_INT_ST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rxfifo_udf_int_st"]
    pub mod RXFIFO_UDF_INT_ST {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_scl_st_to_int_st"]
    pub mod SCL_ST_TO_INT_ST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_scl_main_st_to_int_st"]
    pub mod SCL_MAIN_ST_TO_INT_ST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_det_start_int_st"]
    pub mod DET_START_INT_ST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_slave_stretch_int_st"]
    pub mod SLAVE_STRETCH_INT_ST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_general_call_int_st"]
    pub mod GENERAL_CALL_INT_ST {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_SDA_HOLD_REG"]
pub mod SDA_HOLD {
    #[doc = "reg_sda_hold_time"]
    pub mod TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_SDA_SAMPLE_REG"]
pub mod SDA_SAMPLE {
    #[doc = "reg_sda_sample_time"]
    pub mod TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_SCL_HIGH_PERIOD_REG"]
pub mod SCL_HIGH_PERIOD {
    #[doc = "reg_scl_high_period"]
    pub mod SCL_HIGH_PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_scl_wait_high_period"]
    pub mod SCL_WAIT_HIGH_PERIOD {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_SCL_START_HOLD_REG"]
pub mod SCL_START_HOLD {
    #[doc = "reg_scl_start_hold_time"]
    pub mod TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_SCL_RSTART_SETUP_REG"]
pub mod SCL_RSTART_SETUP {
    #[doc = "reg_scl_rstart_setup_time"]
    pub mod TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_SCL_STOP_HOLD_REG"]
pub mod SCL_STOP_HOLD {
    #[doc = "reg_scl_stop_hold_time"]
    pub mod TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_SCL_STOP_SETUP_REG"]
pub mod SCL_STOP_SETUP {
    #[doc = "reg_scl_stop_setup_time"]
    pub mod TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_FILTER_CFG_REG"]
pub mod FILTER_CFG {
    #[doc = "reg_scl_filter_thres"]
    pub mod SCL_FILTER_THRES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sda_filter_thres"]
    pub mod SDA_FILTER_THRES {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_scl_filter_en"]
    pub mod SCL_FILTER_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sda_filter_en"]
    pub mod SDA_FILTER_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_CLK_CONF_REG"]
pub mod CLK_CONF {
    #[doc = "reg_sclk_div_num"]
    pub mod SCLK_DIV_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sclk_div_a"]
    pub mod SCLK_DIV_A {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sclk_div_b"]
    pub mod SCLK_DIV_B {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sclk_sel"]
    pub mod SCLK_SEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sclk_active"]
    pub mod SCLK_ACTIVE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_COMD%s_REG"]
pub mod COMD {
    #[doc = "reg_command"]
    pub mod COMMAND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_command_done"]
    pub mod COMMAND_DONE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_SCL_ST_TIME_OUT_REG"]
pub mod SCL_ST_TIME_OUT {
    #[doc = "reg_scl_st_to_regno more than 23"]
    pub mod SCL_ST_TO_I2C {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_SCL_MAIN_ST_TIME_OUT_REG"]
pub mod SCL_MAIN_ST_TIME_OUT {
    #[doc = "reg_scl_main_st_to_regno more than 23"]
    pub mod SCL_MAIN_ST_TO_I2C {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_SCL_SP_CONF_REG"]
pub mod SCL_SP_CONF {
    #[doc = "reg_scl_rst_slv_en"]
    pub mod SCL_RST_SLV_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_scl_rst_slv_num"]
    pub mod SCL_RST_SLV_NUM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_scl_pd_en"]
    pub mod SCL_PD_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sda_pd_en"]
    pub mod SDA_PD_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_SCL_STRETCH_CONF_REG"]
pub mod SCL_STRETCH_CONF {
    #[doc = "reg_stretch_protect_num"]
    pub mod STRETCH_PROTECT_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_slave_scl_stretch_en"]
    pub mod SLAVE_SCL_STRETCH_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_slave_scl_stretch_clr"]
    pub mod SLAVE_SCL_STRETCH_CLR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_slave_byte_ack_ctl_en"]
    pub mod SLAVE_BYTE_ACK_CTL_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_slave_byte_ack_lvl"]
    pub mod SLAVE_BYTE_ACK_LVL {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_DATE_REG"]
pub mod DATE {
    #[doc = "reg_date"]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_TXFIFO_START_ADDR_REG"]
pub mod TXFIFO_START_ADDR {
    #[doc = "reg_txfifo_start_addr."]
    pub mod TXFIFO_START_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2C_RXFIFO_START_ADDR_REG"]
pub mod RXFIFO_START_ADDR {
    #[doc = "reg_rxfifo_start_addr."]
    pub mod RXFIFO_START_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
