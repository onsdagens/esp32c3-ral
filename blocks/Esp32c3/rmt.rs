#[doc = "Remote Control"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "RMT_CH0DATA_REG."]
    pub CH0DATA: crate::RWRegister<u32>,
    #[doc = "RMT_CH1DATA_REG."]
    pub CH1DATA: crate::RWRegister<u32>,
    #[doc = "RMT_CH2DATA_REG."]
    pub CH2DATA: crate::RWRegister<u32>,
    #[doc = "RMT_CH3DATA_REG."]
    pub CH3DATA: crate::RWRegister<u32>,
    #[doc = "RMT_CH%sCONF%s_REG."]
    pub CH_TX_CONF0: [crate::RWRegister<u32>; 2usize],
    #[doc = "RMT_CH2CONF0_REG."]
    pub CH_RX_CONF00: crate::RWRegister<u32>,
    #[doc = "RMT_CH2CONF1_REG."]
    pub CH2CONF1: crate::RWRegister<u32>,
    #[doc = "RMT_CH2CONF0_REG."]
    pub CH_RX_CONF01: crate::RWRegister<u32>,
    #[doc = "RMT_CH3CONF1_REG."]
    pub CH3CONF1: crate::RWRegister<u32>,
    #[doc = "RMT_CH0STATUS_REG."]
    pub CH0STATUS: crate::RWRegister<u32>,
    #[doc = "RMT_CH1STATUS_REG."]
    pub CH1STATUS: crate::RWRegister<u32>,
    #[doc = "RMT_CH2STATUS_REG."]
    pub CH2STATUS: crate::RWRegister<u32>,
    #[doc = "RMT_CH3STATUS_REG."]
    pub CH3STATUS: crate::RWRegister<u32>,
    #[doc = "RMT_INT_RAW_REG."]
    pub INT_RAW: crate::RWRegister<u32>,
    #[doc = "RMT_INT_ST_REG."]
    pub INT_ST: crate::RWRegister<u32>,
    #[doc = "RMT_INT_ENA_REG."]
    pub INT_ENA: crate::RWRegister<u32>,
    #[doc = "RMT_INT_CLR_REG."]
    pub INT_CLR: crate::RWRegister<u32>,
    #[doc = "RMT_CH0CARRIER_DUTY_REG."]
    pub CH0CARRIER_DUTY: crate::RWRegister<u32>,
    #[doc = "RMT_CH1CARRIER_DUTY_REG."]
    pub CH1CARRIER_DUTY: crate::RWRegister<u32>,
    #[doc = "RMT_CH2_RX_CARRIER_RM_REG."]
    pub CH2_RX_CARRIER_RM: crate::RWRegister<u32>,
    #[doc = "RMT_CH3_RX_CARRIER_RM_REG."]
    pub CH3_RX_CARRIER_RM: crate::RWRegister<u32>,
    #[doc = "RMT_CH%s_TX_LIM_REG."]
    pub CH_TX_LIM: [crate::RWRegister<u32>; 2usize],
    #[doc = "RMT_CH2_RX_LIM_REG."]
    pub CH_RX_LIM: [crate::RWRegister<u32>; 2usize],
    #[doc = "RMT_SYS_CONF_REG."]
    pub SYS_CONF: crate::RWRegister<u32>,
    #[doc = "RMT_TX_SIM_REG."]
    pub TX_SIM: crate::RWRegister<u32>,
    #[doc = "RMT_REF_CNT_RST_REG."]
    pub REF_CNT_RST: crate::RWRegister<u32>,
    _reserved0: [u8; 0x58],
    #[doc = "RMT_DATE_REG."]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "RMT_CH0DATA_REG."]
