#[doc = "LED Control PWM (Pulse Width Modulation)"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "LEDC_LSCH%s_CONF%s."]
    pub CH_CONF00: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_HPOINT."]
    pub CH_HPOINT0: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_DUTY."]
    pub CH_DUTY0: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_CONF1."]
    pub CH_CONF10: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_DUTY_R."]
    pub CH_DUTY_R0: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_CONF%s."]
    pub CH_CONF01: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_HPOINT."]
    pub CH_HPOINT1: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_DUTY."]
    pub CH_DUTY1: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_CONF1."]
    pub CH_CONF11: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_DUTY_R."]
    pub CH_DUTY_R1: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_CONF%s."]
    pub CH_CONF02: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_HPOINT."]
    pub CH_HPOINT2: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_DUTY."]
    pub CH_DUTY2: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_CONF1."]
    pub CH_CONF12: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_DUTY_R."]
    pub CH_DUTY_R2: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_CONF%s."]
    pub CH_CONF03: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_HPOINT."]
    pub CH_HPOINT3: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_DUTY."]
    pub CH_DUTY3: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_CONF1."]
    pub CH_CONF13: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_DUTY_R."]
    pub CH_DUTY_R3: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_CONF%s."]
    pub CH_CONF04: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_HPOINT."]
    pub CH_HPOINT4: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_DUTY."]
    pub CH_DUTY4: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_CONF1."]
    pub CH_CONF14: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_DUTY_R."]
    pub CH_DUTY_R4: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_CONF%s."]
    pub CH_CONF05: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_HPOINT."]
    pub CH_HPOINT5: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_DUTY."]
    pub CH_DUTY5: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_CONF1."]
    pub CH_CONF15: crate::RWRegister<u32>,
    #[doc = "LEDC_LSCH%s_DUTY_R."]
    pub CH_DUTY_R5: crate::RWRegister<u32>,
    _reserved0: [u8; 0x28],
    #[doc = "LEDC_LSTIMER%s_CONF."]
    pub TIMER_CONF0: crate::RWRegister<u32>,
    #[doc = "LEDC_LSTIMER%s_VALUE."]
    pub TIMER_VALUE0: crate::RWRegister<u32>,
    #[doc = "LEDC_LSTIMER%s_CONF."]
    pub TIMER_CONF1: crate::RWRegister<u32>,
    #[doc = "LEDC_LSTIMER%s_VALUE."]
    pub TIMER_VALUE1: crate::RWRegister<u32>,
    #[doc = "LEDC_LSTIMER%s_CONF."]
    pub TIMER_CONF2: crate::RWRegister<u32>,
    #[doc = "LEDC_LSTIMER%s_VALUE."]
    pub TIMER_VALUE2: crate::RWRegister<u32>,
    #[doc = "LEDC_LSTIMER%s_CONF."]
    pub TIMER_CONF3: crate::RWRegister<u32>,
    #[doc = "LEDC_LSTIMER%s_VALUE."]
    pub TIMER_VALUE3: crate::RWRegister<u32>,
    #[doc = "LEDC_INT_RAW."]
    pub INT_RAW: crate::RWRegister<u32>,
    #[doc = "LEDC_INT_ST."]
    pub INT_ST: crate::RWRegister<u32>,
    #[doc = "LEDC_INT_ENA."]
    pub INT_ENA: crate::RWRegister<u32>,
    #[doc = "LEDC_INT_CLR."]
    pub INT_CLR: crate::RWRegister<u32>,
    #[doc = "LEDC_CONF."]
    pub CONF: crate::RWRegister<u32>,
    _reserved1: [u8; 0x28],
    #[doc = "LEDC_DATE."]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "LEDC_LSCH%s_CONF%s."]
