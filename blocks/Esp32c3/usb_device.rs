#[doc = "Full-speed USB Serial/JTAG Controller"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "USB_DEVICE_EP1_REG."]
    pub EP1: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_EP1_CONF_REG."]
    pub EP1_CONF: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_INT_RAW_REG."]
    pub INT_RAW: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_INT_ST_REG."]
    pub INT_ST: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_INT_ENA_REG."]
    pub INT_ENA: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_INT_CLR_REG."]
    pub INT_CLR: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_CONF0_REG."]
    pub CONF0: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_TEST_REG."]
    pub TEST: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_JFIFO_ST_REG."]
    pub JFIFO_ST: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_FRAM_NUM_REG."]
    pub FRAM_NUM: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_IN_EP0_ST_REG."]
    pub IN_EP0_ST: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_IN_EP1_ST_REG."]
    pub IN_EP1_ST: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_IN_EP2_ST_REG."]
    pub IN_EP2_ST: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_IN_EP3_ST_REG."]
    pub IN_EP3_ST: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_OUT_EP0_ST_REG."]
    pub OUT_EP0_ST: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_OUT_EP1_ST_REG."]
    pub OUT_EP1_ST: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_OUT_EP2_ST_REG."]
    pub OUT_EP2_ST: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_MISC_CONF_REG."]
    pub MISC_CONF: crate::RWRegister<u32>,
    #[doc = "USB_DEVICE_MEM_CONF_REG."]
    pub MEM_CONF: crate::RWRegister<u32>,
    _reserved0: [u8; 0x34],
    #[doc = "USB_DEVICE_DATE_REG."]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "USB_DEVICE_EP1_REG."]
