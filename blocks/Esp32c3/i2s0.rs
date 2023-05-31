#[doc = "I2S (Inter-IC Sound) Controller 0"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    #[doc = "I2S interrupt raw register, valid in level."]
    pub INT_RAW: crate::RWRegister<u32>,
    #[doc = "I2S interrupt status register."]
    pub INT_ST: crate::RWRegister<u32>,
    #[doc = "I2S interrupt enable register."]
    pub INT_ENA: crate::RWRegister<u32>,
    #[doc = "I2S interrupt clear register."]
    pub INT_CLR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "I2S RX configure register"]
    pub RX_CONF: crate::RWRegister<u32>,
    #[doc = "I2S TX configure register"]
    pub TX_CONF: crate::RWRegister<u32>,
    #[doc = "I2S RX configure register 1"]
    pub RX_CONF1: crate::RWRegister<u32>,
    #[doc = "I2S TX configure register 1"]
    pub TX_CONF1: crate::RWRegister<u32>,
    #[doc = "I2S RX clock configure register"]
    pub RX_CLKM_CONF: crate::RWRegister<u32>,
    #[doc = "I2S TX clock configure register"]
    pub TX_CLKM_CONF: crate::RWRegister<u32>,
    #[doc = "I2S RX module clock divider configure register"]
    pub RX_CLKM_DIV_CONF: crate::RWRegister<u32>,
    #[doc = "I2S TX module clock divider configure register"]
    pub TX_CLKM_DIV_CONF: crate::RWRegister<u32>,
    #[doc = "I2S TX PCM2PDM configuration register"]
    pub TX_PCM2PDM_CONF: crate::RWRegister<u32>,
    #[doc = "I2S TX PCM2PDM configuration register"]
    pub TX_PCM2PDM_CONF1: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "I2S TX TDM mode control register"]
    pub RX_TDM_CTRL: crate::RWRegister<u32>,
    #[doc = "I2S TX TDM mode control register"]
    pub TX_TDM_CTRL: crate::RWRegister<u32>,
    #[doc = "I2S RX timing control register"]
    pub RX_TIMING: crate::RWRegister<u32>,
    #[doc = "I2S TX timing control register"]
    pub TX_TIMING: crate::RWRegister<u32>,
    #[doc = "I2S HUNG configure register."]
    pub LC_HUNG_CONF: crate::RWRegister<u32>,
    #[doc = "I2S RX data number control register."]
    pub RXEOF_NUM: crate::RWRegister<u32>,
    #[doc = "I2S signal data register"]
    pub CONF_SIGLE_DATA: crate::RWRegister<u32>,
    #[doc = "I2S TX status register"]
    pub STATE: crate::RWRegister<u32>,
    _reserved3: [u8; 0x10],
    #[doc = "Version control register"]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "I2S interrupt raw register, valid in level."]
