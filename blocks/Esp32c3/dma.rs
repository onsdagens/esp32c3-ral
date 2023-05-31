#[doc = "DMA (Direct Memory Access) Controller"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "DMA_INT_RAW_CH0_REG."]
    pub INT_RAW_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_INT_ST_CH0_REG."]
    pub INT_ST_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_INT_ENA_CH0_REG."]
    pub INT_ENA_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_INT_CLR_CH0_REG."]
    pub INT_CLR_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_INT_RAW_CH1_REG."]
    pub INT_RAW_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_INT_ST_CH1_REG."]
    pub INT_ST_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_INT_ENA_CH1_REG."]
    pub INT_ENA_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_INT_CLR_CH1_REG."]
    pub INT_CLR_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_INT_RAW_CH2_REG."]
    pub INT_RAW_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_INT_ST_CH2_REG."]
    pub INT_ST_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_INT_ENA_CH2_REG."]
    pub INT_ENA_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_INT_CLR_CH2_REG."]
    pub INT_CLR_CH2: crate::RWRegister<u32>,
    _reserved0: [u8; 0x10],
    #[doc = "DMA_AHB_TEST_REG."]
    pub AHB_TEST: crate::RWRegister<u32>,
    #[doc = "DMA_MISC_CONF_REG."]
    pub MISC_CONF: crate::RWRegister<u32>,
    #[doc = "DMA_DATE_REG."]
    pub DATE: crate::RWRegister<u32>,
    _reserved1: [u8; 0x24],
    #[doc = "DMA_IN_CONF0_CH0_REG."]
    pub IN_CONF0_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_IN_CONF1_CH0_REG."]
    pub IN_CONF1_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_INFIFO_STATUS_CH0_REG."]
    pub INFIFO_STATUS_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_IN_POP_CH0_REG."]
    pub IN_POP_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_IN_LINK_CH0_REG."]
    pub IN_LINK_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_IN_STATE_CH0_REG."]
    pub IN_STATE_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH0_REG."]
    pub IN_SUC_EOF_DES_ADDR_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH0_REG."]
    pub IN_ERR_EOF_DES_ADDR_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_IN_DSCR_CH0_REG."]
    pub IN_DSCR_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_IN_DSCR_BF0_CH0_REG."]
    pub IN_DSCR_BF0_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_IN_DSCR_BF1_CH0_REG."]
    pub IN_DSCR_BF1_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_IN_PRI_CH0_REG."]
    pub IN_PRI_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_IN_PERI_SEL_CH0_REG."]
    pub IN_PERI_SEL_CH0: crate::RWRegister<u32>,
    _reserved2: [u8; 0x2c],
    #[doc = "DMA_OUT_CONF0_CH0_REG."]
    pub OUT_CONF0_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_CONF1_CH0_REG."]
    pub OUT_CONF1_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_OUTFIFO_STATUS_CH0_REG."]
    pub OUTFIFO_STATUS_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_PUSH_CH0_REG."]
    pub OUT_PUSH_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_LINK_CH0_REG."]
    pub OUT_LINK_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_STATE_CH0_REG."]
    pub OUT_STATE_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_EOF_DES_ADDR_CH0_REG."]
    pub OUT_EOF_DES_ADDR_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH0_REG."]
    pub OUT_EOF_BFR_DES_ADDR_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_DSCR_CH0_REG."]
    pub OUT_DSCR_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_DSCR_BF0_CH0_REG."]
    pub OUT_DSCR_BF0_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_DSCR_BF1_CH0_REG."]
    pub OUT_DSCR_BF1_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_PRI_CH0_REG."]
    pub OUT_PRI_CH0: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_PERI_SEL_CH0_REG."]
    pub OUT_PERI_SEL_CH0: crate::RWRegister<u32>,
    _reserved3: [u8; 0x2c],
    #[doc = "DMA_IN_CONF0_CH1_REG."]
    pub IN_CONF0_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_IN_CONF1_CH1_REG."]
    pub IN_CONF1_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_INFIFO_STATUS_CH1_REG."]
    pub INFIFO_STATUS_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_IN_POP_CH1_REG."]
    pub IN_POP_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_IN_LINK_CH1_REG."]
    pub IN_LINK_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_IN_STATE_CH1_REG."]
    pub IN_STATE_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH1_REG."]
    pub IN_SUC_EOF_DES_ADDR_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH1_REG."]
    pub IN_ERR_EOF_DES_ADDR_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_IN_DSCR_CH1_REG."]
    pub IN_DSCR_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_IN_DSCR_BF0_CH1_REG."]
    pub IN_DSCR_BF0_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_IN_DSCR_BF1_CH1_REG."]
    pub IN_DSCR_BF1_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_IN_PRI_CH1_REG."]
    pub IN_PRI_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_IN_PERI_SEL_CH1_REG."]
    pub IN_PERI_SEL_CH1: crate::RWRegister<u32>,
    _reserved4: [u8; 0x2c],
    #[doc = "DMA_OUT_CONF0_CH1_REG."]
    pub OUT_CONF0_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_CONF1_CH1_REG."]
    pub OUT_CONF1_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_OUTFIFO_STATUS_CH1_REG."]
    pub OUTFIFO_STATUS_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_PUSH_CH1_REG."]
    pub OUT_PUSH_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_LINK_CH1_REG."]
    pub OUT_LINK_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_STATE_CH1_REG."]
    pub OUT_STATE_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_EOF_DES_ADDR_CH1_REG."]
    pub OUT_EOF_DES_ADDR_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH1_REG."]
    pub OUT_EOF_BFR_DES_ADDR_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_DSCR_CH1_REG."]
    pub OUT_DSCR_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_DSCR_BF0_CH1_REG."]
    pub OUT_DSCR_BF0_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_DSCR_BF1_CH1_REG."]
    pub OUT_DSCR_BF1_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_PRI_CH1_REG."]
    pub OUT_PRI_CH1: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_PERI_SEL_CH1_REG."]
    pub OUT_PERI_SEL_CH1: crate::RWRegister<u32>,
    _reserved5: [u8; 0x2c],
    #[doc = "DMA_IN_CONF0_CH2_REG."]
    pub IN_CONF0_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_IN_CONF1_CH2_REG."]
    pub IN_CONF1_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_INFIFO_STATUS_CH2_REG."]
    pub INFIFO_STATUS_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_IN_POP_CH2_REG."]
    pub IN_POP_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_IN_LINK_CH2_REG."]
    pub IN_LINK_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_IN_STATE_CH2_REG."]
    pub IN_STATE_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH2_REG."]
    pub IN_SUC_EOF_DES_ADDR_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH2_REG."]
    pub IN_ERR_EOF_DES_ADDR_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_IN_DSCR_CH2_REG."]
    pub IN_DSCR_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_IN_DSCR_BF0_CH2_REG."]
    pub IN_DSCR_BF0_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_IN_DSCR_BF1_CH2_REG."]
    pub IN_DSCR_BF1_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_IN_PRI_CH2_REG."]
    pub IN_PRI_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_IN_PERI_SEL_CH2_REG."]
    pub IN_PERI_SEL_CH2: crate::RWRegister<u32>,
    _reserved6: [u8; 0x2c],
    #[doc = "DMA_OUT_CONF0_CH2_REG."]
    pub OUT_CONF0_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_CONF1_CH2_REG."]
    pub OUT_CONF1_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_OUTFIFO_STATUS_CH2_REG."]
    pub OUTFIFO_STATUS_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_PUSH_CH2_REG."]
    pub OUT_PUSH_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_LINK_CH2_REG."]
    pub OUT_LINK_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_STATE_CH2_REG."]
    pub OUT_STATE_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_EOF_DES_ADDR_CH2_REG."]
    pub OUT_EOF_DES_ADDR_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH2_REG."]
    pub OUT_EOF_BFR_DES_ADDR_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_DSCR_CH2_REG."]
    pub OUT_DSCR_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_DSCR_BF0_CH2_REG."]
    pub OUT_DSCR_BF0_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_DSCR_BF1_CH2_REG."]
    pub OUT_DSCR_BF1_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_PRI_CH2_REG."]
    pub OUT_PRI_CH2: crate::RWRegister<u32>,
    #[doc = "DMA_OUT_PERI_SEL_CH2_REG."]
    pub OUT_PERI_SEL_CH2: crate::RWRegister<u32>,
}
#[doc = "DMA_INT_RAW_CH0_REG."]
pub mod INT_RAW_CH0 {
    #[doc = "The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
    pub mod IN_DONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0, the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
    pub mod IN_SUC_EOF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals, this raw interrupt is reserved."]
    pub mod IN_ERR_EOF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
    pub mod OUT_DONE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
    pub mod OUT_EOF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 0."]
    pub mod IN_DSCR_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
    pub mod OUT_DSCR_ERR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed, but there is no more inlink for Rx channel 0."]
    pub mod IN_DSCR_EMPTY {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
    pub mod OUT_TOTAL_EOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
    pub mod INFIFO_OVF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
    pub mod INFIFO_UDF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow."]
    pub mod OUTFIFO_OVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow."]
    pub mod OUTFIFO_UDF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_INT_ST_CH0_REG."]
pub mod INT_ST_CH0 {
    #[doc = "The raw interrupt status bit for the IN_DONE_CH_INT interrupt."]
    pub mod IN_DONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt."]
    pub mod IN_SUC_EOF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt."]
    pub mod IN_ERR_EOF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUT_DONE_CH_INT interrupt."]
    pub mod OUT_DONE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUT_EOF_CH_INT interrupt."]
    pub mod OUT_EOF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt."]
    pub mod IN_DSCR_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    pub mod OUT_DSCR_ERR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    pub mod IN_DSCR_EMPTY {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    pub mod OUT_TOTAL_EOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    pub mod INFIFO_OVF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    pub mod INFIFO_UDF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_OVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_UDF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_INT_ENA_CH0_REG."]
pub mod INT_ENA_CH0 {
    #[doc = "The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    pub mod IN_DONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    pub mod IN_SUC_EOF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    pub mod IN_ERR_EOF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
    pub mod OUT_DONE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
    pub mod OUT_EOF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    pub mod IN_DSCR_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    pub mod OUT_DSCR_ERR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    pub mod IN_DSCR_EMPTY {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    pub mod OUT_TOTAL_EOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    pub mod INFIFO_OVF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    pub mod INFIFO_UDF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_OVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_UDF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_INT_CLR_CH0_REG."]
pub mod INT_CLR_CH0 {
    #[doc = "Set this bit to clear the IN_DONE_CH_INT interrupt."]
    pub mod IN_DONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
    pub mod IN_SUC_EOF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
    pub mod IN_ERR_EOF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUT_DONE_CH_INT interrupt."]
    pub mod OUT_DONE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUT_EOF_CH_INT interrupt."]
    pub mod OUT_EOF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt."]
    pub mod IN_DSCR_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
    pub mod OUT_DSCR_ERR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
    pub mod IN_DSCR_EMPTY {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
    pub mod OUT_TOTAL_EOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
    pub mod INFIFO_OVF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
    pub mod INFIFO_UDF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_OVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_UDF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_INT_RAW_CH1_REG."]
pub mod INT_RAW_CH1 {
    #[doc = "The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 1."]
    pub mod IN_DONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 1. For UHCI0, the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 1."]
    pub mod IN_SUC_EOF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 1. For other peripherals, this raw interrupt is reserved."]
    pub mod IN_ERR_EOF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 1."]
    pub mod OUT_DONE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 1."]
    pub mod OUT_EOF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 1."]
    pub mod IN_DSCR_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 1."]
    pub mod OUT_DSCR_ERR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed, but there is no more inlink for Rx channel 1."]
    pub mod IN_DSCR_EMPTY {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 1."]
    pub mod OUT_TOTAL_EOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This raw interrupt bit turns to high level when level 1 fifo of Rx channel 1 is overflow."]
    pub mod INFIFO_OVF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This raw interrupt bit turns to high level when level 1 fifo of Rx channel 1 is underflow."]
    pub mod INFIFO_UDF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This raw interrupt bit turns to high level when level 1 fifo of Tx channel 1 is overflow."]
    pub mod OUTFIFO_OVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This raw interrupt bit turns to high level when level 1 fifo of Tx channel 1 is underflow."]
    pub mod OUTFIFO_UDF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_INT_ST_CH1_REG."]
pub mod INT_ST_CH1 {
    #[doc = "The raw interrupt status bit for the IN_DONE_CH_INT interrupt."]
    pub mod IN_DONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt."]
    pub mod IN_SUC_EOF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt."]
    pub mod IN_ERR_EOF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUT_DONE_CH_INT interrupt."]
    pub mod OUT_DONE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUT_EOF_CH_INT interrupt."]
    pub mod OUT_EOF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt."]
    pub mod IN_DSCR_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    pub mod OUT_DSCR_ERR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    pub mod IN_DSCR_EMPTY {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    pub mod OUT_TOTAL_EOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    pub mod INFIFO_OVF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    pub mod INFIFO_UDF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_OVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_UDF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_INT_ENA_CH1_REG."]
pub mod INT_ENA_CH1 {
    #[doc = "The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    pub mod IN_DONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    pub mod IN_SUC_EOF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    pub mod IN_ERR_EOF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
    pub mod OUT_DONE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
    pub mod OUT_EOF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    pub mod IN_DSCR_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    pub mod OUT_DSCR_ERR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    pub mod IN_DSCR_EMPTY {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    pub mod OUT_TOTAL_EOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    pub mod INFIFO_OVF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    pub mod INFIFO_UDF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_OVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_UDF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_INT_CLR_CH1_REG."]
pub mod INT_CLR_CH1 {
    #[doc = "Set this bit to clear the IN_DONE_CH_INT interrupt."]
    pub mod IN_DONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
    pub mod IN_SUC_EOF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
    pub mod IN_ERR_EOF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUT_DONE_CH_INT interrupt."]
    pub mod OUT_DONE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUT_EOF_CH_INT interrupt."]
    pub mod OUT_EOF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt."]
    pub mod IN_DSCR_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
    pub mod OUT_DSCR_ERR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
    pub mod IN_DSCR_EMPTY {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
    pub mod OUT_TOTAL_EOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
    pub mod INFIFO_OVF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
    pub mod INFIFO_UDF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_OVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_UDF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_INT_RAW_CH2_REG."]
pub mod INT_RAW_CH2 {
    #[doc = "The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 2."]
    pub mod IN_DONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 2. For UHCI0, the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 2."]
    pub mod IN_SUC_EOF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 2. For other peripherals, this raw interrupt is reserved."]
    pub mod IN_ERR_EOF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 2."]
    pub mod OUT_DONE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 2."]
    pub mod OUT_EOF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 2."]
    pub mod IN_DSCR_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 2."]
    pub mod OUT_DSCR_ERR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed, but there is no more inlink for Rx channel 2."]
    pub mod IN_DSCR_EMPTY {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 2."]
    pub mod OUT_TOTAL_EOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This raw interrupt bit turns to high level when level 1 fifo of Rx channel 2 is overflow."]
    pub mod INFIFO_OVF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This raw interrupt bit turns to high level when level 1 fifo of Rx channel 2 is underflow."]
    pub mod INFIFO_UDF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This raw interrupt bit turns to high level when level 1 fifo of Tx channel 2 is overflow."]
    pub mod OUTFIFO_OVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This raw interrupt bit turns to high level when level 1 fifo of Tx channel 2 is underflow."]
    pub mod OUTFIFO_UDF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_INT_ST_CH2_REG."]
pub mod INT_ST_CH2 {
    #[doc = "The raw interrupt status bit for the IN_DONE_CH_INT interrupt."]
    pub mod IN_DONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt."]
    pub mod IN_SUC_EOF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt."]
    pub mod IN_ERR_EOF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUT_DONE_CH_INT interrupt."]
    pub mod OUT_DONE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUT_EOF_CH_INT interrupt."]
    pub mod OUT_EOF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt."]
    pub mod IN_DSCR_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    pub mod OUT_DSCR_ERR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    pub mod IN_DSCR_EMPTY {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    pub mod OUT_TOTAL_EOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    pub mod INFIFO_OVF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    pub mod INFIFO_UDF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_OVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_UDF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_INT_ENA_CH2_REG."]
pub mod INT_ENA_CH2 {
    #[doc = "The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    pub mod IN_DONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    pub mod IN_SUC_EOF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    pub mod IN_ERR_EOF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
    pub mod OUT_DONE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
    pub mod OUT_EOF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    pub mod IN_DSCR_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    pub mod OUT_DSCR_ERR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    pub mod IN_DSCR_EMPTY {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    pub mod OUT_TOTAL_EOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    pub mod INFIFO_OVF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    pub mod INFIFO_UDF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_OVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_UDF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_INT_CLR_CH2_REG."]
pub mod INT_CLR_CH2 {
    #[doc = "Set this bit to clear the IN_DONE_CH_INT interrupt."]
    pub mod IN_DONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
    pub mod IN_SUC_EOF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
    pub mod IN_ERR_EOF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUT_DONE_CH_INT interrupt."]
    pub mod OUT_DONE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUT_EOF_CH_INT interrupt."]
    pub mod OUT_EOF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt."]
    pub mod IN_DSCR_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
    pub mod OUT_DSCR_ERR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
    pub mod IN_DSCR_EMPTY {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
    pub mod OUT_TOTAL_EOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
    pub mod INFIFO_OVF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
    pub mod INFIFO_UDF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_OVF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
    pub mod OUTFIFO_UDF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_AHB_TEST_REG."]
pub mod AHB_TEST {
    #[doc = "reserved"]
    pub mod AHB_TESTMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod AHB_TESTADDR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_MISC_CONF_REG."]
pub mod MISC_CONF {
    #[doc = "Set this bit, then clear this bit to reset the internal ahb FSM."]
    pub mod AHBM_RST_INTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to disable priority arbitration function."]
    pub mod ARB_PRI_DIS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reg_clk_en"]
    pub mod CLK_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_DATE_REG."]
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
#[doc = "DMA_IN_CONF0_CH0_REG."]
pub mod IN_CONF0_CH0 {
    #[doc = "This bit is used to reset DMA channel 0 Rx FSM and Rx FIFO pointer."]
    pub mod IN_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_LOOP_TEST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
    pub mod INDSCR_BURST_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM."]
    pub mod IN_DATA_BURST_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
    pub mod MEM_TRANS_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_CONF1_CH0_REG."]
pub mod IN_CONF1_CH0 {
    #[doc = "Set this bit to enable checking the owner attribute of the link descriptor."]
    pub mod IN_CHECK_OWNER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_INFIFO_STATUS_CH0_REG."]
pub mod INFIFO_STATUS_CH0 {
    #[doc = "L1 Rx FIFO full signal for Rx channel 0."]
    pub mod INFIFO_FULL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L1 Rx FIFO empty signal for Rx channel 0."]
    pub mod INFIFO_EMPTY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0."]
    pub mod INFIFO_CNT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_REMAIN_UNDER_1B {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_REMAIN_UNDER_2B {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_REMAIN_UNDER_3B {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_REMAIN_UNDER_4B {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_BUF_HUNGRY {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_POP_CH0_REG."]
pub mod IN_POP_CH0 {
    #[doc = "This register stores the data popping from DMA FIFO."]
    pub mod INFIFO_RDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to pop data from DMA FIFO."]
    pub mod INFIFO_POP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_LINK_CH0_REG."]
pub mod IN_LINK_CH0 {
    #[doc = "This register stores the 20 least significant bits of the first inlink descriptor's address."]
    pub mod INLINK_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
    pub mod INLINK_AUTO_RET {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to stop dealing with the inlink descriptors."]
    pub mod INLINK_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to start dealing with the inlink descriptors."]
    pub mod INLINK_START {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to mount a new inlink descriptor."]
    pub mod INLINK_RESTART {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
    pub mod INLINK_PARK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_STATE_CH0_REG."]
pub mod IN_STATE_CH0 {
    #[doc = "This register stores the current inlink descriptor's address."]
    pub mod INLINK_DSCR_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_DSCR_STATE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_STATE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH0_REG."]
pub mod IN_SUC_EOF_DES_ADDR_CH0 {
    #[doc = "This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1."]
    pub mod IN_SUC_EOF_DES_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH0_REG."]
pub mod IN_ERR_EOF_DES_ADDR_CH0 {
    #[doc = "This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0."]
    pub mod IN_ERR_EOF_DES_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_DSCR_CH0_REG."]
pub mod IN_DSCR_CH0 {
    #[doc = "The address of the current inlink descriptor x."]
    pub mod INLINK_DSCR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_DSCR_BF0_CH0_REG."]
pub mod IN_DSCR_BF0_CH0 {
    #[doc = "The address of the last inlink descriptor x-1."]
    pub mod INLINK_DSCR_BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_DSCR_BF1_CH0_REG."]
pub mod IN_DSCR_BF1_CH0 {
    #[doc = "The address of the second-to-last inlink descriptor x-2."]
    pub mod INLINK_DSCR_BF1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_PRI_CH0_REG."]
pub mod IN_PRI_CH0 {
    #[doc = "The priority of Rx channel 0. The larger of the value, the higher of the priority."]
    pub mod RX_PRI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_PERI_SEL_CH0_REG."]
pub mod IN_PERI_SEL_CH0 {
    #[doc = "This register is used to select peripheral for Rx channel 0. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
    pub mod PERI_IN_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_CONF0_CH0_REG."]
pub mod OUT_CONF0_CH0 {
    #[doc = "This bit is used to reset DMA channel 0 Tx FSM and Tx FIFO pointer."]
    pub mod OUT_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_LOOP_TEST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    pub mod OUT_AUTO_WRBACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 0 is generated when data need to transmit has been popped from FIFO in DMA"]
    pub mod OUT_EOF_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
    pub mod OUTDSCR_BURST_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable INCR burst transfer for Tx channel 0 transmitting data when accessing internal SRAM."]
    pub mod OUT_DATA_BURST_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_CONF1_CH0_REG."]
pub mod OUT_CONF1_CH0 {
    #[doc = "Set this bit to enable checking the owner attribute of the link descriptor."]
    pub mod OUT_CHECK_OWNER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUTFIFO_STATUS_CH0_REG."]
pub mod OUTFIFO_STATUS_CH0 {
    #[doc = "L1 Tx FIFO full signal for Tx channel 0."]
    pub mod OUTFIFO_FULL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L1 Tx FIFO empty signal for Tx channel 0."]
    pub mod OUTFIFO_EMPTY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0."]
    pub mod OUTFIFO_CNT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_REMAIN_UNDER_1B {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_REMAIN_UNDER_2B {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_REMAIN_UNDER_3B {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_REMAIN_UNDER_4B {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_PUSH_CH0_REG."]
pub mod OUT_PUSH_CH0 {
    #[doc = "This register stores the data that need to be pushed into DMA FIFO."]
    pub mod OUTFIFO_WDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to push data into DMA FIFO."]
    pub mod OUTFIFO_PUSH {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_LINK_CH0_REG."]
pub mod OUT_LINK_CH0 {
    #[doc = "This register stores the 20 least significant bits of the first outlink descriptor's address."]
    pub mod OUTLINK_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to stop dealing with the outlink descriptors."]
    pub mod OUTLINK_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to start dealing with the outlink descriptors."]
    pub mod OUTLINK_START {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to restart a new outlink from the last address."]
    pub mod OUTLINK_RESTART {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
    pub mod OUTLINK_PARK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_STATE_CH0_REG."]
pub mod OUT_STATE_CH0 {
    #[doc = "This register stores the current outlink descriptor's address."]
    pub mod OUTLINK_DSCR_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_DSCR_STATE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_STATE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_EOF_DES_ADDR_CH0_REG."]
pub mod OUT_EOF_DES_ADDR_CH0 {
    #[doc = "This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1."]
    pub mod OUT_EOF_DES_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH0_REG."]
pub mod OUT_EOF_BFR_DES_ADDR_CH0 {
    #[doc = "This register stores the address of the outlink descriptor before the last outlink descriptor."]
    pub mod OUT_EOF_BFR_DES_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_DSCR_CH0_REG."]
pub mod OUT_DSCR_CH0 {
    #[doc = "The address of the current outlink descriptor y."]
    pub mod OUTLINK_DSCR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_DSCR_BF0_CH0_REG."]
pub mod OUT_DSCR_BF0_CH0 {
    #[doc = "The address of the last outlink descriptor y-1."]
    pub mod OUTLINK_DSCR_BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_DSCR_BF1_CH0_REG."]
pub mod OUT_DSCR_BF1_CH0 {
    #[doc = "The address of the second-to-last inlink descriptor x-2."]
    pub mod OUTLINK_DSCR_BF1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_PRI_CH0_REG."]
pub mod OUT_PRI_CH0 {
    #[doc = "The priority of Tx channel 0. The larger of the value, the higher of the priority."]
    pub mod TX_PRI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_PERI_SEL_CH0_REG."]
pub mod OUT_PERI_SEL_CH0 {
    #[doc = "This register is used to select peripheral for Tx channel 0. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
    pub mod PERI_OUT_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_CONF0_CH1_REG."]
pub mod IN_CONF0_CH1 {
    #[doc = "This bit is used to reset DMA channel 1 Rx FSM and Rx FIFO pointer."]
    pub mod IN_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_LOOP_TEST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable INCR burst transfer for Rx channel 1 reading link descriptor when accessing internal SRAM."]
    pub mod INDSCR_BURST_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable INCR burst transfer for Rx channel 1 receiving data when accessing internal SRAM."]
    pub mod IN_DATA_BURST_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
    pub mod MEM_TRANS_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_CONF1_CH1_REG."]
pub mod IN_CONF1_CH1 {
    #[doc = "Set this bit to enable checking the owner attribute of the link descriptor."]
    pub mod IN_CHECK_OWNER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_INFIFO_STATUS_CH1_REG."]
pub mod INFIFO_STATUS_CH1 {
    #[doc = "L1 Rx FIFO full signal for Rx channel 1."]
    pub mod INFIFO_FULL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L1 Rx FIFO empty signal for Rx channel 1."]
    pub mod INFIFO_EMPTY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The register stores the byte number of the data in L1 Rx FIFO for Rx channel 1."]
    pub mod INFIFO_CNT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_REMAIN_UNDER_1B {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_REMAIN_UNDER_2B {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_REMAIN_UNDER_3B {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_REMAIN_UNDER_4B {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_BUF_HUNGRY {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_POP_CH1_REG."]
pub mod IN_POP_CH1 {
    #[doc = "This register stores the data popping from DMA FIFO."]
    pub mod INFIFO_RDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to pop data from DMA FIFO."]
    pub mod INFIFO_POP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_LINK_CH1_REG."]
pub mod IN_LINK_CH1 {
    #[doc = "This register stores the 20 least significant bits of the first inlink descriptor's address."]
    pub mod INLINK_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
    pub mod INLINK_AUTO_RET {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to stop dealing with the inlink descriptors."]
    pub mod INLINK_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to start dealing with the inlink descriptors."]
    pub mod INLINK_START {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to mount a new inlink descriptor."]
    pub mod INLINK_RESTART {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
    pub mod INLINK_PARK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_STATE_CH1_REG."]
pub mod IN_STATE_CH1 {
    #[doc = "This register stores the current inlink descriptor's address."]
    pub mod INLINK_DSCR_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_DSCR_STATE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_STATE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH1_REG."]
pub mod IN_SUC_EOF_DES_ADDR_CH1 {
    #[doc = "This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1."]
    pub mod IN_SUC_EOF_DES_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH1_REG."]
pub mod IN_ERR_EOF_DES_ADDR_CH1 {
    #[doc = "This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0."]
    pub mod IN_ERR_EOF_DES_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_DSCR_CH1_REG."]
pub mod IN_DSCR_CH1 {
    #[doc = "The address of the current inlink descriptor x."]
    pub mod INLINK_DSCR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_DSCR_BF0_CH1_REG."]
pub mod IN_DSCR_BF0_CH1 {
    #[doc = "The address of the last inlink descriptor x-1."]
    pub mod INLINK_DSCR_BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_DSCR_BF1_CH1_REG."]
pub mod IN_DSCR_BF1_CH1 {
    #[doc = "The address of the second-to-last inlink descriptor x-2."]
    pub mod INLINK_DSCR_BF1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_PRI_CH1_REG."]
pub mod IN_PRI_CH1 {
    #[doc = "The priority of Rx channel 1. The larger of the value, the higher of the priority."]
    pub mod RX_PRI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_PERI_SEL_CH1_REG."]
pub mod IN_PERI_SEL_CH1 {
    #[doc = "This register is used to select peripheral for Rx channel 1. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
    pub mod PERI_IN_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_CONF0_CH1_REG."]
pub mod OUT_CONF0_CH1 {
    #[doc = "This bit is used to reset DMA channel 1 Tx FSM and Tx FIFO pointer."]
    pub mod OUT_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_LOOP_TEST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    pub mod OUT_AUTO_WRBACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 1 is generated when data need to transmit has been popped from FIFO in DMA"]
    pub mod OUT_EOF_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable INCR burst transfer for Tx channel 1 reading link descriptor when accessing internal SRAM."]
    pub mod OUTDSCR_BURST_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable INCR burst transfer for Tx channel 1 transmitting data when accessing internal SRAM."]
    pub mod OUT_DATA_BURST_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_CONF1_CH1_REG."]
pub mod OUT_CONF1_CH1 {
    #[doc = "Set this bit to enable checking the owner attribute of the link descriptor."]
    pub mod OUT_CHECK_OWNER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUTFIFO_STATUS_CH1_REG."]
pub mod OUTFIFO_STATUS_CH1 {
    #[doc = "L1 Tx FIFO full signal for Tx channel 1."]
    pub mod OUTFIFO_FULL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L1 Tx FIFO empty signal for Tx channel 1."]
    pub mod OUTFIFO_EMPTY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The register stores the byte number of the data in L1 Tx FIFO for Tx channel 1."]
    pub mod OUTFIFO_CNT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_REMAIN_UNDER_1B {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_REMAIN_UNDER_2B {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_REMAIN_UNDER_3B {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_REMAIN_UNDER_4B {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_PUSH_CH1_REG."]
pub mod OUT_PUSH_CH1 {
    #[doc = "This register stores the data that need to be pushed into DMA FIFO."]
    pub mod OUTFIFO_WDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to push data into DMA FIFO."]
    pub mod OUTFIFO_PUSH {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_LINK_CH1_REG."]
pub mod OUT_LINK_CH1 {
    #[doc = "This register stores the 20 least significant bits of the first outlink descriptor's address."]
    pub mod OUTLINK_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to stop dealing with the outlink descriptors."]
    pub mod OUTLINK_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to start dealing with the outlink descriptors."]
    pub mod OUTLINK_START {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to restart a new outlink from the last address."]
    pub mod OUTLINK_RESTART {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
    pub mod OUTLINK_PARK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_STATE_CH1_REG."]
pub mod OUT_STATE_CH1 {
    #[doc = "This register stores the current outlink descriptor's address."]
    pub mod OUTLINK_DSCR_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_DSCR_STATE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_STATE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_EOF_DES_ADDR_CH1_REG."]
pub mod OUT_EOF_DES_ADDR_CH1 {
    #[doc = "This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1."]
    pub mod OUT_EOF_DES_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH1_REG."]
pub mod OUT_EOF_BFR_DES_ADDR_CH1 {
    #[doc = "This register stores the address of the outlink descriptor before the last outlink descriptor."]
    pub mod OUT_EOF_BFR_DES_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_DSCR_CH1_REG."]
pub mod OUT_DSCR_CH1 {
    #[doc = "The address of the current outlink descriptor y."]
    pub mod OUTLINK_DSCR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_DSCR_BF0_CH1_REG."]
pub mod OUT_DSCR_BF0_CH1 {
    #[doc = "The address of the last outlink descriptor y-1."]
    pub mod OUTLINK_DSCR_BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_DSCR_BF1_CH1_REG."]
pub mod OUT_DSCR_BF1_CH1 {
    #[doc = "The address of the second-to-last inlink descriptor x-2."]
    pub mod OUTLINK_DSCR_BF1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_PRI_CH1_REG."]
pub mod OUT_PRI_CH1 {
    #[doc = "The priority of Tx channel 1. The larger of the value, the higher of the priority."]
    pub mod TX_PRI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_PERI_SEL_CH1_REG."]
pub mod OUT_PERI_SEL_CH1 {
    #[doc = "This register is used to select peripheral for Tx channel 1. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
    pub mod PERI_OUT_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_CONF0_CH2_REG."]
pub mod IN_CONF0_CH2 {
    #[doc = "This bit is used to reset DMA channel 2 Rx FSM and Rx FIFO pointer."]
    pub mod IN_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_LOOP_TEST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable INCR burst transfer for Rx channel 2 reading link descriptor when accessing internal SRAM."]
    pub mod INDSCR_BURST_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable INCR burst transfer for Rx channel 2 receiving data when accessing internal SRAM."]
    pub mod IN_DATA_BURST_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
    pub mod MEM_TRANS_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_CONF1_CH2_REG."]
pub mod IN_CONF1_CH2 {
    #[doc = "Set this bit to enable checking the owner attribute of the link descriptor."]
    pub mod IN_CHECK_OWNER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_INFIFO_STATUS_CH2_REG."]
pub mod INFIFO_STATUS_CH2 {
    #[doc = "L1 Rx FIFO full signal for Rx channel 2."]
    pub mod INFIFO_FULL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L1 Rx FIFO empty signal for Rx channel 2."]
    pub mod INFIFO_EMPTY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The register stores the byte number of the data in L1 Rx FIFO for Rx channel 2."]
    pub mod INFIFO_CNT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_REMAIN_UNDER_1B {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_REMAIN_UNDER_2B {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_REMAIN_UNDER_3B {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_REMAIN_UNDER_4B {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_BUF_HUNGRY {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_POP_CH2_REG."]
pub mod IN_POP_CH2 {
    #[doc = "This register stores the data popping from DMA FIFO."]
    pub mod INFIFO_RDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to pop data from DMA FIFO."]
    pub mod INFIFO_POP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_LINK_CH2_REG."]
pub mod IN_LINK_CH2 {
    #[doc = "This register stores the 20 least significant bits of the first inlink descriptor's address."]
    pub mod INLINK_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
    pub mod INLINK_AUTO_RET {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to stop dealing with the inlink descriptors."]
    pub mod INLINK_STOP {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to start dealing with the inlink descriptors."]
    pub mod INLINK_START {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to mount a new inlink descriptor."]
    pub mod INLINK_RESTART {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
    pub mod INLINK_PARK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_STATE_CH2_REG."]
pub mod IN_STATE_CH2 {
    #[doc = "This register stores the current inlink descriptor's address."]
    pub mod INLINK_DSCR_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_DSCR_STATE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod IN_STATE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH2_REG."]
pub mod IN_SUC_EOF_DES_ADDR_CH2 {
    #[doc = "This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1."]
    pub mod IN_SUC_EOF_DES_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH2_REG."]
pub mod IN_ERR_EOF_DES_ADDR_CH2 {
    #[doc = "This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0."]
    pub mod IN_ERR_EOF_DES_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_DSCR_CH2_REG."]
pub mod IN_DSCR_CH2 {
    #[doc = "The address of the current inlink descriptor x."]
    pub mod INLINK_DSCR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_DSCR_BF0_CH2_REG."]
pub mod IN_DSCR_BF0_CH2 {
    #[doc = "The address of the last inlink descriptor x-1."]
    pub mod INLINK_DSCR_BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_DSCR_BF1_CH2_REG."]
pub mod IN_DSCR_BF1_CH2 {
    #[doc = "The address of the second-to-last inlink descriptor x-2."]
    pub mod INLINK_DSCR_BF1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_PRI_CH2_REG."]
pub mod IN_PRI_CH2 {
    #[doc = "The priority of Rx channel 2. The larger of the value, the higher of the priority."]
    pub mod RX_PRI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_IN_PERI_SEL_CH2_REG."]
pub mod IN_PERI_SEL_CH2 {
    #[doc = "This register is used to select peripheral for Rx channel 2. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
    pub mod PERI_IN_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_CONF0_CH2_REG."]
pub mod OUT_CONF0_CH2 {
    #[doc = "This bit is used to reset DMA channel 2 Tx FSM and Tx FIFO pointer."]
    pub mod OUT_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_LOOP_TEST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    pub mod OUT_AUTO_WRBACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 2 is generated when data need to transmit has been popped from FIFO in DMA"]
    pub mod OUT_EOF_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable INCR burst transfer for Tx channel 2 reading link descriptor when accessing internal SRAM."]
    pub mod OUTDSCR_BURST_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable INCR burst transfer for Tx channel 2 transmitting data when accessing internal SRAM."]
    pub mod OUT_DATA_BURST_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_CONF1_CH2_REG."]
pub mod OUT_CONF1_CH2 {
    #[doc = "Set this bit to enable checking the owner attribute of the link descriptor."]
    pub mod OUT_CHECK_OWNER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUTFIFO_STATUS_CH2_REG."]
pub mod OUTFIFO_STATUS_CH2 {
    #[doc = "L1 Tx FIFO full signal for Tx channel 2."]
    pub mod OUTFIFO_FULL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "L1 Tx FIFO empty signal for Tx channel 2."]
    pub mod OUTFIFO_EMPTY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The register stores the byte number of the data in L1 Tx FIFO for Tx channel 2."]
    pub mod OUTFIFO_CNT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_REMAIN_UNDER_1B {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_REMAIN_UNDER_2B {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_REMAIN_UNDER_3B {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_REMAIN_UNDER_4B {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_PUSH_CH2_REG."]
pub mod OUT_PUSH_CH2 {
    #[doc = "This register stores the data that need to be pushed into DMA FIFO."]
    pub mod OUTFIFO_WDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to push data into DMA FIFO."]
    pub mod OUTFIFO_PUSH {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_LINK_CH2_REG."]
pub mod OUT_LINK_CH2 {
    #[doc = "This register stores the 20 least significant bits of the first outlink descriptor's address."]
    pub mod OUTLINK_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to stop dealing with the outlink descriptors."]
    pub mod OUTLINK_STOP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to start dealing with the outlink descriptors."]
    pub mod OUTLINK_START {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to restart a new outlink from the last address."]
    pub mod OUTLINK_RESTART {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
    pub mod OUTLINK_PARK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_STATE_CH2_REG."]
pub mod OUT_STATE_CH2 {
    #[doc = "This register stores the current outlink descriptor's address."]
    pub mod OUTLINK_DSCR_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_DSCR_STATE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod OUT_STATE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_EOF_DES_ADDR_CH2_REG."]
pub mod OUT_EOF_DES_ADDR_CH2 {
    #[doc = "This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1."]
    pub mod OUT_EOF_DES_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH2_REG."]
pub mod OUT_EOF_BFR_DES_ADDR_CH2 {
    #[doc = "This register stores the address of the outlink descriptor before the last outlink descriptor."]
    pub mod OUT_EOF_BFR_DES_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_DSCR_CH2_REG."]
pub mod OUT_DSCR_CH2 {
    #[doc = "The address of the current outlink descriptor y."]
    pub mod OUTLINK_DSCR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_DSCR_BF0_CH2_REG."]
pub mod OUT_DSCR_BF0_CH2 {
    #[doc = "The address of the last outlink descriptor y-1."]
    pub mod OUTLINK_DSCR_BF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_DSCR_BF1_CH2_REG."]
pub mod OUT_DSCR_BF1_CH2 {
    #[doc = "The address of the second-to-last inlink descriptor x-2."]
    pub mod OUTLINK_DSCR_BF1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_PRI_CH2_REG."]
pub mod OUT_PRI_CH2 {
    #[doc = "The priority of Tx channel 2. The larger of the value, the higher of the priority."]
    pub mod TX_PRI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA_OUT_PERI_SEL_CH2_REG."]
pub mod OUT_PERI_SEL_CH2 {
    #[doc = "This register is used to select peripheral for Tx channel 2. 0:SPI2. 1: reserved. 2: UHCI0. 3: I2S0. 4: reserved. 5: reserved. 6: AES. 7: SHA. 8: ADC_DAC."]
    pub mod PERI_OUT_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
