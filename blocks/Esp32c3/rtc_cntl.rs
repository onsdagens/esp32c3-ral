#[doc = "Real-Time Clock Control"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "rtc configure register"]
    pub OPTIONS0: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub SLP_TIMER0: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub SLP_TIMER1: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub TIME_UPDATE: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub TIME_LOW0: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub TIME_HIGH0: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub STATE0: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub TIMER1: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub TIMER2: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub TIMER3: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub TIMER4: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub TIMER5: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub TIMER6: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub ANA_CONF: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub RESET_STATE: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub WAKEUP_STATE: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub INT_ENA_RTC: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub INT_RAW_RTC: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub INT_ST_RTC: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub INT_CLR_RTC: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub STORE0: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub STORE1: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub STORE2: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub STORE3: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub EXT_XTL_CONF: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub EXT_WAKEUP_CONF: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub SLP_REJECT_CONF: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub CPU_PERIOD_CONF: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub CLK_CONF: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub SLOW_CLK_CONF: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub SDIO_CONF: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub BIAS_CONF: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub RTC_CNTL: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub PWC: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub DIG_PWC: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub DIG_ISO: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub WDTCONFIG0: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub WDTCONFIG1: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub WDTCONFIG2: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub WDTCONFIG3: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub WDTCONFIG4: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub WDTFEED: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub WDTWPROTECT: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub SWD_CONF: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub SWD_WPROTECT: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub SW_CPU_STALL: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub STORE4: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub STORE5: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub STORE6: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub STORE7: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub LOW_POWER_ST: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub DIAG0: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub PAD_HOLD: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub DIG_PAD_HOLD: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub BROWN_OUT: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub TIME_LOW1: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub TIME_HIGH1: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub XTAL32K_CLK_FACTOR: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub XTAL32K_CONF: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub USB_CONF: crate::RWRegister<u32>,
    #[doc = "RTC_CNTL_RTC_SLP_REJECT_CAUSE_REG"]
    pub SLP_REJECT_CAUSE: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub OPTION1: crate::RWRegister<u32>,
    #[doc = "RTC_CNTL_RTC_SLP_WAKEUP_CAUSE_REG"]
    pub SLP_WAKEUP_CAUSE: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub ULP_CP_TIMER_1: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub INT_ENA_RTC_W1TS: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub INT_ENA_RTC_W1TC: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub RETENTION_CTRL: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub FIB_SEL: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub GPIO_WAKEUP: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub DBG_SEL: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub DBG_MAP: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub SENSOR_CTRL: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub DBG_SAR_SEL: crate::RWRegister<u32>,
    #[doc = "rtc configure register"]
    pub PG_CTRL: crate::RWRegister<u32>,
    _reserved0: [u8; 0xd4],
    #[doc = "rtc configure register"]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "rtc configure register"]
