#[doc = "SENSITIVE Peripheral"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "SENSITIVE_ROM_TABLE_LOCK_REG"]
    pub ROM_TABLE_LOCK: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_ROM_TABLE_REG"]
    pub ROM_TABLE: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_PRIVILEGE_MODE_SEL_LOCK_REG"]
    pub PRIVILEGE_MODE_SEL_LOCK: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_PRIVILEGE_MODE_SEL_REG"]
    pub PRIVILEGE_MODE_SEL: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_APB_PERIPHERAL_ACCESS_0_REG"]
    pub APB_PERIPHERAL_ACCESS_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_APB_PERIPHERAL_ACCESS_1_REG"]
    pub APB_PERIPHERAL_ACCESS_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_0_REG"]
    pub INTERNAL_SRAM_USAGE_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_1_REG"]
    pub INTERNAL_SRAM_USAGE_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_3_REG"]
    pub INTERNAL_SRAM_USAGE_3: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_4_REG"]
    pub INTERNAL_SRAM_USAGE_4: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CACHE_TAG_ACCESS_0_REG"]
    pub CACHE_TAG_ACCESS_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CACHE_TAG_ACCESS_1_REG"]
    pub CACHE_TAG_ACCESS_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CACHE_MMU_ACCESS_0_REG"]
    pub CACHE_MMU_ACCESS_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CACHE_MMU_ACCESS_1_REG"]
    pub CACHE_MMU_ACCESS_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_REG"]
    pub DMA_APBPERI_SPI2_PMS_CONSTRAIN_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_REG"]
    pub DMA_APBPERI_SPI2_PMS_CONSTRAIN_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0_REG"]
    pub DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1_REG"]
    pub DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_0_REG"]
    pub DMA_APBPERI_I2S0_PMS_CONSTRAIN_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_1_REG"]
    pub DMA_APBPERI_I2S0_PMS_CONSTRAIN_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_0_REG"]
    pub DMA_APBPERI_MAC_PMS_CONSTRAIN_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_1_REG"]
    pub DMA_APBPERI_MAC_PMS_CONSTRAIN_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0_REG"]
    pub DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1_REG"]
    pub DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_0_REG"]
    pub DMA_APBPERI_LC_PMS_CONSTRAIN_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_1_REG"]
    pub DMA_APBPERI_LC_PMS_CONSTRAIN_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_0_REG"]
    pub DMA_APBPERI_AES_PMS_CONSTRAIN_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_1_REG"]
    pub DMA_APBPERI_AES_PMS_CONSTRAIN_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_0_REG"]
    pub DMA_APBPERI_SHA_PMS_CONSTRAIN_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_1_REG"]
    pub DMA_APBPERI_SHA_PMS_CONSTRAIN_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0_REG"]
    pub DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_REG"]
    pub DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_0_REG"]
    pub DMA_APBPERI_PMS_MONITOR_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_1_REG"]
    pub DMA_APBPERI_PMS_MONITOR_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_2_REG"]
    pub DMA_APBPERI_PMS_MONITOR_2: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_3_REG"]
    pub DMA_APBPERI_PMS_MONITOR_3: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0_REG"]
    pub CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_REG"]
    pub CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2_REG"]
    pub CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_REG"]
    pub CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_REG"]
    pub CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_REG"]
    pub CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_0_REG"]
    pub CORE_X_IRAM0_PMS_CONSTRAIN_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_1_REG"]
    pub CORE_X_IRAM0_PMS_CONSTRAIN_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2_REG"]
    pub CORE_X_IRAM0_PMS_CONSTRAIN_2: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_0_REG"]
    pub CORE_0_IRAM0_PMS_MONITOR_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1_REG"]
    pub CORE_0_IRAM0_PMS_MONITOR_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_2_REG"]
    pub CORE_0_IRAM0_PMS_MONITOR_2: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_0_REG"]
    pub CORE_X_DRAM0_PMS_CONSTRAIN_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_1_REG"]
    pub CORE_X_DRAM0_PMS_CONSTRAIN_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_0_REG"]
    pub CORE_0_DRAM0_PMS_MONITOR_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_1_REG"]
    pub CORE_0_DRAM0_PMS_MONITOR_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_2_REG"]
    pub CORE_0_DRAM0_PMS_MONITOR_2: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_3_REG"]
    pub CORE_0_DRAM0_PMS_MONITOR_3: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_0_REG"]
    pub CORE_0_PIF_PMS_CONSTRAIN_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_1_REG"]
    pub CORE_0_PIF_PMS_CONSTRAIN_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_2_REG"]
    pub CORE_0_PIF_PMS_CONSTRAIN_2: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_3_REG"]
    pub CORE_0_PIF_PMS_CONSTRAIN_3: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4_REG"]
    pub CORE_0_PIF_PMS_CONSTRAIN_4: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5_REG"]
    pub CORE_0_PIF_PMS_CONSTRAIN_5: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6_REG"]
    pub CORE_0_PIF_PMS_CONSTRAIN_6: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7_REG"]
    pub CORE_0_PIF_PMS_CONSTRAIN_7: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_8_REG"]
    pub CORE_0_PIF_PMS_CONSTRAIN_8: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9_REG"]
    pub CORE_0_PIF_PMS_CONSTRAIN_9: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10_REG"]
    pub CORE_0_PIF_PMS_CONSTRAIN_10: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_0_REG"]
    pub REGION_PMS_CONSTRAIN_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_1_REG"]
    pub REGION_PMS_CONSTRAIN_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_2_REG"]
    pub REGION_PMS_CONSTRAIN_2: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_3_REG"]
    pub REGION_PMS_CONSTRAIN_3: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_4_REG"]
    pub REGION_PMS_CONSTRAIN_4: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_5_REG"]
    pub REGION_PMS_CONSTRAIN_5: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_6_REG"]
    pub REGION_PMS_CONSTRAIN_6: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_7_REG"]
    pub REGION_PMS_CONSTRAIN_7: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_8_REG"]
    pub REGION_PMS_CONSTRAIN_8: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_9_REG"]
    pub REGION_PMS_CONSTRAIN_9: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_10_REG"]
    pub REGION_PMS_CONSTRAIN_10: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_0_REG"]
    pub CORE_0_PIF_PMS_MONITOR_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_1_REG"]
    pub CORE_0_PIF_PMS_MONITOR_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_2_REG"]
    pub CORE_0_PIF_PMS_MONITOR_2: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_3_REG"]
    pub CORE_0_PIF_PMS_MONITOR_3: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_4_REG"]
    pub CORE_0_PIF_PMS_MONITOR_4: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_5_REG"]
    pub CORE_0_PIF_PMS_MONITOR_5: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_6_REG"]
    pub CORE_0_PIF_PMS_MONITOR_6: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_0_REG"]
    pub BACKUP_BUS_PMS_CONSTRAIN_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_1_REG"]
    pub BACKUP_BUS_PMS_CONSTRAIN_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_2_REG"]
    pub BACKUP_BUS_PMS_CONSTRAIN_2: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_3_REG"]
    pub BACKUP_BUS_PMS_CONSTRAIN_3: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4_REG"]
    pub BACKUP_BUS_PMS_CONSTRAIN_4: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_0_REG"]
    pub BACKUP_BUS_PMS_MONITOR_0: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_1_REG"]
    pub BACKUP_BUS_PMS_MONITOR_1: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_2_REG"]
    pub BACKUP_BUS_PMS_MONITOR_2: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_3_REG"]
    pub BACKUP_BUS_PMS_MONITOR_3: crate::RWRegister<u32>,
    #[doc = "SENSITIVE_CLOCK_GATE_REG_REG"]
    pub CLOCK_GATE: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0e88],
    #[doc = "SENSITIVE_DATE_REG"]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "SENSITIVE_ROM_TABLE_LOCK_REG"]
