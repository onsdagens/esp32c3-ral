#[doc = "Interrupt Controller (Core 0)"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "mac intr map register"]
    pub MAC_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "mac nmi_intr map register"]
    pub MAC_NMI_MAP: crate::RWRegister<u32>,
    #[doc = "pwr intr map register"]
    pub PWR_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "bb intr map register"]
    pub BB_INT_MAP: crate::RWRegister<u32>,
    #[doc = "bt intr map register"]
    pub BT_MAC_INT_MAP: crate::RWRegister<u32>,
    #[doc = "bb_bt intr map register"]
    pub BT_BB_INT_MAP: crate::RWRegister<u32>,
    #[doc = "bb_bt_nmi intr map register"]
    pub BT_BB_NMI_MAP: crate::RWRegister<u32>,
    #[doc = "rwbt intr map register"]
    pub RWBT_IRQ_MAP: crate::RWRegister<u32>,
    #[doc = "rwble intr map register"]
    pub RWBLE_IRQ_MAP: crate::RWRegister<u32>,
    #[doc = "rwbt_nmi intr map register"]
    pub RWBT_NMI_MAP: crate::RWRegister<u32>,
    #[doc = "rwble_nmi intr map register"]
    pub RWBLE_NMI_MAP: crate::RWRegister<u32>,
    #[doc = "i2c intr map register"]
    pub I2C_MST_INT_MAP: crate::RWRegister<u32>,
    #[doc = "slc0 intr map register"]
    pub SLC0_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "slc1 intr map register"]
    pub SLC1_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "apb_ctrl intr map register"]
    pub APB_CTRL_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "uchi0 intr map register"]
    pub UHCI0_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "gpio intr map register"]
    pub GPIO_INTERRUPT_PRO_MAP: crate::RWRegister<u32>,
    #[doc = "gpio_pro intr map register"]
    pub GPIO_INTERRUPT_PRO_NMI_MAP: crate::RWRegister<u32>,
    #[doc = "gpio_pro_nmi intr map register"]
    pub SPI_INTR_1_MAP: crate::RWRegister<u32>,
    #[doc = "spi1 intr map register"]
    pub SPI_INTR_2_MAP: crate::RWRegister<u32>,
    #[doc = "spi2 intr map register"]
    pub I2S1_INT_MAP: crate::RWRegister<u32>,
    #[doc = "i2s1 intr map register"]
    pub UART_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "uart1 intr map register"]
    pub UART1_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "ledc intr map register"]
    pub LEDC_INT_MAP: crate::RWRegister<u32>,
    #[doc = "efuse intr map register"]
    pub EFUSE_INT_MAP: crate::RWRegister<u32>,
    #[doc = "can intr map register"]
    pub CAN_INT_MAP: crate::RWRegister<u32>,
    #[doc = "usb intr map register"]
    pub USB_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "rtc intr map register"]
    pub RTC_CORE_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "rmt intr map register"]
    pub RMT_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "i2c intr map register"]
    pub I2C_EXT0_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "timer1 intr map register"]
    pub TIMER_INT1_MAP: crate::RWRegister<u32>,
    #[doc = "timer2 intr map register"]
    pub TIMER_INT2_MAP: crate::RWRegister<u32>,
    #[doc = "tg to intr map register"]
    pub TG_T0_INT_MAP: crate::RWRegister<u32>,
    #[doc = "tg wdt intr map register"]
    pub TG_WDT_INT_MAP: crate::RWRegister<u32>,
    #[doc = "tg1 to intr map register"]
    pub TG1_T0_INT_MAP: crate::RWRegister<u32>,
    #[doc = "tg1 wdt intr map register"]
    pub TG1_WDT_INT_MAP: crate::RWRegister<u32>,
    #[doc = "cache ia intr map register"]
    pub CACHE_IA_INT_MAP: crate::RWRegister<u32>,
    #[doc = "systimer intr map register"]
    pub SYSTIMER_TARGET0_INT_MAP: crate::RWRegister<u32>,
    #[doc = "systimer target1 intr map register"]
    pub SYSTIMER_TARGET1_INT_MAP: crate::RWRegister<u32>,
    #[doc = "systimer target2 intr map register"]
    pub SYSTIMER_TARGET2_INT_MAP: crate::RWRegister<u32>,
    #[doc = "spi mem reject intr map register"]
    pub SPI_MEM_REJECT_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "icache perload intr map register"]
    pub ICACHE_PRELOAD_INT_MAP: crate::RWRegister<u32>,
    #[doc = "icache sync intr map register"]
    pub ICACHE_SYNC_INT_MAP: crate::RWRegister<u32>,
    #[doc = "adc intr map register"]
    pub APB_ADC_INT_MAP: crate::RWRegister<u32>,
    #[doc = "dma ch0 intr map register"]
    pub DMA_CH0_INT_MAP: crate::RWRegister<u32>,
    #[doc = "dma ch1 intr map register"]
    pub DMA_CH1_INT_MAP: crate::RWRegister<u32>,
    #[doc = "dma ch2 intr map register"]
    pub DMA_CH2_INT_MAP: crate::RWRegister<u32>,
    #[doc = "rsa intr map register"]
    pub RSA_INT_MAP: crate::RWRegister<u32>,
    #[doc = "aes intr map register"]
    pub AES_INT_MAP: crate::RWRegister<u32>,
    #[doc = "sha intr map register"]
    pub SHA_INT_MAP: crate::RWRegister<u32>,
    #[doc = "cpu from cpu 0 intr map register"]
    pub CPU_INTR_FROM_CPU_0_MAP: crate::RWRegister<u32>,
    #[doc = "cpu from cpu 0 intr map register"]
    pub CPU_INTR_FROM_CPU_1_MAP: crate::RWRegister<u32>,
    #[doc = "cpu from cpu 1 intr map register"]
    pub CPU_INTR_FROM_CPU_2_MAP: crate::RWRegister<u32>,
    #[doc = "cpu from cpu 3 intr map register"]
    pub CPU_INTR_FROM_CPU_3_MAP: crate::RWRegister<u32>,
    #[doc = "assist debug intr map register"]
    pub ASSIST_DEBUG_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "dma pms violatile intr map register"]
    pub DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "iram0 pms violatile intr map register"]
    pub CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub BACKUP_PMS_VIOLATE_INTR_MAP: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CACHE_CORE0_ACS_INT_MAP: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub INTR_STATUS_REG_0: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub INTR_STATUS_REG_1: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CLOCK_GATE: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_ENABLE: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_TYPE: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_CLEAR: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_EIP_STATUS: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_0: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_1: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_2: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_3: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_4: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_5: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_6: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_7: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_8: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_9: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_10: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_11: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_12: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_13: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_14: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_15: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_16: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_17: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_18: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_19: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_20: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_21: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_22: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_23: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_24: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_25: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_26: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_27: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_28: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_29: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_30: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_PRI_31: crate::RWRegister<u32>,
    #[doc = "mac intr map register"]
    pub CPU_INT_THRESH: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0664],
    #[doc = "mac intr map register"]
    pub INTERRUPT_REG_DATE: crate::RWRegister<u32>,
}
#[doc = "mac intr map register"]
pub mod MAC_INTR_MAP {
    #[doc = "core0_mac_intr_map"]
    pub mod MAC_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac nmi_intr map register"]
pub mod MAC_NMI_MAP {
    #[doc = "reg_core0_mac_nmi_map"]
    pub mod MAC_NMI_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "pwr intr map register"]
pub mod PWR_INTR_MAP {
    #[doc = "reg_core0_pwr_intr_map"]
    pub mod PWR_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "bb intr map register"]
pub mod BB_INT_MAP {
    #[doc = "reg_core0_bb_int_map"]
    pub mod BB_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "bt intr map register"]
pub mod BT_MAC_INT_MAP {
    #[doc = "reg_core0_bt_mac_int_map"]
    pub mod BT_MAC_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "bb_bt intr map register"]
pub mod BT_BB_INT_MAP {
    #[doc = "reg_core0_bt_bb_int_map"]
    pub mod BT_BB_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "bb_bt_nmi intr map register"]
pub mod BT_BB_NMI_MAP {
    #[doc = "reg_core0_bt_bb_nmi_map"]
    pub mod BT_BB_NMI_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rwbt intr map register"]
pub mod RWBT_IRQ_MAP {
    #[doc = "reg_core0_rwbt_irq_map"]
    pub mod RWBT_IRQ_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rwble intr map register"]
pub mod RWBLE_IRQ_MAP {
    #[doc = "reg_core0_rwble_irq_map"]
    pub mod RWBLE_IRQ_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rwbt_nmi intr map register"]
pub mod RWBT_NMI_MAP {
    #[doc = "reg_core0_rwbt_nmi_map"]
    pub mod RWBT_NMI_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rwble_nmi intr map register"]
pub mod RWBLE_NMI_MAP {
    #[doc = "reg_core0_rwble_nmi_map"]
    pub mod RWBLE_NMI_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "i2c intr map register"]
pub mod I2C_MST_INT_MAP {
    #[doc = "reg_core0_i2c_mst_int_map"]
    pub mod I2C_MST_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "slc0 intr map register"]
pub mod SLC0_INTR_MAP {
    #[doc = "reg_core0_slc0_intr_map"]
    pub mod SLC0_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "slc1 intr map register"]
pub mod SLC1_INTR_MAP {
    #[doc = "reg_core0_slc1_intr_map"]
    pub mod SLC1_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "apb_ctrl intr map register"]
pub mod APB_CTRL_INTR_MAP {
    #[doc = "reg_core0_apb_ctrl_intr_map"]
    pub mod APB_CTRL_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "uchi0 intr map register"]
pub mod UHCI0_INTR_MAP {
    #[doc = "reg_core0_uhci0_intr_map"]
    pub mod UHCI0_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "gpio intr map register"]
pub mod GPIO_INTERRUPT_PRO_MAP {
    #[doc = "reg_core0_gpio_interrupt_pro_map"]
    pub mod GPIO_INTERRUPT_PRO_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "gpio_pro intr map register"]
pub mod GPIO_INTERRUPT_PRO_NMI_MAP {
    #[doc = "reg_core0_gpio_interrupt_pro_nmi_map"]
    pub mod GPIO_INTERRUPT_PRO_NMI_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "gpio_pro_nmi intr map register"]
pub mod SPI_INTR_1_MAP {
    #[doc = "reg_core0_spi_intr_1_map"]
    pub mod SPI_INTR_1_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "spi1 intr map register"]
pub mod SPI_INTR_2_MAP {
    #[doc = "reg_core0_spi_intr_2_map"]
    pub mod SPI_INTR_2_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "spi2 intr map register"]
pub mod I2S1_INT_MAP {
    #[doc = "reg_core0_i2s1_int_map"]
    pub mod I2S1_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "i2s1 intr map register"]
pub mod UART_INTR_MAP {
    #[doc = "reg_core0_uart_intr_map"]
    pub mod UART_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "uart1 intr map register"]
pub mod UART1_INTR_MAP {
    #[doc = "reg_core0_uart1_intr_map"]
    pub mod UART1_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ledc intr map register"]
pub mod LEDC_INT_MAP {
    #[doc = "reg_core0_ledc_int_map"]
    pub mod LEDC_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "efuse intr map register"]
pub mod EFUSE_INT_MAP {
    #[doc = "reg_core0_efuse_int_map"]
    pub mod EFUSE_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "can intr map register"]
pub mod CAN_INT_MAP {
    #[doc = "reg_core0_can_int_map"]
    pub mod CAN_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "usb intr map register"]
pub mod USB_INTR_MAP {
    #[doc = "reg_core0_usb_intr_map"]
    pub mod USB_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rtc intr map register"]
pub mod RTC_CORE_INTR_MAP {
    #[doc = "reg_core0_rtc_core_intr_map"]
    pub mod RTC_CORE_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rmt intr map register"]
pub mod RMT_INTR_MAP {
    #[doc = "reg_core0_rmt_intr_map"]
    pub mod RMT_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "i2c intr map register"]
pub mod I2C_EXT0_INTR_MAP {
    #[doc = "reg_core0_i2c_ext0_intr_map"]
    pub mod I2C_EXT0_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "timer1 intr map register"]
pub mod TIMER_INT1_MAP {
    #[doc = "reg_core0_timer_int1_map"]
    pub mod TIMER_INT1_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "timer2 intr map register"]
pub mod TIMER_INT2_MAP {
    #[doc = "reg_core0_timer_int2_map"]
    pub mod TIMER_INT2_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "tg to intr map register"]
pub mod TG_T0_INT_MAP {
    #[doc = "reg_core0_tg_t0_int_map"]
    pub mod TG_T0_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "tg wdt intr map register"]
pub mod TG_WDT_INT_MAP {
    #[doc = "reg_core0_tg_wdt_int_map"]
    pub mod TG_WDT_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "tg1 to intr map register"]
pub mod TG1_T0_INT_MAP {
    #[doc = "reg_core0_tg1_t0_int_map"]
    pub mod TG1_T0_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "tg1 wdt intr map register"]
pub mod TG1_WDT_INT_MAP {
    #[doc = "reg_core0_tg1_wdt_int_map"]
    pub mod TG1_WDT_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "cache ia intr map register"]
pub mod CACHE_IA_INT_MAP {
    #[doc = "reg_core0_cache_ia_int_map"]
    pub mod CACHE_IA_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "systimer intr map register"]
pub mod SYSTIMER_TARGET0_INT_MAP {
    #[doc = "reg_core0_systimer_target0_int_map"]
    pub mod SYSTIMER_TARGET0_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "systimer target1 intr map register"]
pub mod SYSTIMER_TARGET1_INT_MAP {
    #[doc = "reg_core0_systimer_target1_int_map"]
    pub mod SYSTIMER_TARGET1_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "systimer target2 intr map register"]
pub mod SYSTIMER_TARGET2_INT_MAP {
    #[doc = "reg_core0_systimer_target2_int_map"]
    pub mod SYSTIMER_TARGET2_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "spi mem reject intr map register"]
pub mod SPI_MEM_REJECT_INTR_MAP {
    #[doc = "reg_core0_spi_mem_reject_intr_map"]
    pub mod SPI_MEM_REJECT_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "icache perload intr map register"]
pub mod ICACHE_PRELOAD_INT_MAP {
    #[doc = "reg_core0_icache_preload_int_map"]
    pub mod ICACHE_PRELOAD_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "icache sync intr map register"]
pub mod ICACHE_SYNC_INT_MAP {
    #[doc = "reg_core0_icache_sync_int_map"]
    pub mod ICACHE_SYNC_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "adc intr map register"]
pub mod APB_ADC_INT_MAP {
    #[doc = "reg_core0_apb_adc_int_map"]
    pub mod APB_ADC_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "dma ch0 intr map register"]
pub mod DMA_CH0_INT_MAP {
    #[doc = "reg_core0_dma_ch0_int_map"]
    pub mod DMA_CH0_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "dma ch1 intr map register"]
pub mod DMA_CH1_INT_MAP {
    #[doc = "reg_core0_dma_ch1_int_map"]
    pub mod DMA_CH1_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "dma ch2 intr map register"]
pub mod DMA_CH2_INT_MAP {
    #[doc = "reg_core0_dma_ch2_int_map"]
    pub mod DMA_CH2_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rsa intr map register"]
pub mod RSA_INT_MAP {
    #[doc = "reg_core0_rsa_int_map"]
    pub mod RSA_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "aes intr map register"]
pub mod AES_INT_MAP {
    #[doc = "reg_core0_aes_int_map"]
    pub mod AES_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "sha intr map register"]
pub mod SHA_INT_MAP {
    #[doc = "reg_core0_sha_int_map"]
    pub mod SHA_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "cpu from cpu 0 intr map register"]
pub mod CPU_INTR_FROM_CPU_0_MAP {
    #[doc = "reg_core0_cpu_intr_from_cpu_0_map"]
    pub mod CPU_INTR_FROM_CPU_0_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "cpu from cpu 0 intr map register"]
pub mod CPU_INTR_FROM_CPU_1_MAP {
    #[doc = "reg_core0_cpu_intr_from_cpu_1_map"]
    pub mod CPU_INTR_FROM_CPU_1_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "cpu from cpu 1 intr map register"]
pub mod CPU_INTR_FROM_CPU_2_MAP {
    #[doc = "reg_core0_cpu_intr_from_cpu_2_map"]
    pub mod CPU_INTR_FROM_CPU_2_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "cpu from cpu 3 intr map register"]
pub mod CPU_INTR_FROM_CPU_3_MAP {
    #[doc = "reg_core0_cpu_intr_from_cpu_3_map"]
    pub mod CPU_INTR_FROM_CPU_3_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "assist debug intr map register"]
pub mod ASSIST_DEBUG_INTR_MAP {
    #[doc = "reg_core0_assist_debug_intr_map"]
    pub mod ASSIST_DEBUG_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "dma pms violatile intr map register"]
pub mod DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP {
    #[doc = "reg_core0_dma_apbperi_pms_monitor_violate_intr_map"]
    pub mod DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "iram0 pms violatile intr map register"]
pub mod CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP {
    #[doc = "reg_core0_core_0_iram0_pms_monitor_violate_intr_map"]
    pub mod CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP {
    #[doc = "reg_core0_core_0_dram0_pms_monitor_violate_intr_map"]
    pub mod CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP {
    #[doc = "reg_core0_core_0_pif_pms_monitor_violate_intr_map"]
    pub mod CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP {
    #[doc = "reg_core0_core_0_pif_pms_monitor_violate_size_intr_map"]
    pub mod CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod BACKUP_PMS_VIOLATE_INTR_MAP {
    #[doc = "reg_core0_backup_pms_violate_intr_map"]
    pub mod BACKUP_PMS_VIOLATE_INTR_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CACHE_CORE0_ACS_INT_MAP {
    #[doc = "reg_core0_cache_core0_acs_int_map"]
    pub mod CACHE_CORE0_ACS_INT_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod INTR_STATUS_REG_0 {
    #[doc = "reg_core0_intr_status_0"]
    pub mod INTR_STATUS_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod INTR_STATUS_REG_1 {
    #[doc = "reg_core0_intr_status_1"]
    pub mod INTR_STATUS_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CLOCK_GATE {
    #[doc = "reg_core0_reg_clk_en"]
    pub mod REG_CLK_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_ENABLE {
    #[doc = "reg_core0_cpu_int_enable"]
    pub mod CPU_INT_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_TYPE {
    #[doc = "reg_core0_cpu_int_type"]
    pub mod CPU_INT_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_CLEAR {
    #[doc = "reg_core0_cpu_int_clear"]
    pub mod CPU_INT_CLEAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_EIP_STATUS {
    #[doc = "reg_core0_cpu_int_eip_status"]
    pub mod CPU_INT_EIP_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_0 {
    #[doc = "reg_core0_cpu_pri_0_map"]
    pub mod CPU_PRI_0_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_1 {
    #[doc = "reg_core0_cpu_pri_1_map"]
    pub mod CPU_PRI_1_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_2 {
    #[doc = "reg_core0_cpu_pri_2_map"]
    pub mod CPU_PRI_2_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_3 {
    #[doc = "reg_core0_cpu_pri_3_map"]
    pub mod CPU_PRI_3_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_4 {
    #[doc = "reg_core0_cpu_pri_4_map"]
    pub mod CPU_PRI_4_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_5 {
    #[doc = "reg_core0_cpu_pri_5_map"]
    pub mod CPU_PRI_5_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_6 {
    #[doc = "reg_core0_cpu_pri_6_map"]
    pub mod CPU_PRI_6_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_7 {
    #[doc = "reg_core0_cpu_pri_7_map"]
    pub mod CPU_PRI_7_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_8 {
    #[doc = "reg_core0_cpu_pri_8_map"]
    pub mod CPU_PRI_8_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_9 {
    #[doc = "reg_core0_cpu_pri_9_map"]
    pub mod CPU_PRI_9_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_10 {
    #[doc = "reg_core0_cpu_pri_10_map"]
    pub mod CPU_PRI_10_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_11 {
    #[doc = "reg_core0_cpu_pri_11_map"]
    pub mod CPU_PRI_11_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_12 {
    #[doc = "reg_core0_cpu_pri_12_map"]
    pub mod CPU_PRI_12_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_13 {
    #[doc = "reg_core0_cpu_pri_13_map"]
    pub mod CPU_PRI_13_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_14 {
    #[doc = "reg_core0_cpu_pri_14_map"]
    pub mod CPU_PRI_14_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_15 {
    #[doc = "reg_core0_cpu_pri_15_map"]
    pub mod CPU_PRI_15_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_16 {
    #[doc = "reg_core0_cpu_pri_16_map"]
    pub mod CPU_PRI_16_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_17 {
    #[doc = "reg_core0_cpu_pri_17_map"]
    pub mod CPU_PRI_17_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_18 {
    #[doc = "reg_core0_cpu_pri_18_map"]
    pub mod CPU_PRI_18_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_19 {
    #[doc = "reg_core0_cpu_pri_19_map"]
    pub mod CPU_PRI_19_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_20 {
    #[doc = "reg_core0_cpu_pri_20_map"]
    pub mod CPU_PRI_20_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_21 {
    #[doc = "reg_core0_cpu_pri_21_map"]
    pub mod CPU_PRI_21_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_22 {
    #[doc = "reg_core0_cpu_pri_22_map"]
    pub mod CPU_PRI_22_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_23 {
    #[doc = "reg_core0_cpu_pri_23_map"]
    pub mod CPU_PRI_23_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_24 {
    #[doc = "reg_core0_cpu_pri_24_map"]
    pub mod CPU_PRI_24_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_25 {
    #[doc = "reg_core0_cpu_pri_25_map"]
    pub mod CPU_PRI_25_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_26 {
    #[doc = "reg_core0_cpu_pri_26_map"]
    pub mod CPU_PRI_26_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_27 {
    #[doc = "reg_core0_cpu_pri_27_map"]
    pub mod CPU_PRI_27_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_28 {
    #[doc = "reg_core0_cpu_pri_28_map"]
    pub mod CPU_PRI_28_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_29 {
    #[doc = "reg_core0_cpu_pri_29_map"]
    pub mod CPU_PRI_29_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_30 {
    #[doc = "reg_core0_cpu_pri_30_map"]
    pub mod CPU_PRI_30_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_PRI_31 {
    #[doc = "reg_core0_cpu_pri_31_map"]
    pub mod CPU_PRI_31_MAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod CPU_INT_THRESH {
    #[doc = "reg_core0_cpu_int_thresh"]
    pub mod CPU_INT_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mac intr map register"]
pub mod INTERRUPT_REG_DATE {
    #[doc = "reg_core0_interrupt_reg_date"]
    pub mod INTERRUPT_REG_DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
