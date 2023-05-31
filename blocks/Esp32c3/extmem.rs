#[doc = "External Memory"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_CTRL: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_CTRL1: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_TAG_POWER_CTRL: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_PRELOCK_CTRL: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_PRELOCK_SCT0_ADDR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_PRELOCK_SCT1_ADDR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_PRELOCK_SCT_SIZE: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_LOCK_CTRL: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_LOCK_ADDR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_LOCK_SIZE: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_SYNC_CTRL: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_SYNC_ADDR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_SYNC_SIZE: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_PRELOAD_CTRL: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_PRELOAD_ADDR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_PRELOAD_SIZE: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_AUTOLOAD_CTRL: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_AUTOLOAD_SCT0_ADDR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_AUTOLOAD_SCT0_SIZE: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_AUTOLOAD_SCT1_ADDR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_AUTOLOAD_SCT1_SIZE: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub IBUS_TO_FLASH_START_VADDR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub IBUS_TO_FLASH_END_VADDR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub DBUS_TO_FLASH_START_VADDR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub DBUS_TO_FLASH_END_VADDR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_ACS_CNT_CLR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub IBUS_ACS_MISS_CNT: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub IBUS_ACS_CNT: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub DBUS_ACS_FLASH_MISS_CNT: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub DBUS_ACS_CNT: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_ILG_INT_ENA: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_ILG_INT_CLR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_ILG_INT_ST: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CORE0_ACS_CACHE_INT_ENA: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CORE0_ACS_CACHE_INT_CLR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CORE0_ACS_CACHE_INT_ST: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CORE0_DBUS_REJECT_ST: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CORE0_DBUS_REJECT_VADDR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CORE0_IBUS_REJECT_ST: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CORE0_IBUS_REJECT_VADDR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_MMU_FAULT_CONTENT: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_MMU_FAULT_VADDR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_WRAP_AROUND_CTRL: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_MMU_POWER_CTRL: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_STATE: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_PRELOAD_INT_CTRL: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_SYNC_INT_CTRL: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_MMU_OWNER: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_CONF_MISC: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_FREEZE: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub ICACHE_ATOMIC_OPERATE_ENA: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CACHE_REQUEST: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub IBUS_PMS_TBL_LOCK: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub IBUS_PMS_TBL_BOUNDARY0: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub IBUS_PMS_TBL_BOUNDARY1: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub IBUS_PMS_TBL_BOUNDARY2: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub IBUS_PMS_TBL_ATTR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub DBUS_PMS_TBL_LOCK: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub DBUS_PMS_TBL_BOUNDARY0: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub DBUS_PMS_TBL_BOUNDARY1: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub DBUS_PMS_TBL_BOUNDARY2: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub DBUS_PMS_TBL_ATTR: crate::RWRegister<u32>,
    #[doc = "This description will be updated in the near future."]
    pub CLOCK_GATE: crate::RWRegister<u32>,
    _reserved0: [u8; 0x02f8],
    #[doc = "This description will be updated in the near future."]
    pub REG_DATE: crate::RWRegister<u32>,
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_CTRL {
    #[doc = "The bit is used to activate the data cache. 0: disable, 1: enable"]
    pub mod ICACHE_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_CTRL1 {
    #[doc = "The bit is used to disable core0 ibus, 0: enable, 1: disable"]
    pub mod ICACHE_SHUT_IBUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to disable core1 ibus, 0: enable, 1: disable"]
    pub mod ICACHE_SHUT_DBUS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_TAG_POWER_CTRL {
    #[doc = "The bit is used to close clock gating of icache tag memory. 1: close gating, 0: open clock gating."]
    pub mod ICACHE_TAG_MEM_FORCE_ON {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to power icache tag memory down, 0: follow rtc_lslp, 1: power down"]
    pub mod ICACHE_TAG_MEM_FORCE_PD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to power icache tag memory up, 0: follow rtc_lslp, 1: power up"]
    pub mod ICACHE_TAG_MEM_FORCE_PU {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_PRELOCK_CTRL {
    #[doc = "The bit is used to enable the first section of prelock function."]
    pub mod ICACHE_PRELOCK_SCT0_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable the second section of prelock function."]
    pub mod ICACHE_PRELOCK_SCT1_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_PRELOCK_SCT0_ADDR {
    #[doc = "The bits are used to configure the first start virtual address of data prelock, which is combined with ICACHE_PRELOCK_SCT0_SIZE_REG"]
    pub mod ICACHE_PRELOCK_SCT0_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_PRELOCK_SCT1_ADDR {
    #[doc = "The bits are used to configure the second start virtual address of data prelock, which is combined with ICACHE_PRELOCK_SCT1_SIZE_REG"]
    pub mod ICACHE_PRELOCK_SCT1_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_PRELOCK_SCT_SIZE {
    #[doc = "The bits are used to configure the second length of data locking, which is combined with ICACHE_PRELOCK_SCT1_ADDR_REG"]
    pub mod ICACHE_PRELOCK_SCT1_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bits are used to configure the first length of data locking, which is combined with ICACHE_PRELOCK_SCT0_ADDR_REG"]
    pub mod ICACHE_PRELOCK_SCT0_SIZE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_LOCK_CTRL {
    #[doc = "The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
    pub mod ICACHE_LOCK_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
    pub mod ICACHE_UNLOCK_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate unlock/lock operation is finished."]
    pub mod ICACHE_LOCK_DONE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_LOCK_ADDR {
    #[doc = "The bits are used to configure the start virtual address for lock operations. It should be combined with ICACHE_LOCK_SIZE_REG."]
    pub mod ICACHE_LOCK_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_LOCK_SIZE {
    #[doc = "The bits are used to configure the length for lock operations. The bits are the counts of cache block. It should be combined with ICACHE_LOCK_ADDR_REG."]
    pub mod ICACHE_LOCK_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_SYNC_CTRL {
    #[doc = "The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done."]
    pub mod ICACHE_INVALIDATE_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate invalidate operation is finished."]
    pub mod ICACHE_SYNC_DONE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_SYNC_ADDR {
    #[doc = "The bits are used to configure the start virtual address for clean operations. It should be combined with ICACHE_SYNC_SIZE_REG."]
    pub mod ICACHE_SYNC_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_SYNC_SIZE {
    #[doc = "The bits are used to configure the length for sync operations. The bits are the counts of cache block. It should be combined with ICACHE_SYNC_ADDR_REG."]
    pub mod ICACHE_SYNC_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x007f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_PRELOAD_CTRL {
    #[doc = "The bit is used to enable preload operation. It will be cleared by hardware after preload operation done."]
    pub mod ICACHE_PRELOAD_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate preload operation is finished."]
    pub mod ICACHE_PRELOAD_DONE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to configure the direction of preload operation. 1: descending, 0: ascending."]
    pub mod ICACHE_PRELOAD_ORDER {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_PRELOAD_ADDR {
    #[doc = "The bits are used to configure the start virtual address for preload operation. It should be combined with ICACHE_PRELOAD_SIZE_REG."]
    pub mod ICACHE_PRELOAD_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_PRELOAD_SIZE {
    #[doc = "The bits are used to configure the length for preload operation. The bits are the counts of cache block. It should be combined with ICACHE_PRELOAD_ADDR_REG.."]
    pub mod ICACHE_PRELOAD_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_AUTOLOAD_CTRL {
    #[doc = "The bits are used to enable the first section for autoload operation."]
    pub mod ICACHE_AUTOLOAD_SCT0_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bits are used to enable the second section for autoload operation."]
    pub mod ICACHE_AUTOLOAD_SCT1_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable and disable autoload operation. It is combined with icache_autoload_done. 1: enable, 0: disable."]
    pub mod ICACHE_AUTOLOAD_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate autoload operation is finished."]
    pub mod ICACHE_AUTOLOAD_DONE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
    pub mod ICACHE_AUTOLOAD_ORDER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    pub mod ICACHE_AUTOLOAD_RQST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_AUTOLOAD_SCT0_ADDR {
    #[doc = "The bits are used to configure the start virtual address of the first section for autoload operation. It should be combined with icache_autoload_sct0_ena."]
    pub mod ICACHE_AUTOLOAD_SCT0_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_AUTOLOAD_SCT0_SIZE {
    #[doc = "The bits are used to configure the length of the first section for autoload operation. It should be combined with icache_autoload_sct0_ena."]
    pub mod ICACHE_AUTOLOAD_SCT0_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_AUTOLOAD_SCT1_ADDR {
    #[doc = "The bits are used to configure the start virtual address of the second section for autoload operation. It should be combined with icache_autoload_sct1_ena."]
    pub mod ICACHE_AUTOLOAD_SCT1_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_AUTOLOAD_SCT1_SIZE {
    #[doc = "The bits are used to configure the length of the second section for autoload operation. It should be combined with icache_autoload_sct1_ena."]
    pub mod ICACHE_AUTOLOAD_SCT1_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod IBUS_TO_FLASH_START_VADDR {
    #[doc = "The bits are used to configure the start virtual address of ibus to access flash. The register is used to give constraints to ibus access counter."]
    pub mod IBUS_TO_FLASH_START_VADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod IBUS_TO_FLASH_END_VADDR {
    #[doc = "The bits are used to configure the end virtual address of ibus to access flash. The register is used to give constraints to ibus access counter."]
    pub mod IBUS_TO_FLASH_END_VADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod DBUS_TO_FLASH_START_VADDR {
    #[doc = "The bits are used to configure the start virtual address of dbus to access flash. The register is used to give constraints to dbus access counter."]
    pub mod DBUS_TO_FLASH_START_VADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod DBUS_TO_FLASH_END_VADDR {
    #[doc = "The bits are used to configure the end virtual address of dbus to access flash. The register is used to give constraints to dbus access counter."]
    pub mod DBUS_TO_FLASH_END_VADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_ACS_CNT_CLR {
    #[doc = "The bit is used to clear ibus counter."]
    pub mod IBUS_ACS_CNT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to clear dbus counter."]
    pub mod DBUS_ACS_CNT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod IBUS_ACS_MISS_CNT {
    #[doc = "The bits are used to count the number of the cache miss caused by ibus access flash."]
    pub mod IBUS_ACS_MISS_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod IBUS_ACS_CNT {
    #[doc = "The bits are used to count the number of ibus access flash through icache."]
    pub mod IBUS_ACS_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod DBUS_ACS_FLASH_MISS_CNT {
    #[doc = "The bits are used to count the number of the cache miss caused by dbus access flash."]
    pub mod DBUS_ACS_FLASH_MISS_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod DBUS_ACS_CNT {
    #[doc = "The bits are used to count the number of dbus access flash through icache."]
    pub mod DBUS_ACS_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_ILG_INT_ENA {
    #[doc = "The bit is used to enable interrupt by sync configurations fault."]
    pub mod ICACHE_SYNC_OP_FAULT_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable interrupt by preload configurations fault."]
    pub mod ICACHE_PRELOAD_OP_FAULT_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable interrupt by mmu entry fault."]
    pub mod MMU_ENTRY_FAULT_INT_ENA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable interrupt by ibus counter overflow."]
    pub mod IBUS_CNT_OVF_INT_ENA {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable interrupt by dbus counter overflow."]
    pub mod DBUS_CNT_OVF_INT_ENA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_ILG_INT_CLR {
    #[doc = "The bit is used to clear interrupt by sync configurations fault."]
    pub mod ICACHE_SYNC_OP_FAULT_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to clear interrupt by preload configurations fault."]
    pub mod ICACHE_PRELOAD_OP_FAULT_INT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to clear interrupt by mmu entry fault."]
    pub mod MMU_ENTRY_FAULT_INT_CLR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to clear interrupt by ibus counter overflow."]
    pub mod IBUS_CNT_OVF_INT_CLR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to clear interrupt by dbus counter overflow."]
    pub mod DBUS_CNT_OVF_INT_CLR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_ILG_INT_ST {
    #[doc = "The bit is used to indicate interrupt by sync configurations fault."]
    pub mod ICACHE_SYNC_OP_FAULT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate interrupt by preload configurations fault."]
    pub mod ICACHE_PRELOAD_OP_FAULT_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate interrupt by mmu entry fault."]
    pub mod MMU_ENTRY_FAULT_ST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate interrupt by ibus access flash/spiram counter overflow."]
    pub mod IBUS_ACS_CNT_OVF_ST {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate interrupt by ibus access flash/spiram miss counter overflow."]
    pub mod IBUS_ACS_MISS_CNT_OVF_ST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate interrupt by dbus access flash/spiram counter overflow."]
    pub mod DBUS_ACS_CNT_OVF_ST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate interrupt by dbus access flash miss counter overflow."]
    pub mod DBUS_ACS_FLASH_MISS_CNT_OVF_ST {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CORE0_ACS_CACHE_INT_ENA {
    #[doc = "The bit is used to enable interrupt by cpu access icache while the corresponding ibus is disabled which include speculative access."]
    pub mod CORE0_IBUS_ACS_MSK_IC_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable interrupt by ibus trying to write icache"]
    pub mod CORE0_IBUS_WR_IC_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable interrupt by authentication fail."]
    pub mod CORE0_IBUS_REJECT_INT_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable interrupt by cpu access icache while the corresponding dbus is disabled which include speculative access."]
    pub mod CORE0_DBUS_ACS_MSK_IC_INT_ENA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable interrupt by authentication fail."]
    pub mod CORE0_DBUS_REJECT_INT_ENA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable interrupt by dbus trying to write icache"]
    pub mod CORE0_DBUS_WR_IC_INT_ENA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CORE0_ACS_CACHE_INT_CLR {
    #[doc = "The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access."]
    pub mod CORE0_IBUS_ACS_MSK_IC_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to clear interrupt by ibus trying to write icache"]
    pub mod CORE0_IBUS_WR_IC_INT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to clear interrupt by authentication fail."]
    pub mod CORE0_IBUS_REJECT_INT_CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to clear interrupt by cpu access icache while the corresponding dbus is disabled or icache is disabled which include speculative access."]
    pub mod CORE0_DBUS_ACS_MSK_IC_INT_CLR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to clear interrupt by authentication fail."]
    pub mod CORE0_DBUS_REJECT_INT_CLR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to clear interrupt by dbus trying to write icache"]
    pub mod CORE0_DBUS_WR_IC_INT_CLR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CORE0_ACS_CACHE_INT_ST {
    #[doc = "The bit is used to indicate interrupt by cpu access icache while the core0_ibus is disabled or icache is disabled which include speculative access."]
    pub mod CORE0_IBUS_ACS_MSK_ICACHE_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate interrupt by ibus trying to write icache"]
    pub mod CORE0_IBUS_WR_ICACHE_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate interrupt by authentication fail."]
    pub mod CORE0_IBUS_REJECT_ST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate interrupt by cpu access icache while the core0_dbus is disabled or icache is disabled which include speculative access."]
    pub mod CORE0_DBUS_ACS_MSK_ICACHE_ST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate interrupt by authentication fail."]
    pub mod CORE0_DBUS_REJECT_ST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate interrupt by dbus trying to write icache"]
    pub mod CORE0_DBUS_WR_ICACHE_ST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CORE0_DBUS_REJECT_ST {
    #[doc = "The bits are used to indicate the attribute of CPU access dbus when authentication fail. 0: invalidate, 1: execute-able, 2: read-able, 4: write-able."]
    pub mod CORE0_DBUS_ATTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate the world of CPU access dbus when authentication fail. 0: WORLD0, 1: WORLD1"]
    pub mod CORE0_DBUS_WORLD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CORE0_DBUS_REJECT_VADDR {
    #[doc = "The bits are used to indicate the virtual address of CPU access dbus when authentication fail."]
    pub mod CORE0_DBUS_VADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CORE0_IBUS_REJECT_ST {
    #[doc = "The bits are used to indicate the attribute of CPU access ibus when authentication fail. 0: invalidate, 1: execute-able, 2: read-able"]
    pub mod CORE0_IBUS_ATTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate the world of CPU access ibus when authentication fail. 0: WORLD0, 1: WORLD1"]
    pub mod CORE0_IBUS_WORLD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CORE0_IBUS_REJECT_VADDR {
    #[doc = "The bits are used to indicate the virtual address of CPU access ibus when authentication fail."]
    pub mod CORE0_IBUS_VADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_MMU_FAULT_CONTENT {
    #[doc = "The bits are used to indicate the content of mmu entry which cause mmu fault.."]
    pub mod CACHE_MMU_FAULT_CONTENT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The right-most 3 bits are used to indicate the operations which cause mmu fault occurrence. 0: default, 1: cpu miss, 2: preload miss, 3: writeback, 4: cpu miss evict recovery address, 5: load miss evict recovery address, 6: external dma tx, 7: external dma rx. The most significant bit is used to indicate this operation occurs in which one icache."]
    pub mod CACHE_MMU_FAULT_CODE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_MMU_FAULT_VADDR {
    #[doc = "The bits are used to indicate the virtual address which cause mmu fault.."]
    pub mod CACHE_MMU_FAULT_VADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_WRAP_AROUND_CTRL {
    #[doc = "The bit is used to enable wrap around mode when read data from flash."]
    pub mod CACHE_FLASH_WRAP_AROUND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_MMU_POWER_CTRL {
    #[doc = "The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
    pub mod CACHE_MMU_MEM_FORCE_ON {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
    pub mod CACHE_MMU_MEM_FORCE_PD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
    pub mod CACHE_MMU_MEM_FORCE_PU {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_STATE {
    #[doc = "The bit is used to indicate whether icache main fsm is in idle state or not. 1: in idle state, 0: not in idle state"]
    pub mod ICACHE_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE {
    #[doc = "Reserved."]
    pub mod RECORD_DISABLE_DB_ENCRYPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved."]
    pub mod RECORD_DISABLE_G0CB_DECRYPT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON {
    #[doc = "The bit is used to close clock gating of manual crypt clock. 1: close gating, 0: open clock gating."]
    pub mod CLK_FORCE_ON_MANUAL_CRYPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to close clock gating of automatic crypt clock. 1: close gating, 0: open clock gating."]
    pub mod CLK_FORCE_ON_AUTO_CRYPT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to close clock gating of external memory encrypt and decrypt clock. 1: close gating, 0: open clock gating."]
    pub mod CLK_FORCE_ON_CRYPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_PRELOAD_INT_CTRL {
    #[doc = "The bit is used to indicate the interrupt by icache pre-load done."]
    pub mod ICACHE_PRELOAD_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable the interrupt by icache pre-load done."]
    pub mod ICACHE_PRELOAD_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to clear the interrupt by icache pre-load done."]
    pub mod ICACHE_PRELOAD_INT_CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_SYNC_INT_CTRL {
    #[doc = "The bit is used to indicate the interrupt by icache sync done."]
    pub mod ICACHE_SYNC_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable the interrupt by icache sync done."]
    pub mod ICACHE_SYNC_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to clear the interrupt by icache sync done."]
    pub mod ICACHE_SYNC_INT_CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_MMU_OWNER {
    #[doc = "The bits are used to specify the owner of MMU.bit0/bit2: ibus, bit1/bit3: dbus"]
    pub mod CACHE_MMU_OWNER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_CONF_MISC {
    #[doc = "The bit is used to disable checking mmu entry fault by preload operation."]
    pub mod CACHE_IGNORE_PRELOAD_MMU_ENTRY_FAULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to disable checking mmu entry fault by sync operation."]
    pub mod CACHE_IGNORE_SYNC_MMU_ENTRY_FAULT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to enable cache trace function."]
    pub mod CACHE_TRACE_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_FREEZE {
    #[doc = "The bit is used to enable icache freeze mode"]
    pub mod ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to configure freeze mode, 0: assert busy if CPU miss 1: assert hit if CPU miss"]
    pub mod MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to indicate icache freeze success"]
    pub mod DONE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod ICACHE_ATOMIC_OPERATE_ENA {
    #[doc = "The bit is used to activate icache atomic operation protection. In this case, sync/lock operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
    pub mod ICACHE_ATOMIC_OPERATE_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CACHE_REQUEST {
    #[doc = "The bit is used to disable request recording which could cause performance issue"]
    pub mod BYPASS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod IBUS_PMS_TBL_LOCK {
    #[doc = "The bit is used to configure the ibus permission control section boundary0"]
    pub mod IBUS_PMS_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod IBUS_PMS_TBL_BOUNDARY0 {
    #[doc = "The bit is used to configure the ibus permission control section boundary0"]
    pub mod IBUS_PMS_BOUNDARY0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod IBUS_PMS_TBL_BOUNDARY1 {
    #[doc = "The bit is used to configure the ibus permission control section boundary1"]
    pub mod IBUS_PMS_BOUNDARY1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod IBUS_PMS_TBL_BOUNDARY2 {
    #[doc = "The bit is used to configure the ibus permission control section boundary2"]
    pub mod IBUS_PMS_BOUNDARY2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod IBUS_PMS_TBL_ATTR {
    #[doc = "The bit is used to configure attribute of the ibus permission control section1, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
    pub mod IBUS_PMS_SCT1_ATTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to configure attribute of the ibus permission control section2, bit0: fetch in world0, bit1: load in world0, bit2: fetch in world1, bit3: load in world1"]
    pub mod IBUS_PMS_SCT2_ATTR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod DBUS_PMS_TBL_LOCK {
    #[doc = "The bit is used to configure the ibus permission control section boundary0"]
    pub mod DBUS_PMS_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod DBUS_PMS_TBL_BOUNDARY0 {
    #[doc = "The bit is used to configure the dbus permission control section boundary0"]
    pub mod DBUS_PMS_BOUNDARY0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod DBUS_PMS_TBL_BOUNDARY1 {
    #[doc = "The bit is used to configure the dbus permission control section boundary1"]
    pub mod DBUS_PMS_BOUNDARY1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod DBUS_PMS_TBL_BOUNDARY2 {
    #[doc = "The bit is used to configure the dbus permission control section boundary2"]
    pub mod DBUS_PMS_BOUNDARY2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod DBUS_PMS_TBL_ATTR {
    #[doc = "The bit is used to configure attribute of the dbus permission control section1, bit0: load in world0, bit2: load in world1"]
    pub mod DBUS_PMS_SCT1_ATTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bit is used to configure attribute of the dbus permission control section2, bit0: load in world0, bit2: load in world1"]
    pub mod DBUS_PMS_SCT2_ATTR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod CLOCK_GATE {
    #[doc = "clock gate enable."]
    pub mod CLK_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "This description will be updated in the near future."]
pub mod REG_DATE {
    #[doc = "version information"]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
