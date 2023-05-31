#[doc = "System Timer"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "SYSTIMER_CONF."]
    pub CONF: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_UNIT0_OP."]
    pub UNIT0_OP: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_UNIT1_OP."]
    pub UNIT1_OP: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_UNIT0_LOAD_HI."]
    pub UNIT0_LOAD_HI: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_UNIT0_LOAD_LO."]
    pub UNIT0_LOAD_LO: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_UNIT1_LOAD_HI."]
    pub UNIT1_LOAD_HI: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_UNIT1_LOAD_LO."]
    pub UNIT1_LOAD_LO: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_TARGET0_HI."]
    pub TARGET0_HI: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_TARGET0_LO."]
    pub TARGET0_LO: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_TARGET1_HI."]
    pub TARGET1_HI: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_TARGET1_LO."]
    pub TARGET1_LO: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_TARGET2_HI."]
    pub TARGET2_HI: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_TARGET2_LO."]
    pub TARGET2_LO: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_TARGET0_CONF."]
    pub TARGET0_CONF: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_TARGET1_CONF."]
    pub TARGET1_CONF: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_TARGET2_CONF."]
    pub TARGET2_CONF: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_UNIT0_VALUE_HI."]
    pub UNIT0_VALUE_HI: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_UNIT0_VALUE_LO."]
    pub UNIT0_VALUE_LO: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_UNIT1_VALUE_HI."]
    pub UNIT1_VALUE_HI: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_UNIT1_VALUE_LO."]
    pub UNIT1_VALUE_LO: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_COMP0_LOAD."]
    pub COMP0_LOAD: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_COMP1_LOAD."]
    pub COMP1_LOAD: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_COMP2_LOAD."]
    pub COMP2_LOAD: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_UNIT0_LOAD."]
    pub UNIT0_LOAD: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_UNIT1_LOAD."]
    pub UNIT1_LOAD: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_INT_ENA."]
    pub INT_ENA: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_INT_RAW."]
    pub INT_RAW: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_INT_CLR."]
    pub INT_CLR: crate::RWRegister<u32>,
    #[doc = "SYSTIMER_INT_ST."]
    pub INT_ST: crate::RWRegister<u32>,
    _reserved0: [u8; 0x88],
    #[doc = "SYSTIMER_DATE."]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "SYSTIMER_CONF."]
pub mod CONF {
    #[doc = "systimer clock force on"]
    pub mod SYSTIMER_CLK_FO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "target2 work enable"]
    pub mod TARGET2_WORK_EN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "target1 work enable"]
    pub mod TARGET1_WORK_EN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "target0 work enable"]
    pub mod TARGET0_WORK_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If timer unit1 is stalled when core1 stalled"]
    pub mod TIMER_UNIT1_CORE1_STALL_EN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If timer unit1 is stalled when core0 stalled"]
    pub mod TIMER_UNIT1_CORE0_STALL_EN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If timer unit0 is stalled when core1 stalled"]
    pub mod TIMER_UNIT0_CORE1_STALL_EN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If timer unit0 is stalled when core0 stalled"]
    pub mod TIMER_UNIT0_CORE0_STALL_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "timer unit1 work enable"]
    pub mod TIMER_UNIT1_WORK_EN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "timer unit0 work enable"]
    pub mod TIMER_UNIT0_WORK_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "register file clk gating"]
    pub mod CLK_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_UNIT0_OP."]
