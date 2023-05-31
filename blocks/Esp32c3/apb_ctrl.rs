#[doc = "APB (Advanced Peripheral Bus) Controller"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "APB_CTRL_SYSCLK_CONF_REG"]
    pub SYSCLK_CONF: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_TICK_CONF_REG"]
    pub TICK_CONF: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_CLK_OUT_EN_REG"]
    pub CLK_OUT_EN: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_WIFI_BB_CFG_REG"]
    pub WIFI_BB_CFG: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_WIFI_BB_CFG_2_REG"]
    pub WIFI_BB_CFG_2: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_WIFI_CLK_EN_REG"]
    pub WIFI_CLK_EN: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_WIFI_RST_EN_REG"]
    pub WIFI_RST_EN: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_HOST_INF_SEL_REG"]
    pub HOST_INF_SEL: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_EXT_MEM_PMS_LOCK_REG"]
    pub EXT_MEM_PMS_LOCK: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "APB_CTRL_FLASH_ACE0_ATTR_REG"]
    pub FLASH_ACE0_ATTR: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_FLASH_ACE1_ATTR_REG"]
    pub FLASH_ACE1_ATTR: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_FLASH_ACE2_ATTR_REG"]
    pub FLASH_ACE2_ATTR: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_FLASH_ACE3_ATTR_REG"]
    pub FLASH_ACE3_ATTR: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_FLASH_ACE0_ADDR_REG"]
    pub FLASH_ACE0_ADDR: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_FLASH_ACE1_ADDR_REG"]
    pub FLASH_ACE1_ADDR: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_FLASH_ACE2_ADDR_REG"]
    pub FLASH_ACE2_ADDR: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_FLASH_ACE3_ADDR_REG"]
    pub FLASH_ACE3_ADDR: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_FLASH_ACE0_SIZE_REG"]
    pub FLASH_ACE0_SIZE: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_FLASH_ACE1_SIZE_REG"]
    pub FLASH_ACE1_SIZE: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_FLASH_ACE2_SIZE_REG"]
    pub FLASH_ACE2_SIZE: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_FLASH_ACE3_SIZE_REG"]
    pub FLASH_ACE3_SIZE: crate::RWRegister<u32>,
    _reserved1: [u8; 0x30],
    #[doc = "APB_CTRL_SPI_MEM_PMS_CTRL_REG"]
    pub SPI_MEM_PMS_CTRL: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_SPI_MEM_REJECT_ADDR_REG"]
    pub SPI_MEM_REJECT_ADDR: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_SDIO_CTRL_REG"]
    pub SDIO_CTRL: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_REDCY_SIG0_REG_REG"]
    pub REDCY_SIG0: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_REDCY_SIG1_REG_REG"]
    pub REDCY_SIG1: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_FRONT_END_MEM_PD_REG"]
    pub FRONT_END_MEM_PD: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_RETENTION_CTRL_REG"]
    pub RETENTION_CTRL: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_CLKGATE_FORCE_ON_REG"]
    pub CLKGATE_FORCE_ON: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_MEM_POWER_DOWN_REG"]
    pub MEM_POWER_DOWN: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_MEM_POWER_UP_REG"]
    pub MEM_POWER_UP: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_RND_DATA_REG"]
    pub RND_DATA: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_PERI_BACKUP_CONFIG_REG_REG"]
    pub PERI_BACKUP_CONFIG: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_PERI_BACKUP_APB_ADDR_REG_REG"]
    pub PERI_BACKUP_APB_ADDR: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_PERI_BACKUP_MEM_ADDR_REG_REG"]
    pub PERI_BACKUP_MEM_ADDR: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_PERI_BACKUP_INT_RAW_REG"]
    pub PERI_BACKUP_INT_RAW: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_PERI_BACKUP_INT_ST_REG"]
    pub PERI_BACKUP_INT_ST: crate::RWRegister<u32>,
    #[doc = "APB_CTRL_PERI_BACKUP_INT_ENA_REG"]
    pub PERI_BACKUP_INT_ENA: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "APB_CTRL_PERI_BACKUP_INT_CLR_REG"]
    pub PERI_BACKUP_INT_CLR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0328],
    #[doc = "APB_CTRL_DATE_REG"]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "APB_CTRL_SYSCLK_CONF_REG"]
