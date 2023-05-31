#[doc = "SAR (Successive Approximation Register) Analog-to-Digital Converter"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "digital saradc configure register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub CTRL2: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub FILTER_CTRL1: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub FSM_WAIT: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub SAR1_STATUS: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub SAR2_STATUS: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub SAR_PATT_TAB1: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub SAR_PATT_TAB2: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub ONETIME_SAMPLE: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub ARB_CTRL: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub FILTER_CTRL0: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub SAR1DATA_STATUS: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub SAR2DATA_STATUS: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub THRES0_CTRL: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub THRES1_CTRL: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub THRES_CTRL: crate::RWRegister<u32>,
    #[doc = "digital saradc int register"]
    pub INT_ENA: crate::RWRegister<u32>,
    #[doc = "digital saradc int register"]
    pub INT_RAW: crate::RWRegister<u32>,
    #[doc = "digital saradc int register"]
    pub INT_ST: crate::RWRegister<u32>,
    #[doc = "digital saradc int register"]
    pub INT_CLR: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub DMA_CONF: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub CLKM_CONF: crate::RWRegister<u32>,
    #[doc = "digital tsens configure register"]
    pub APB_TSENS_CTRL: crate::RWRegister<u32>,
    #[doc = "digital tsens configure register"]
    pub TSENS_CTRL2: crate::RWRegister<u32>,
    #[doc = "digital saradc configure register"]
    pub CALI: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0398],
    #[doc = "version"]
    pub CTRL_DATE: crate::RWRegister<u32>,
}
#[doc = "digital saradc configure register"]
pub mod CTRL {
    #[doc = "select software enable saradc sample"]
    pub mod SARADC_START_FORCE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "software enable saradc sample"]
    pub mod SARADC_START {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAR clock gated"]
    pub mod SARADC_SAR_CLK_GATED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SAR clock divider"]
    pub mod SARADC_SAR_CLK_DIV {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 ~ 15 means length 1 ~ 16"]
    pub mod SARADC_SAR_PATT_LEN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clear the pointer of pattern table for DIG ADC1 CTRL"]
    pub mod SARADC_SAR_PATT_P_CLEAR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "force option to xpd sar blocks"]
    pub mod SARADC_XPD_SAR_FORCE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wait arbit signal stable after sar_done"]
    pub mod SARADC_WAIT_ARB_CYCLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod CTRL2 {
    #[doc = "enable max meas num"]
    pub mod SARADC_MEAS_NUM_LIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "max conversion number"]
    pub mod SARADC_MAX_MEAS_NUM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: data to DIG ADC1 CTRL is inverted, otherwise not"]
    pub mod SARADC_SAR1_INV {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: data to DIG ADC2 CTRL is inverted, otherwise not"]
    pub mod SARADC_SAR2_INV {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "to set saradc timer target"]
    pub mod SARADC_TIMER_TARGET {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "to enable saradc timer trigger"]
    pub mod SARADC_TIMER_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod FILTER_CTRL1 {
    #[doc = "Factor of saradc filter1"]
    pub mod APB_SARADC_FILTER_FACTOR1 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Factor of saradc filter0"]
    pub mod APB_SARADC_FILTER_FACTOR0 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod FSM_WAIT {
    #[doc = "saradc_xpd_wait"]
    pub mod SARADC_XPD_WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc_rstb_wait"]
    pub mod SARADC_RSTB_WAIT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc_standby_wait"]
    pub mod SARADC_STANDBY_WAIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod SAR1_STATUS {
    #[doc = "saradc1 status about data and channel"]
    pub mod SARADC_SAR1_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod SAR2_STATUS {
    #[doc = "saradc2 status about data and channel"]
    pub mod SARADC_SAR2_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod SAR_PATT_TAB1 {
    #[doc = "item 0 ~ 3 for pattern table 1 (each item one byte)"]
    pub mod SARADC_SAR_PATT_TAB1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod SAR_PATT_TAB2 {
    #[doc = "Item 4 ~ 7 for pattern table 1 (each item one byte)"]
    pub mod SARADC_SAR_PATT_TAB2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod ONETIME_SAMPLE {
    #[doc = "configure onetime atten"]
    pub mod SARADC_ONETIME_ATTEN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configure onetime channel"]
    pub mod SARADC_ONETIME_CHANNEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "trigger adc onetime sample"]
    pub mod SARADC_ONETIME_START {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable adc2 onetime sample"]
    pub mod SARADC2_ONETIME_SAMPLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable adc1 onetime sample"]
    pub mod SARADC1_ONETIME_SAMPLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod ARB_CTRL {
    #[doc = "adc2 arbiter force to enableapb controller"]
    pub mod ADC_ARB_APB_FORCE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "adc2 arbiter force to enable rtc controller"]
    pub mod ADC_ARB_RTC_FORCE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "adc2 arbiter force to enable wifi controller"]
    pub mod ADC_ARB_WIFI_FORCE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "adc2 arbiter force grant"]
    pub mod ADC_ARB_GRANT_FORCE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set adc2 arbiterapb priority"]
    pub mod ADC_ARB_APB_PRIORITY {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set adc2 arbiter rtc priority"]
    pub mod ADC_ARB_RTC_PRIORITY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set adc2 arbiter wifi priority"]
    pub mod ADC_ARB_WIFI_PRIORITY {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "adc2 arbiter uses fixed priority"]
    pub mod ADC_ARB_FIX_PRIORITY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod FILTER_CTRL0 {
    #[doc = "configure filter1 to adc channel"]
    pub mod APB_SARADC_FILTER_CHANNEL1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configure filter0 to adc channel"]
    pub mod APB_SARADC_FILTER_CHANNEL0 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable apb_adc1_filter"]
    pub mod APB_SARADC_FILTER_RESET {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod SAR1DATA_STATUS {
    #[doc = "saradc1 data"]
    pub mod APB_SARADC1_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod SAR2DATA_STATUS {
    #[doc = "saradc2 data"]
    pub mod APB_SARADC2_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod THRES0_CTRL {
    #[doc = "configure thres0 to adc channel"]
    pub mod APB_SARADC_THRES0_CHANNEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres0 monitor thres"]
    pub mod APB_SARADC_THRES0_HIGH {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres0 monitor thres"]
    pub mod APB_SARADC_THRES0_LOW {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod THRES1_CTRL {
    #[doc = "configure thres1 to adc channel"]
    pub mod APB_SARADC_THRES1_CHANNEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres1 monitor thres"]
    pub mod APB_SARADC_THRES1_HIGH {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres1 monitor thres"]
    pub mod APB_SARADC_THRES1_LOW {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod THRES_CTRL {
    #[doc = "enable thres to all channel"]
    pub mod APB_SARADC_THRES_ALL_EN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable thres1"]
    pub mod APB_SARADC_THRES1_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable thres0"]
    pub mod APB_SARADC_THRES0_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc int register"]
pub mod INT_ENA {
    #[doc = "saradc thres1 low interrupt enable"]
    pub mod APB_SARADC_THRES1_LOW_INT_ENA {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres0 low interrupt enable"]
    pub mod APB_SARADC_THRES0_LOW_INT_ENA {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres1 high interrupt enable"]
    pub mod APB_SARADC_THRES1_HIGH_INT_ENA {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres0 high interrupt enable"]
    pub mod APB_SARADC_THRES0_HIGH_INT_ENA {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc2 done interrupt enable"]
    pub mod APB_SARADC2_DONE_INT_ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc1 done interrupt enable"]
    pub mod APB_SARADC1_DONE_INT_ENA {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc int register"]
pub mod INT_RAW {
    #[doc = "saradc thres1 low interrupt raw"]
    pub mod APB_SARADC_THRES1_LOW_INT_RAW {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres0 low interrupt raw"]
    pub mod APB_SARADC_THRES0_LOW_INT_RAW {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres1 high interrupt raw"]
    pub mod APB_SARADC_THRES1_HIGH_INT_RAW {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres0 high interrupt raw"]
    pub mod APB_SARADC_THRES0_HIGH_INT_RAW {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc2 done interrupt raw"]
    pub mod APB_SARADC2_DONE_INT_RAW {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc1 done interrupt raw"]
    pub mod APB_SARADC1_DONE_INT_RAW {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc int register"]
pub mod INT_ST {
    #[doc = "saradc thres1 low interrupt state"]
    pub mod APB_SARADC_THRES1_LOW_INT_ST {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres0 low interrupt state"]
    pub mod APB_SARADC_THRES0_LOW_INT_ST {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres1 high interrupt state"]
    pub mod APB_SARADC_THRES1_HIGH_INT_ST {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres0 high interrupt state"]
    pub mod APB_SARADC_THRES0_HIGH_INT_ST {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc2 done interrupt state"]
    pub mod APB_SARADC2_DONE_INT_ST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc1 done interrupt state"]
    pub mod APB_SARADC1_DONE_INT_ST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc int register"]
pub mod INT_CLR {
    #[doc = "saradc thres1 low interrupt clear"]
    pub mod APB_SARADC_THRES1_LOW_INT_CLR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres0 low interrupt clear"]
    pub mod APB_SARADC_THRES0_LOW_INT_CLR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres1 high interrupt clear"]
    pub mod APB_SARADC_THRES1_HIGH_INT_CLR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc thres0 high interrupt clear"]
    pub mod APB_SARADC_THRES0_HIGH_INT_CLR {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc2 done interrupt clear"]
    pub mod APB_SARADC2_DONE_INT_CLR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "saradc1 done interrupt clear"]
    pub mod APB_SARADC1_DONE_INT_CLR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod DMA_CONF {
    #[doc = "the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
    pub mod APB_ADC_EOF_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reset_apb_adc_state"]
    pub mod APB_ADC_RESET_FSM {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable apb_adc use spi_dma"]
    pub mod APB_ADC_TRANS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod CLKM_CONF {
    #[doc = "Integral I2S clock divider value"]
    pub mod CLKM_DIV_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fractional clock divider numerator value"]
    pub mod CLKM_DIV_B {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fractional clock divider denominator value"]
    pub mod CLKM_DIV_A {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg clk en"]
    pub mod CLK_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable clk_apll"]
    pub mod CLK_SEL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital tsens configure register"]
pub mod APB_TSENS_CTRL {
    #[doc = "temperature sensor data out"]
    pub mod TSENS_OUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "invert temperature sensor data"]
    pub mod TSENS_IN_INV {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "temperature sensor clock divider"]
    pub mod TSENS_CLK_DIV {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "temperature sensor power up"]
    pub mod TSENS_PU {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital tsens configure register"]
pub mod TSENS_CTRL2 {
    #[doc = "the time that power up tsens need wait"]
    pub mod TSENS_XPD_WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "force power up tsens"]
    pub mod TSENS_XPD_FORCE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "inv tsens clk"]
    pub mod TSENS_CLK_INV {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tsens clk select"]
    pub mod TSENS_CLK_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "digital saradc configure register"]
pub mod CALI {
    #[doc = "saradc cali factor"]
    pub mod APB_SARADC_CALI_CFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "version"]
pub mod CTRL_DATE {
    #[doc = "version"]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