pub mod UNIT0_OP {
    #[doc = "reg_timer_unit0_value_valid"]
    pub mod TIMER_UNIT0_VALUE_VALID {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "update timer_unit0"]
    pub mod TIMER_UNIT0_UPDATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_UNIT1_OP."]
pub mod UNIT1_OP {
    #[doc = "timer value is sync and valid"]
    pub mod TIMER_UNIT1_VALUE_VALID {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "update timer unit1"]
    pub mod TIMER_UNIT1_UPDATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_UNIT0_LOAD_HI."]
pub mod UNIT0_LOAD_HI {
    #[doc = "timer unit0 load high 32 bit"]
    pub mod TIMER_UNIT0_LOAD_HI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_UNIT0_LOAD_LO."]
pub mod UNIT0_LOAD_LO {
    #[doc = "timer unit0 load low 32 bit"]
    pub mod TIMER_UNIT0_LOAD_LO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_UNIT1_LOAD_HI."]
pub mod UNIT1_LOAD_HI {
    #[doc = "timer unit1 load high 32 bit"]
    pub mod TIMER_UNIT1_LOAD_HI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_UNIT1_LOAD_LO."]
pub mod UNIT1_LOAD_LO {
    #[doc = "timer unit1 load low 32 bit"]
    pub mod TIMER_UNIT1_LOAD_LO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_TARGET0_HI."]
pub mod TARGET0_HI {
    #[doc = "timer taget0 high 32 bit"]
    pub mod TIMER_TARGET0_HI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_TARGET0_LO."]
pub mod TARGET0_LO {
    #[doc = "timer taget0 low 32 bit"]
    pub mod TIMER_TARGET0_LO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_TARGET1_HI."]
pub mod TARGET1_HI {
    #[doc = "timer taget1 high 32 bit"]
    pub mod TIMER_TARGET1_HI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_TARGET1_LO."]
pub mod TARGET1_LO {
    #[doc = "timer taget1 low 32 bit"]
    pub mod TIMER_TARGET1_LO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_TARGET2_HI."]
pub mod TARGET2_HI {
    #[doc = "timer taget2 high 32 bit"]
    pub mod TIMER_TARGET2_HI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_TARGET2_LO."]
pub mod TARGET2_LO {
    #[doc = "timer taget2 low 32 bit"]
    pub mod TIMER_TARGET2_LO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_TARGET0_CONF."]
pub mod TARGET0_CONF {
    #[doc = "target0 period"]
    pub mod TARGET0_PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set target0 to period mode"]
    pub mod TARGET0_PERIOD_MODE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select which unit to compare"]
    pub mod TARGET0_TIMER_UNIT_SEL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_TARGET1_CONF."]
pub mod TARGET1_CONF {
    #[doc = "target1 period"]
    pub mod TARGET1_PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set target1 to period mode"]
    pub mod TARGET1_PERIOD_MODE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select which unit to compare"]
    pub mod TARGET1_TIMER_UNIT_SEL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_TARGET2_CONF."]
pub mod TARGET2_CONF {
    #[doc = "target2 period"]
    pub mod TARGET2_PERIOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set target2 to period mode"]
    pub mod TARGET2_PERIOD_MODE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select which unit to compare"]
    pub mod TARGET2_TIMER_UNIT_SEL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_UNIT0_VALUE_HI."]
pub mod UNIT0_VALUE_HI {
    #[doc = "timer read value high 32bit"]
    pub mod TIMER_UNIT0_VALUE_HI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_UNIT0_VALUE_LO."]
pub mod UNIT0_VALUE_LO {
    #[doc = "timer read value low 32bit"]
    pub mod TIMER_UNIT0_VALUE_LO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_UNIT1_VALUE_HI."]
pub mod UNIT1_VALUE_HI {
    #[doc = "timer read value high 32bit"]
    pub mod TIMER_UNIT1_VALUE_HI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_UNIT1_VALUE_LO."]
pub mod UNIT1_VALUE_LO {
    #[doc = "timer read value low 32bit"]
    pub mod TIMER_UNIT1_VALUE_LO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_COMP0_LOAD."]
pub mod COMP0_LOAD {
    #[doc = "timer comp0 load value"]
    pub mod TIMER_COMP0_LOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_COMP1_LOAD."]
pub mod COMP1_LOAD {
    #[doc = "timer comp1 load value"]
    pub mod TIMER_COMP1_LOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_COMP2_LOAD."]
pub mod COMP2_LOAD {
    #[doc = "timer comp2 load value"]
    pub mod TIMER_COMP2_LOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_UNIT0_LOAD."]
pub mod UNIT0_LOAD {
    #[doc = "timer unit0 load value"]
    pub mod TIMER_UNIT0_LOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_UNIT1_LOAD."]
pub mod UNIT1_LOAD {
    #[doc = "timer unit1 load value"]
    pub mod TIMER_UNIT1_LOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_INT_ENA."]
pub mod INT_ENA {
    #[doc = "interupt0 enable"]
    pub mod TARGET0_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt1 enable"]
    pub mod TARGET1_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt2 enable"]
    pub mod TARGET2_INT_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_INT_RAW."]
pub mod INT_RAW {
    #[doc = "interupt0 raw"]
    pub mod TARGET0_INT_RAW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt1 raw"]
    pub mod TARGET1_INT_RAW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt2 raw"]
    pub mod TARGET2_INT_RAW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_INT_CLR."]
pub mod INT_CLR {
    #[doc = "interupt0 clear"]
    pub mod TARGET0_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt1 clear"]
    pub mod TARGET1_INT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt2 clear"]
    pub mod TARGET2_INT_CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_INT_ST."]
pub mod INT_ST {
    #[doc = "reg_target0_int_st"]
    pub mod TARGET0_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_target1_int_st"]
    pub mod TARGET1_INT_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_target2_int_st"]
    pub mod TARGET2_INT_ST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTIMER_DATE."]
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