pub mod SYSCLK_CONF {
    #[doc = "reg_pre_div_cnt"]
    pub mod PRE_DIV_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_320m_en"]
    pub mod CLK_320M_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_en"]
    pub mod CLK_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rst_tick_cnt"]
    pub mod RST_TICK_CNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_TICK_CONF_REG"]
pub mod TICK_CONF {
    #[doc = "reg_xtal_tick_num"]
    pub mod XTAL_TICK_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ck8m_tick_num"]
    pub mod CK8M_TICK_NUM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_tick_enable"]
    pub mod TICK_ENABLE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_CLK_OUT_EN_REG"]
pub mod CLK_OUT_EN {
    #[doc = "reg_clk20_oen"]
    pub mod CLK20_OEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk22_oen"]
    pub mod CLK22_OEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk44_oen"]
    pub mod CLK44_OEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_bb_oen"]
    pub mod CLK_BB_OEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk80_oen"]
    pub mod CLK80_OEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk160_oen"]
    pub mod CLK160_OEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_320m_oen"]
    pub mod CLK_320M_OEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_adc_inf_oen"]
    pub mod CLK_ADC_INF_OEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_dac_cpu_oen"]
    pub mod CLK_DAC_CPU_OEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk40x_bb_oen"]
    pub mod CLK40X_BB_OEN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_xtal_oen"]
    pub mod CLK_XTAL_OEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_WIFI_BB_CFG_REG"]
pub mod WIFI_BB_CFG {
    #[doc = "reg_wifi_bb_cfg"]
    pub mod WIFI_BB_CFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_WIFI_BB_CFG_2_REG"]
pub mod WIFI_BB_CFG_2 {
    #[doc = "reg_wifi_bb_cfg_2"]
    pub mod WIFI_BB_CFG_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_WIFI_CLK_EN_REG"]
pub mod WIFI_CLK_EN {
    #[doc = "reg_wifi_clk_en"]
    pub mod WIFI_CLK_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_WIFI_RST_EN_REG"]
pub mod WIFI_RST_EN {
    #[doc = "reg_wifi_rst"]
    pub mod WIFI_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_HOST_INF_SEL_REG"]
pub mod HOST_INF_SEL {
    #[doc = "reg_peri_io_swap"]
    pub mod PERI_IO_SWAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_EXT_MEM_PMS_LOCK_REG"]
pub mod EXT_MEM_PMS_LOCK {
    #[doc = "reg_ext_mem_pms_lock"]
    pub mod EXT_MEM_PMS_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_FLASH_ACE0_ATTR_REG"]
pub mod FLASH_ACE0_ATTR {
    #[doc = "reg_flash_ace0_attr"]
    pub mod FLASH_ACE0_ATTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_FLASH_ACE1_ATTR_REG"]
pub mod FLASH_ACE1_ATTR {
    #[doc = "reg_flash_ace1_attr"]
    pub mod FLASH_ACE1_ATTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_FLASH_ACE2_ATTR_REG"]
pub mod FLASH_ACE2_ATTR {
    #[doc = "reg_flash_ace2_attr"]
    pub mod FLASH_ACE2_ATTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_FLASH_ACE3_ATTR_REG"]
pub mod FLASH_ACE3_ATTR {
    #[doc = "reg_flash_ace3_attr"]
    pub mod FLASH_ACE3_ATTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_FLASH_ACE0_ADDR_REG"]
pub mod FLASH_ACE0_ADDR {
    #[doc = "reg_flash_ace0_addr_s"]
    pub mod S {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_FLASH_ACE1_ADDR_REG"]
pub mod FLASH_ACE1_ADDR {
    #[doc = "reg_flash_ace1_addr_s"]
    pub mod S {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_FLASH_ACE2_ADDR_REG"]
pub mod FLASH_ACE2_ADDR {
    #[doc = "reg_flash_ace2_addr_s"]
    pub mod S {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_FLASH_ACE3_ADDR_REG"]
pub mod FLASH_ACE3_ADDR {
    #[doc = "reg_flash_ace3_addr_s"]
    pub mod S {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_FLASH_ACE0_SIZE_REG"]
pub mod FLASH_ACE0_SIZE {
    #[doc = "reg_flash_ace0_size"]
    pub mod FLASH_ACE0_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_FLASH_ACE1_SIZE_REG"]
pub mod FLASH_ACE1_SIZE {
    #[doc = "reg_flash_ace1_size"]
    pub mod FLASH_ACE1_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_FLASH_ACE2_SIZE_REG"]
pub mod FLASH_ACE2_SIZE {
    #[doc = "reg_flash_ace2_size"]
    pub mod FLASH_ACE2_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_FLASH_ACE3_SIZE_REG"]
pub mod FLASH_ACE3_SIZE {
    #[doc = "reg_flash_ace3_size"]
    pub mod FLASH_ACE3_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_SPI_MEM_PMS_CTRL_REG"]
pub mod SPI_MEM_PMS_CTRL {
    #[doc = "reg_spi_mem_reject_int"]
    pub mod SPI_MEM_REJECT_INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_spi_mem_reject_clr"]
    pub mod SPI_MEM_REJECT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_spi_mem_reject_cde"]
    pub mod SPI_MEM_REJECT_CDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_SPI_MEM_REJECT_ADDR_REG"]
pub mod SPI_MEM_REJECT_ADDR {
    #[doc = "reg_spi_mem_reject_addr"]
    pub mod SPI_MEM_REJECT_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_SDIO_CTRL_REG"]
pub mod SDIO_CTRL {
    #[doc = "reg_sdio_win_access_en"]
    pub mod SDIO_WIN_ACCESS_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_REDCY_SIG0_REG_REG"]
pub mod REDCY_SIG0 {
    #[doc = "reg_redcy_sig0"]
    pub mod REDCY_SIG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_redcy_andor"]
    pub mod REDCY_ANDOR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_REDCY_SIG1_REG_REG"]
pub mod REDCY_SIG1 {
    #[doc = "reg_redcy_sig1"]
    pub mod REDCY_SIG1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_redcy_nandor"]
    pub mod REDCY_NANDOR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_FRONT_END_MEM_PD_REG"]
pub mod FRONT_END_MEM_PD {
    #[doc = "reg_agc_mem_force_pu"]
    pub mod AGC_MEM_FORCE_PU {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_agc_mem_force_pd"]
    pub mod AGC_MEM_FORCE_PD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_pbus_mem_force_pu"]
    pub mod PBUS_MEM_FORCE_PU {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_pbus_mem_force_pd"]
    pub mod PBUS_MEM_FORCE_PD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_dc_mem_force_pu"]
    pub mod DC_MEM_FORCE_PU {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_dc_mem_force_pd"]
    pub mod DC_MEM_FORCE_PD {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_RETENTION_CTRL_REG"]
pub mod RETENTION_CTRL {
    #[doc = "reg_retention_link_addr"]
    pub mod RETENTION_LINK_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_nobypass_cpu_iso_rst"]
    pub mod NOBYPASS_CPU_ISO_RST {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_CLKGATE_FORCE_ON_REG"]
pub mod CLKGATE_FORCE_ON {
    #[doc = "reg_rom_clkgate_force_on"]
    pub mod ROM_CLKGATE_FORCE_ON {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sram_clkgate_force_on"]
    pub mod SRAM_CLKGATE_FORCE_ON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_MEM_POWER_DOWN_REG"]
pub mod MEM_POWER_DOWN {
    #[doc = "reg_rom_power_down"]
    pub mod ROM_POWER_DOWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sram_power_down"]
    pub mod SRAM_POWER_DOWN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_MEM_POWER_UP_REG"]
pub mod MEM_POWER_UP {
    #[doc = "reg_rom_power_up"]
    pub mod ROM_POWER_UP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sram_power_up"]
    pub mod SRAM_POWER_UP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_RND_DATA_REG"]
pub mod RND_DATA {
    #[doc = "reg_rnd_data"]
    pub mod RND_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_CONFIG_REG_REG"]
pub mod PERI_BACKUP_CONFIG {
    #[doc = "reg_peri_backup_flow_err"]
    pub mod PERI_BACKUP_FLOW_ERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_peri_backup_burst_limit"]
    pub mod PERI_BACKUP_BURST_LIMIT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_peri_backup_tout_thres"]
    pub mod PERI_BACKUP_TOUT_THRES {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_peri_backup_size"]
    pub mod PERI_BACKUP_SIZE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_peri_backup_start"]
    pub mod PERI_BACKUP_START {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_peri_backup_to_mem"]
    pub mod PERI_BACKUP_TO_MEM {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_peri_backup_ena"]
    pub mod PERI_BACKUP_ENA {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_APB_ADDR_REG_REG"]
pub mod PERI_BACKUP_APB_ADDR {
    #[doc = "reg_backup_apb_start_addr"]
    pub mod BACKUP_APB_START_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_MEM_ADDR_REG_REG"]
pub mod PERI_BACKUP_MEM_ADDR {
    #[doc = "reg_backup_mem_start_addr"]
    pub mod BACKUP_MEM_START_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_INT_RAW_REG"]
pub mod PERI_BACKUP_INT_RAW {
    #[doc = "reg_peri_backup_done_int_raw"]
    pub mod PERI_BACKUP_DONE_INT_RAW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_peri_backup_err_int_raw"]
    pub mod PERI_BACKUP_ERR_INT_RAW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_INT_ST_REG"]
pub mod PERI_BACKUP_INT_ST {
    #[doc = "reg_peri_backup_done_int_st"]
    pub mod PERI_BACKUP_DONE_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_peri_backup_err_int_st"]
    pub mod PERI_BACKUP_ERR_INT_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_INT_ENA_REG"]
pub mod PERI_BACKUP_INT_ENA {
    #[doc = "reg_peri_backup_done_int_ena"]
    pub mod PERI_BACKUP_DONE_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_peri_backup_err_int_ena"]
    pub mod PERI_BACKUP_ERR_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_INT_CLR_REG"]
pub mod PERI_BACKUP_INT_CLR {
    #[doc = "reg_peri_backup_done_int_clr"]
    pub mod PERI_BACKUP_DONE_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_peri_backup_err_int_clr"]
    pub mod PERI_BACKUP_ERR_INT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APB_CTRL_DATE_REG"]
pub mod DATE {
    #[doc = "reg_dateVersion control"]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