pub mod EP1 {
    #[doc = "Write and read byte data to/from UART Tx/Rx FIFO through this field. When USB_DEVICE_SERIAL_IN_EMPTY_INT is set, then user can write data (up to 64 bytes) into UART Tx FIFO. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is set, user can check USB_DEVICE_OUT_EP1_WR_ADDR USB_DEVICE_OUT_EP0_RD_ADDR to know how many data is received, then read data from UART Rx FIFO."]
    pub mod RDWR_BYTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_EP1_CONF_REG."]
pub mod EP1_CONF {
    #[doc = "Set this bit to indicate writing byte data to UART Tx FIFO is done."]
    pub mod WR_DONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1'b1: Indicate UART Tx FIFO is not full and can write data into in. After writing USB_DEVICE_WR_DONE, this bit would be 0 until data in UART Tx FIFO is read by USB Host."]
    pub mod SERIAL_IN_EP_DATA_FREE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1'b1: Indicate there is data in UART Rx FIFO."]
    pub mod SERIAL_OUT_EP_DATA_AVAIL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_INT_RAW_REG."]
pub mod INT_RAW {
    #[doc = "The raw interrupt bit turns to high level when flush cmd is received for IN endpoint 2 of JTAG."]
    pub mod JTAG_IN_FLUSH_INT_RAW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when SOF frame is received."]
    pub mod SOF_INT_RAW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when Serial Port OUT Endpoint received one packet."]
    pub mod SERIAL_OUT_RECV_PKT_INT_RAW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when Serial Port IN Endpoint is empty."]
    pub mod SERIAL_IN_EMPTY_INT_RAW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when pid error is detected."]
    pub mod PID_ERR_INT_RAW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when CRC5 error is detected."]
    pub mod CRC5_ERR_INT_RAW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when CRC16 error is detected."]
    pub mod CRC16_ERR_INT_RAW {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when stuff error is detected."]
    pub mod STUFF_ERR_INT_RAW {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when IN token for IN endpoint 1 is received."]
    pub mod IN_TOKEN_REC_IN_EP1_INT_RAW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when usb bus reset is detected."]
    pub mod USB_BUS_RESET_INT_RAW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when OUT endpoint 1 received packet with zero palyload."]
    pub mod OUT_EP1_ZERO_PAYLOAD_INT_RAW {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when OUT endpoint 2 received packet with zero palyload."]
    pub mod OUT_EP2_ZERO_PAYLOAD_INT_RAW {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_INT_ST_REG."]
pub mod INT_ST {
    #[doc = "The raw interrupt status bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
    pub mod JTAG_IN_FLUSH_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the USB_DEVICE_SOF_INT interrupt."]
    pub mod SOF_INT_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
    pub mod SERIAL_OUT_RECV_PKT_INT_ST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
    pub mod SERIAL_IN_EMPTY_INT_ST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the USB_DEVICE_PID_ERR_INT interrupt."]
    pub mod PID_ERR_INT_ST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
    pub mod CRC5_ERR_INT_ST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
    pub mod CRC16_ERR_INT_ST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
    pub mod STUFF_ERR_INT_ST {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    pub mod IN_TOKEN_REC_IN_EP1_INT_ST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
    pub mod USB_BUS_RESET_INT_ST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    pub mod OUT_EP1_ZERO_PAYLOAD_INT_ST {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    pub mod OUT_EP2_ZERO_PAYLOAD_INT_ST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_INT_ENA_REG."]
pub mod INT_ENA {
    #[doc = "The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
    pub mod JTAG_IN_FLUSH_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt."]
    pub mod SOF_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
    pub mod SERIAL_OUT_RECV_PKT_INT_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
    pub mod SERIAL_IN_EMPTY_INT_ENA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt."]
    pub mod PID_ERR_INT_ENA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
    pub mod CRC5_ERR_INT_ENA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
    pub mod CRC16_ERR_INT_ENA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
    pub mod STUFF_ERR_INT_ENA {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    pub mod IN_TOKEN_REC_IN_EP1_INT_ENA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
    pub mod USB_BUS_RESET_INT_ENA {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    pub mod OUT_EP1_ZERO_PAYLOAD_INT_ENA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    pub mod OUT_EP2_ZERO_PAYLOAD_INT_ENA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_INT_CLR_REG."]
pub mod INT_CLR {
    #[doc = "Set this bit to clear the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
    pub mod JTAG_IN_FLUSH_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the USB_DEVICE_JTAG_SOF_INT interrupt."]
    pub mod SOF_INT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
    pub mod SERIAL_OUT_RECV_PKT_INT_CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
    pub mod SERIAL_IN_EMPTY_INT_CLR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the USB_DEVICE_PID_ERR_INT interrupt."]
    pub mod PID_ERR_INT_CLR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the USB_DEVICE_CRC5_ERR_INT interrupt."]
    pub mod CRC5_ERR_INT_CLR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the USB_DEVICE_CRC16_ERR_INT interrupt."]
    pub mod CRC16_ERR_INT_CLR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the USB_DEVICE_STUFF_ERR_INT interrupt."]
    pub mod STUFF_ERR_INT_CLR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the USB_DEVICE_IN_TOKEN_IN_EP1_INT interrupt."]
    pub mod IN_TOKEN_REC_IN_EP1_INT_CLR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
    pub mod USB_BUS_RESET_INT_CLR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    pub mod OUT_EP1_ZERO_PAYLOAD_INT_CLR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    pub mod OUT_EP2_ZERO_PAYLOAD_INT_CLR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_CONF0_REG."]
pub mod CONF0 {
    #[doc = "Select internal/external PHY"]
    pub mod PHY_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable software control USB D+ D- exchange"]
    pub mod EXCHG_PINS_OVERRIDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB D+ D- exchange"]
    pub mod EXCHG_PINS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control single-end input high threshold,1.76V to 2V, step 80mV"]
    pub mod VREFH {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control single-end input low threshold,0.8V to 1.04V, step 80mV"]
    pub mod VREFL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable software control input threshold"]
    pub mod VREF_OVERRIDE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable software control USB D+ D- pullup pulldown"]
    pub mod PAD_PULL_OVERRIDE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control USB D+ pull up."]
    pub mod DP_PULLUP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control USB D+ pull down."]
    pub mod DP_PULLDOWN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control USB D- pull up."]
    pub mod DM_PULLUP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control USB D- pull down."]
    pub mod DM_PULLDOWN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control pull up value."]
    pub mod PULLUP_VALUE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable USB pad function."]
    pub mod USB_PAD_ENABLE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_TEST_REG."]
pub mod TEST {
    #[doc = "Enable test of the USB pad"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB pad oen in test"]
    pub mod USB_OE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB D+ tx value in test"]
    pub mod TX_DP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB D- tx value in test"]
    pub mod TX_DM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_JFIFO_ST_REG."]
pub mod JFIFO_ST {
    #[doc = "JTAT in fifo counter."]
    pub mod IN_FIFO_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: JTAG in fifo is empty."]
    pub mod IN_FIFO_EMPTY {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: JTAG in fifo is full."]
    pub mod IN_FIFO_FULL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "JTAT out fifo counter."]
    pub mod OUT_FIFO_CNT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: JTAG out fifo is empty."]
    pub mod OUT_FIFO_EMPTY {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: JTAG out fifo is full."]
    pub mod OUT_FIFO_FULL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write 1 to reset JTAG in fifo."]
    pub mod IN_FIFO_RESET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write 1 to reset JTAG out fifo."]
    pub mod OUT_FIFO_RESET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_FRAM_NUM_REG."]
pub mod FRAM_NUM {
    #[doc = "Frame index of received SOF frame."]
    pub mod SOF_FRAME_INDEX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_IN_EP0_ST_REG."]
pub mod IN_EP0_ST {
    #[doc = "State of IN Endpoint 0."]
    pub mod IN_EP0_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data address of IN endpoint 0."]
    pub mod IN_EP0_WR_ADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data address of IN endpoint 0."]
    pub mod IN_EP0_RD_ADDR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_IN_EP1_ST_REG."]
pub mod IN_EP1_ST {
    #[doc = "State of IN Endpoint 1."]
    pub mod IN_EP1_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data address of IN endpoint 1."]
    pub mod IN_EP1_WR_ADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data address of IN endpoint 1."]
    pub mod IN_EP1_RD_ADDR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_IN_EP2_ST_REG."]
pub mod IN_EP2_ST {
    #[doc = "State of IN Endpoint 2."]
    pub mod IN_EP2_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data address of IN endpoint 2."]
    pub mod IN_EP2_WR_ADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data address of IN endpoint 2."]
    pub mod IN_EP2_RD_ADDR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_IN_EP3_ST_REG."]
pub mod IN_EP3_ST {
    #[doc = "State of IN Endpoint 3."]
    pub mod IN_EP3_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data address of IN endpoint 3."]
    pub mod IN_EP3_WR_ADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data address of IN endpoint 3."]
    pub mod IN_EP3_RD_ADDR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_OUT_EP0_ST_REG."]
pub mod OUT_EP0_ST {
    #[doc = "State of OUT Endpoint 0."]
    pub mod OUT_EP0_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data address of OUT endpoint 0. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP0_WR_ADDR-2 bytes data in OUT EP0."]
    pub mod OUT_EP0_WR_ADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data address of OUT endpoint 0."]
    pub mod OUT_EP0_RD_ADDR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_OUT_EP1_ST_REG."]
pub mod OUT_EP1_ST {
    #[doc = "State of OUT Endpoint 1."]
    pub mod OUT_EP1_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data address of OUT endpoint 1. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP1_WR_ADDR-2 bytes data in OUT EP1."]
    pub mod OUT_EP1_WR_ADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data address of OUT endpoint 1."]
    pub mod OUT_EP1_RD_ADDR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data count in OUT endpoint 1 when one packet is received."]
    pub mod OUT_EP1_REC_DATA_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_OUT_EP2_ST_REG."]
pub mod OUT_EP2_ST {
    #[doc = "State of OUT Endpoint 2."]
    pub mod OUT_EP2_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write data address of OUT endpoint 2. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP2_WR_ADDR-2 bytes data in OUT EP2."]
    pub mod OUT_EP2_WR_ADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read data address of OUT endpoint 2."]
    pub mod OUT_EP2_RD_ADDR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_MISC_CONF_REG."]
pub mod MISC_CONF {
    #[doc = "1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    pub mod CLK_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_MEM_CONF_REG."]
pub mod MEM_CONF {
    #[doc = "1: power down usb memory."]
    pub mod USB_MEM_PD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Force clock on for usb memory."]
    pub mod USB_MEM_CLK_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB_DEVICE_DATE_REG."]
pub mod DATE {
    #[doc = "register version."]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