pub mod ROM_TABLE_LOCK {
    #[doc = "rom_table_lock"]
    pub mod ROM_TABLE_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_ROM_TABLE_REG"]
pub mod ROM_TABLE {
    #[doc = "rom_table"]
    pub mod ROM_TABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_PRIVILEGE_MODE_SEL_LOCK_REG"]
pub mod PRIVILEGE_MODE_SEL_LOCK {
    #[doc = "privilege_mode_sel_lock"]
    pub mod PRIVILEGE_MODE_SEL_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_PRIVILEGE_MODE_SEL_REG"]
pub mod PRIVILEGE_MODE_SEL {
    #[doc = "privilege_mode_sel"]
    pub mod PRIVILEGE_MODE_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_APB_PERIPHERAL_ACCESS_0_REG"]
pub mod APB_PERIPHERAL_ACCESS_0 {
    #[doc = "apb_peripheral_access_lock"]
    pub mod APB_PERIPHERAL_ACCESS_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_APB_PERIPHERAL_ACCESS_1_REG"]
pub mod APB_PERIPHERAL_ACCESS_1 {
    #[doc = "apb_peripheral_access_split_burst"]
    pub mod APB_PERIPHERAL_ACCESS_SPLIT_BURST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_0_REG"]
pub mod INTERNAL_SRAM_USAGE_0 {
    #[doc = "internal_sram_usage_lock"]
    pub mod INTERNAL_SRAM_USAGE_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_1_REG"]
pub mod INTERNAL_SRAM_USAGE_1 {
    #[doc = "internal_sram_usage_cpu_cache"]
    pub mod INTERNAL_SRAM_USAGE_CPU_CACHE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "internal_sram_usage_cpu_sram"]
    pub mod INTERNAL_SRAM_USAGE_CPU_SRAM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_3_REG"]
pub mod INTERNAL_SRAM_USAGE_3 {
    #[doc = "internal_sram_usage_mac_dump_sram"]
    pub mod INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "internal_sram_alloc_mac_dump"]
    pub mod INTERNAL_SRAM_ALLOC_MAC_DUMP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_4_REG"]
pub mod INTERNAL_SRAM_USAGE_4 {
    #[doc = "internal_sram_usage_log_sram"]
    pub mod INTERNAL_SRAM_USAGE_LOG_SRAM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CACHE_TAG_ACCESS_0_REG"]
pub mod CACHE_TAG_ACCESS_0 {
    #[doc = "cache_tag_access_lock"]
    pub mod CACHE_TAG_ACCESS_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CACHE_TAG_ACCESS_1_REG"]
pub mod CACHE_TAG_ACCESS_1 {
    #[doc = "pro_i_tag_rd_acs"]
    pub mod PRO_I_TAG_RD_ACS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pro_i_tag_wr_acs"]
    pub mod PRO_I_TAG_WR_ACS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pro_d_tag_rd_acs"]
    pub mod PRO_D_TAG_RD_ACS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pro_d_tag_wr_acs"]
    pub mod PRO_D_TAG_WR_ACS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CACHE_MMU_ACCESS_0_REG"]
pub mod CACHE_MMU_ACCESS_0 {
    #[doc = "cache_mmu_access_lock"]
    pub mod CACHE_MMU_ACCESS_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CACHE_MMU_ACCESS_1_REG"]
pub mod CACHE_MMU_ACCESS_1 {
    #[doc = "pro_mmu_rd_acs"]
    pub mod PRO_MMU_RD_ACS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pro_mmu_wr_acs"]
    pub mod PRO_MMU_WR_ACS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_0_REG"]
pub mod DMA_APBPERI_SPI2_PMS_CONSTRAIN_0 {
    #[doc = "dma_apbperi_spi2_pms_constrain_lock"]
    pub mod DMA_APBPERI_SPI2_PMS_CONSTRAIN_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_SPI2_PMS_CONSTRAIN_1_REG"]
pub mod DMA_APBPERI_SPI2_PMS_CONSTRAIN_1 {
    #[doc = "dma_apbperi_spi2_pms_constrain_sram_world_0_pms_0"]
    pub mod DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_spi2_pms_constrain_sram_world_0_pms_1"]
    pub mod DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_spi2_pms_constrain_sram_world_0_pms_2"]
    pub mod DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_spi2_pms_constrain_sram_world_0_pms_3"]
    pub mod DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_spi2_pms_constrain_sram_world_1_pms_0"]
    pub mod DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_spi2_pms_constrain_sram_world_1_pms_1"]
    pub mod DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_spi2_pms_constrain_sram_world_1_pms_2"]
    pub mod DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_spi2_pms_constrain_sram_world_1_pms_3"]
    pub mod DMA_APBPERI_SPI2_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0_REG"]
pub mod DMA_APBPERI_UCHI0_PMS_CONSTRAIN_0 {
    #[doc = "dma_apbperi_uchi0_pms_constrain_lock"]
    pub mod DMA_APBPERI_UCHI0_PMS_CONSTRAIN_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1_REG"]
pub mod DMA_APBPERI_UCHI0_PMS_CONSTRAIN_1 {
    #[doc = "dma_apbperi_uchi0_pms_constrain_sram_world_0_pms_0"]
    pub mod DMA_APBPERI_UCHI0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_uchi0_pms_constrain_sram_world_0_pms_1"]
    pub mod DMA_APBPERI_UCHI0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_uchi0_pms_constrain_sram_world_0_pms_2"]
    pub mod DMA_APBPERI_UCHI0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_uchi0_pms_constrain_sram_world_0_pms_3"]
    pub mod DMA_APBPERI_UCHI0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_uchi0_pms_constrain_sram_world_1_pms_0"]
    pub mod DMA_APBPERI_UCHI0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_uchi0_pms_constrain_sram_world_1_pms_1"]
    pub mod DMA_APBPERI_UCHI0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_uchi0_pms_constrain_sram_world_1_pms_2"]
    pub mod DMA_APBPERI_UCHI0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_uchi0_pms_constrain_sram_world_1_pms_3"]
    pub mod DMA_APBPERI_UCHI0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_0_REG"]
pub mod DMA_APBPERI_I2S0_PMS_CONSTRAIN_0 {
    #[doc = "dma_apbperi_i2s0_pms_constrain_lock"]
    pub mod DMA_APBPERI_I2S0_PMS_CONSTRAIN_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_I2S0_PMS_CONSTRAIN_1_REG"]
pub mod DMA_APBPERI_I2S0_PMS_CONSTRAIN_1 {
    #[doc = "dma_apbperi_i2s0_pms_constrain_sram_world_0_pms_0"]
    pub mod DMA_APBPERI_I2S0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_i2s0_pms_constrain_sram_world_0_pms_1"]
    pub mod DMA_APBPERI_I2S0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_i2s0_pms_constrain_sram_world_0_pms_2"]
    pub mod DMA_APBPERI_I2S0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_i2s0_pms_constrain_sram_world_0_pms_3"]
    pub mod DMA_APBPERI_I2S0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_i2s0_pms_constrain_sram_world_1_pms_0"]
    pub mod DMA_APBPERI_I2S0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_i2s0_pms_constrain_sram_world_1_pms_1"]
    pub mod DMA_APBPERI_I2S0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_i2s0_pms_constrain_sram_world_1_pms_2"]
    pub mod DMA_APBPERI_I2S0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_i2s0_pms_constrain_sram_world_1_pms_3"]
    pub mod DMA_APBPERI_I2S0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_0_REG"]
pub mod DMA_APBPERI_MAC_PMS_CONSTRAIN_0 {
    #[doc = "dma_apbperi_mac_pms_constrain_lock"]
    pub mod DMA_APBPERI_MAC_PMS_CONSTRAIN_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_MAC_PMS_CONSTRAIN_1_REG"]
pub mod DMA_APBPERI_MAC_PMS_CONSTRAIN_1 {
    #[doc = "dma_apbperi_mac_pms_constrain_sram_world_0_pms_0"]
    pub mod DMA_APBPERI_MAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_mac_pms_constrain_sram_world_0_pms_1"]
    pub mod DMA_APBPERI_MAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_mac_pms_constrain_sram_world_0_pms_2"]
    pub mod DMA_APBPERI_MAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_mac_pms_constrain_sram_world_0_pms_3"]
    pub mod DMA_APBPERI_MAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_mac_pms_constrain_sram_world_1_pms_0"]
    pub mod DMA_APBPERI_MAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_mac_pms_constrain_sram_world_1_pms_1"]
    pub mod DMA_APBPERI_MAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_mac_pms_constrain_sram_world_1_pms_2"]
    pub mod DMA_APBPERI_MAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_mac_pms_constrain_sram_world_1_pms_3"]
    pub mod DMA_APBPERI_MAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0_REG"]
pub mod DMA_APBPERI_BACKUP_PMS_CONSTRAIN_0 {
    #[doc = "dma_apbperi_backup_pms_constrain_lock"]
    pub mod DMA_APBPERI_BACKUP_PMS_CONSTRAIN_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1_REG"]
pub mod DMA_APBPERI_BACKUP_PMS_CONSTRAIN_1 {
    #[doc = "dma_apbperi_backup_pms_constrain_sram_world_0_pms_0"]
    pub mod DMA_APBPERI_BACKUP_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_backup_pms_constrain_sram_world_0_pms_1"]
    pub mod DMA_APBPERI_BACKUP_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_backup_pms_constrain_sram_world_0_pms_2"]
    pub mod DMA_APBPERI_BACKUP_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_backup_pms_constrain_sram_world_0_pms_3"]
    pub mod DMA_APBPERI_BACKUP_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_backup_pms_constrain_sram_world_1_pms_0"]
    pub mod DMA_APBPERI_BACKUP_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_backup_pms_constrain_sram_world_1_pms_1"]
    pub mod DMA_APBPERI_BACKUP_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_backup_pms_constrain_sram_world_1_pms_2"]
    pub mod DMA_APBPERI_BACKUP_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_backup_pms_constrain_sram_world_1_pms_3"]
    pub mod DMA_APBPERI_BACKUP_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_0_REG"]
pub mod DMA_APBPERI_LC_PMS_CONSTRAIN_0 {
    #[doc = "dma_apbperi_lc_pms_constrain_lock"]
    pub mod DMA_APBPERI_LC_PMS_CONSTRAIN_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_LC_PMS_CONSTRAIN_1_REG"]
pub mod DMA_APBPERI_LC_PMS_CONSTRAIN_1 {
    #[doc = "dma_apbperi_lc_pms_constrain_sram_world_0_pms_0"]
    pub mod DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_lc_pms_constrain_sram_world_0_pms_1"]
    pub mod DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_lc_pms_constrain_sram_world_0_pms_2"]
    pub mod DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_lc_pms_constrain_sram_world_0_pms_3"]
    pub mod DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_lc_pms_constrain_sram_world_1_pms_0"]
    pub mod DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_lc_pms_constrain_sram_world_1_pms_1"]
    pub mod DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_lc_pms_constrain_sram_world_1_pms_2"]
    pub mod DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_lc_pms_constrain_sram_world_1_pms_3"]
    pub mod DMA_APBPERI_LC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_0_REG"]
pub mod DMA_APBPERI_AES_PMS_CONSTRAIN_0 {
    #[doc = "dma_apbperi_aes_pms_constrain_lock"]
    pub mod DMA_APBPERI_AES_PMS_CONSTRAIN_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_AES_PMS_CONSTRAIN_1_REG"]
pub mod DMA_APBPERI_AES_PMS_CONSTRAIN_1 {
    #[doc = "dma_apbperi_aes_pms_constrain_sram_world_0_pms_0"]
    pub mod DMA_APBPERI_AES_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_aes_pms_constrain_sram_world_0_pms_1"]
    pub mod DMA_APBPERI_AES_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_aes_pms_constrain_sram_world_0_pms_2"]
    pub mod DMA_APBPERI_AES_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_aes_pms_constrain_sram_world_0_pms_3"]
    pub mod DMA_APBPERI_AES_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_aes_pms_constrain_sram_world_1_pms_0"]
    pub mod DMA_APBPERI_AES_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_aes_pms_constrain_sram_world_1_pms_1"]
    pub mod DMA_APBPERI_AES_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_aes_pms_constrain_sram_world_1_pms_2"]
    pub mod DMA_APBPERI_AES_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_aes_pms_constrain_sram_world_1_pms_3"]
    pub mod DMA_APBPERI_AES_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_0_REG"]
pub mod DMA_APBPERI_SHA_PMS_CONSTRAIN_0 {
    #[doc = "dma_apbperi_sha_pms_constrain_lock"]
    pub mod DMA_APBPERI_SHA_PMS_CONSTRAIN_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_SHA_PMS_CONSTRAIN_1_REG"]
pub mod DMA_APBPERI_SHA_PMS_CONSTRAIN_1 {
    #[doc = "dma_apbperi_sha_pms_constrain_sram_world_0_pms_0"]
    pub mod DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_sha_pms_constrain_sram_world_0_pms_1"]
    pub mod DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_sha_pms_constrain_sram_world_0_pms_2"]
    pub mod DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_sha_pms_constrain_sram_world_0_pms_3"]
    pub mod DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_sha_pms_constrain_sram_world_1_pms_0"]
    pub mod DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_sha_pms_constrain_sram_world_1_pms_1"]
    pub mod DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_sha_pms_constrain_sram_world_1_pms_2"]
    pub mod DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_sha_pms_constrain_sram_world_1_pms_3"]
    pub mod DMA_APBPERI_SHA_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0_REG"]
pub mod DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_0 {
    #[doc = "dma_apbperi_adc_dac_pms_constrain_lock"]
    pub mod DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_REG"]
pub mod DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1 {
    #[doc = "dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_0"]
    pub mod DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_1"]
    pub mod DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_2"]
    pub mod DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_3"]
    pub mod DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_0"]
    pub mod DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_1"]
    pub mod DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_2"]
    pub mod DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_3"]
    pub mod DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_0_REG"]
pub mod DMA_APBPERI_PMS_MONITOR_0 {
    #[doc = "dma_apbperi_pms_monitor_lock"]
    pub mod DMA_APBPERI_PMS_MONITOR_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_1_REG"]
pub mod DMA_APBPERI_PMS_MONITOR_1 {
    #[doc = "dma_apbperi_pms_monitor_violate_clr"]
    pub mod DMA_APBPERI_PMS_MONITOR_VIOLATE_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_pms_monitor_violate_en"]
    pub mod DMA_APBPERI_PMS_MONITOR_VIOLATE_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_2_REG"]
pub mod DMA_APBPERI_PMS_MONITOR_2 {
    #[doc = "dma_apbperi_pms_monitor_violate_intr"]
    pub mod DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_pms_monitor_violate_status_world"]
    pub mod DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_pms_monitor_violate_status_addr"]
    pub mod DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_3_REG"]
pub mod DMA_APBPERI_PMS_MONITOR_3 {
    #[doc = "dma_apbperi_pms_monitor_violate_status_wr"]
    pub mod DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma_apbperi_pms_monitor_violate_status_byteen"]
    pub mod DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_BYTEEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0_REG"]
pub mod CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_0 {
    #[doc = "core_x_iram0_dram0_dma_split_line_constrain_lock"]
    pub mod CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1_REG"]
pub mod CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_1 {
    #[doc = "core_x_iram0_dram0_dma_sram_category_0"]
    pub mod CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_dram0_dma_sram_category_1"]
    pub mod CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_dram0_dma_sram_category_2"]
    pub mod CORE_X_IRAM0_DRAM0_DMA_SRAM_CATEGORY_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_dram0_dma_sram_splitaddr"]
    pub mod CORE_X_IRAM0_DRAM0_DMA_SRAM_SPLITADDR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2_REG"]
pub mod CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2 {
    #[doc = "core_x_iram0_sram_line_0_category_0"]
    pub mod CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_sram_line_0_category_1"]
    pub mod CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_sram_line_0_category_2"]
    pub mod CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_sram_line_0_splitaddr"]
    pub mod CORE_X_IRAM0_SRAM_LINE_0_SPLITADDR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3_REG"]
pub mod CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_3 {
    #[doc = "core_x_iram0_sram_line_1_category_0"]
    pub mod CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_sram_line_1_category_1"]
    pub mod CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_sram_line_1_category_2"]
    pub mod CORE_X_IRAM0_SRAM_LINE_1_CATEGORY_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_sram_line_1_splitaddr"]
    pub mod CORE_X_IRAM0_SRAM_LINE_1_SPLITADDR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_REG"]
pub mod CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4 {
    #[doc = "core_x_dram0_dma_sram_line_0_category_0"]
    pub mod CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_dram0_dma_sram_line_0_category_1"]
    pub mod CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_dram0_dma_sram_line_0_category_2"]
    pub mod CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_dram0_dma_sram_line_0_splitaddr"]
    pub mod CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5_REG"]
pub mod CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_5 {
    #[doc = "core_x_dram0_dma_sram_line_1_category_0"]
    pub mod CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_dram0_dma_sram_line_1_category_1"]
    pub mod CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_dram0_dma_sram_line_1_category_2"]
    pub mod CORE_X_DRAM0_DMA_SRAM_LINE_1_CATEGORY_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_dram0_dma_sram_line_1_splitaddr"]
    pub mod CORE_X_DRAM0_DMA_SRAM_LINE_1_SPLITADDR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_0_REG"]
pub mod CORE_X_IRAM0_PMS_CONSTRAIN_0 {
    #[doc = "core_x_iram0_pms_constrain_lock"]
    pub mod CORE_X_IRAM0_PMS_CONSTRAIN_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_1_REG"]
pub mod CORE_X_IRAM0_PMS_CONSTRAIN_1 {
    #[doc = "core_x_iram0_pms_constrain_sram_world_1_pms_0"]
    pub mod CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_pms_constrain_sram_world_1_pms_1"]
    pub mod CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_pms_constrain_sram_world_1_pms_2"]
    pub mod CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_pms_constrain_sram_world_1_pms_3"]
    pub mod CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_pms_constrain_sram_world_1_cachedataarray_pms_0"]
    pub mod CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_CACHEDATAARRAY_PMS_0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_pms_constrain_rom_world_1_pms"]
    pub mod CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_X_IRAM0_PMS_CONSTRAIN_2_REG"]
pub mod CORE_X_IRAM0_PMS_CONSTRAIN_2 {
    #[doc = "core_x_iram0_pms_constrain_sram_world_0_pms_0"]
    pub mod CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_pms_constrain_sram_world_0_pms_1"]
    pub mod CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_pms_constrain_sram_world_0_pms_2"]
    pub mod CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_pms_constrain_sram_world_0_pms_3"]
    pub mod CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_pms_constrain_sram_world_0_cachedataarray_pms_0"]
    pub mod CORE_X_IRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_CACHEDATAARRAY_PMS_0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_iram0_pms_constrain_rom_world_0_pms"]
    pub mod CORE_X_IRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_0_REG"]
pub mod CORE_0_IRAM0_PMS_MONITOR_0 {
    #[doc = "core_0_iram0_pms_monitor_lock"]
    pub mod CORE_0_IRAM0_PMS_MONITOR_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_1_REG"]
pub mod CORE_0_IRAM0_PMS_MONITOR_1 {
    #[doc = "core_0_iram0_pms_monitor_violate_clr"]
    pub mod CORE_0_IRAM0_PMS_MONITOR_VIOLATE_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_iram0_pms_monitor_violate_en"]
    pub mod CORE_0_IRAM0_PMS_MONITOR_VIOLATE_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_IRAM0_PMS_MONITOR_2_REG"]
pub mod CORE_0_IRAM0_PMS_MONITOR_2 {
    #[doc = "core_0_iram0_pms_monitor_violate_intr"]
    pub mod CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_iram0_pms_monitor_violate_status_wr"]
    pub mod CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_iram0_pms_monitor_violate_status_loadstore"]
    pub mod CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_LOADSTORE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_iram0_pms_monitor_violate_status_world"]
    pub mod CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_iram0_pms_monitor_violate_status_addr"]
    pub mod CORE_0_IRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_0_REG"]
pub mod CORE_X_DRAM0_PMS_CONSTRAIN_0 {
    #[doc = "core_x_dram0_pms_constrain_lock"]
    pub mod CORE_X_DRAM0_PMS_CONSTRAIN_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_X_DRAM0_PMS_CONSTRAIN_1_REG"]
pub mod CORE_X_DRAM0_PMS_CONSTRAIN_1 {
    #[doc = "core_x_dram0_pms_constrain_sram_world_0_pms_0"]
    pub mod CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_dram0_pms_constrain_sram_world_0_pms_1"]
    pub mod CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_dram0_pms_constrain_sram_world_0_pms_2"]
    pub mod CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_dram0_pms_constrain_sram_world_0_pms_3"]
    pub mod CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_dram0_pms_constrain_sram_world_1_pms_0"]
    pub mod CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_dram0_pms_constrain_sram_world_1_pms_1"]
    pub mod CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_dram0_pms_constrain_sram_world_1_pms_2"]
    pub mod CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_dram0_pms_constrain_sram_world_1_pms_3"]
    pub mod CORE_X_DRAM0_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_dram0_pms_constrain_rom_world_0_pms"]
    pub mod CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_0_PMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_x_dram0_pms_constrain_rom_world_1_pms"]
    pub mod CORE_X_DRAM0_PMS_CONSTRAIN_ROM_WORLD_1_PMS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_0_REG"]
pub mod CORE_0_DRAM0_PMS_MONITOR_0 {
    #[doc = "core_0_dram0_pms_monitor_lock"]
    pub mod CORE_0_DRAM0_PMS_MONITOR_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_1_REG"]
pub mod CORE_0_DRAM0_PMS_MONITOR_1 {
    #[doc = "core_0_dram0_pms_monitor_violate_clr"]
    pub mod CORE_0_DRAM0_PMS_MONITOR_VIOLATE_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_dram0_pms_monitor_violate_en"]
    pub mod CORE_0_DRAM0_PMS_MONITOR_VIOLATE_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_2_REG"]
pub mod CORE_0_DRAM0_PMS_MONITOR_2 {
    #[doc = "core_0_dram0_pms_monitor_violate_intr"]
    pub mod CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_dram0_pms_monitor_violate_status_lock"]
    pub mod CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_dram0_pms_monitor_violate_status_world"]
    pub mod CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_dram0_pms_monitor_violate_status_addr"]
    pub mod CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_3_REG"]
pub mod CORE_0_DRAM0_PMS_MONITOR_3 {
    #[doc = "core_0_dram0_pms_monitor_violate_status_wr"]
    pub mod CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_dram0_pms_monitor_violate_status_byteen"]
    pub mod CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_BYTEEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_0_REG"]
pub mod CORE_0_PIF_PMS_CONSTRAIN_0 {
    #[doc = "core_0_pif_pms_constrain_lock"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_1_REG"]
pub mod CORE_0_PIF_PMS_CONSTRAIN_1 {
    #[doc = "core_0_pif_pms_constrain_world_0_uart"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_g0spi_1"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_g0spi_0"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_gpio"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_fe2"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_fe"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_timer"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_rtc"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_io_mux"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_wdg"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_misc"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_i2c"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_uart1"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_2_REG"]
pub mod CORE_0_PIF_PMS_CONSTRAIN_2 {
    #[doc = "core_0_pif_pms_constrain_world_0_bt"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_i2c_ext0"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_uhci0"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_rmt"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_ledc"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_bb"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_timergroup"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_timergroup1"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_systimer"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_3_REG"]
pub mod CORE_0_PIF_PMS_CONSTRAIN_3 {
    #[doc = "core_0_pif_pms_constrain_world_0_spi_2"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_apb_ctrl"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_can"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_i2s1"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_rwbt"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_wifimac"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_pwr"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_4_REG"]
pub mod CORE_0_PIF_PMS_CONSTRAIN_4 {
    #[doc = "core_0_pif_pms_constrain_world_0_usb_wrap"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_WRAP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_crypto_peri"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_PERI {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_crypto_dma"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CRYPTO_DMA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_apb_adc"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_ADC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_bt_pwr"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_PWR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_usb_device"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_USB_DEVICE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_system"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTEM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_sensitive"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SENSITIVE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_interrupt"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_INTERRUPT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_dma_copy"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DMA_COPY {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_cache_config"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CACHE_CONFIG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_ad"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_AD {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_dio"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_DIO {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_0_world_controller"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WORLD_CONTROLLER {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_5_REG"]
pub mod CORE_0_PIF_PMS_CONSTRAIN_5 {
    #[doc = "core_0_pif_pms_constrain_world_1_uart"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_g0spi_1"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_g0spi_0"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_G0SPI_0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_gpio"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_GPIO {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_fe2"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_fe"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_FE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_timer"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_rtc"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RTC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_io_mux"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_IO_MUX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_wdg"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WDG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_misc"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_MISC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_i2c"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_uart1"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UART1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6_REG"]
pub mod CORE_0_PIF_PMS_CONSTRAIN_6 {
    #[doc = "core_0_pif_pms_constrain_world_1_bt"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_i2c_ext0"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_uhci0"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_rmt"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_ledc"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_bb"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_timergroup"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_timergroup1"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_systimer"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_7_REG"]
pub mod CORE_0_PIF_PMS_CONSTRAIN_7 {
    #[doc = "core_0_pif_pms_constrain_world_1_spi_2"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SPI_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_apb_ctrl"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_CTRL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_can"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CAN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_i2s1"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2S1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_rwbt"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RWBT {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_wifimac"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WIFIMAC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_pwr"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_PWR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_8_REG"]
pub mod CORE_0_PIF_PMS_CONSTRAIN_8 {
    #[doc = "core_0_pif_pms_constrain_world_1_usb_wrap"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_WRAP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_crypto_peri"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_PERI {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_crypto_dma"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CRYPTO_DMA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_apb_adc"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_APB_ADC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_bt_pwr"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_PWR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_usb_device"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_USB_DEVICE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_system"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTEM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_sensitive"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SENSITIVE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_interrupt"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_INTERRUPT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_dma_copy"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DMA_COPY {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_cache_config"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_CACHE_CONFIG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_ad"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_AD {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_dio"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_DIO {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_world_1_world_controller"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_WORLD_CONTROLLER {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_9_REG"]
pub mod CORE_0_PIF_PMS_CONSTRAIN_9 {
    #[doc = "core_0_pif_pms_constrain_rtcfast_spltaddr_world_0"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_rtcfast_spltaddr_world_1"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_SPLTADDR_WORLD_1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10_REG"]
pub mod CORE_0_PIF_PMS_CONSTRAIN_10 {
    #[doc = "core_0_pif_pms_constrain_rtcfast_world_0_l"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_rtcfast_world_0_h"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_rtcfast_world_1_l"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_constrain_rtcfast_world_1_h"]
    pub mod CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_0_REG"]
pub mod REGION_PMS_CONSTRAIN_0 {
    #[doc = "region_pms_constrain_lock"]
    pub mod REGION_PMS_CONSTRAIN_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_1_REG"]
pub mod REGION_PMS_CONSTRAIN_1 {
    #[doc = "region_pms_constrain_world_0_area_0"]
    pub mod REGION_PMS_CONSTRAIN_WORLD_0_AREA_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "region_pms_constrain_world_0_area_1"]
    pub mod REGION_PMS_CONSTRAIN_WORLD_0_AREA_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "region_pms_constrain_world_0_area_2"]
    pub mod REGION_PMS_CONSTRAIN_WORLD_0_AREA_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "region_pms_constrain_world_0_area_3"]
    pub mod REGION_PMS_CONSTRAIN_WORLD_0_AREA_3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "region_pms_constrain_world_0_area_4"]
    pub mod REGION_PMS_CONSTRAIN_WORLD_0_AREA_4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "region_pms_constrain_world_0_area_5"]
    pub mod REGION_PMS_CONSTRAIN_WORLD_0_AREA_5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "region_pms_constrain_world_0_area_6"]
    pub mod REGION_PMS_CONSTRAIN_WORLD_0_AREA_6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_2_REG"]
pub mod REGION_PMS_CONSTRAIN_2 {
    #[doc = "region_pms_constrain_world_1_area_0"]
    pub mod REGION_PMS_CONSTRAIN_WORLD_1_AREA_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "region_pms_constrain_world_1_area_1"]
    pub mod REGION_PMS_CONSTRAIN_WORLD_1_AREA_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "region_pms_constrain_world_1_area_2"]
    pub mod REGION_PMS_CONSTRAIN_WORLD_1_AREA_2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "region_pms_constrain_world_1_area_3"]
    pub mod REGION_PMS_CONSTRAIN_WORLD_1_AREA_3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "region_pms_constrain_world_1_area_4"]
    pub mod REGION_PMS_CONSTRAIN_WORLD_1_AREA_4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "region_pms_constrain_world_1_area_5"]
    pub mod REGION_PMS_CONSTRAIN_WORLD_1_AREA_5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "region_pms_constrain_world_1_area_6"]
    pub mod REGION_PMS_CONSTRAIN_WORLD_1_AREA_6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_3_REG"]
pub mod REGION_PMS_CONSTRAIN_3 {
    #[doc = "region_pms_constrain_addr_0"]
    pub mod REGION_PMS_CONSTRAIN_ADDR_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_4_REG"]
pub mod REGION_PMS_CONSTRAIN_4 {
    #[doc = "region_pms_constrain_addr_1"]
    pub mod REGION_PMS_CONSTRAIN_ADDR_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_5_REG"]
pub mod REGION_PMS_CONSTRAIN_5 {
    #[doc = "region_pms_constrain_addr_2"]
    pub mod REGION_PMS_CONSTRAIN_ADDR_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_6_REG"]
pub mod REGION_PMS_CONSTRAIN_6 {
    #[doc = "region_pms_constrain_addr_3"]
    pub mod REGION_PMS_CONSTRAIN_ADDR_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_7_REG"]
pub mod REGION_PMS_CONSTRAIN_7 {
    #[doc = "region_pms_constrain_addr_4"]
    pub mod REGION_PMS_CONSTRAIN_ADDR_4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_8_REG"]
pub mod REGION_PMS_CONSTRAIN_8 {
    #[doc = "region_pms_constrain_addr_5"]
    pub mod REGION_PMS_CONSTRAIN_ADDR_5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_9_REG"]
pub mod REGION_PMS_CONSTRAIN_9 {
    #[doc = "region_pms_constrain_addr_6"]
    pub mod REGION_PMS_CONSTRAIN_ADDR_6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_10_REG"]
pub mod REGION_PMS_CONSTRAIN_10 {
    #[doc = "region_pms_constrain_addr_7"]
    pub mod REGION_PMS_CONSTRAIN_ADDR_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_0_REG"]
pub mod CORE_0_PIF_PMS_MONITOR_0 {
    #[doc = "core_0_pif_pms_monitor_lock"]
    pub mod CORE_0_PIF_PMS_MONITOR_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_1_REG"]
pub mod CORE_0_PIF_PMS_MONITOR_1 {
    #[doc = "core_0_pif_pms_monitor_violate_clr"]
    pub mod CORE_0_PIF_PMS_MONITOR_VIOLATE_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_monitor_violate_en"]
    pub mod CORE_0_PIF_PMS_MONITOR_VIOLATE_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_2_REG"]
pub mod CORE_0_PIF_PMS_MONITOR_2 {
    #[doc = "core_0_pif_pms_monitor_violate_intr"]
    pub mod CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_monitor_violate_status_hport_0"]
    pub mod CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HPORT_0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_monitor_violate_status_hsize"]
    pub mod CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HSIZE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_monitor_violate_status_hwrite"]
    pub mod CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWRITE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_monitor_violate_status_hworld"]
    pub mod CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HWORLD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_3_REG"]
pub mod CORE_0_PIF_PMS_MONITOR_3 {
    #[doc = "core_0_pif_pms_monitor_violate_status_haddr"]
    pub mod CORE_0_PIF_PMS_MONITOR_VIOLATE_STATUS_HADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_4_REG"]
pub mod CORE_0_PIF_PMS_MONITOR_4 {
    #[doc = "core_0_pif_pms_monitor_nonword_violate_clr"]
    pub mod CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_monitor_nonword_violate_en"]
    pub mod CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_5_REG"]
pub mod CORE_0_PIF_PMS_MONITOR_5 {
    #[doc = "core_0_pif_pms_monitor_nonword_violate_intr"]
    pub mod CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_INTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_monitor_nonword_violate_status_hsize"]
    pub mod CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HSIZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "core_0_pif_pms_monitor_nonword_violate_status_hworld"]
    pub mod CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HWORLD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_MONITOR_6_REG"]
pub mod CORE_0_PIF_PMS_MONITOR_6 {
    #[doc = "core_0_pif_pms_monitor_nonword_violate_status_haddr"]
    pub mod CORE_0_PIF_PMS_MONITOR_NONWORD_VIOLATE_STATUS_HADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_0_REG"]
pub mod BACKUP_BUS_PMS_CONSTRAIN_0 {
    #[doc = "backup_bus_pms_constrain_lock"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_1_REG"]
pub mod BACKUP_BUS_PMS_CONSTRAIN_1 {
    #[doc = "backup_bus_pms_constrain_uart"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_UART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_g0spi_1"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_g0spi_0"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_gpio"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_GPIO {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_fe2"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_FE2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_fe"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_FE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_timer"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_TIMER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_rtc"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_RTC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_io_mux"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_IO_MUX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_wdg"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_WDG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_misc"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_MISC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_i2c"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_I2C {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_uart1"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_UART1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_2_REG"]
pub mod BACKUP_BUS_PMS_CONSTRAIN_2 {
    #[doc = "backup_bus_pms_constrain_bt"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_BT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_i2c_ext0"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_I2C_EXT0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_uhci0"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_UHCI0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_rmt"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_RMT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_ledc"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_LEDC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_bb"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_BB {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_timergroup"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_timergroup1"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_TIMERGROUP1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_systimer"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_SYSTIMER {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_3_REG"]
pub mod BACKUP_BUS_PMS_CONSTRAIN_3 {
    #[doc = "backup_bus_pms_constrain_spi_2"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_SPI_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_apb_ctrl"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_APB_CTRL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_can"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_CAN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_i2s1"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_I2S1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_rwbt"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_RWBT {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_wifimac"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_WIFIMAC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_pwr"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_PWR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_4_REG"]
pub mod BACKUP_BUS_PMS_CONSTRAIN_4 {
    #[doc = "backup_bus_pms_constrain_usb_wrap"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_USB_WRAP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_crypto_peri"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_PERI {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_crypto_dma"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_CRYPTO_DMA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_apb_adc"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_APB_ADC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_bt_pwr"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_BT_PWR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_constrain_usb_device"]
    pub mod BACKUP_BUS_PMS_CONSTRAIN_USB_DEVICE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_0_REG"]
pub mod BACKUP_BUS_PMS_MONITOR_0 {
    #[doc = "backup_bus_pms_monitor_lock"]
    pub mod BACKUP_BUS_PMS_MONITOR_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_1_REG"]
pub mod BACKUP_BUS_PMS_MONITOR_1 {
    #[doc = "backup_bus_pms_monitor_violate_clr"]
    pub mod BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_monitor_violate_en"]
    pub mod BACKUP_BUS_PMS_MONITOR_VIOLATE_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_2_REG"]
pub mod BACKUP_BUS_PMS_MONITOR_2 {
    #[doc = "backup_bus_pms_monitor_violate_intr"]
    pub mod BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_monitor_violate_status_htrans"]
    pub mod BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_monitor_violate_status_hsize"]
    pub mod BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "backup_bus_pms_monitor_violate_status_hwrite"]
    pub mod BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_3_REG"]
pub mod BACKUP_BUS_PMS_MONITOR_3 {
    #[doc = "backup_bus_pms_monitor_violate_haddr"]
    pub mod BACKUP_BUS_PMS_MONITOR_VIOLATE_HADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_CLOCK_GATE_REG_REG"]
pub mod CLOCK_GATE {
    #[doc = "clk_en"]
    pub mod CLK_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SENSITIVE_DATE_REG"]
pub mod DATE {
    #[doc = "reg_date"]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
