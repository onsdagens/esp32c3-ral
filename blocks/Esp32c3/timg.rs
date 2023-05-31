#[doc = "Timer Group 0"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "TIMG_T0CONFIG_REG."]
    pub T0CONFIG: crate::RWRegister<u32>,
    #[doc = "TIMG_T0LO_REG."]
    pub T0LO: crate::RWRegister<u32>,
    #[doc = "TIMG_T0HI_REG."]
    pub T0HI: crate::RWRegister<u32>,
    #[doc = "TIMG_T0UPDATE_REG."]
    pub T0UPDATE: crate::RWRegister<u32>,
    #[doc = "TIMG_T0ALARMLO_REG."]
    pub T0ALARMLO: crate::RWRegister<u32>,
    #[doc = "TIMG_T0ALARMHI_REG."]
    pub T0ALARMHI: crate::RWRegister<u32>,
    #[doc = "TIMG_T0LOADLO_REG."]
    pub T0LOADLO: crate::RWRegister<u32>,
    #[doc = "TIMG_T0LOADHI_REG."]
    pub T0LOADHI: crate::RWRegister<u32>,
    #[doc = "TIMG_T0LOAD_REG."]
    pub T0LOAD: crate::RWRegister<u32>,
    _reserved0: [u8; 0x24],
    #[doc = "TIMG_WDTCONFIG0_REG."]
    pub WDTCONFIG0: crate::RWRegister<u32>,
    #[doc = "TIMG_WDTCONFIG1_REG."]
    pub WDTCONFIG1: crate::RWRegister<u32>,
    #[doc = "TIMG_WDTCONFIG2_REG."]
    pub WDTCONFIG2: crate::RWRegister<u32>,
    #[doc = "TIMG_WDTCONFIG3_REG."]
    pub WDTCONFIG3: crate::RWRegister<u32>,
    #[doc = "TIMG_WDTCONFIG4_REG."]
    pub WDTCONFIG4: crate::RWRegister<u32>,
    #[doc = "TIMG_WDTCONFIG5_REG."]
    pub WDTCONFIG5: crate::RWRegister<u32>,
    #[doc = "TIMG_WDTFEED_REG."]
    pub WDTFEED: crate::RWRegister<u32>,
    #[doc = "TIMG_WDTWPROTECT_REG."]
    pub WDTWPROTECT: crate::RWRegister<u32>,
    #[doc = "TIMG_RTCCALICFG_REG."]
    pub RTCCALICFG: crate::RWRegister<u32>,
    #[doc = "TIMG_RTCCALICFG1_REG."]
    pub RTCCALICFG1: crate::RWRegister<u32>,
    #[doc = "INT_ENA_TIMG_REG"]
    pub INT_ENA_TIMERS: crate::RWRegister<u32>,
    #[doc = "INT_RAW_TIMG_REG"]
    pub INT_RAW_TIMERS: crate::RWRegister<u32>,
    #[doc = "INT_ST_TIMG_REG"]
    pub INT_ST_TIMERS: crate::RWRegister<u32>,
    #[doc = "INT_CLR_TIMG_REG"]
    pub INT_CLR_TIMERS: crate::RWRegister<u32>,
    #[doc = "TIMG_RTCCALICFG2_REG."]
    pub RTCCALICFG2: crate::RWRegister<u32>,
    _reserved1: [u8; 0x74],
    #[doc = "TIMG_NTIMG_DATE_REG."]
    pub NTIMG_DATE: crate::RWRegister<u32>,
    #[doc = "TIMG_REGCLK_REG."]
    pub REGCLK: crate::RWRegister<u32>,
}
#[doc = "TIMG_T0CONFIG_REG."]
pub mod T0CONFIG {
    #[doc = "reg_t0_use_xtal."]
    pub mod USE_XTAL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_t0_alarm_en."]
    pub mod ALARM_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_t0_divcnt_rst."]
    pub mod DIVCNT_RST {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_t0_divider."]
    pub mod DIVIDER {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_t0_autoreload."]
    pub mod AUTORELOAD {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_t0_increase."]
    pub mod INCREASE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_t0_en."]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_T0LO_REG."]
pub mod T0LO {
    #[doc = "t0_lo"]
    pub mod LO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_T0HI_REG."]
pub mod T0HI {
    #[doc = "t0_hi"]
    pub mod HI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x003f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_T0UPDATE_REG."]
pub mod T0UPDATE {
    #[doc = "t0_update"]
    pub mod UPDATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_T0ALARMLO_REG."]
pub mod T0ALARMLO {
    #[doc = "reg_t0_alarm_lo."]
    pub mod ALARM_LO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_T0ALARMHI_REG."]
pub mod T0ALARMHI {
    #[doc = "reg_t0_alarm_hi."]
    pub mod ALARM_HI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x003f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_T0LOADLO_REG."]
pub mod T0LOADLO {
    #[doc = "reg_t0_load_lo."]
    pub mod LOAD_LO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_T0LOADHI_REG."]
pub mod T0LOADHI {
    #[doc = "reg_t0_load_hi."]
    pub mod LOAD_HI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x003f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_T0LOAD_REG."]
pub mod T0LOAD {
    #[doc = "t0_load"]
    pub mod LOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_WDTCONFIG0_REG."]
pub mod WDTCONFIG0 {
    #[doc = "reg_wdt_appcpu_reset_en."]
    pub mod WDT_APPCPU_RESET_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_wdt_procpu_reset_en."]
    pub mod WDT_PROCPU_RESET_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_wdt_flashboot_mod_en."]
    pub mod WDT_FLASHBOOT_MOD_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_wdt_sys_reset_length."]
    pub mod WDT_SYS_RESET_LENGTH {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_wdt_cpu_reset_length."]
    pub mod WDT_CPU_RESET_LENGTH {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_wdt_use_xtal."]
    pub mod WDT_USE_XTAL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_wdt_conf_update_en."]
    pub mod WDT_CONF_UPDATE_EN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_wdt_stg3."]
    pub mod WDT_STG3 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_wdt_stg2."]
    pub mod WDT_STG2 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_wdt_stg1."]
    pub mod WDT_STG1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_wdt_stg0."]
    pub mod WDT_STG0 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_wdt_en."]
    pub mod WDT_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_WDTCONFIG1_REG."]
pub mod WDTCONFIG1 {
    #[doc = "reg_wdt_divcnt_rst."]
    pub mod WDT_DIVCNT_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_wdt_clk_prescale."]
    pub mod WDT_CLK_PRESCALE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_WDTCONFIG2_REG."]
pub mod WDTCONFIG2 {
    #[doc = "reg_wdt_stg0_hold."]
    pub mod WDT_STG0_HOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_WDTCONFIG3_REG."]
pub mod WDTCONFIG3 {
    #[doc = "reg_wdt_stg1_hold."]
    pub mod WDT_STG1_HOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_WDTCONFIG4_REG."]
pub mod WDTCONFIG4 {
    #[doc = "reg_wdt_stg2_hold."]
    pub mod WDT_STG2_HOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_WDTCONFIG5_REG."]
pub mod WDTCONFIG5 {
    #[doc = "reg_wdt_stg3_hold."]
    pub mod WDT_STG3_HOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_WDTFEED_REG."]
pub mod WDTFEED {
    #[doc = "wdt_feed"]
    pub mod WDT_FEED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_WDTWPROTECT_REG."]
pub mod WDTWPROTECT {
    #[doc = "reg_wdt_wkey."]
    pub mod WDT_WKEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_RTCCALICFG_REG."]
pub mod RTCCALICFG {
    #[doc = "reg_rtc_cali_start_cycling."]
    pub mod RTC_CALI_START_CYCLING {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rtc_cali_clk_sel.0:rtcslowclock.1:clk_80m.2:xtal_32k"]
    pub mod RTC_CALI_CLK_SEL {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "rtc_cali_rdy"]
    pub mod RTC_CALI_RDY {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rtc_cali_max."]
    pub mod RTC_CALI_MAX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rtc_cali_start."]
    pub mod RTC_CALI_START {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_RTCCALICFG1_REG."]
pub mod RTCCALICFG1 {
    #[doc = "rtc_cali_cycling_data_vld"]
    pub mod RTC_CALI_CYCLING_DATA_VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "rtc_cali_value"]
    pub mod RTC_CALI_VALUE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "INT_ENA_TIMG_REG"]
pub mod INT_ENA_TIMERS {
    #[doc = "t0_int_ena"]
    pub mod T0_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wdt_int_ena"]
    pub mod WDT_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "INT_RAW_TIMG_REG"]
pub mod INT_RAW_TIMERS {
    #[doc = "t0_int_raw"]
    pub mod T0_INT_RAW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wdt_int_raw"]
    pub mod WDT_INT_RAW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "INT_ST_TIMG_REG"]
pub mod INT_ST_TIMERS {
    #[doc = "t0_int_st"]
    pub mod T0_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wdt_int_st"]
    pub mod WDT_INT_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "INT_CLR_TIMG_REG"]
pub mod INT_CLR_TIMERS {
    #[doc = "t0_int_clr"]
    pub mod T0_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wdt_int_clr"]
    pub mod WDT_INT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_RTCCALICFG2_REG."]
pub mod RTCCALICFG2 {
    #[doc = "timeoutindicator"]
    pub mod RTC_CALI_TIMEOUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rtc_cali_timeout_rst_cnt.Cyclesthatreleasecalibrationtimeoutreset"]
    pub mod RTC_CALI_TIMEOUT_RST_CNT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rtc_cali_timeout_thres.timeoutifcalivaluecountsoverthreshold"]
    pub mod RTC_CALI_TIMEOUT_THRES {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_NTIMG_DATE_REG."]
pub mod NTIMG_DATE {
    #[doc = "reg_ntimers_date."]
    pub mod NTIMGS_DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TIMG_REGCLK_REG."]
pub mod REGCLK {
    #[doc = "reg_wdt_clk_is_active."]
    pub mod WDT_CLK_IS_ACTIVE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_timer_clk_is_active."]
    pub mod TIMER_CLK_IS_ACTIVE {
        pub const offset: u32 = 30;
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
