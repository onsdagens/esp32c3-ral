#[doc = "Universal Host Controller Interface 0"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "a"]
    pub CONF0: crate::RWRegister<u32>,
    #[doc = "a"]
    pub INT_RAW: crate::RWRegister<u32>,
    #[doc = "a"]
    pub INT_ST: crate::RWRegister<u32>,
    #[doc = "a"]
    pub INT_ENA: crate::RWRegister<u32>,
    #[doc = "a"]
    pub INT_CLR: crate::RWRegister<u32>,
    #[doc = "a"]
    pub CONF1: crate::RWRegister<u32>,
    #[doc = "a"]
    pub STATE0: crate::RWRegister<u32>,
    #[doc = "a"]
    pub STATE1: crate::RWRegister<u32>,
    #[doc = "a"]
    pub ESCAPE_CONF: crate::RWRegister<u32>,
    #[doc = "a"]
    pub HUNG_CONF: crate::RWRegister<u32>,
    #[doc = "a"]
    pub ACK_NUM: crate::RWRegister<u32>,
    #[doc = "a"]
    pub RX_HEAD: crate::RWRegister<u32>,
    #[doc = "a"]
    pub QUICK_SENT: crate::RWRegister<u32>,
    #[doc = "a"]
    pub REG_Q0_WORD0: crate::RWRegister<u32>,
    #[doc = "a"]
    pub REG_Q0_WORD1: crate::RWRegister<u32>,
    #[doc = "a"]
    pub REG_Q1_WORD0: crate::RWRegister<u32>,
    #[doc = "a"]
    pub REG_Q1_WORD1: crate::RWRegister<u32>,
    #[doc = "a"]
    pub REG_Q2_WORD0: crate::RWRegister<u32>,
    #[doc = "a"]
    pub REG_Q2_WORD1: crate::RWRegister<u32>,
    #[doc = "a"]
    pub REG_Q3_WORD0: crate::RWRegister<u32>,
    #[doc = "a"]
    pub REG_Q3_WORD1: crate::RWRegister<u32>,
    #[doc = "a"]
    pub REG_Q4_WORD0: crate::RWRegister<u32>,
    #[doc = "a"]
    pub REG_Q4_WORD1: crate::RWRegister<u32>,
    #[doc = "a"]
    pub REG_Q5_WORD0: crate::RWRegister<u32>,
    #[doc = "a"]
    pub REG_Q5_WORD1: crate::RWRegister<u32>,
    #[doc = "a"]
    pub REG_Q6_WORD0: crate::RWRegister<u32>,
    #[doc = "a"]
    pub REG_Q6_WORD1: crate::RWRegister<u32>,
    #[doc = "a"]
    pub ESC_CONF0: crate::RWRegister<u32>,
    #[doc = "a"]
    pub ESC_CONF1: crate::RWRegister<u32>,
    #[doc = "a"]
    pub ESC_CONF2: crate::RWRegister<u32>,
    #[doc = "a"]
    pub ESC_CONF3: crate::RWRegister<u32>,
    #[doc = "a"]
    pub PKT_THRES: crate::RWRegister<u32>,
    #[doc = "a"]
    pub DATE: crate::RWRegister<u32>,
}
#[doc = "a"]
pub mod CONF0 {
    #[doc = "Write 1, then write 0 to this bit to reset decode state machine."]
    pub mod TX_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write 1, then write 0 to this bit to reset encode state machine."]
    pub mod RX_RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to link up HCI and UART0."]
    pub mod UART0_CE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to link up HCI and UART1."]
    pub mod UART1_CE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to separate the data frame using a special char."]
    pub mod SEPER_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to encode the data packet with a formatting header."]
    pub mod HEAD_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable UHCI to receive the 16 bit CRC."]
    pub mod CRC_REC_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If this bit is set to 1, UHCI will end the payload receiving process when UART has been in idle state."]
    pub mod UART_IDLE_EOF_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If this bit is set to 1, UHCI decoder receiving payload data is end when the receiving byte count has reached the specified value. The value is payload length indicated by UHCI packet header when UHCI_HEAD_EN is 1 or the value is configuration value when UHCI_HEAD_EN is 0. If this bit is set to 0, UHCI decoder receiving payload data is end when 0xc0 is received."]
    pub mod LEN_EOF_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to enable data integrity checking by appending a 16 bit CCITT-CRC to end of the payload."]
    pub mod ENCODE_CRC_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1'b1: Force clock on for register. 1'b0: Support clock only when application writes registers."]
    pub mod CLK_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If this bit is set to 1, UHCI will end payload receive process when NULL frame is received by UART."]
    pub mod UART_RX_BRK_EOF_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod INT_RAW {
    #[doc = "a"]
    pub mod RX_START_INT_RAW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod TX_START_INT_RAW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod RX_HUNG_INT_RAW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod TX_HUNG_INT_RAW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod SEND_S_REG_Q_INT_RAW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod SEND_A_REG_Q_INT_RAW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is the interrupt raw bit. Triggered when there are some errors in EOF in the"]
    pub mod OUT_EOF_INT_RAW {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Soft control int raw bit."]
    pub mod APP_CTRL0_INT_RAW {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Soft control int raw bit."]
    pub mod APP_CTRL1_INT_RAW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod INT_ST {
    #[doc = "a"]
    pub mod RX_START_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod TX_START_INT_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod RX_HUNG_INT_ST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod TX_HUNG_INT_ST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod SEND_S_REG_Q_INT_ST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod SEND_A_REG_Q_INT_ST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod OUTLINK_EOF_ERR_INT_ST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod APP_CTRL0_INT_ST {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod APP_CTRL1_INT_ST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod INT_ENA {
    #[doc = "a"]
    pub mod RX_START_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod TX_START_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod RX_HUNG_INT_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod TX_HUNG_INT_ENA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod SEND_S_REG_Q_INT_ENA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod SEND_A_REG_Q_INT_ENA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod OUTLINK_EOF_ERR_INT_ENA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod APP_CTRL0_INT_ENA {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod APP_CTRL1_INT_ENA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod INT_CLR {
    #[doc = "a"]
    pub mod RX_START_INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod TX_START_INT_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod RX_HUNG_INT_CLR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod TX_HUNG_INT_CLR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod SEND_S_REG_Q_INT_CLR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod SEND_A_REG_Q_INT_CLR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod OUTLINK_EOF_ERR_INT_CLR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod APP_CTRL0_INT_CLR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod APP_CTRL1_INT_CLR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod CONF1 {
    #[doc = "a"]
    pub mod CHECK_SUM_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod CHECK_SEQ_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod CRC_DISABLE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod SAVE_HEAD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod TX_CHECK_SUM_RE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod TX_ACK_NUM_RE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod WAIT_SW_START {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod SW_START {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod STATE0 {
    #[doc = "a"]
    pub mod RX_ERR_CAUSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod DECODE_STATE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod STATE1 {
    #[doc = "a"]
    pub mod ENCODE_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod ESCAPE_CONF {
    #[doc = "a"]
    pub mod TX_C0_ESC_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod TX_DB_ESC_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod TX_11_ESC_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod TX_13_ESC_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod RX_C0_ESC_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod RX_DB_ESC_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod RX_11_ESC_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod RX_13_ESC_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod HUNG_CONF {
    #[doc = "a"]
    pub mod TXFIFO_TIMEOUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod TXFIFO_TIMEOUT_SHIFT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod TXFIFO_TIMEOUT_ENA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod RXFIFO_TIMEOUT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod RXFIFO_TIMEOUT_SHIFT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod RXFIFO_TIMEOUT_ENA {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod ACK_NUM {
    #[doc = "a"]
    pub mod ACK_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod LOAD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod RX_HEAD {
    #[doc = "a"]
    pub mod RX_HEAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod QUICK_SENT {
    #[doc = "a"]
    pub mod SINGLE_SEND_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod SINGLE_SEND_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod ALWAYS_SEND_NUM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod ALWAYS_SEND_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod REG_Q0_WORD0 {
    #[doc = "a"]
    pub mod SEND_Q0_WORD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod REG_Q0_WORD1 {
    #[doc = "a"]
    pub mod SEND_Q0_WORD1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod REG_Q1_WORD0 {
    #[doc = "a"]
    pub mod SEND_Q1_WORD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod REG_Q1_WORD1 {
    #[doc = "a"]
    pub mod SEND_Q1_WORD1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod REG_Q2_WORD0 {
    #[doc = "a"]
    pub mod SEND_Q2_WORD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod REG_Q2_WORD1 {
    #[doc = "a"]
    pub mod SEND_Q2_WORD1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod REG_Q3_WORD0 {
    #[doc = "a"]
    pub mod SEND_Q3_WORD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod REG_Q3_WORD1 {
    #[doc = "a"]
    pub mod SEND_Q3_WORD1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod REG_Q4_WORD0 {
    #[doc = "a"]
    pub mod SEND_Q4_WORD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod REG_Q4_WORD1 {
    #[doc = "a"]
    pub mod SEND_Q4_WORD1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod REG_Q5_WORD0 {
    #[doc = "a"]
    pub mod SEND_Q5_WORD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod REG_Q5_WORD1 {
    #[doc = "a"]
    pub mod SEND_Q5_WORD1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod REG_Q6_WORD0 {
    #[doc = "a"]
    pub mod SEND_Q6_WORD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod REG_Q6_WORD1 {
    #[doc = "a"]
    pub mod SEND_Q6_WORD1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod ESC_CONF0 {
    #[doc = "a"]
    pub mod SEPER_CHAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod SEPER_ESC_CHAR0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod SEPER_ESC_CHAR1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod ESC_CONF1 {
    #[doc = "a"]
    pub mod ESC_SEQ0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod ESC_SEQ0_CHAR0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod ESC_SEQ0_CHAR1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod ESC_CONF2 {
    #[doc = "a"]
    pub mod ESC_SEQ1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod ESC_SEQ1_CHAR0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod ESC_SEQ1_CHAR1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod ESC_CONF3 {
    #[doc = "a"]
    pub mod ESC_SEQ2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod ESC_SEQ2_CHAR0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "a"]
    pub mod ESC_SEQ2_CHAR1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod PKT_THRES {
    #[doc = "a"]
    pub mod PKT_THRS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "a"]
pub mod DATE {
    #[doc = "a"]
    pub mod DATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
