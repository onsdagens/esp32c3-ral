#[doc = "Debug Assist"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "ASSIST_DEBUG_C0RE_0_MONTR_ENA_REG"]
    pub C0RE_0_MONTR_ENA: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_INTR_RAW_REG"]
    pub CORE_0_INTR_RAW: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_INTR_ENA_REG"]
    pub CORE_0_INTR_ENA: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_INTR_CLR_REG"]
    pub CORE_0_INTR_CLR: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN_REG"]
    pub CORE_0_AREA_DRAM0_0_MIN: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX_REG"]
    pub CORE_0_AREA_DRAM0_0_MAX: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN_REG"]
    pub CORE_0_AREA_DRAM0_1_MIN: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX_REG"]
    pub CORE_0_AREA_DRAM0_1_MAX: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN_REG"]
    pub CORE_0_AREA_PIF_0_MIN: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX_REG"]
    pub CORE_0_AREA_PIF_0_MAX: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN_REG"]
    pub CORE_0_AREA_PIF_1_MIN: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX_REG"]
    pub CORE_0_AREA_PIF_1_MAX: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_AREA_PC_REG"]
    pub CORE_0_AREA_PC: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_AREA_SP_REG"]
    pub CORE_0_AREA_SP: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_SP_MIN_REG"]
    pub CORE_0_SP_MIN: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_SP_MAX_REG"]
    pub CORE_0_SP_MAX: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_SP_PC_REG"]
    pub CORE_0_SP_PC: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_RCD_EN_REG"]
    pub CORE_0_RCD_EN: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC_REG"]
    pub CORE_0_RCD_PDEBUGPC: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP_REG"]
    pub CORE_0_RCD_PDEBUGSP: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP_REG"]
    pub CORE_0_IRAM0_EXCEPTION_MONITOR_0: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1_REG"]
    pub CORE_0_IRAM0_EXCEPTION_MONITOR_1: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0_REG"]
    pub CORE_0_DRAM0_EXCEPTION_MONITOR_0: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1_REG"]
    pub CORE_0_DRAM0_EXCEPTION_MONITOR_1: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1_REG"]
    pub CORE_0_DRAM0_EXCEPTION_MONITOR_2: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3_REG"]
    pub CORE_0_DRAM0_EXCEPTION_MONITOR_3: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_REG"]
    pub CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_REG"]
    pub CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_LOG_SETTING"]
    pub LOG_SETTING: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_LOG_DATA_0_REG"]
    pub LOG_DATA_0: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_LOG_DATA_MASK_REG"]
    pub LOG_DATA_MASK: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_LOG_MIN_REG"]
    pub LOG_MIN: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_LOG_MAX_REG"]
    pub LOG_MAX: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_LOG_MEM_START_REG"]
    pub LOG_MEM_START: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_LOG_MEM_END_REG"]
    pub LOG_MEM_END: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_LOG_MEM_WRITING_ADDR_REG"]
    pub LOG_MEM_WRITING_ADDR: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_LOG_MEM_FULL_FLAG_REG"]
    pub LOG_MEM_FULL_FLAG: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION"]
    pub C0RE_0_LASTPC_BEFORE_EXCEPTION: crate::RWRegister<u32>,
    #[doc = "ASSIST_DEBUG_C0RE_0_DEBUG_MODE"]
    pub C0RE_0_DEBUG_MODE: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0160],
    #[doc = "ASSIST_DEBUG_DATE_REG"]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "ASSIST_DEBUG_C0RE_0_MONTR_ENA_REG"]