pub mod OPTIONS0 {
    #[doc = "{reg_sw_stall_appcpu_c1\\[5:0\\], reg_sw_stall_appcpu_c0\\[1:0\\]} == 0x86 will stall APP CPU"]
    pub mod SW_STALL_APPCPU_C0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "{reg_sw_stall_procpu_c1\\[5:0\\], reg_sw_stall_procpu_c0\\[1:0\\]} == 0x86 will stall PRO CPU"]
    pub mod SW_STALL_PROCPU_C0 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APP CPU SW reset"]
    pub mod SW_APPCPU_RST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PRO CPU SW reset"]
    pub mod SW_PROCPU_RST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BB_I2C force power down"]
    pub mod BB_I2C_FORCE_PD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BB_I2C force power up"]
    pub mod BB_I2C_FORCE_PU {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BB_PLL _I2C force power down"]
    pub mod BBPLL_I2C_FORCE_PD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BB_PLL_I2C force power up"]
    pub mod BBPLL_I2C_FORCE_PU {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BB_PLL force power down"]
    pub mod BBPLL_FORCE_PD {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BB_PLL force power up"]
    pub mod BBPLL_FORCE_PU {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "crystall force power down"]
    pub mod XTL_FORCE_PD {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "crystall force power up"]
    pub mod XTL_FORCE_PU {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wait bias_sleep and current source wakeup"]
    pub mod XTL_EN_WAIT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "analog configure"]
    pub mod XTL_EXT_CTR_SEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "analog configure"]
    pub mod XTL_FORCE_ISO {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "analog configure"]
    pub mod PLL_FORCE_ISO {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "analog configure"]
    pub mod ANALOG_FORCE_ISO {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "analog configure"]
    pub mod XTL_FORCE_NOISO {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "analog configure"]
    pub mod PLL_FORCE_NOISO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "analog configure"]
    pub mod ANALOG_FORCE_NOISO {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital wrap force reset in deep sleep"]
    pub mod DG_WRAP_FORCE_RST {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital core force no reset in deep sleep"]
    pub mod DG_WRAP_FORCE_NORST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SW system reset"]
    pub mod SW_SYS_RST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod SLP_TIMER0 {
    #[doc = "configure the sleep time"]
    pub mod SLP_VAL_LO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod SLP_TIMER1 {
    #[doc = "RTC sleep timer high 16 bits"]
    pub mod SLP_VAL_HI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "timer alarm enable bit"]
    pub mod MAIN_TIMER_ALARM_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod TIME_UPDATE {
    #[doc = "Enable to record system stall time"]
    pub mod TIMER_SYS_STALL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable to record 40M XTAL OFF time"]
    pub mod TIMER_XTL_OFF {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable to record system reset time"]
    pub mod TIMER_SYS_RST {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set 1: to update register with RTC timer"]
    pub mod TIME_UPDATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod TIME_LOW0 {
    #[doc = "RTC timer low 32 bits"]
    pub mod TIMER_VALUE0_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod TIME_HIGH0 {
    #[doc = "RTC timer high 16 bits"]
    pub mod TIMER_VALUE0_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod STATE0 {
    #[doc = "rtc software interrupt to main cpu"]
    pub mod SW_CPU_INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clear rtc sleep reject cause"]
    pub mod SLP_REJECT_CAUSE_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: APB to RTC using bridge"]
    pub mod APB2RTC_BRIDGE_SEL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SDIO active indication"]
    pub mod SDIO_ACTIVE_IND {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "leep wakeup bit"]
    pub mod SLP_WAKEUP {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "leep reject bit"]
    pub mod SLP_REJECT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sleep enable bit"]
    pub mod SLEEP_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod TIMER1 {
    #[doc = "CPU stall enable bit"]
    pub mod CPU_STALL_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU stall wait cycles in fast_clk_rtc"]
    pub mod CPU_STALL_WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CK8M wait cycles in slow_clk_rtc"]
    pub mod CK8M_WAIT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "XTAL wait cycles in slow_clk_rtc"]
    pub mod XTL_BUF_WAIT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL wait cycles in slow_clk_rtc"]
    pub mod PLL_BUF_WAIT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod TIMER2 {
    #[doc = "minimal cycles in slow_clk_rtc for CK8M in power down state"]
    pub mod MIN_TIME_CK8M_OFF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod TIMER3 {
    #[doc = "wifi power domain wakeup time"]
    pub mod WIFI_WAIT_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wifi power domain power on time"]
    pub mod WIFI_POWERUP_TIMER {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bt power domain wakeup time"]
    pub mod BT_WAIT_TIMER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bt power domain power on time"]
    pub mod BT_POWERUP_TIMER {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod TIMER4 {
    #[doc = "cpu top power domain wakeup time"]
    pub mod CPU_TOP_WAIT_TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "cpu top power domain power on time"]
    pub mod CPU_TOP_POWERUP_TIMER {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital wrap power domain wakeup time"]
    pub mod DG_WRAP_WAIT_TIMER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital wrap power domain power on time"]
    pub mod DG_WRAP_POWERUP_TIMER {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod TIMER5 {
    #[doc = "minimal sleep cycles in slow_clk_rtc"]
    pub mod MIN_SLP_VAL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod TIMER6 {
    #[doc = "digital peri power domain wakeup time"]
    pub mod DG_PERI_WAIT_TIMER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital peri power domain power on time"]
    pub mod DG_PERI_POWERUP_TIMER {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod ANA_CONF {
    #[doc = "force no bypass i2c power on reset"]
    pub mod RESET_POR_FORCE_PD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "force bypass i2c power on reset"]
    pub mod RESET_POR_FORCE_PU {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable glitch reset"]
    pub mod GLITCH_RST_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLLA force power up"]
    pub mod SAR_I2C_PU {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLLA force power down"]
    pub mod PLLA_FORCE_PD {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLLA force power up"]
    pub mod PLLA_FORCE_PU {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "start BBPLL calibration during sleep"]
    pub mod BBPLL_CAL_SLP_START {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: PVTMON power up"]
    pub mod PVTMON_PU {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: TXRF_I2C power up"]
    pub mod TXRF_I2C_PU {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: RFRX_PBUS power up"]
    pub mod RFRX_PBUS_PU {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: CKGEN_I2C power up"]
    pub mod CKGEN_I2C_PU {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "power up pll i2c"]
    pub mod PLL_I2C_PU {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod RESET_STATE {
    #[doc = "reset cause of PRO CPU"]
    pub mod RESET_CAUSE_PROCPU {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reset cause of APP CPU"]
    pub mod RESET_CAUSE_APPCPU {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APP CPU state vector sel"]
    pub mod STAT_VECTOR_SEL_APPCPU {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PRO CPU state vector sel"]
    pub mod STAT_VECTOR_SEL_PROCPU {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PRO CPU reset_flag"]
    pub mod ALL_RESET_FLAG_PROCPU {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APP CPU reset flag"]
    pub mod ALL_RESET_FLAG_APPCPU {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clear PRO CPU reset_flag"]
    pub mod ALL_RESET_FLAG_CLR_PROCPU {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clear APP CPU reset flag"]
    pub mod ALL_RESET_FLAG_CLR_APPCPU {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APPCPU OcdHaltOnReset"]
    pub mod OCD_HALT_ON_RESET_APPCPU {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PROCPU OcdHaltOnReset"]
    pub mod OCD_HALT_ON_RESET_PROCPU {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configure jtag reset configure"]
    pub mod JTAG_RESET_FLAG_PROCPU {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configure jtag reset configure"]
    pub mod JTAG_RESET_FLAG_APPCPU {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configure jtag reset configure"]
    pub mod JTAG_RESET_FLAG_CLR_PROCPU {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configure jtag reset configure"]
    pub mod JTAG_RESET_FLAG_CLR_APPCPU {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configure dreset configure"]
    pub mod DRESET_MASK_APPCPU {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configure dreset configure"]
    pub mod DRESET_MASK_PROCPU {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod WAKEUP_STATE {
    #[doc = "wakeup enable bitmap"]
    pub mod WAKEUP_ENA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod INT_ENA_RTC {
    #[doc = "enable sleep wakeup interrupt"]
    pub mod SLP_WAKEUP_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable sleep reject interrupt"]
    pub mod SLP_REJECT_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable RTC WDT interrupt"]
    pub mod WDT_INT_ENA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable brown out interrupt"]
    pub mod BROWN_OUT_INT_ENA {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable RTC main timer interrupt"]
    pub mod MAIN_TIMER_INT_ENA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable super watch dog interrupt"]
    pub mod SWD_INT_ENA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable xtal32k_dead interrupt"]
    pub mod XTAL32K_DEAD_INT_ENA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enbale gitch det interrupt"]
    pub mod GLITCH_DET_INT_ENA {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enbale bbpll cal end interrupt"]
    pub mod BBPLL_CAL_INT_ENA {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod INT_RAW_RTC {
    #[doc = "sleep wakeup interrupt raw"]
    pub mod SLP_WAKEUP_INT_RAW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sleep reject interrupt raw"]
    pub mod SLP_REJECT_INT_RAW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RTC WDT interrupt raw"]
    pub mod WDT_INT_RAW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "brown out interrupt raw"]
    pub mod BROWN_OUT_INT_RAW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RTC main timer interrupt raw"]
    pub mod MAIN_TIMER_INT_RAW {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "super watch dog interrupt raw"]
    pub mod SWD_INT_RAW {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "xtal32k dead detection interrupt raw"]
    pub mod XTAL32K_DEAD_INT_RAW {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "glitch_det_interrupt_raw"]
    pub mod GLITCH_DET_INT_RAW {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bbpll cal end interrupt state"]
    pub mod BBPLL_CAL_INT_RAW {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod INT_ST_RTC {
    #[doc = "sleep wakeup interrupt state"]
    pub mod SLP_WAKEUP_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sleep reject interrupt state"]
    pub mod SLP_REJECT_INT_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RTC WDT interrupt state"]
    pub mod WDT_INT_ST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "brown out interrupt state"]
    pub mod BROWN_OUT_INT_ST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RTC main timer interrupt state"]
    pub mod MAIN_TIMER_INT_ST {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "super watch dog interrupt state"]
    pub mod SWD_INT_ST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "xtal32k dead detection interrupt state"]
    pub mod XTAL32K_DEAD_INT_ST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "glitch_det_interrupt state"]
    pub mod GLITCH_DET_INT_ST {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bbpll cal end interrupt state"]
    pub mod BBPLL_CAL_INT_ST {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod INT_CLR_RTC {
    #[doc = "Clear sleep wakeup interrupt state"]
    pub mod SLP_WAKEUP_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear sleep reject interrupt state"]
    pub mod SLP_REJECT_INT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear RTC WDT interrupt state"]
    pub mod WDT_INT_CLR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear brown out interrupt state"]
    pub mod BROWN_OUT_INT_CLR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear RTC main timer interrupt state"]
    pub mod MAIN_TIMER_INT_CLR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear super watch dog interrupt state"]
    pub mod SWD_INT_CLR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear RTC WDT interrupt state"]
    pub mod XTAL32K_DEAD_INT_CLR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear glitch det interrupt state"]
    pub mod GLITCH_DET_INT_CLR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clear bbpll cal end interrupt state"]
    pub mod BBPLL_CAL_INT_CLR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod STORE0 {
    #[doc = "reserved register"]
    pub mod SCRATCH0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod STORE1 {
    #[doc = "reserved register"]
    pub mod SCRATCH1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod STORE2 {
    #[doc = "reserved register"]
    pub mod SCRATCH2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod STORE3 {
    #[doc = "reserved register"]
    pub mod SCRATCH3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod EXT_XTL_CONF {
    #[doc = "xtal 32k watch dog enable"]
    pub mod XTAL32K_WDT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "xtal 32k watch dog clock force on"]
    pub mod XTAL32K_WDT_CLK_FO {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "xtal 32k watch dog sw reset"]
    pub mod XTAL32K_WDT_RESET {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "xtal 32k external xtal clock force on"]
    pub mod XTAL32K_EXT_CLK_FO {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "xtal 32k switch to back up clock when xtal is dead"]
    pub mod XTAL32K_AUTO_BACKUP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "xtal 32k restart xtal when xtal is dead"]
    pub mod XTAL32K_AUTO_RESTART {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "xtal 32k switch back xtal when xtal is restarted"]
    pub mod XTAL32K_AUTO_RETURN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Xtal 32k xpd control by sw or fsm"]
    pub mod XTAL32K_XPD_FORCE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "apply an internal clock to help xtal 32k to start"]
    pub mod ENCKINIT_XTAL_32K {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: single-end buffer 1: differential buffer"]
    pub mod DBUF_XTAL_32K {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "xtal_32k gm control"]
    pub mod DGM_XTAL_32K {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DRES_XTAL_32K"]
    pub mod DRES_XTAL_32K {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "XPD_XTAL_32K"]
    pub mod XPD_XTAL_32K {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DAC_XTAL_32K"]
    pub mod DAC_XTAL_32K {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "state of 32k_wdt"]
    pub mod WDT_STATE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "XTAL_32K sel. 0: external XTAL_32K"]
    pub mod XTAL32K_GPIO_SEL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: power down XTAL at high level"]
    pub mod XTL_EXT_CTR_LV {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable gpio configure xtal power on"]
    pub mod XTL_EXT_CTR_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod EXT_WAKEUP_CONF {
    #[doc = "enable filter for gpio wakeup event"]
    pub mod GPIO_WAKEUP_FILTER {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod SLP_REJECT_CONF {
    #[doc = "sleep reject enable"]
    pub mod SLEEP_REJECT_ENA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable reject for light sleep"]
    pub mod LIGHT_SLP_REJECT_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable reject for deep sleep"]
    pub mod DEEP_SLP_REJECT_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod CPU_PERIOD_CONF {
    #[doc = "CPU sel option"]
    pub mod CPUSEL_CONF {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU clk sel option"]
    pub mod CPUPERIOD_SEL {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod CLK_CONF {
    #[doc = "efuse_clk_force_gating"]
    pub mod EFUSE_CLK_FORCE_GATING {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "efuse_clk_force_nogating"]
    pub mod EFUSE_CLK_FORCE_NOGATING {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "used to sync reg_ck8m_div_sel bus. Clear vld before set reg_ck8m_div_sel"]
    pub mod CK8M_DIV_SEL_VLD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CK8M_D256_OUT divider. 00: div128"]
    pub mod CK8M_DIV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "disable CK8M and CK8M_D256_OUT"]
    pub mod ENB_CK8M {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: CK8M_D256_OUT is actually CK8M"]
    pub mod ENB_CK8M_DIV {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
    pub mod DIG_XTAL32K_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
    pub mod DIG_CLK8M_D256_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable CK8M for digital core (no relationship with RTC core)"]
    pub mod DIG_CLK8M_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "divider = reg_ck8m_div_sel + 1"]
    pub mod CK8M_DIV_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "XTAL force no gating during sleep"]
    pub mod XTAL_FORCE_NOGATING {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CK8M force no gating during sleep"]
    pub mod CK8M_FORCE_NOGATING {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CK8M_DFREQ"]
    pub mod CK8M_DFREQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CK8M force power down"]
    pub mod CK8M_FORCE_PD {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CK8M force power up"]
    pub mod CK8M_FORCE_PU {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "force enable xtal clk gating"]
    pub mod XTAL_GLOBAL_FORCE_GATING {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "force bypass xtal clk gating"]
    pub mod XTAL_GLOBAL_FORCE_NOGATING {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "fast_clk_rtc sel. 0: XTAL div 4"]
    pub mod FAST_CLK_RTC_SEL {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "slelect rtc slow clk"]
    pub mod ANA_CLK_RTC_SEL {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod SLOW_CLK_CONF {
    #[doc = "used to sync div bus. clear vld before set reg_rtc_ana_clk_div"]
    pub mod ANA_CLK_DIV_VLD {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the clk divider num of RTC_CLK"]
    pub mod ANA_CLK_DIV {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag rtc_slow_clk_next_edge"]
    pub mod SLOW_CLK_NEXT_EDGE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod SDIO_CONF {
    #[doc = "timer count to apply reg_sdio_dcap after sdio power on"]
    pub mod SDIO_TIMER_TARGET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tieh = 1 mode drive ability. Initially set to 0 to limit charge current"]
    pub mod SDIO_DTHDRV {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ability to prevent LDO from overshoot"]
    pub mod SDIO_DCAP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "add resistor from ldo output to ground. 0: no res"]
    pub mod SDIO_INITI {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 to set init\\[1:0\\]=0"]
    pub mod SDIO_EN_INITI {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tune current limit threshold when tieh = 0. About 800mA/(8+d)"]
    pub mod SDIO_DCURLIM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select current limit mode"]
    pub mod SDIO_MODECURLIM {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable current limit"]
    pub mod SDIO_ENCURLIM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
    pub mod SDIO_REG_PD_EN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: use SW option to control SDIO_REG"]
    pub mod SDIO_FORCE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
    pub mod SDIO_TIEH {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "read only register for REG1P8_READY"]
    pub mod _1P8_READY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
    pub mod DREFL_SDIO {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
    pub mod DREFM_SDIO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
    pub mod DREFH_SDIO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    pub mod XPD_SDIO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod BIAS_CONF {
    pub mod DG_VDD_DRV_B_SLP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    pub mod DG_VDD_DRV_B_SLP_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bias buf when rtc in normal work state"]
    pub mod BIAS_BUF_IDLE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bias buf when rtc in wakeup state"]
    pub mod BIAS_BUF_WAKE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bias buf when rtc in sleep state"]
    pub mod BIAS_BUF_DEEP_SLP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bias buf when rtc in monitor state"]
    pub mod BIAS_BUF_MONITOR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "xpd cur when rtc in sleep_state"]
    pub mod PD_CUR_DEEP_SLP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "xpd cur when rtc in monitor state"]
    pub mod PD_CUR_MONITOR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bias_sleep when rtc in sleep_state"]
    pub mod BIAS_SLEEP_DEEP_SLP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bias_sleep when rtc in monitor state"]
    pub mod BIAS_SLEEP_MONITOR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DBG_ATTEN when rtc in sleep state"]
    pub mod DBG_ATTEN_DEEP_SLP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DBG_ATTEN when rtc in monitor state"]
    pub mod DBG_ATTEN_MONITOR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod RTC_CNTL {
    #[doc = "software enable digital regulator cali"]
    pub mod DIG_REG_CAL_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SCK_DCAP"]
    pub mod SCK_DCAP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RTC_DBOOST force power down"]
    pub mod DBOOST_FORCE_PD {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RTC_DBOOST force power up"]
    pub mod DBOOST_FORCE_PU {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    pub mod REGULATOR_FORCE_PD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RTC_REG force power up"]
    pub mod REGULATOR_FORCE_PU {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod PWC {
    #[doc = "rtc pad force hold"]
    pub mod PAD_FORCE_HOLD {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod DIG_PWC {
    #[doc = "vdd_spi drv's software value"]
    pub mod VDD_SPI_PWR_DRV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "vdd_spi drv use software value"]
    pub mod VDD_SPI_PWR_FORCE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "memories in digital core force PD in sleep"]
    pub mod LSLP_MEM_FORCE_PD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "memories in digital core force PU in sleep"]
    pub mod LSLP_MEM_FORCE_PU {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bt force power down"]
    pub mod BT_FORCE_PD {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bt force power up"]
    pub mod BT_FORCE_PU {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital peri force power down"]
    pub mod DG_PERI_FORCE_PD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital peri force power up"]
    pub mod DG_PERI_FORCE_PU {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "fastmemory retention mode in sleep"]
    pub mod FASTMEM_FORCE_LPD {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "fastmemory donlt entry retention mode in sleep"]
    pub mod FASTMEM_FORCE_LPU {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wifi force power down"]
    pub mod WIFI_FORCE_PD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wifi force power up"]
    pub mod WIFI_FORCE_PU {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital core force power down"]
    pub mod DG_WRAP_FORCE_PD {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital core force power up"]
    pub mod DG_WRAP_FORCE_PU {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "cpu core force power down"]
    pub mod CPU_TOP_FORCE_PD {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "cpu force power up"]
    pub mod CPU_TOP_FORCE_PU {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable power down bt in sleep"]
    pub mod BT_PD_EN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable power down digital peri in sleep"]
    pub mod DG_PERI_PD_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable power down cpu in sleep"]
    pub mod CPU_TOP_PD_EN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable power down wifi in sleep"]
    pub mod WIFI_PD_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable power down digital wrap in sleep"]
    pub mod DG_WRAP_PD_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod DIG_ISO {
    #[doc = "DIG_ISO force off"]
    pub mod FORCE_OFF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DIG_ISO force on"]
    pub mod FORCE_ON {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "read only register to indicate digital pad auto-hold status"]
    pub mod DG_PAD_AUTOHOLD {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wtite only register to clear digital pad auto-hold"]
    pub mod CLR_DG_PAD_AUTOHOLD {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital pad enable auto-hold"]
    pub mod DG_PAD_AUTOHOLD_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital pad force no ISO"]
    pub mod DG_PAD_FORCE_NOISO {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital pad force ISO"]
    pub mod DG_PAD_FORCE_ISO {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital pad force un-hold"]
    pub mod DG_PAD_FORCE_UNHOLD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital pad force hold"]
    pub mod DG_PAD_FORCE_HOLD {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bt force ISO"]
    pub mod BT_FORCE_ISO {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bt force no ISO"]
    pub mod BT_FORCE_NOISO {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Digital peri force ISO"]
    pub mod DG_PERI_FORCE_ISO {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital peri force no ISO"]
    pub mod DG_PERI_FORCE_NOISO {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "cpu force ISO"]
    pub mod CPU_TOP_FORCE_ISO {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "cpu force no ISO"]
    pub mod CPU_TOP_FORCE_NOISO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wifi force ISO"]
    pub mod WIFI_FORCE_ISO {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wifi force no ISO"]
    pub mod WIFI_FORCE_NOISO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital core force ISO"]
    pub mod DG_WRAP_FORCE_ISO {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital core force no ISO"]
    pub mod DG_WRAP_FORCE_NOISO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod WDTCONFIG0 {
    #[doc = "chip reset siginal pulse width"]
    pub mod WDT_CHIP_RESET_WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wdt reset whole chip enable"]
    pub mod WDT_CHIP_RESET_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pause WDT in sleep"]
    pub mod WDT_PAUSE_IN_SLP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable WDT reset APP CPU"]
    pub mod WDT_APPCPU_RESET_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable WDT reset PRO CPU"]
    pub mod WDT_PROCPU_RESET_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable WDT in flash boot"]
    pub mod WDT_FLASHBOOT_MOD_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "system reset counter length"]
    pub mod WDT_SYS_RESET_LENGTH {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU reset counter length"]
    pub mod WDT_CPU_RESET_LENGTH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: interrupt stage en"]
    pub mod WDT_STG3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: interrupt stage en"]
    pub mod WDT_STG2 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: interrupt stage en"]
    pub mod WDT_STG1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: interrupt stage en"]
    pub mod WDT_STG0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable rtc wdt"]
    pub mod WDT_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod WDTCONFIG1 {
    #[doc = "the hold time of stage0"]
    pub mod WDT_STG0_HOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod WDTCONFIG2 {
    #[doc = "the hold time of stage1"]
    pub mod WDT_STG1_HOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod WDTCONFIG3 {
    #[doc = "the hold time of stage2"]
    pub mod WDT_STG2_HOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod WDTCONFIG4 {
    #[doc = "the hold time of stage3"]
    pub mod WDT_STG3_HOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod WDTFEED {
    #[doc = "sw feed rtc wdt"]
    pub mod WDT_FEED {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod WDTWPROTECT {
    #[doc = "the key of rtc wdt"]
    pub mod WDT_WKEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod SWD_CONF {
    #[doc = "swd reset flag"]
    pub mod SWD_RESET_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "swd interrupt for feeding"]
    pub mod SWD_FEED_INT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass swd rst"]
    pub mod SWD_BYPASS_RST {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "adjust signal width send to swd"]
    pub mod SWD_SIGNAL_WIDTH {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reset swd reset flag"]
    pub mod SWD_RST_FLAG_CLR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sw feed swd"]
    pub mod SWD_FEED {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "disabel SWD"]
    pub mod SWD_DISABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "automatically feed swd when int comes"]
    pub mod SWD_AUTO_FEED_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod SWD_WPROTECT {
    #[doc = "the key of super wdt"]
    pub mod SWD_WKEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod SW_CPU_STALL {
    #[doc = "{reg_sw_stall_appcpu_c1\\[5:0\\]"]
    pub mod SW_STALL_APPCPU_C1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "stall cpu by software"]
    pub mod SW_STALL_PROCPU_C1 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod STORE4 {
    #[doc = "reserved register"]
    pub mod SCRATCH4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod STORE5 {
    #[doc = "reserved register"]
    pub mod SCRATCH5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod STORE6 {
    #[doc = "reserved register"]
    pub mod SCRATCH6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod STORE7 {
    #[doc = "reserved register"]
    pub mod SCRATCH7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod LOW_POWER_ST {
    #[doc = "rom0 power down"]
    pub mod XPD_ROM0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External DCDC power down"]
    pub mod XPD_DIG_DCDC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "rtc peripheral iso"]
    pub mod PERI_ISO {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "rtc peripheral power down"]
    pub mod XPD_RTC_PERI {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wifi iso"]
    pub mod WIFI_ISO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wifi wrap power down"]
    pub mod XPD_WIFI {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital wrap iso"]
    pub mod DIG_ISO {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "digital wrap power down"]
    pub mod XPD_DIG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "touch should start to work"]
    pub mod TOUCH_STATE_START {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "touch is about to working. Switch rtc main state"]
    pub mod TOUCH_STATE_SWITCH {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "touch is in sleep state"]
    pub mod TOUCH_STATE_SLP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "touch is done"]
    pub mod TOUCH_STATE_DONE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ulp/cocpu should start to work"]
    pub mod COCPU_STATE_START {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ulp/cocpu is about to working. Switch rtc main state"]
    pub mod COCPU_STATE_SWITCH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ulp/cocpu is in sleep state"]
    pub mod COCPU_STATE_SLP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ulp/cocpu is done"]
    pub mod COCPU_STATE_DONE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "no use any more"]
    pub mod MAIN_STATE_XTAL_ISO {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "rtc main state machine is in states that pll should be running"]
    pub mod MAIN_STATE_PLL_ON {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "rtc is ready to receive wake up trigger from wake up source"]
    pub mod RDY_FOR_WAKEUP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "rtc main state machine has been waited for some cycles"]
    pub mod MAIN_STATE_WAIT_END {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "rtc main state machine is in the states of wakeup process"]
    pub mod IN_WAKEUP_STATE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "rtc main state machine is in the states of low power"]
    pub mod IN_LOW_POWER_STATE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "rtc main state machine is in wait 8m state"]
    pub mod MAIN_STATE_IN_WAIT_8M {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "rtc main state machine is in wait pll state"]
    pub mod MAIN_STATE_IN_WAIT_PLL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "rtc main state machine is in wait xtal state"]
    pub mod MAIN_STATE_IN_WAIT_XTL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "rtc main state machine is in sleep state"]
    pub mod MAIN_STATE_IN_SLP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "rtc main state machine is in idle state"]
    pub mod MAIN_STATE_IN_IDLE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "rtc main state machine status"]
    pub mod MAIN_STATE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod DIAG0 {
    pub mod LOW_POWER_DIAG1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod PAD_HOLD {
    #[doc = "the hold configure of rtc gpio0"]
    pub mod GPIO_PIN0_HOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the hold configure of rtc gpio1"]
    pub mod GPIO_PIN1_HOLD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the hold configure of rtc gpio2"]
    pub mod GPIO_PIN2_HOLD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the hold configure of rtc gpio3"]
    pub mod GPIO_PIN3_HOLD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the hold configure of rtc gpio4"]
    pub mod GPIO_PIN4_HOLD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the hold configure of rtc gpio5"]
    pub mod GPIO_PIN5_HOLD {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod DIG_PAD_HOLD {
    #[doc = "the configure of digital pad"]
    pub mod DIG_PAD_HOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod BROWN_OUT {
    #[doc = "brown out interrupt wait cycles"]
    pub mod BROWN_OUT_INT_WAIT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable close flash when brown out happens"]
    pub mod BROWN_OUT_CLOSE_FLASH_ENA {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable power down RF when brown out happens"]
    pub mod BROWN_OUT_PD_RF_ENA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "brown out reset wait cycles"]
    pub mod BROWN_OUT_RST_WAIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable brown out reset"]
    pub mod BROWN_OUT_RST_ENA {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: 4-pos reset"]
    pub mod BROWN_OUT_RST_SEL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "brown_out origin reset enable"]
    pub mod BROWN_OUT_ANA_RST_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clear brown out counter"]
    pub mod BROWN_OUT_CNT_CLR {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable brown out"]
    pub mod BROWN_OUT_ENA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the flag of brown det from analog"]
    pub mod DET {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod TIME_LOW1 {
    #[doc = "RTC timer low 32 bits"]
    pub mod TIMER_VALUE1_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod TIME_HIGH1 {
    #[doc = "RTC timer high 16 bits"]
    pub mod TIMER_VALUE1_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod XTAL32K_CLK_FACTOR {
    #[doc = "xtal 32k watch dog backup clock factor"]
    pub mod XTAL32K_CLK_FACTOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod XTAL32K_CONF {
    #[doc = "cycles to wait to return noral xtal 32k"]
    pub mod XTAL32K_RETURN_WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "cycles to wait to repower on xtal 32k"]
    pub mod XTAL32K_RESTART_WAIT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If no clock detected for this amount of time"]
    pub mod XTAL32K_WDT_TIMEOUT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "if restarted xtal32k period is smaller than this"]
    pub mod XTAL32K_STABLE_THRES {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod USB_CONF {
    #[doc = "disable io_mux reset"]
    pub mod IO_MUX_RESET_DISABLE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RTC_CNTL_RTC_SLP_REJECT_CAUSE_REG"]
pub mod SLP_REJECT_CAUSE {
    #[doc = "sleep reject cause"]
    pub mod REJECT_CAUSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod OPTION1 {
    #[doc = "force chip entry download mode"]
    pub mod FORCE_DOWNLOAD_BOOT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RTC_CNTL_RTC_SLP_WAKEUP_CAUSE_REG"]
pub mod SLP_WAKEUP_CAUSE {
    #[doc = "sleep wakeup cause"]
    pub mod WAKEUP_CAUSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod ULP_CP_TIMER_1 {
    #[doc = "sleep cycles for ULP-coprocessor timer"]
    pub mod ULP_CP_TIMER_SLP_CYCLE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod INT_ENA_RTC_W1TS {
    #[doc = "enable sleep wakeup interrupt"]
    pub mod SLP_WAKEUP_INT_ENA_W1TS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable sleep reject interrupt"]
    pub mod SLP_REJECT_INT_ENA_W1TS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable RTC WDT interrupt"]
    pub mod WDT_INT_ENA_W1TS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable brown out interrupt"]
    pub mod BROWN_OUT_INT_ENA_W1TS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable RTC main timer interrupt"]
    pub mod MAIN_TIMER_INT_ENA_W1TS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable super watch dog interrupt"]
    pub mod SWD_INT_ENA_W1TS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable xtal32k_dead interrupt"]
    pub mod XTAL32K_DEAD_INT_ENA_W1TS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enbale gitch det interrupt"]
    pub mod GLITCH_DET_INT_ENA_W1TS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enbale bbpll cal interrupt"]
    pub mod BBPLL_CAL_INT_ENA_W1TS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod INT_ENA_RTC_W1TC {
    #[doc = "clear sleep wakeup interrupt enable"]
    pub mod SLP_WAKEUP_INT_ENA_W1TC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clear sleep reject interrupt enable"]
    pub mod SLP_REJECT_INT_ENA_W1TC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clear RTC WDT interrupt enable"]
    pub mod WDT_INT_ENA_W1TC {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clear brown out interrupt enable"]
    pub mod BROWN_OUT_INT_ENA_W1TC {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear RTC main timer interrupt enable"]
    pub mod MAIN_TIMER_INT_ENA_W1TC {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clear super watch dog interrupt enable"]
    pub mod SWD_INT_ENA_W1TC {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clear xtal32k_dead interrupt enable"]
    pub mod XTAL32K_DEAD_INT_ENA_W1TC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clear gitch det interrupt enable"]
    pub mod GLITCH_DET_INT_ENA_W1TC {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clear bbpll cal interrupt enable"]
    pub mod BBPLL_CAL_INT_ENA_W1TC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod RETENTION_CTRL {
    #[doc = "Retention clk sel"]
    pub mod RETENTION_CLK_SEL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Retention done wait time"]
    pub mod RETENTION_DONE_WAIT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Retention clkoff wait time"]
    pub mod RETENTION_CLKOFF_WAIT {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable cpu retention when light sleep"]
    pub mod RETENTION_EN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wait cycles for rention operation"]
    pub mod RETENTION_WAIT {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod FIB_SEL {
    #[doc = "select use analog fib signal"]
    pub mod FIB_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod GPIO_WAKEUP {
    #[doc = "rtc gpio wakeup flag"]
    pub mod GPIO_WAKEUP_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clear rtc gpio wakeup flag"]
    pub mod GPIO_WAKEUP_STATUS_CLR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable rtc io clk gate"]
    pub mod GPIO_PIN_CLK_GATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configure gpio wakeup type"]
    pub mod GPIO_PIN5_INT_TYPE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configure gpio wakeup type"]
    pub mod GPIO_PIN4_INT_TYPE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configure gpio wakeup type"]
    pub mod GPIO_PIN3_INT_TYPE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configure gpio wakeup type"]
    pub mod GPIO_PIN2_INT_TYPE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configure gpio wakeup type"]
    pub mod GPIO_PIN1_INT_TYPE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configure gpio wakeup type"]
    pub mod GPIO_PIN0_INT_TYPE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable wakeup from rtc gpio5"]
    pub mod GPIO_PIN5_WAKEUP_ENABLE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable wakeup from rtc gpio4"]
    pub mod GPIO_PIN4_WAKEUP_ENABLE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable wakeup from rtc gpio3"]
    pub mod GPIO_PIN3_WAKEUP_ENABLE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable wakeup from rtc gpio2"]
    pub mod GPIO_PIN2_WAKEUP_ENABLE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable wakeup from rtc gpio1"]
    pub mod GPIO_PIN1_WAKEUP_ENABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable wakeup from rtc gpio0"]
    pub mod GPIO_PIN0_WAKEUP_ENABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod DBG_SEL {
    #[doc = "use for debug"]
    pub mod DEBUG_12M_NO_GATING {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod DEBUG_BIT_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod DEBUG_SEL0 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod DEBUG_SEL1 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod DEBUG_SEL2 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod DEBUG_SEL3 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod DEBUG_SEL4 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod DBG_MAP {
    #[doc = "use for debug"]
    pub mod GPIO_PIN5_MUX_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod GPIO_PIN4_MUX_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod GPIO_PIN3_MUX_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod GPIO_PIN2_MUX_SEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod GPIO_PIN1_MUX_SEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod GPIO_PIN0_MUX_SEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod GPIO_PIN5_FUN_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod GPIO_PIN4_FUN_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod GPIO_PIN3_FUN_SEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod GPIO_PIN2_FUN_SEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod GPIO_PIN1_FUN_SEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use for debug"]
    pub mod GPIO_PIN0_FUN_SEL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod SENSOR_CTRL {
    #[doc = "reg_sar2_pwdet_cct"]
    pub mod SAR2_PWDET_CCT {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "force power up SAR"]
    pub mod FORCE_XPD_SAR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod DBG_SAR_SEL {
    #[doc = "use for debug"]
    pub mod SAR_DEBUG_SEL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod PG_CTRL {
    #[doc = "power glitch desense"]
    pub mod POWER_GLITCH_DSENSE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "force disable power glitch"]
    pub mod POWER_GLITCH_FORCE_PD {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "force enable power glitch"]
    pub mod POWER_GLITCH_FORCE_PU {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "use efuse value control power glitch enable"]
    pub mod POWER_GLITCH_EFUSE_SEL {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable power glitch"]
    pub mod POWER_GLITCH_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc configure register"]
pub mod DATE {
    #[doc = "verision"]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
