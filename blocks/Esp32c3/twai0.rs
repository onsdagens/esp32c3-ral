#[doc = "Two-Wire Automotive Interface"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Mode Register"]
    pub MODE: crate::RWRegister<u32>,
    #[doc = "Command Register"]
    pub CMD: crate::RWRegister<u32>,
    #[doc = "Status register"]
    pub STATUS: crate::RWRegister<u32>,
    #[doc = "Interrupt Register"]
    pub INT_RAW: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable Register"]
    pub INT_ENA: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Bus Timing Register 0"]
    pub BUS_TIMING_0: crate::RWRegister<u32>,
    #[doc = "Bus Timing Register 1"]
    pub BUS_TIMING_1: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "Arbitration Lost Capture Register"]
    pub ARB_LOST_CAP: crate::RWRegister<u32>,
    #[doc = "Error Code Capture Register"]
    pub ERR_CODE_CAP: crate::RWRegister<u32>,
    #[doc = "Error Warning Limit Register"]
    pub ERR_WARNING_LIMIT: crate::RWRegister<u32>,
    #[doc = "Receive Error Counter Register"]
    pub RX_ERR_CNT: crate::RWRegister<u32>,
    #[doc = "Transmit Error Counter Register"]
    pub TX_ERR_CNT: crate::RWRegister<u32>,
    #[doc = "Data register 0"]
    pub DATA_0: crate::RWRegister<u32>,
    #[doc = "Data register 1"]
    pub DATA_1: crate::RWRegister<u32>,
    #[doc = "Data register 2"]
    pub DATA_2: crate::RWRegister<u32>,
    #[doc = "Data register 3"]
    pub DATA_3: crate::RWRegister<u32>,
    #[doc = "Data register 4"]
    pub DATA_4: crate::RWRegister<u32>,
    #[doc = "Data register 5"]
    pub DATA_5: crate::RWRegister<u32>,
    #[doc = "Data register 6"]
    pub DATA_6: crate::RWRegister<u32>,
    #[doc = "Data register 7"]
    pub DATA_7: crate::RWRegister<u32>,
    #[doc = "Data register 8"]
    pub DATA_8: crate::RWRegister<u32>,
    #[doc = "Data register 9"]
    pub DATA_9: crate::RWRegister<u32>,
    #[doc = "Data register 10"]
    pub DATA_10: crate::RWRegister<u32>,
    #[doc = "Data register 11"]
    pub DATA_11: crate::RWRegister<u32>,
    #[doc = "Data register 12"]
    pub DATA_12: crate::RWRegister<u32>,
    #[doc = "Receive Message Counter Register"]
    pub RX_MESSAGE_CNT: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "Clock Divider register"]
    pub CLOCK_DIVIDER: crate::RWRegister<u32>,
}
#[doc = "Mode Register"]
pub mod MODE {
    #[doc = "This bit is used to configure the operating mode of the TWAI Controller. 1: Reset mode; 0: Operating mode."]
    pub mod RESET_MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Listen only mode. In this mode the nodes will only receive messages from the bus, without generating the acknowledge signal nor updating the RX error counter."]
    pub mod LISTEN_ONLY_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Self test mode. In this mode the TX nodes can perform a successful transmission without receiving the acknowledge signal. This mode is often used to test a single node with the self reception request command."]
    pub mod SELF_TEST_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is used to configure the filter mode. 0: Dual filter mode; 1: Single filter mode."]
    pub mod RX_FILTER_MODE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Register"]
pub mod CMD {
    #[doc = "Set the bit to 1 to allow the driving nodes start transmission."]
    pub mod TX_REQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set the bit to 1 to cancel a pending transmission request."]
    pub mod ABORT_TX {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set the bit to 1 to release the RX buffer."]
    pub mod RELEASE_BUF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set the bit to 1 to clear the data overrun status bit."]
    pub mod CLR_OVERRUN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Self reception request command. Set the bit to 1 to allow a message be transmitted and received simultaneously."]
    pub mod SELF_RX_REQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status register"]
pub mod STATUS {
    #[doc = "1: The data in the RX buffer is not empty, with at least one received data packet."]
    pub mod RX_BUF_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: The RX FIFO is full and data overrun has occurred."]
    pub mod OVERRUN_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: The TX buffer is empty, the CPU may write a message into it."]
    pub mod TX_BUF_ST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: The TWAI controller has successfully received a packet from the bus."]
    pub mod TX_COMPLETE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: The TWAI Controller is receiving a message from the bus."]
    pub mod RX_ST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: The TWAI Controller is transmitting a message to the bus."]
    pub mod TX_ST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: At least one of the RX/TX error counter has reached or exceeded the value set in register TWAI_ERR_WARNING_LIMIT_REG."]
    pub mod ERR_ST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: In bus-off status, the TWAI Controller is no longer involved in bus activities."]
    pub mod BUS_OFF_ST {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit reflects whether the data packet in the RX FIFO is complete. 1: The current packet is missing; 0: The current packet is complete"]
    pub mod MISS_ST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Register"]
pub mod INT_RAW {
    #[doc = "Receive interrupt. If this bit is set to 1, it indicates there are messages to be handled in the RX FIFO."]
    pub mod RX_INT_ST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit interrupt. If this bit is set to 1, it indicates the message transmitting mis- sion is finished and a new transmission is able to execute."]
    pub mod TX_INT_ST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error warning interrupt. If this bit is set to 1, it indicates the error status signal and the bus-off status signal of Status register have changed (e.g., switched from 0 to 1 or from 1 to 0)."]
    pub mod ERR_WARN_INT_ST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data overrun interrupt. If this bit is set to 1, it indicates a data overrun interrupt is generated in the RX FIFO."]
    pub mod OVERRUN_INT_ST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error passive interrupt. If this bit is set to 1, it indicates the TWAI Controller is switched between error active status and error passive status due to the change of error counters."]
    pub mod ERR_PASSIVE_INT_ST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Arbitration lost interrupt. If this bit is set to 1, it indicates an arbitration lost interrupt is generated."]
    pub mod ARB_LOST_INT_ST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error interrupt. If this bit is set to 1, it indicates an error is detected on the bus."]
    pub mod BUS_ERR_INT_ST {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable Register"]
pub mod INT_ENA {
    #[doc = "Set this bit to 1 to enable receive interrupt."]
    pub mod RX_INT_ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable transmit interrupt."]
    pub mod TX_INT_ENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable error warning interrupt."]
    pub mod ERR_WARN_INT_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable data overrun interrupt."]
    pub mod OVERRUN_INT_ENA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable error passive interrupt."]
    pub mod ERR_PASSIVE_INT_ENA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable arbitration lost interrupt."]
    pub mod ARB_LOST_INT_ENA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Set this bit to 1 to enable error interrupt."]
    pub mod BUS_ERR_INT_ENA {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bus Timing Register 0"]
pub mod BUS_TIMING_0 {
    #[doc = "Baud Rate Prescaler, determines the frequency dividing ratio."]
    pub mod BAUD_PRESC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Synchronization Jump Width (SJW), 1 \\verb+~+ 14 Tq wide."]
    pub mod SYNC_JUMP_WIDTH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bus Timing Register 1"]
pub mod BUS_TIMING_1 {
    #[doc = "The width of PBS1."]
    pub mod TIME_SEG1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The width of PBS2."]
    pub mod TIME_SEG2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The number of sample points. 0: the bus is sampled once; 1: the bus is sampled three times"]
    pub mod TIME_SAMP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Arbitration Lost Capture Register"]
pub mod ARB_LOST_CAP {
    #[doc = "This register contains information about the bit position of lost arbitration."]
    pub mod ARB_LOST_CAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Code Capture Register"]
pub mod ERR_CODE_CAP {
    #[doc = "This register contains information about the location of errors, see Table 181 for details."]
    pub mod ECC_SEGMENT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register contains information about transmission direction of the node when error occurs. 1: Error occurs when receiving a message; 0: Error occurs when transmitting a message"]
    pub mod ECC_DIRECTION {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register contains information about error types: 00: bit error; 01: form error; 10: stuff error; 11: other type of error"]
    pub mod ECC_TYPE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Warning Limit Register"]
pub mod ERR_WARNING_LIMIT {
    #[doc = "Error warning threshold. In the case when any of a error counter value exceeds the threshold, or all the error counter values are below the threshold, an error warning interrupt will be triggered (given the enable signal is valid)."]
    pub mod ERR_WARNING_LIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Error Counter Register"]
pub mod RX_ERR_CNT {
    #[doc = "The RX error counter register, reflects value changes under reception status."]
    pub mod RX_ERR_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Error Counter Register"]
pub mod TX_ERR_CNT {
    #[doc = "The TX error counter register, reflects value changes under transmission status."]
    pub mod TX_ERR_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data register 0"]
pub mod DATA_0 {
    #[doc = "In reset mode, it is acceptance code register 0 with R/W Permission. In operation mode, it stores the 0th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    pub mod TX_BYTE_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data register 1"]
pub mod DATA_1 {
    #[doc = "In reset mode, it is acceptance code register 1 with R/W Permission. In operation mode, it stores the 1st byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    pub mod TX_BYTE_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data register 2"]
pub mod DATA_2 {
    #[doc = "In reset mode, it is acceptance code register 2 with R/W Permission. In operation mode, it stores the 2nd byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    pub mod TX_BYTE_2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data register 3"]
pub mod DATA_3 {
    #[doc = "In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, it stores the 3rd byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    pub mod TX_BYTE_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data register 4"]
pub mod DATA_4 {
    #[doc = "In reset mode, it is acceptance code register 4 with R/W Permission. In operation mode, it stores the 4th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    pub mod TX_BYTE_4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data register 5"]
pub mod DATA_5 {
    #[doc = "In reset mode, it is acceptance code register 5 with R/W Permission. In operation mode, it stores the 5th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    pub mod TX_BYTE_5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data register 6"]
pub mod DATA_6 {
    #[doc = "In reset mode, it is acceptance code register 6 with R/W Permission. In operation mode, it stores the 6th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    pub mod TX_BYTE_6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data register 7"]
pub mod DATA_7 {
    #[doc = "In reset mode, it is acceptance code register 7 with R/W Permission. In operation mode, it stores the 7th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    pub mod TX_BYTE_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data register 8"]
pub mod DATA_8 {
    #[doc = "In operation mode, it stores the 8th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    pub mod TX_BYTE_8 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data register 9"]
pub mod DATA_9 {
    #[doc = "In operation mode, it stores the 9th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    pub mod TX_BYTE_9 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data register 10"]
pub mod DATA_10 {
    #[doc = "In operation mode, it stores the 10th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    pub mod TX_BYTE_10 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data register 11"]
pub mod DATA_11 {
    #[doc = "In operation mode, it stores the 11th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    pub mod TX_BYTE_11 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data register 12"]
pub mod DATA_12 {
    #[doc = "In operation mode, it stores the 12th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    pub mod TX_BYTE_12 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Message Counter Register"]
pub mod RX_MESSAGE_CNT {
    #[doc = "This register reflects the number of messages available within the RX FIFO."]
    pub mod RX_MESSAGE_COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock Divider register"]
pub mod CLOCK_DIVIDER {
    #[doc = "These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
    pub mod CD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
    pub mod CLOCK_OFF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