pub mod CH_CONF00 {
    #[doc = "reg_timer_sel_lsch0."]
    pub mod TIMER_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sig_out_en_lsch0."]
    pub mod SIG_OUT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_idle_lv_lsch0."]
    pub mod IDLE_LV {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_para_up_lsch0."]
    pub mod PARA_UP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_num_lsch0."]
    pub mod OVF_NUM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_en_lsch0."]
    pub mod OVF_CNT_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_reset_lsch0."]
    pub mod OVF_CNT_RESET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_HPOINT."]
pub mod CH_HPOINT0 {
    #[doc = "reg_hpoint_lsch0."]
    pub mod HPOINT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_DUTY."]
pub mod CH_DUTY0 {
    #[doc = "reg_duty_lsch0."]
    pub mod DUTY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_CONF1."]
pub mod CH_CONF10 {
    #[doc = "reg_duty_scale_lsch0."]
    pub mod DUTY_SCALE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_cycle_lsch0."]
    pub mod DUTY_CYCLE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_num_lsch0."]
    pub mod DUTY_NUM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_inc_lsch0."]
    pub mod DUTY_INC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_start_lsch0."]
    pub mod DUTY_START {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_DUTY_R."]
pub mod CH_DUTY_R0 {
    #[doc = "reg_duty_lsch0_r."]
    pub mod DUTY_R {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_CONF%s."]
pub mod CH_CONF01 {
    #[doc = "reg_timer_sel_lsch0."]
    pub mod TIMER_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sig_out_en_lsch0."]
    pub mod SIG_OUT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_idle_lv_lsch0."]
    pub mod IDLE_LV {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_para_up_lsch0."]
    pub mod PARA_UP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_num_lsch0."]
    pub mod OVF_NUM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_en_lsch0."]
    pub mod OVF_CNT_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_reset_lsch0."]
    pub mod OVF_CNT_RESET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_HPOINT."]
pub mod CH_HPOINT1 {
    #[doc = "reg_hpoint_lsch0."]
    pub mod HPOINT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_DUTY."]
pub mod CH_DUTY1 {
    #[doc = "reg_duty_lsch0."]
    pub mod DUTY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_CONF1."]
pub mod CH_CONF11 {
    #[doc = "reg_duty_scale_lsch0."]
    pub mod DUTY_SCALE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_cycle_lsch0."]
    pub mod DUTY_CYCLE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_num_lsch0."]
    pub mod DUTY_NUM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_inc_lsch0."]
    pub mod DUTY_INC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_start_lsch0."]
    pub mod DUTY_START {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_DUTY_R."]
pub mod CH_DUTY_R1 {
    #[doc = "reg_duty_lsch0_r."]
    pub mod DUTY_R {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_CONF%s."]
pub mod CH_CONF02 {
    #[doc = "reg_timer_sel_lsch0."]
    pub mod TIMER_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sig_out_en_lsch0."]
    pub mod SIG_OUT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_idle_lv_lsch0."]
    pub mod IDLE_LV {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_para_up_lsch0."]
    pub mod PARA_UP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_num_lsch0."]
    pub mod OVF_NUM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_en_lsch0."]
    pub mod OVF_CNT_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_reset_lsch0."]
    pub mod OVF_CNT_RESET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_HPOINT."]
pub mod CH_HPOINT2 {
    #[doc = "reg_hpoint_lsch0."]
    pub mod HPOINT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_DUTY."]
pub mod CH_DUTY2 {
    #[doc = "reg_duty_lsch0."]
    pub mod DUTY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_CONF1."]
pub mod CH_CONF12 {
    #[doc = "reg_duty_scale_lsch0."]
    pub mod DUTY_SCALE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_cycle_lsch0."]
    pub mod DUTY_CYCLE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_num_lsch0."]
    pub mod DUTY_NUM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_inc_lsch0."]
    pub mod DUTY_INC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_start_lsch0."]
    pub mod DUTY_START {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_DUTY_R."]
pub mod CH_DUTY_R2 {
    #[doc = "reg_duty_lsch0_r."]
    pub mod DUTY_R {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_CONF%s."]
pub mod CH_CONF03 {
    #[doc = "reg_timer_sel_lsch0."]
    pub mod TIMER_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sig_out_en_lsch0."]
    pub mod SIG_OUT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_idle_lv_lsch0."]
    pub mod IDLE_LV {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_para_up_lsch0."]
    pub mod PARA_UP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_num_lsch0."]
    pub mod OVF_NUM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_en_lsch0."]
    pub mod OVF_CNT_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_reset_lsch0."]
    pub mod OVF_CNT_RESET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_HPOINT."]
pub mod CH_HPOINT3 {
    #[doc = "reg_hpoint_lsch0."]
    pub mod HPOINT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_DUTY."]
pub mod CH_DUTY3 {
    #[doc = "reg_duty_lsch0."]
    pub mod DUTY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_CONF1."]
pub mod CH_CONF13 {
    #[doc = "reg_duty_scale_lsch0."]
    pub mod DUTY_SCALE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_cycle_lsch0."]
    pub mod DUTY_CYCLE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_num_lsch0."]
    pub mod DUTY_NUM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_inc_lsch0."]
    pub mod DUTY_INC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_start_lsch0."]
    pub mod DUTY_START {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_DUTY_R."]
pub mod CH_DUTY_R3 {
    #[doc = "reg_duty_lsch0_r."]
    pub mod DUTY_R {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_CONF%s."]
pub mod CH_CONF04 {
    #[doc = "reg_timer_sel_lsch0."]
    pub mod TIMER_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sig_out_en_lsch0."]
    pub mod SIG_OUT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_idle_lv_lsch0."]
    pub mod IDLE_LV {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_para_up_lsch0."]
    pub mod PARA_UP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_num_lsch0."]
    pub mod OVF_NUM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_en_lsch0."]
    pub mod OVF_CNT_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_reset_lsch0."]
    pub mod OVF_CNT_RESET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_HPOINT."]
pub mod CH_HPOINT4 {
    #[doc = "reg_hpoint_lsch0."]
    pub mod HPOINT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_DUTY."]
pub mod CH_DUTY4 {
    #[doc = "reg_duty_lsch0."]
    pub mod DUTY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_CONF1."]
pub mod CH_CONF14 {
    #[doc = "reg_duty_scale_lsch0."]
    pub mod DUTY_SCALE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_cycle_lsch0."]
    pub mod DUTY_CYCLE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_num_lsch0."]
    pub mod DUTY_NUM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_inc_lsch0."]
    pub mod DUTY_INC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_start_lsch0."]
    pub mod DUTY_START {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_DUTY_R."]
pub mod CH_DUTY_R4 {
    #[doc = "reg_duty_lsch0_r."]
    pub mod DUTY_R {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_CONF%s."]
pub mod CH_CONF05 {
    #[doc = "reg_timer_sel_lsch0."]
    pub mod TIMER_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sig_out_en_lsch0."]
    pub mod SIG_OUT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_idle_lv_lsch0."]
    pub mod IDLE_LV {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_para_up_lsch0."]
    pub mod PARA_UP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_num_lsch0."]
    pub mod OVF_NUM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_en_lsch0."]
    pub mod OVF_CNT_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_reset_lsch0."]
    pub mod OVF_CNT_RESET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_HPOINT."]
pub mod CH_HPOINT5 {
    #[doc = "reg_hpoint_lsch0."]
    pub mod HPOINT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_DUTY."]
pub mod CH_DUTY5 {
    #[doc = "reg_duty_lsch0."]
    pub mod DUTY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_CONF1."]
pub mod CH_CONF15 {
    #[doc = "reg_duty_scale_lsch0."]
    pub mod DUTY_SCALE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_cycle_lsch0."]
    pub mod DUTY_CYCLE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_num_lsch0."]
    pub mod DUTY_NUM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_inc_lsch0."]
    pub mod DUTY_INC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_start_lsch0."]
    pub mod DUTY_START {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSCH%s_DUTY_R."]
pub mod CH_DUTY_R5 {
    #[doc = "reg_duty_lsch0_r."]
    pub mod DUTY_R {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSTIMER%s_CONF."]
pub mod TIMER_CONF0 {
    #[doc = "reg_lstimer0_duty_res."]
    pub mod DUTY_RES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_div_lstimer0."]
    pub mod CLK_DIV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer0_pause."]
    pub mod PAUSE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer0_rst."]
    pub mod RST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_tick_sel_lstimer0."]
    pub mod TICK_SEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer0_para_up."]
    pub mod PARA_UP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSTIMER%s_VALUE."]
pub mod TIMER_VALUE0 {
    #[doc = "reg_lstimer0_cnt."]
    pub mod CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSTIMER%s_CONF."]
pub mod TIMER_CONF1 {
    #[doc = "reg_lstimer0_duty_res."]
    pub mod DUTY_RES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_div_lstimer0."]
    pub mod CLK_DIV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer0_pause."]
    pub mod PAUSE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer0_rst."]
    pub mod RST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_tick_sel_lstimer0."]
    pub mod TICK_SEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer0_para_up."]
    pub mod PARA_UP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSTIMER%s_VALUE."]
pub mod TIMER_VALUE1 {
    #[doc = "reg_lstimer0_cnt."]
    pub mod CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSTIMER%s_CONF."]
pub mod TIMER_CONF2 {
    #[doc = "reg_lstimer0_duty_res."]
    pub mod DUTY_RES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_div_lstimer0."]
    pub mod CLK_DIV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer0_pause."]
    pub mod PAUSE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer0_rst."]
    pub mod RST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_tick_sel_lstimer0."]
    pub mod TICK_SEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer0_para_up."]
    pub mod PARA_UP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSTIMER%s_VALUE."]
pub mod TIMER_VALUE2 {
    #[doc = "reg_lstimer0_cnt."]
    pub mod CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSTIMER%s_CONF."]
pub mod TIMER_CONF3 {
    #[doc = "reg_lstimer0_duty_res."]
    pub mod DUTY_RES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_div_lstimer0."]
    pub mod CLK_DIV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer0_pause."]
    pub mod PAUSE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer0_rst."]
    pub mod RST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_tick_sel_lstimer0."]
    pub mod TICK_SEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer0_para_up."]
    pub mod PARA_UP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_LSTIMER%s_VALUE."]
pub mod TIMER_VALUE3 {
    #[doc = "reg_lstimer0_cnt."]
    pub mod CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_INT_RAW."]
pub mod INT_RAW {
    #[doc = "reg_lstimer0_ovf_int_raw."]
    pub mod LSTIMER0_OVF_INT_RAW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer1_ovf_int_raw."]
    pub mod LSTIMER1_OVF_INT_RAW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer2_ovf_int_raw."]
    pub mod LSTIMER2_OVF_INT_RAW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer3_ovf_int_raw."]
    pub mod LSTIMER3_OVF_INT_RAW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch0_int_raw."]
    pub mod DUTY_CHNG_END_LSCH0_INT_RAW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch1_int_raw."]
    pub mod DUTY_CHNG_END_LSCH1_INT_RAW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch2_int_raw."]
    pub mod DUTY_CHNG_END_LSCH2_INT_RAW {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch3_int_raw."]
    pub mod DUTY_CHNG_END_LSCH3_INT_RAW {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch4_int_raw."]
    pub mod DUTY_CHNG_END_LSCH4_INT_RAW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch5_int_raw."]
    pub mod DUTY_CHNG_END_LSCH5_INT_RAW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch0_int_raw."]
    pub mod OVF_CNT_LSCH0_INT_RAW {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch1_int_raw."]
    pub mod OVF_CNT_LSCH1_INT_RAW {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch2_int_raw."]
    pub mod OVF_CNT_LSCH2_INT_RAW {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch3_int_raw."]
    pub mod OVF_CNT_LSCH3_INT_RAW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch4_int_raw."]
    pub mod OVF_CNT_LSCH4_INT_RAW {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch5_int_raw."]
    pub mod OVF_CNT_LSCH5_INT_RAW {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_INT_ST."]
pub mod INT_ST {
    #[doc = "reg_lstimer0_ovf_int_st."]
    pub mod LSTIMER0_OVF_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer1_ovf_int_st."]
    pub mod LSTIMER1_OVF_INT_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer2_ovf_int_st."]
    pub mod LSTIMER2_OVF_INT_ST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer3_ovf_int_st."]
    pub mod LSTIMER3_OVF_INT_ST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch0_int_st."]
    pub mod DUTY_CHNG_END_LSCH0_INT_ST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch1_int_st."]
    pub mod DUTY_CHNG_END_LSCH1_INT_ST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch2_int_st."]
    pub mod DUTY_CHNG_END_LSCH2_INT_ST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch3_int_st."]
    pub mod DUTY_CHNG_END_LSCH3_INT_ST {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch4_int_st."]
    pub mod DUTY_CHNG_END_LSCH4_INT_ST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch5_int_st."]
    pub mod DUTY_CHNG_END_LSCH5_INT_ST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch0_int_st."]
    pub mod OVF_CNT_LSCH0_INT_ST {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch1_int_st."]
    pub mod OVF_CNT_LSCH1_INT_ST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch2_int_st."]
    pub mod OVF_CNT_LSCH2_INT_ST {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch3_int_st."]
    pub mod OVF_CNT_LSCH3_INT_ST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch4_int_st."]
    pub mod OVF_CNT_LSCH4_INT_ST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch5_int_st."]
    pub mod OVF_CNT_LSCH5_INT_ST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_INT_ENA."]
pub mod INT_ENA {
    #[doc = "reg_lstimer0_ovf_int_ena."]
    pub mod LSTIMER0_OVF_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer1_ovf_int_ena."]
    pub mod LSTIMER1_OVF_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer2_ovf_int_ena."]
    pub mod LSTIMER2_OVF_INT_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer3_ovf_int_ena."]
    pub mod LSTIMER3_OVF_INT_ENA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch0_int_ena."]
    pub mod DUTY_CHNG_END_LSCH0_INT_ENA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch1_int_ena."]
    pub mod DUTY_CHNG_END_LSCH1_INT_ENA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch2_int_ena."]
    pub mod DUTY_CHNG_END_LSCH2_INT_ENA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch3_int_ena."]
    pub mod DUTY_CHNG_END_LSCH3_INT_ENA {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch4_int_ena."]
    pub mod DUTY_CHNG_END_LSCH4_INT_ENA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch5_int_ena."]
    pub mod DUTY_CHNG_END_LSCH5_INT_ENA {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch0_int_ena."]
    pub mod OVF_CNT_LSCH0_INT_ENA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch1_int_ena."]
    pub mod OVF_CNT_LSCH1_INT_ENA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch2_int_ena."]
    pub mod OVF_CNT_LSCH2_INT_ENA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch3_int_ena."]
    pub mod OVF_CNT_LSCH3_INT_ENA {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch4_int_ena."]
    pub mod OVF_CNT_LSCH4_INT_ENA {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch5_int_ena."]
    pub mod OVF_CNT_LSCH5_INT_ENA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_INT_CLR."]
pub mod INT_CLR {
    #[doc = "reg_lstimer0_ovf_int_clr."]
    pub mod LSTIMER0_OVF_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer1_ovf_int_clr."]
    pub mod LSTIMER1_OVF_INT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer2_ovf_int_clr."]
    pub mod LSTIMER2_OVF_INT_CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lstimer3_ovf_int_clr."]
    pub mod LSTIMER3_OVF_INT_CLR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch0_int_clr."]
    pub mod DUTY_CHNG_END_LSCH0_INT_CLR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch1_int_clr."]
    pub mod DUTY_CHNG_END_LSCH1_INT_CLR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch2_int_clr."]
    pub mod DUTY_CHNG_END_LSCH2_INT_CLR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch3_int_clr."]
    pub mod DUTY_CHNG_END_LSCH3_INT_CLR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch4_int_clr."]
    pub mod DUTY_CHNG_END_LSCH4_INT_CLR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_duty_chng_end_lsch5_int_clr."]
    pub mod DUTY_CHNG_END_LSCH5_INT_CLR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch0_int_clr."]
    pub mod OVF_CNT_LSCH0_INT_CLR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch1_int_clr."]
    pub mod OVF_CNT_LSCH1_INT_CLR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch2_int_clr."]
    pub mod OVF_CNT_LSCH2_INT_CLR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch3_int_clr."]
    pub mod OVF_CNT_LSCH3_INT_CLR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch4_int_clr."]
    pub mod OVF_CNT_LSCH4_INT_CLR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ovf_cnt_lsch5_int_clr."]
    pub mod OVF_CNT_LSCH5_INT_CLR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LEDC_CONF."]
pub mod CONF {
    #[doc = "reg_apb_clk_sel."]
    pub mod APB_CLK_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
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
#[doc = "LEDC_DATE."]
pub mod DATE {
    #[doc = "reg_ledc_date."]
    pub mod LEDC_DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