pub mod C0RE_0_MONTR_ENA {
    #[doc = "reg_core_0_area_dram0_0_rd_ena"]
    pub mod CORE_0_AREA_DRAM0_0_RD_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_dram0_0_wr_ena"]
    pub mod CORE_0_AREA_DRAM0_0_WR_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_dram0_1_rd_ena"]
    pub mod CORE_0_AREA_DRAM0_1_RD_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_dram0_1_wr_ena"]
    pub mod CORE_0_AREA_DRAM0_1_WR_ENA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_0_rd_ena"]
    pub mod CORE_0_AREA_PIF_0_RD_ENA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_0_wr_ena"]
    pub mod CORE_0_AREA_PIF_0_WR_ENA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_1_rd_ena"]
    pub mod CORE_0_AREA_PIF_1_RD_ENA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_1_wr_ena"]
    pub mod CORE_0_AREA_PIF_1_WR_ENA {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_sp_spill_min_ena"]
    pub mod CORE_0_SP_SPILL_MIN_ENA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_sp_spill_max_ena"]
    pub mod CORE_0_SP_SPILL_MAX_ENA {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_iram0_exception_monitor_ena"]
    pub mod CORE_0_IRAM0_EXCEPTION_MONITOR_ENA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_dram0_exception_monitor_ena"]
    pub mod CORE_0_DRAM0_EXCEPTION_MONITOR_ENA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_INTR_RAW_REG"]
pub mod CORE_0_INTR_RAW {
    #[doc = "reg_core_0_area_dram0_0_rd_raw"]
    pub mod CORE_0_AREA_DRAM0_0_RD_RAW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_dram0_0_wr_raw"]
    pub mod CORE_0_AREA_DRAM0_0_WR_RAW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_dram0_1_rd_raw"]
    pub mod CORE_0_AREA_DRAM0_1_RD_RAW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_dram0_1_wr_raw"]
    pub mod CORE_0_AREA_DRAM0_1_WR_RAW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_0_rd_raw"]
    pub mod CORE_0_AREA_PIF_0_RD_RAW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_0_wr_raw"]
    pub mod CORE_0_AREA_PIF_0_WR_RAW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_1_rd_raw"]
    pub mod CORE_0_AREA_PIF_1_RD_RAW {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_1_wr_raw"]
    pub mod CORE_0_AREA_PIF_1_WR_RAW {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_sp_spill_min_raw"]
    pub mod CORE_0_SP_SPILL_MIN_RAW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_sp_spill_max_raw"]
    pub mod CORE_0_SP_SPILL_MAX_RAW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_iram0_exception_monitor_raw"]
    pub mod CORE_0_IRAM0_EXCEPTION_MONITOR_RAW {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_dram0_exception_monitor_raw"]
    pub mod CORE_0_DRAM0_EXCEPTION_MONITOR_RAW {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_INTR_ENA_REG"]
pub mod CORE_0_INTR_ENA {
    #[doc = "reg_core_0_area_dram0_0_rd_intr_ena"]
    pub mod CORE_0_AREA_DRAM0_0_RD_INTR_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_dram0_0_wr_intr_ena"]
    pub mod CORE_0_AREA_DRAM0_0_WR_INTR_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_dram0_1_rd_intr_ena"]
    pub mod CORE_0_AREA_DRAM0_1_RD_INTR_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_dram0_1_wr_intr_ena"]
    pub mod CORE_0_AREA_DRAM0_1_WR_INTR_ENA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_0_rd_intr_ena"]
    pub mod CORE_0_AREA_PIF_0_RD_INTR_ENA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_0_wr_intr_ena"]
    pub mod CORE_0_AREA_PIF_0_WR_INTR_ENA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_1_rd_intr_ena"]
    pub mod CORE_0_AREA_PIF_1_RD_INTR_ENA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_1_wr_intr_ena"]
    pub mod CORE_0_AREA_PIF_1_WR_INTR_ENA {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_sp_spill_min_intr_ena"]
    pub mod CORE_0_SP_SPILL_MIN_INTR_ENA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_sp_spill_max_intr_ena"]
    pub mod CORE_0_SP_SPILL_MAX_INTR_ENA {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_iram0_exception_monitor_ena"]
    pub mod CORE_0_IRAM0_EXCEPTION_MONITOR_RLS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_dram0_exception_monitor_ena"]
    pub mod CORE_0_DRAM0_EXCEPTION_MONITOR_RLS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_INTR_CLR_REG"]
pub mod CORE_0_INTR_CLR {
    #[doc = "reg_core_0_area_dram0_0_rd_clr"]
    pub mod CORE_0_AREA_DRAM0_0_RD_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_dram0_0_wr_clr"]
    pub mod CORE_0_AREA_DRAM0_0_WR_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_dram0_1_rd_clr"]
    pub mod CORE_0_AREA_DRAM0_1_RD_CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_dram0_1_wr_clr"]
    pub mod CORE_0_AREA_DRAM0_1_WR_CLR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_0_rd_clr"]
    pub mod CORE_0_AREA_PIF_0_RD_CLR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_0_wr_clr"]
    pub mod CORE_0_AREA_PIF_0_WR_CLR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_1_rd_clr"]
    pub mod CORE_0_AREA_PIF_1_RD_CLR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_area_pif_1_wr_clr"]
    pub mod CORE_0_AREA_PIF_1_WR_CLR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_sp_spill_min_clr"]
    pub mod CORE_0_SP_SPILL_MIN_CLR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_sp_spill_max_clr"]
    pub mod CORE_0_SP_SPILL_MAX_CLR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_iram0_exception_monitor_clr"]
    pub mod CORE_0_IRAM0_EXCEPTION_MONITOR_CLR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_dram0_exception_monitor_clr"]
    pub mod CORE_0_DRAM0_EXCEPTION_MONITOR_CLR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN_REG"]
pub mod CORE_0_AREA_DRAM0_0_MIN {
    #[doc = "reg_core_0_area_dram0_0_min"]
    pub mod CORE_0_AREA_DRAM0_0_MIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX_REG"]
pub mod CORE_0_AREA_DRAM0_0_MAX {
    #[doc = "reg_core_0_area_dram0_0_max"]
    pub mod CORE_0_AREA_DRAM0_0_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN_REG"]
pub mod CORE_0_AREA_DRAM0_1_MIN {
    #[doc = "reg_core_0_area_dram0_1_min"]
    pub mod CORE_0_AREA_DRAM0_1_MIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX_REG"]
pub mod CORE_0_AREA_DRAM0_1_MAX {
    #[doc = "reg_core_0_area_dram0_1_max"]
    pub mod CORE_0_AREA_DRAM0_1_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN_REG"]
pub mod CORE_0_AREA_PIF_0_MIN {
    #[doc = "reg_core_0_area_pif_0_min"]
    pub mod CORE_0_AREA_PIF_0_MIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX_REG"]
pub mod CORE_0_AREA_PIF_0_MAX {
    #[doc = "reg_core_0_area_pif_0_max"]
    pub mod CORE_0_AREA_PIF_0_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN_REG"]
pub mod CORE_0_AREA_PIF_1_MIN {
    #[doc = "reg_core_0_area_pif_1_min"]
    pub mod CORE_0_AREA_PIF_1_MIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX_REG"]
pub mod CORE_0_AREA_PIF_1_MAX {
    #[doc = "reg_core_0_area_pif_1_max"]
    pub mod CORE_0_AREA_PIF_1_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PC_REG"]
pub mod CORE_0_AREA_PC {
    #[doc = "reg_core_0_area_pc"]
    pub mod CORE_0_AREA_PC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_SP_REG"]
pub mod CORE_0_AREA_SP {
    #[doc = "reg_core_0_area_sp"]
    pub mod CORE_0_AREA_SP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_SP_MIN_REG"]
pub mod CORE_0_SP_MIN {
    #[doc = "reg_core_0_sp_min"]
    pub mod CORE_0_SP_MIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_SP_MAX_REG"]
pub mod CORE_0_SP_MAX {
    #[doc = "reg_core_0_sp_max"]
    pub mod CORE_0_SP_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_SP_PC_REG"]
pub mod CORE_0_SP_PC {
    #[doc = "reg_core_0_sp_pc"]
    pub mod CORE_0_SP_PC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_RCD_EN_REG"]
pub mod CORE_0_RCD_EN {
    #[doc = "reg_core_0_rcd_recorden"]
    pub mod CORE_0_RCD_RECORDEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_rcd_pdebugen"]
    pub mod CORE_0_RCD_PDEBUGEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC_REG"]
pub mod CORE_0_RCD_PDEBUGPC {
    #[doc = "reg_core_0_rcd_pdebugpc"]
    pub mod CORE_0_RCD_PDEBUGPC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP_REG"]
pub mod CORE_0_RCD_PDEBUGSP {
    #[doc = "reg_core_0_rcd_pdebugsp"]
    pub mod CORE_0_RCD_PDEBUGSP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP_REG"]
pub mod CORE_0_IRAM0_EXCEPTION_MONITOR_0 {
    #[doc = "reg_core_0_iram0_recording_addr_0"]
    pub mod CORE_0_IRAM0_RECORDING_ADDR_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_iram0_recording_wr_0"]
    pub mod CORE_0_IRAM0_RECORDING_WR_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_iram0_recording_loadstore_0"]
    pub mod CORE_0_IRAM0_RECORDING_LOADSTORE_0 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1_REG"]
pub mod CORE_0_IRAM0_EXCEPTION_MONITOR_1 {
    #[doc = "reg_core_0_iram0_recording_addr_1"]
    pub mod CORE_0_IRAM0_RECORDING_ADDR_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_iram0_recording_wr_1"]
    pub mod CORE_0_IRAM0_RECORDING_WR_1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_iram0_recording_loadstore_1"]
    pub mod CORE_0_IRAM0_RECORDING_LOADSTORE_1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0_REG"]
pub mod CORE_0_DRAM0_EXCEPTION_MONITOR_0 {
    #[doc = "reg_core_0_dram0_recording_addr_0"]
    pub mod CORE_0_DRAM0_RECORDING_ADDR_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_dram0_recording_wr_0"]
    pub mod CORE_0_DRAM0_RECORDING_WR_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_dram0_recording_byteen_0"]
    pub mod CORE_0_DRAM0_RECORDING_BYTEEN_0 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1_REG"]
pub mod CORE_0_DRAM0_EXCEPTION_MONITOR_1 {
    #[doc = "reg_core_0_dram0_recording_pc_0"]
    pub mod CORE_0_DRAM0_RECORDING_PC_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1_REG"]
pub mod CORE_0_DRAM0_EXCEPTION_MONITOR_2 {
    #[doc = "reg_core_0_dram0_recording_addr_1"]
    pub mod CORE_0_DRAM0_RECORDING_ADDR_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_dram0_recording_wr_1"]
    pub mod CORE_0_DRAM0_RECORDING_WR_1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_dram0_recording_byteen_1"]
    pub mod CORE_0_DRAM0_RECORDING_BYTEEN_1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3_REG"]
pub mod CORE_0_DRAM0_EXCEPTION_MONITOR_3 {
    #[doc = "reg_core_0_dram0_recording_pc_1"]
    pub mod CORE_0_DRAM0_RECORDING_PC_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_REG"]
pub mod CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 {
    #[doc = "reg_core_x_iram0_dram0_limit_cycle_0"]
    pub mod CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_REG"]
pub mod CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 {
    #[doc = "reg_core_x_iram0_dram0_limit_cycle_1"]
    pub mod CORE_X_IRAM0_DRAM0_LIMIT_CYCLE_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_LOG_SETTING"]
pub mod LOG_SETTING {
    #[doc = "reg_log_ena"]
    pub mod LOG_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_log_mode"]
    pub mod LOG_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_log_mem_loop_enable"]
    pub mod LOG_MEM_LOOP_ENABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_LOG_DATA_0_REG"]
pub mod LOG_DATA_0 {
    #[doc = "reg_log_data_0"]
    pub mod LOG_DATA_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_LOG_DATA_MASK_REG"]
pub mod LOG_DATA_MASK {
    #[doc = "reg_log_data_size"]
    pub mod LOG_DATA_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_LOG_MIN_REG"]
pub mod LOG_MIN {
    #[doc = "reg_log_min"]
    pub mod LOG_MIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_LOG_MAX_REG"]
pub mod LOG_MAX {
    #[doc = "reg_log_max"]
    pub mod LOG_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_LOG_MEM_START_REG"]
pub mod LOG_MEM_START {
    #[doc = "reg_log_mem_start"]
    pub mod LOG_MEM_START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_LOG_MEM_END_REG"]
pub mod LOG_MEM_END {
    #[doc = "reg_log_mem_end"]
    pub mod LOG_MEM_END {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_LOG_MEM_WRITING_ADDR_REG"]
pub mod LOG_MEM_WRITING_ADDR {
    #[doc = "reg_log_mem_writing_addr"]
    pub mod LOG_MEM_WRITING_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_LOG_MEM_FULL_FLAG_REG"]
pub mod LOG_MEM_FULL_FLAG {
    #[doc = "reg_log_mem_full_flag"]
    pub mod LOG_MEM_FULL_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clr_log_mem_full_flag"]
    pub mod CLR_LOG_MEM_FULL_FLAG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION"]
pub mod C0RE_0_LASTPC_BEFORE_EXCEPTION {
    #[doc = "reg_core_0_lastpc_before_exc"]
    pub mod CORE_0_LASTPC_BEFORE_EXC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_C0RE_0_DEBUG_MODE"]
pub mod C0RE_0_DEBUG_MODE {
    #[doc = "reg_core_0_debug_mode"]
    pub mod CORE_0_DEBUG_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_core_0_debug_module_active"]
    pub mod CORE_0_DEBUG_MODULE_ACTIVE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSIST_DEBUG_DATE_REG"]
pub mod DATE {
    #[doc = "reg_assist_debug_date"]
    pub mod ASSIST_DEBUG_DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
