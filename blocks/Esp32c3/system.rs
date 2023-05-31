#[doc = "System Configuration Registers"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "cpu_peripheral clock gating register"]
    pub CPU_PERI_CLK_EN: crate::RWRegister<u32>,
    #[doc = "cpu_peripheral reset register"]
    pub CPU_PERI_RST_EN: crate::RWRegister<u32>,
    #[doc = "cpu clock config register"]
    pub CPU_PER_CONF: crate::RWRegister<u32>,
    #[doc = "memory power down mask register"]
    pub MEM_PD_MASK: crate::RWRegister<u32>,
    #[doc = "peripheral clock gating register"]
    pub PERIP_CLK_EN0: crate::RWRegister<u32>,
    #[doc = "peripheral clock gating register"]
    pub PERIP_CLK_EN1: crate::RWRegister<u32>,
    #[doc = "reserved"]
    pub PERIP_RST_EN0: crate::RWRegister<u32>,
    #[doc = "peripheral reset register"]
    pub PERIP_RST_EN1: crate::RWRegister<u32>,
    #[doc = "clock config register"]
    pub BT_LPCK_DIV_INT: crate::RWRegister<u32>,
    #[doc = "clock config register"]
    pub BT_LPCK_DIV_FRAC: crate::RWRegister<u32>,
    #[doc = "interrupt generate register"]
    pub CPU_INTR_FROM_CPU_0: crate::RWRegister<u32>,
    #[doc = "interrupt generate register"]
    pub CPU_INTR_FROM_CPU_1: crate::RWRegister<u32>,
    #[doc = "interrupt generate register"]
    pub CPU_INTR_FROM_CPU_2: crate::RWRegister<u32>,
    #[doc = "interrupt generate register"]
    pub CPU_INTR_FROM_CPU_3: crate::RWRegister<u32>,
    #[doc = "rsa memory power control register"]
    pub RSA_PD_CTRL: crate::RWRegister<u32>,
    #[doc = "edma clcok and reset register"]
    pub EDMA_CTRL: crate::RWRegister<u32>,
    #[doc = "cache control register"]
    pub CACHE_CONTROL: crate::RWRegister<u32>,
    #[doc = "SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_REG"]
    pub EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL: crate::RWRegister<u32>,
    #[doc = "fast memory config register"]
    pub RTC_FASTMEM_CONFIG: crate::RWRegister<u32>,
    #[doc = "reserved"]
    pub RTC_FASTMEM_CRC: crate::RWRegister<u32>,
    #[doc = "eco register"]
    pub REDUNDANT_ECO_CTRL: crate::RWRegister<u32>,
    #[doc = "clock gating register"]
    pub CLOCK_GATE: crate::RWRegister<u32>,
    #[doc = "system clock config register"]
    pub SYSCLK_CONF: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub MEM_PVT: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub COMB_PVT_LVT_CONF: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub COMB_PVT_NVT_CONF: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub COMB_PVT_HVT_CONF: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub COMB_PVT_ERR_LVT_SITE0: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub COMB_PVT_ERR_NVT_SITE0: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub COMB_PVT_ERR_HVT_SITE0: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub COMB_PVT_ERR_LVT_SITE1: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub COMB_PVT_ERR_NVT_SITE1: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub COMB_PVT_ERR_HVT_SITE1: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub COMB_PVT_ERR_LVT_SITE2: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub COMB_PVT_ERR_NVT_SITE2: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub COMB_PVT_ERR_HVT_SITE2: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub COMB_PVT_ERR_LVT_SITE3: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub COMB_PVT_ERR_NVT_SITE3: crate::RWRegister<u32>,
    #[doc = "mem pvt register"]
    pub COMB_PVT_ERR_HVT_SITE3: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0f60],
    #[doc = "Version register"]
    pub SYSTEM_REG_DATE: crate::RWRegister<u32>,
}
#[doc = "cpu_peripheral clock gating register"]
pub mod CPU_PERI_CLK_EN {
    #[doc = "reg_clk_en_assist_debug"]
    pub mod CLK_EN_ASSIST_DEBUG {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_en_dedicated_gpio"]
    pub mod CLK_EN_DEDICATED_GPIO {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "cpu_peripheral reset register"]
pub mod CPU_PERI_RST_EN {
    #[doc = "reg_rst_en_assist_debug"]
    pub mod RST_EN_ASSIST_DEBUG {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rst_en_dedicated_gpio"]
    pub mod RST_EN_DEDICATED_GPIO {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "cpu clock config register"]
pub mod CPU_PER_CONF {
    #[doc = "reg_cpuperiod_sel"]
    pub mod CPUPERIOD_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_pll_freq_sel"]
    pub mod PLL_FREQ_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_cpu_wait_mode_force_on"]
    pub mod CPU_WAIT_MODE_FORCE_ON {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_cpu_waiti_delay_num"]
    pub mod CPU_WAITI_DELAY_NUM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "memory power down mask register"]
pub mod MEM_PD_MASK {
    #[doc = "reg_lslp_mem_pd_mask"]
    pub mod LSLP_MEM_PD_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "peripheral clock gating register"]
pub mod PERIP_CLK_EN0 {
    #[doc = "reg_timers_clk_en"]
    pub mod TIMERS_CLK_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_spi01_clk_en"]
    pub mod SPI01_CLK_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_uart_clk_en"]
    pub mod UART_CLK_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_wdg_clk_en"]
    pub mod WDG_CLK_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_i2s0_clk_en"]
    pub mod I2S0_CLK_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_uart1_clk_en"]
    pub mod UART1_CLK_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_spi2_clk_en"]
    pub mod SPI2_CLK_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ext0_clk_en"]
    pub mod I2C_EXT0_CLK_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_uhci0_clk_en"]
    pub mod UHCI0_CLK_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rmt_clk_en"]
    pub mod RMT_CLK_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_pcnt_clk_en"]
    pub mod PCNT_CLK_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ledc_clk_en"]
    pub mod LEDC_CLK_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_uhci1_clk_en"]
    pub mod UHCI1_CLK_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_timergroup_clk_en"]
    pub mod TIMERGROUP_CLK_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_efuse_clk_en"]
    pub mod EFUSE_CLK_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_timergroup1_clk_en"]
    pub mod TIMERGROUP1_CLK_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_spi3_clk_en"]
    pub mod SPI3_CLK_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_pwm0_clk_en"]
    pub mod PWM0_CLK_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ext1_clk_en"]
    pub mod EXT1_CLK_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_can_clk_en"]
    pub mod TWAI_CLK_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_pwm1_clk_en"]
    pub mod PWM1_CLK_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_i2s1_clk_en"]
    pub mod I2S1_CLK_EN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_spi2_dma_clk_en"]
    pub mod SPI2_DMA_CLK_EN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_usb_device_clk_en"]
    pub mod USB_DEVICE_CLK_EN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_uart_mem_clk_en"]
    pub mod UART_MEM_CLK_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_pwm2_clk_en"]
    pub mod PWM2_CLK_EN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_pwm3_clk_en"]
    pub mod PWM3_CLK_EN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_spi3_dma_clk_en"]
    pub mod SPI3_DMA_CLK_EN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_saradc_clk_en"]
    pub mod APB_SARADC_CLK_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_systimer_clk_en"]
    pub mod SYSTIMER_CLK_EN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_adc2_arb_clk_en"]
    pub mod ADC2_ARB_CLK_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_spi4_clk_en"]
    pub mod SPI4_CLK_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "peripheral clock gating register"]
pub mod PERIP_CLK_EN1 {
    #[doc = "reg_crypto_aes_clk_en"]
    pub mod CRYPTO_AES_CLK_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_crypto_sha_clk_en"]
    pub mod CRYPTO_SHA_CLK_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_crypto_rsa_clk_en"]
    pub mod CRYPTO_RSA_CLK_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_crypto_ds_clk_en"]
    pub mod CRYPTO_DS_CLK_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_crypto_hmac_clk_en"]
    pub mod CRYPTO_HMAC_CLK_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_dma_clk_en"]
    pub mod DMA_CLK_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sdio_host_clk_en"]
    pub mod SDIO_HOST_CLK_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lcd_cam_clk_en"]
    pub mod LCD_CAM_CLK_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_uart2_clk_en"]
    pub mod UART2_CLK_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_tsens_clk_en"]
    pub mod TSENS_CLK_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "reserved"]
pub mod PERIP_RST_EN0 {
    #[doc = "reg_timers_rst"]
    pub mod TIMERS_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_spi01_rst"]
    pub mod SPI01_RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_uart_rst"]
    pub mod UART_RST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_wdg_rst"]
    pub mod WDG_RST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_i2s0_rst"]
    pub mod I2S0_RST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_uart1_rst"]
    pub mod UART1_RST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_spi2_rst"]
    pub mod SPI2_RST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ext0_rst"]
    pub mod I2C_EXT0_RST {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_uhci0_rst"]
    pub mod UHCI0_RST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rmt_rst"]
    pub mod RMT_RST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_pcnt_rst"]
    pub mod PCNT_RST {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ledc_rst"]
    pub mod LEDC_RST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_uhci1_rst"]
    pub mod UHCI1_RST {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_timergroup_rst"]
    pub mod TIMERGROUP_RST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_efuse_rst"]
    pub mod EFUSE_RST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_timergroup1_rst"]
    pub mod TIMERGROUP1_RST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_spi3_rst"]
    pub mod SPI3_RST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_pwm0_rst"]
    pub mod PWM0_RST {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_ext1_rst"]
    pub mod EXT1_RST {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_can_rst"]
    pub mod TWAI_RST {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_pwm1_rst"]
    pub mod PWM1_RST {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_i2s1_rst"]
    pub mod I2S1_RST {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_spi2_dma_rst"]
    pub mod SPI2_DMA_RST {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_usb_device_rst"]
    pub mod USB_DEVICE_RST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_uart_mem_rst"]
    pub mod UART_MEM_RST {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_pwm2_rst"]
    pub mod PWM2_RST {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_pwm3_rst"]
    pub mod PWM3_RST {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_spi3_dma_rst"]
    pub mod SPI3_DMA_RST {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_apb_saradc_rst"]
    pub mod APB_SARADC_RST {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_systimer_rst"]
    pub mod SYSTIMER_RST {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_adc2_arb_rst"]
    pub mod ADC2_ARB_RST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_spi4_rst"]
    pub mod SPI4_RST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "peripheral reset register"]
pub mod PERIP_RST_EN1 {
    #[doc = "reg_crypto_aes_rst"]
    pub mod CRYPTO_AES_RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_crypto_sha_rst"]
    pub mod CRYPTO_SHA_RST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_crypto_rsa_rst"]
    pub mod CRYPTO_RSA_RST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_crypto_ds_rst"]
    pub mod CRYPTO_DS_RST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_crypto_hmac_rst"]
    pub mod CRYPTO_HMAC_RST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_dma_rst"]
    pub mod DMA_RST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_sdio_host_rst"]
    pub mod SDIO_HOST_RST {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lcd_cam_rst"]
    pub mod LCD_CAM_RST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_uart2_rst"]
    pub mod UART2_RST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_tsens_rst"]
    pub mod TSENS_RST {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "clock config register"]
pub mod BT_LPCK_DIV_INT {
    #[doc = "reg_bt_lpck_div_num"]
    pub mod BT_LPCK_DIV_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "clock config register"]
pub mod BT_LPCK_DIV_FRAC {
    #[doc = "reg_bt_lpck_div_b"]
    pub mod BT_LPCK_DIV_B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_bt_lpck_div_a"]
    pub mod BT_LPCK_DIV_A {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lpclk_sel_rtc_slow"]
    pub mod LPCLK_SEL_RTC_SLOW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lpclk_sel_8m"]
    pub mod LPCLK_SEL_8M {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lpclk_sel_xtal"]
    pub mod LPCLK_SEL_XTAL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lpclk_sel_xtal32k"]
    pub mod LPCLK_SEL_XTAL32K {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_lpclk_rtc_en"]
    pub mod LPCLK_RTC_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "interrupt generate register"]
pub mod CPU_INTR_FROM_CPU_0 {
    #[doc = "reg_cpu_intr_from_cpu_0"]
    pub mod CPU_INTR_FROM_CPU_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "interrupt generate register"]
pub mod CPU_INTR_FROM_CPU_1 {
    #[doc = "reg_cpu_intr_from_cpu_1"]
    pub mod CPU_INTR_FROM_CPU_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "interrupt generate register"]
pub mod CPU_INTR_FROM_CPU_2 {
    #[doc = "reg_cpu_intr_from_cpu_2"]
    pub mod CPU_INTR_FROM_CPU_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "interrupt generate register"]
pub mod CPU_INTR_FROM_CPU_3 {
    #[doc = "reg_cpu_intr_from_cpu_3"]
    pub mod CPU_INTR_FROM_CPU_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "rsa memory power control register"]
pub mod RSA_PD_CTRL {
    #[doc = "reg_rsa_mem_pd"]
    pub mod RSA_MEM_PD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rsa_mem_force_pu"]
    pub mod RSA_MEM_FORCE_PU {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rsa_mem_force_pd"]
    pub mod RSA_MEM_FORCE_PD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "edma clcok and reset register"]
pub mod EDMA_CTRL {
    #[doc = "reg_edma_clk_on"]
    pub mod EDMA_CLK_ON {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_edma_reset"]
    pub mod EDMA_RESET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "cache control register"]
pub mod CACHE_CONTROL {
    #[doc = "reg_icache_clk_on"]
    pub mod ICACHE_CLK_ON {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_icache_reset"]
    pub mod ICACHE_RESET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_dcache_clk_on"]
    pub mod DCACHE_CLK_ON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_dcache_reset"]
    pub mod DCACHE_RESET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_REG"]
pub mod EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL {
    #[doc = "reg_enable_spi_manual_encrypt"]
    pub mod ENABLE_SPI_MANUAL_ENCRYPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_enable_download_db_encrypt"]
    pub mod ENABLE_DOWNLOAD_DB_ENCRYPT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_enable_download_g0cb_decrypt"]
    pub mod ENABLE_DOWNLOAD_G0CB_DECRYPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_enable_download_manual_encrypt"]
    pub mod ENABLE_DOWNLOAD_MANUAL_ENCRYPT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "fast memory config register"]
pub mod RTC_FASTMEM_CONFIG {
    #[doc = "reg_rtc_mem_crc_start"]
    pub mod RTC_MEM_CRC_START {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rtc_mem_crc_addr"]
    pub mod RTC_MEM_CRC_ADDR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rtc_mem_crc_len"]
    pub mod RTC_MEM_CRC_LEN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_rtc_mem_crc_finish"]
    pub mod RTC_MEM_CRC_FINISH {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "reserved"]
pub mod RTC_FASTMEM_CRC {
    #[doc = "reg_rtc_mem_crc_res"]
    pub mod RTC_MEM_CRC_RES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "eco register"]
pub mod REDUNDANT_ECO_CTRL {
    #[doc = "reg_redundant_eco_drive"]
    pub mod REDUNDANT_ECO_DRIVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_redundant_eco_result"]
    pub mod REDUNDANT_ECO_RESULT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "clock gating register"]
pub mod CLOCK_GATE {
    #[doc = "reg_clk_en"]
    pub mod CLK_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "system clock config register"]
pub mod SYSCLK_CONF {
    #[doc = "reg_pre_div_cnt"]
    pub mod PRE_DIV_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_soc_clk_sel"]
    pub mod SOC_CLK_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_xtal_freq"]
    pub mod CLK_XTAL_FREQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_div_en"]
    pub mod CLK_DIV_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod MEM_PVT {
    #[doc = "reg_mem_path_len"]
    pub mod MEM_PATH_LEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_err_cnt_clr"]
    pub mod MEM_ERR_CNT_CLR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_pvt_monitor_en"]
    pub mod MONITOR_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_timing_err_cnt"]
    pub mod MEM_TIMING_ERR_CNT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_mem_vt_sel"]
    pub mod MEM_VT_SEL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod COMB_PVT_LVT_CONF {
    #[doc = "reg_comb_path_len_lvt"]
    pub mod COMB_PATH_LEN_LVT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_comb_err_cnt_clr_lvt"]
    pub mod COMB_ERR_CNT_CLR_LVT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_comb_pvt_monitor_en_lvt"]
    pub mod COMB_PVT_MONITOR_EN_LVT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod COMB_PVT_NVT_CONF {
    #[doc = "reg_comb_path_len_nvt"]
    pub mod COMB_PATH_LEN_NVT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_comb_err_cnt_clr_nvt"]
    pub mod COMB_ERR_CNT_CLR_NVT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_comb_pvt_monitor_en_nvt"]
    pub mod COMB_PVT_MONITOR_EN_NVT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod COMB_PVT_HVT_CONF {
    #[doc = "reg_comb_path_len_hvt"]
    pub mod COMB_PATH_LEN_HVT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_comb_err_cnt_clr_hvt"]
    pub mod COMB_ERR_CNT_CLR_HVT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_comb_pvt_monitor_en_hvt"]
    pub mod COMB_PVT_MONITOR_EN_HVT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod COMB_PVT_ERR_LVT_SITE0 {
    #[doc = "reg_comb_timing_err_cnt_lvt_site0"]
    pub mod COMB_TIMING_ERR_CNT_LVT_SITE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod COMB_PVT_ERR_NVT_SITE0 {
    #[doc = "reg_comb_timing_err_cnt_nvt_site0"]
    pub mod COMB_TIMING_ERR_CNT_NVT_SITE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod COMB_PVT_ERR_HVT_SITE0 {
    #[doc = "reg_comb_timing_err_cnt_hvt_site0"]
    pub mod COMB_TIMING_ERR_CNT_HVT_SITE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod COMB_PVT_ERR_LVT_SITE1 {
    #[doc = "reg_comb_timing_err_cnt_lvt_site1"]
    pub mod COMB_TIMING_ERR_CNT_LVT_SITE1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod COMB_PVT_ERR_NVT_SITE1 {
    #[doc = "reg_comb_timing_err_cnt_nvt_site1"]
    pub mod COMB_TIMING_ERR_CNT_NVT_SITE1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod COMB_PVT_ERR_HVT_SITE1 {
    #[doc = "reg_comb_timing_err_cnt_hvt_site1"]
    pub mod COMB_TIMING_ERR_CNT_HVT_SITE1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod COMB_PVT_ERR_LVT_SITE2 {
    #[doc = "reg_comb_timing_err_cnt_lvt_site2"]
    pub mod COMB_TIMING_ERR_CNT_LVT_SITE2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod COMB_PVT_ERR_NVT_SITE2 {
    #[doc = "reg_comb_timing_err_cnt_nvt_site2"]
    pub mod COMB_TIMING_ERR_CNT_NVT_SITE2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod COMB_PVT_ERR_HVT_SITE2 {
    #[doc = "reg_comb_timing_err_cnt_hvt_site2"]
    pub mod COMB_TIMING_ERR_CNT_HVT_SITE2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod COMB_PVT_ERR_LVT_SITE3 {
    #[doc = "reg_comb_timing_err_cnt_lvt_site3"]
    pub mod COMB_TIMING_ERR_CNT_LVT_SITE3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod COMB_PVT_ERR_NVT_SITE3 {
    #[doc = "reg_comb_timing_err_cnt_nvt_site3"]
    pub mod COMB_TIMING_ERR_CNT_NVT_SITE3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "mem pvt register"]
pub mod COMB_PVT_ERR_HVT_SITE3 {
    #[doc = "reg_comb_timing_err_cnt_hvt_site3"]
    pub mod COMB_TIMING_ERR_CNT_HVT_SITE3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Version register"]
pub mod SYSTEM_REG_DATE {
    #[doc = "reg_system_reg_date"]
    pub mod SYSTEM_REG_DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