pub mod CH0DATA {
    #[doc = "Reserved."]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH1DATA_REG."]
pub mod CH1DATA {
    #[doc = "Reserved."]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH2DATA_REG."]
pub mod CH2DATA {
    #[doc = "Reserved."]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH3DATA_REG."]
pub mod CH3DATA {
    #[doc = "Reserved."]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH%sCONF%s_REG."]
pub mod CH_TX_CONF0 {
    #[doc = "reg_tx_start_ch0."]
    pub mod TX_START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_rd_rst_ch0."]
    pub mod MEM_RD_RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_mem_rst_ch0."]
    pub mod APB_MEM_RST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_tx_conti_mode_ch0."]
    pub mod TX_CONTI_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_tx_wrap_en_ch0."]
    pub mod MEM_TX_WRAP_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_idle_out_lv_ch0."]
    pub mod IDLE_OUT_LV {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_idle_out_en_ch0."]
    pub mod IDLE_OUT_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_tx_stop_ch0."]
    pub mod TX_STOP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_div_cnt_ch0."]
    pub mod DIV_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_size_ch0."]
    pub mod MEM_SIZE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_carrier_eff_en_ch0."]
    pub mod CARRIER_EFF_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_carrier_en_ch0."]
    pub mod CARRIER_EN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_carrier_out_lv_ch0."]
    pub mod CARRIER_OUT_LV {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_afifo_rst_ch0."]
    pub mod AFIFO_RST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_reg_conf_update_ch0."]
    pub mod CONF_UPDATE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH2CONF0_REG."]
pub mod CH_RX_CONF00 {
    #[doc = "reg_div_cnt_ch2."]
    pub mod DIV_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_idle_thres_ch2."]
    pub mod IDLE_THRES {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_size_ch2."]
    pub mod MEM_SIZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_carrier_en_ch2."]
    pub mod CARRIER_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_carrier_out_lv_ch2."]
    pub mod CARRIER_OUT_LV {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH2CONF1_REG."]
pub mod CH2CONF1 {
    #[doc = "reg_rx_en_ch2."]
    pub mod RX_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_wr_rst_ch2."]
    pub mod MEM_WR_RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_mem_rst_ch2."]
    pub mod APB_MEM_RST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_owner_ch2."]
    pub mod MEM_OWNER {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rx_filter_en_ch2."]
    pub mod RX_FILTER_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rx_filter_thres_ch2."]
    pub mod RX_FILTER_THRES {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_rx_wrap_en_ch2."]
    pub mod MEM_RX_WRAP_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_afifo_rst_ch2."]
    pub mod AFIFO_RST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_conf_update_ch2."]
    pub mod CONF_UPDATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH2CONF0_REG."]
pub mod CH_RX_CONF01 {
    #[doc = "reg_div_cnt_ch2."]
    pub mod DIV_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_idle_thres_ch2."]
    pub mod IDLE_THRES {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_size_ch2."]
    pub mod MEM_SIZE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_carrier_en_ch2."]
    pub mod CARRIER_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_carrier_out_lv_ch2."]
    pub mod CARRIER_OUT_LV {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH3CONF1_REG."]
pub mod CH3CONF1 {
    #[doc = "reg_rx_en_ch3."]
    pub mod RX_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_wr_rst_ch3."]
    pub mod MEM_WR_RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_mem_rst_ch3."]
    pub mod APB_MEM_RST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_owner_ch3."]
    pub mod MEM_OWNER {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rx_filter_en_ch3."]
    pub mod RX_FILTER_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rx_filter_thres_ch3."]
    pub mod RX_FILTER_THRES {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_rx_wrap_en_ch3."]
    pub mod MEM_RX_WRAP_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_afifo_rst_ch3."]
    pub mod AFIFO_RST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_conf_update_ch3."]
    pub mod CONF_UPDATE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH0STATUS_REG."]
pub mod CH0STATUS {
    #[doc = "reg_mem_raddr_ex_ch0."]
    pub mod MEM_RADDR_EX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_state_ch0."]
    pub mod STATE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_mem_waddr_ch0."]
    pub mod APB_MEM_WADDR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_mem_rd_err_ch0."]
    pub mod APB_MEM_RD_ERR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_empty_ch0."]
    pub mod MEM_EMPTY {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_mem_wr_err_ch0."]
    pub mod APB_MEM_WR_ERR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_mem_raddr_ch0."]
    pub mod APB_MEM_RADDR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH1STATUS_REG."]
pub mod CH1STATUS {
    #[doc = "reg_mem_raddr_ex_ch1."]
    pub mod MEM_RADDR_EX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_state_ch1."]
    pub mod STATE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_mem_waddr_ch1."]
    pub mod APB_MEM_WADDR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_mem_rd_err_ch1."]
    pub mod APB_MEM_RD_ERR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_empty_ch1."]
    pub mod MEM_EMPTY {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_mem_wr_err_ch1."]
    pub mod APB_MEM_WR_ERR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_mem_raddr_ch1."]
    pub mod APB_MEM_RADDR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH2STATUS_REG."]
pub mod CH2STATUS {
    #[doc = "reg_mem_waddr_ex_ch2."]
    pub mod MEM_WADDR_EX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_mem_raddr_ch2."]
    pub mod APB_MEM_RADDR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_state_ch2."]
    pub mod STATE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_owner_err_ch2."]
    pub mod MEM_OWNER_ERR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_full_ch2."]
    pub mod MEM_FULL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_mem_rd_err_ch2."]
    pub mod APB_MEM_RD_ERR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH3STATUS_REG."]
pub mod CH3STATUS {
    #[doc = "reg_mem_waddr_ex_ch3."]
    pub mod MEM_WADDR_EX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_mem_raddr_ch3."]
    pub mod APB_MEM_RADDR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_state_ch3."]
    pub mod STATE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_owner_err_ch3."]
    pub mod MEM_OWNER_ERR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_full_ch3."]
    pub mod MEM_FULL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_mem_rd_err_ch3."]
    pub mod APB_MEM_RD_ERR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_INT_RAW_REG."]
pub mod INT_RAW {
    #[doc = "reg_ch%s_tx_end_int_raw."]
    pub mod CH_TX_END_INT_RAW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch2_rx_end_int_raw."]
    pub mod CH_RX_END_INT_RAW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch%s_err_int_raw."]
    pub mod CH_TX_ERR_INT_RAW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch2_err_int_raw."]
    pub mod CH_RX_ERR_INT_RAW {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch%s_tx_thr_event_int_raw."]
    pub mod CH_TX_THR_EVENT_INT_RAW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch2_rx_thr_event_int_raw."]
    pub mod CH2_RX_THR_EVENT_INT_RAW {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch3_rx_thr_event_int_raw."]
    pub mod CH3_RX_THR_EVENT_INT_RAW {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch%s_tx_loop_int_raw."]
    pub mod CH_TX_LOOP_INT_RAW {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_INT_ST_REG."]
pub mod INT_ST {
    #[doc = "reg_ch%s_tx_end_int_st."]
    pub mod CH_TX_END_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch2_rx_end_int_st."]
    pub mod CH_RX_END_INT_ST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch%s_err_int_st."]
    pub mod CH_TX_ERR_INT_ST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch2_err_int_st."]
    pub mod CH_RX_ERR_INT_ST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch%s_tx_thr_event_int_st."]
    pub mod CH_TX_THR_EVENT_INT_ST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch2_rx_thr_event_int_st."]
    pub mod CH2_RX_THR_EVENT_INT_ST {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch3_rx_thr_event_int_st."]
    pub mod CH3_RX_THR_EVENT_INT_ST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch%s_tx_loop_int_st."]
    pub mod CH_TX_LOOP_INT_ST {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_INT_ENA_REG."]
pub mod INT_ENA {
    #[doc = "reg_ch%s_tx_end_int_ena."]
    pub mod CH_TX_END_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch2_rx_end_int_ena."]
    pub mod CH_RX_END_INT_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch%s_err_int_ena."]
    pub mod CH_TX_ERR_INT_ENA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch2_err_int_ena."]
    pub mod CH_RX_ERR_INT_ENA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch%s_tx_thr_event_int_ena."]
    pub mod CH_TX_THR_EVENT_INT_ENA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch2_rx_thr_event_int_ena."]
    pub mod CH2_RX_THR_EVENT_INT_ENA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch3_rx_thr_event_int_ena."]
    pub mod CH3_RX_THR_EVENT_INT_ENA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch%s_tx_loop_int_ena."]
    pub mod CH_TX_LOOP_INT_ENA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_INT_CLR_REG."]
pub mod INT_CLR {
    #[doc = "reg_ch%s_tx_end_int_clr."]
    pub mod CH_TX_END_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch2_rx_end_int_clr."]
    pub mod CH_RX_END_INT_CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch%s_err_int_clr."]
    pub mod CH_TX_ERR_INT_CLR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch2_err_int_clr."]
    pub mod CH_RX_ERR_INT_CLR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch%s_tx_thr_event_int_clr."]
    pub mod CH_TX_THR_EVENT_INT_CLR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch2_rx_thr_event_int_clr."]
    pub mod CH2_RX_THR_EVENT_INT_CLR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch3_rx_thr_event_int_clr."]
    pub mod CH3_RX_THR_EVENT_INT_CLR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ch%s_tx_loop_int_clr."]
    pub mod CH_TX_LOOP_INT_CLR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH0CARRIER_DUTY_REG."]
pub mod CH0CARRIER_DUTY {
    #[doc = "reg_carrier_low_ch0."]
    pub mod CARRIER_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_carrier_high_ch0."]
    pub mod CARRIER_HIGH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH1CARRIER_DUTY_REG."]
pub mod CH1CARRIER_DUTY {
    #[doc = "reg_carrier_low_ch1."]
    pub mod CARRIER_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_carrier_high_ch1."]
    pub mod CARRIER_HIGH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH2_RX_CARRIER_RM_REG."]
pub mod CH2_RX_CARRIER_RM {
    #[doc = "reg_carrier_low_thres_ch2."]
    pub mod CARRIER_LOW_THRES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_carrier_high_thres_ch2."]
    pub mod CARRIER_HIGH_THRES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH3_RX_CARRIER_RM_REG."]
pub mod CH3_RX_CARRIER_RM {
    #[doc = "reg_carrier_low_thres_ch3."]
    pub mod CARRIER_LOW_THRES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_carrier_high_thres_ch3."]
    pub mod CARRIER_HIGH_THRES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH%s_TX_LIM_REG."]
pub mod CH_TX_LIM {
    #[doc = "reg_rmt_tx_lim_ch0."]
    pub mod TX_LIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rmt_tx_loop_num_ch0."]
    pub mod TX_LOOP_NUM {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rmt_tx_loop_cnt_en_ch0."]
    pub mod TX_LOOP_CNT_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_loop_count_reset_ch0."]
    pub mod LOOP_COUNT_RESET {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_CH2_RX_LIM_REG."]
pub mod CH_RX_LIM {
    #[doc = "reg_rmt_rx_lim_ch2."]
    pub mod RX_LIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_SYS_CONF_REG."]
pub mod SYS_CONF {
    #[doc = "reg_apb_fifo_mask."]
    pub mod APB_FIFO_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_clk_force_on."]
    pub mod MEM_CLK_FORCE_ON {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rmt_mem_force_pd."]
    pub mod MEM_FORCE_PD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rmt_mem_force_pu."]
    pub mod MEM_FORCE_PU {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rmt_sclk_div_num."]
    pub mod SCLK_DIV_NUM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rmt_sclk_div_a."]
    pub mod SCLK_DIV_A {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rmt_sclk_div_b."]
    pub mod SCLK_DIV_B {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rmt_sclk_sel."]
    pub mod SCLK_SEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rmt_sclk_active."]
    pub mod SCLK_ACTIVE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_en."]
    pub mod CLK_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_TX_SIM_REG."]
pub mod TX_SIM {
    #[doc = "reg_rmt_tx_sim_ch0."]
    pub mod TX_SIM_CH0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rmt_tx_sim_ch1."]
    pub mod TX_SIM_CH1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rmt_tx_sim_en."]
    pub mod TX_SIM_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_REF_CNT_RST_REG."]
pub mod REF_CNT_RST {
    #[doc = "reg_ref_cnt_rst_ch0."]
    pub mod CH0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ref_cnt_rst_ch1."]
    pub mod CH1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ref_cnt_rst_ch2."]
    pub mod CH2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ref_cnt_rst_ch3."]
    pub mod CH3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RMT_DATE_REG."]
pub mod DATE {
    #[doc = "reg_rmt_date."]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