pub mod INT_RAW {
    #[doc = "The raw interrupt status bit for the i2s_rx_done_int interrupt"]
    pub mod RX_DONE_INT_RAW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the i2s_tx_done_int interrupt"]
    pub mod TX_DONE_INT_RAW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the i2s_rx_hung_int interrupt"]
    pub mod RX_HUNG_INT_RAW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the i2s_tx_hung_int interrupt"]
    pub mod TX_HUNG_INT_RAW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S interrupt status register."]
pub mod INT_ST {
    #[doc = "The masked interrupt status bit for the i2s_rx_done_int interrupt"]
    pub mod RX_DONE_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The masked interrupt status bit for the i2s_tx_done_int interrupt"]
    pub mod TX_DONE_INT_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The masked interrupt status bit for the i2s_rx_hung_int interrupt"]
    pub mod RX_HUNG_INT_ST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The masked interrupt status bit for the i2s_tx_hung_int interrupt"]
    pub mod TX_HUNG_INT_ST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S interrupt enable register."]
pub mod INT_ENA {
    #[doc = "The interrupt enable bit for the i2s_rx_done_int interrupt"]
    pub mod RX_DONE_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the i2s_tx_done_int interrupt"]
    pub mod TX_DONE_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the i2s_rx_hung_int interrupt"]
    pub mod RX_HUNG_INT_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the i2s_tx_hung_int interrupt"]
    pub mod TX_HUNG_INT_ENA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S interrupt clear register."]
pub mod INT_CLR {
    #[doc = "Set this bit to clear the i2s_rx_done_int interrupt"]
    pub mod RX_DONE_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the i2s_tx_done_int interrupt"]
    pub mod TX_DONE_INT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the i2s_rx_hung_int interrupt"]
    pub mod RX_HUNG_INT_CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the i2s_tx_hung_int interrupt"]
    pub mod TX_HUNG_INT_CLR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S RX configure register"]
pub mod RX_CONF {
    #[doc = "Set this bit to reset receiver"]
    pub mod RX_RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to reset Rx AFIFO"]
    pub mod RX_FIFO_RESET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to start receiving data"]
    pub mod RX_START {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable slave receiver mode"]
    pub mod RX_SLAVE_MOD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable receiver in mono mode"]
    pub mod RX_MONO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S Rx byte endian, 1: low addr value to high addr. 0: low addr with low addr value."]
    pub mod RX_BIG_ENDIAN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set 1 to update I2S RX registers from APB clock domain to I2S RX clock domain. This bit will be cleared by hardware after update register done."]
    pub mod RX_UPDATE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: The first channel data value is valid in I2S RX mono mode. 0: The second channel data value is valid in I2S RX mono mode."]
    pub mod RX_MONO_FST_VLD {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S RX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &"]
    pub mod RX_PCM_CONF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to bypass Compress/Decompress module for received data."]
    pub mod RX_PCM_BYPASS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full."]
    pub mod RX_STOP_MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: I2S RX left alignment mode. 0: I2S RX right alignment mode."]
    pub mod RX_LEFT_ALIGN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: store 24 channel bits to 32 bits. 0:store 24 channel bits to 24 bits."]
    pub mod RX_24_FILL_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: WS should be 0 when receiving left channel data, and WS is 1in right channel. 1: WS should be 1 when receiving left channel data, and WS is 0in right channel."]
    pub mod RX_WS_IDLE_POL {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S Rx bit endian. 1:small endian, the LSB is received first. 0:big endian, the MSB is received first."]
    pub mod RX_BIT_ORDER {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable I2S TDM Rx mode . 0: Disable."]
    pub mod RX_TDM_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable I2S PDM Rx mode . 0: Disable."]
    pub mod RX_PDM_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S TX configure register"]
pub mod TX_CONF {
    #[doc = "Set this bit to reset transmitter"]
    pub mod TX_RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to reset Tx AFIFO"]
    pub mod TX_FIFO_RESET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to start transmitting data"]
    pub mod TX_START {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable slave transmitter mode"]
    pub mod TX_SLAVE_MOD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable transmitter in mono mode"]
    pub mod TX_MONO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: The value of Left channel data is equal to the value of right channel data in I2S TX mono mode or TDM channel select mode. 0: The invalid channel data is reg_i2s_single_data in I2S TX mono mode or TDM channel select mode."]
    pub mod TX_CHAN_EQUAL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S Tx byte endian, 1: low addr value to high addr. 0: low addr with low addr value."]
    pub mod TX_BIG_ENDIAN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set 1 to update I2S TX registers from APB clock domain to I2S TX clock domain. This bit will be cleared by hardware after update register done."]
    pub mod TX_UPDATE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: The first channel data value is valid in I2S TX mono mode. 0: The second channel data value is valid in I2S TX mono mode."]
    pub mod TX_MONO_FST_VLD {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S TX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &"]
    pub mod TX_PCM_CONF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to bypass Compress/Decompress module for transmitted data."]
    pub mod TX_PCM_BYPASS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to stop disable output BCK signal and WS signal when tx FIFO is emtpy"]
    pub mod TX_STOP_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: I2S TX left alignment mode. 0: I2S TX right alignment mode."]
    pub mod TX_LEFT_ALIGN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Sent 32 bits in 24 channel bits mode. 0: Sent 24 bits in 24 channel bits mode"]
    pub mod TX_24_FILL_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: WS should be 0 when sending left channel data, and WS is 1in right channel. 1: WS should be 1 when sending left channel data, and WS is 0in right channel."]
    pub mod TX_WS_IDLE_POL {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S Tx bit endian. 1:small endian, the LSB is sent first. 0:big endian, the MSB is sent first."]
    pub mod TX_BIT_ORDER {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable I2S TDM Tx mode . 0: Disable."]
    pub mod TX_TDM_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable I2S PDM Tx mode . 0: Disable."]
    pub mod TX_PDM_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S transmitter channel mode configuration bits."]
    pub mod TX_CHAN_MOD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable signal loop back mode with transmitter module and receiver module sharing the same WS and BCK signals."]
    pub mod SIG_LOOPBACK {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S RX configure register 1"]
pub mod RX_CONF1 {
    #[doc = "The width of rx_ws_out in TDM mode is (I2S_RX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
    pub mod RX_TDM_WS_WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit clock configuration bits in receiver mode."]
    pub mod RX_BCK_DIV_NUM {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
    pub mod RX_BITS_MOD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S Rx half sample bits -1."]
    pub mod RX_HALF_SAMPLE_BITS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The Rx bit number for each channel minus 1in TDM mode."]
    pub mod RX_TDM_CHAN_BITS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable receiver in Phillips standard mode"]
    pub mod RX_MSB_SHIFT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S TX configure register 1"]
pub mod TX_CONF1 {
    #[doc = "The width of tx_ws_out in TDM mode is (I2S_TX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
    pub mod TX_TDM_WS_WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit clock configuration bits in transmitter mode."]
    pub mod TX_BCK_DIV_NUM {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set the bits to configure the valid data bit length of I2S transmitter channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
    pub mod TX_BITS_MOD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S Tx half sample bits -1."]
    pub mod TX_HALF_SAMPLE_BITS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The Tx bit number for each channel minus 1in TDM mode."]
    pub mod TX_TDM_CHAN_BITS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable transmitter in Phillips standard mode"]
    pub mod TX_MSB_SHIFT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: BCK is not delayed to generate pos/neg edge in master mode. 0: BCK is delayed to generate pos/neg edge in master mode."]
    pub mod TX_BCK_NO_DLY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S RX clock configure register"]
pub mod RX_CLKM_CONF {
    #[doc = "Integral I2S clock divider value"]
    pub mod RX_CLKM_DIV_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S Rx module clock enable signal."]
    pub mod RX_CLK_ACTIVE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select I2S Rx module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
    pub mod RX_CLK_SEL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: UseI2S Tx module clock as I2S_MCLK_OUT. 1: UseI2S Rx module clock as I2S_MCLK_OUT."]
    pub mod MCLK_SEL {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S TX clock configure register"]
pub mod TX_CLKM_CONF {
    #[doc = "Integral I2S TX clock divider value. f_I2S_CLK = f_I2S_CLK_S/(N+b/a). There will be (a-b) * n-div and b * (n+1)-div. So the average combination will be: for b <= a/2, z * \\[x * n-div + (n+1)-div\\] + y * n-div. For b > a/2, z * \\[n-div + x * (n+1)-div\\] + y * (n+1)-div."]
    pub mod TX_CLKM_DIV_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S Tx module clock enable signal."]
    pub mod TX_CLK_ACTIVE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select I2S Tx module source clock. 0: XTAL clock. 1: APLL. 2: CLK160. 3: I2S_MCLK_in."]
    pub mod TX_CLK_SEL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable clk gate"]
    pub mod CLK_EN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S RX module clock divider configure register"]
pub mod RX_CLKM_DIV_CONF {
    #[doc = "For b <= a/2, the value of I2S_RX_CLKM_DIV_Z is b. For b > a/2, the value of I2S_RX_CLKM_DIV_Z is (a-b)."]
    pub mod RX_CLKM_DIV_Z {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For b <= a/2, the value of I2S_RX_CLKM_DIV_Y is (a%b) . For b > a/2, the value of I2S_RX_CLKM_DIV_Y is (a%(a-b))."]
    pub mod RX_CLKM_DIV_Y {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For b <= a/2, the value of I2S_RX_CLKM_DIV_X is (a/b) - 1. For b > a/2, the value of I2S_RX_CLKM_DIV_X is (a/(a-b)) - 1."]
    pub mod RX_CLKM_DIV_X {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For b <= a/2, the value of I2S_RX_CLKM_DIV_YN1 is 0 . For b > a/2, the value of I2S_RX_CLKM_DIV_YN1 is 1."]
    pub mod RX_CLKM_DIV_YN1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S TX module clock divider configure register"]
pub mod TX_CLKM_DIV_CONF {
    #[doc = "For b <= a/2, the value of I2S_TX_CLKM_DIV_Z is b. For b > a/2, the value of I2S_TX_CLKM_DIV_Z is (a-b)."]
    pub mod TX_CLKM_DIV_Z {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For b <= a/2, the value of I2S_TX_CLKM_DIV_Y is (a%b) . For b > a/2, the value of I2S_TX_CLKM_DIV_Y is (a%(a-b))."]
    pub mod TX_CLKM_DIV_Y {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For b <= a/2, the value of I2S_TX_CLKM_DIV_X is (a/b) - 1. For b > a/2, the value of I2S_TX_CLKM_DIV_X is (a/(a-b)) - 1."]
    pub mod TX_CLKM_DIV_X {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "For b <= a/2, the value of I2S_TX_CLKM_DIV_YN1 is 0 . For b > a/2, the value of I2S_TX_CLKM_DIV_YN1 is 1."]
    pub mod TX_CLKM_DIV_YN1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S TX PCM2PDM configuration register"]
pub mod TX_PCM2PDM_CONF {
    #[doc = "I2S TX PDM bypass hp filter or not. The option has been removed."]
    pub mod TX_PDM_HP_BYPASS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S TX PDM OSR2 value"]
    pub mod TX_PDM_SINC_OSR2 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S TX PDM prescale for sigmadelta"]
    pub mod TX_PDM_PRESCALE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    pub mod TX_PDM_HP_IN_SHIFT {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    pub mod TX_PDM_LP_IN_SHIFT {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    pub mod TX_PDM_SINC_IN_SHIFT {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    pub mod TX_PDM_SIGMADELTA_IN_SHIFT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S TX PDM sigmadelta dither2 value"]
    pub mod TX_PDM_SIGMADELTA_DITHER2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S TX PDM sigmadelta dither value"]
    pub mod TX_PDM_SIGMADELTA_DITHER {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S TX PDM dac mode enable"]
    pub mod TX_PDM_DAC_2OUT_EN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S TX PDM dac 2channel enable"]
    pub mod TX_PDM_DAC_MODE_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S TX PDM Converter enable"]
    pub mod PCM2PDM_CONV_EN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S TX PCM2PDM configuration register"]
pub mod TX_PCM2PDM_CONF1 {
    #[doc = "I2S TX PDM Fp"]
    pub mod TX_PDM_FP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S TX PDM Fs"]
    pub mod TX_PDM_FS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The fourth parameter of PDM TX IIR_HP filter stage 2 is (504 + I2S_TX_IIR_HP_MULT12_5\\[2:0\\])"]
    pub mod TX_IIR_HP_MULT12_5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The fourth parameter of PDM TX IIR_HP filter stage 1 is (504 + I2S_TX_IIR_HP_MULT12_0\\[2:0\\])"]
    pub mod TX_IIR_HP_MULT12_0 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S TX TDM mode control register"]
pub mod RX_TDM_CTRL {
    #[doc = "1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_PDM_CHAN0_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_PDM_CHAN1_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data input of I2S RX TDM or PDM channel 2. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_PDM_CHAN2_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data input of I2S RX TDM or PDM channel 3. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_PDM_CHAN3_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data input of I2S RX TDM or PDM channel 4. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_PDM_CHAN4_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data input of I2S RX TDM or PDM channel 5. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_PDM_CHAN5_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data input of I2S RX TDM or PDM channel 6. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_PDM_CHAN6_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data input of I2S RX TDM or PDM channel 7. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_PDM_CHAN7_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data input of I2S RX TDM channel 8. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_CHAN8_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data input of I2S RX TDM channel 9. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_CHAN9_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data input of I2S RX TDM channel 10. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_CHAN10_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data input of I2S RX TDM channel 11. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_CHAN11_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data input of I2S RX TDM channel 12. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_CHAN12_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data input of I2S RX TDM channel 13. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_CHAN13_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data input of I2S RX TDM channel 14. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_CHAN14_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data input of I2S RX TDM channel 15. 0: Disable, just input 0 in this channel."]
    pub mod RX_TDM_CHAN15_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The total channel number of I2S TX TDM mode."]
    pub mod RX_TDM_TOT_CHAN_NUM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S TX TDM mode control register"]
pub mod TX_TDM_CTRL {
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 0. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN0_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 1. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN1_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 2. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN2_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 3. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN3_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 4. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN4_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 5. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN5_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 6. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN6_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 7. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN7_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 8. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN8_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 9. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN9_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 10. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN10_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 11. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN11_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 12. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN12_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 13. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN13_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 14. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN14_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Enable the valid data output of I2S TX TDM channel 15. 0: Disable, just output 0 in this channel."]
    pub mod TX_TDM_CHAN15_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The total channel number of I2S TX TDM mode."]
    pub mod TX_TDM_TOT_CHAN_NUM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When DMA TX buffer stores the data of (REG_TX_TDM_TOT_CHAN_NUM + 1) channels, and only the data of the enabled channels is sent, then this bit should be set. Clear it when all the data stored in DMA TX buffer is for enabled channels."]
    pub mod TX_TDM_SKIP_MSK_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S RX timing control register"]
pub mod RX_TIMING {
    #[doc = "The delay mode of I2S Rx SD input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    pub mod RX_SD_IN_DM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The delay mode of I2S Rx WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    pub mod RX_WS_OUT_DM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The delay mode of I2S Rx BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    pub mod RX_BCK_OUT_DM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The delay mode of I2S Rx WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    pub mod RX_WS_IN_DM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The delay mode of I2S Rx BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    pub mod RX_BCK_IN_DM {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S TX timing control register"]
pub mod TX_TIMING {
    #[doc = "The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    pub mod TX_SD_OUT_DM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    pub mod TX_SD1_OUT_DM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    pub mod TX_WS_OUT_DM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    pub mod TX_BCK_OUT_DM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    pub mod TX_WS_IN_DM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    pub mod TX_BCK_IN_DM {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S HUNG configure register."]
pub mod LC_HUNG_CONF {
    #[doc = "the i2s_tx_hung_int interrupt or the i2s_rx_hung_int interrupt will be triggered when fifo hung counter is equal to this value"]
    pub mod LC_FIFO_TIMEOUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The bits are used to scale tick counter threshold. The tick counter is reset when counter value >= 88000/2^i2s_lc_fifo_timeout_shift"]
    pub mod LC_FIFO_TIMEOUT_SHIFT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The enable bit for FIFO timeout"]
    pub mod LC_FIFO_TIMEOUT_ENA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S RX data number control register."]
pub mod RXEOF_NUM {
    #[doc = "The receive data bit length is (I2S_RX_BITS_MOD\\[4:0\\] + 1) * (REG_RX_EOF_NUM\\[11:0\\] + 1) . It will trigger in_suc_eof interrupt in the configured DMA RX channel."]
    pub mod RX_EOF_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S signal data register"]
pub mod CONF_SIGLE_DATA {
    #[doc = "The configured constant channel data to be sent out."]
    pub mod SINGLE_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I2S TX status register"]
pub mod STATE {
    #[doc = "1: i2s_tx is idle state. 0: i2s_tx is working."]
    pub mod TX_IDLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Version control register"]
pub mod DATE {
    #[doc = "I2S version control register"]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
