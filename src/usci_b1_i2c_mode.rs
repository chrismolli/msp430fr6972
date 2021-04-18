#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B1 Control Register 1"]
    pub ucb1ctl1: crate::Reg<ucb1ctl1::UCB1CTL1_SPEC>,
    #[doc = "0x01 - USCI B1 Control Register 0"]
    pub ucb1ctl0: crate::Reg<ucb1ctl0::UCB1CTL0_SPEC>,
    #[doc = "0x02 - USCI B1 Control Word Register 1"]
    pub ucb1ctlw1: crate::Reg<ucb1ctlw1::UCB1CTLW1_SPEC>,
    _reserved3: [u8; 2usize],
    #[doc = "0x06 - USCI B1 Baud Rate 0"]
    pub ucb1br0: crate::Reg<ucb1br0::UCB1BR0_SPEC>,
    #[doc = "0x07 - USCI B1 Baud Rate 1"]
    pub ucb1br1: crate::Reg<ucb1br1::UCB1BR1_SPEC>,
    #[doc = "0x08 - USCI B1 Status Register"]
    pub ucb1stat_i2c: crate::Reg<ucb1stat_i2c::UCB1STAT_I2C_SPEC>,
    #[doc = "0x09 - USCI B1 Byte Counter Register"]
    pub ucb1bcnt_i2c: crate::Reg<ucb1bcnt_i2c::UCB1BCNT_I2C_SPEC>,
    #[doc = "0x0a - USCI B1 Byte Counter Threshold Register"]
    pub ucb1tbcnt: crate::Reg<ucb1tbcnt::UCB1TBCNT_SPEC>,
    #[doc = "0x0c - USCI B1 Receive Buffer"]
    pub ucb1rxbuf: crate::Reg<ucb1rxbuf::UCB1RXBUF_SPEC>,
    #[doc = "0x0e - USCI B1 Transmit Buffer"]
    pub ucb1txbuf: crate::Reg<ucb1txbuf::UCB1TXBUF_SPEC>,
    _reserved10: [u8; 4usize],
    #[doc = "0x14 - USCI B1 I2C Own Address 0"]
    pub ucb1i2coa0: crate::Reg<ucb1i2coa0::UCB1I2COA0_SPEC>,
    #[doc = "0x16 - USCI B1 I2C Own Address 1"]
    pub ucb1i2coa1: crate::Reg<ucb1i2coa1::UCB1I2COA1_SPEC>,
    #[doc = "0x18 - USCI B1 I2C Own Address 2"]
    pub ucb1i2coa2: crate::Reg<ucb1i2coa2::UCB1I2COA2_SPEC>,
    #[doc = "0x1a - USCI B1 I2C Own Address 3"]
    pub ucb1i2coa3: crate::Reg<ucb1i2coa3::UCB1I2COA3_SPEC>,
    #[doc = "0x1c - USCI B1 Received Address Register"]
    pub ucb1addrx: crate::Reg<ucb1addrx::UCB1ADDRX_SPEC>,
    #[doc = "0x1e - USCI B1 Address Mask Register"]
    pub ucb1addmask: crate::Reg<ucb1addmask::UCB1ADDMASK_SPEC>,
    #[doc = "0x20 - USCI B1 I2C Slave Address"]
    pub ucb1i2csa: crate::Reg<ucb1i2csa::UCB1I2CSA_SPEC>,
    _reserved17: [u8; 8usize],
    _reserved_17_ucb1: [u8; 2usize],
    _reserved_18_ucb1: [u8; 2usize],
    #[doc = "0x2e - USCI B1 Interrupt Vector Register"]
    pub ucb1iv: crate::Reg<ucb1iv::UCB1IV_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x2a - USCI B1 Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb1ie_i2c(&self) -> &crate::Reg<ucb1ie_i2c::UCB1IE_I2C_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(42usize)
                as *const crate::Reg<ucb1ie_i2c::UCB1IE_I2C_SPEC>)
        }
    }
    #[doc = "0x2a - USCI B1 Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb1ie(&self) -> &crate::Reg<ucb1ie::UCB1IE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(42usize)
                as *const crate::Reg<ucb1ie::UCB1IE_SPEC>)
        }
    }
    #[doc = "0x2c - USCI B1 Interrupt Flags Register"]
    #[inline(always)]
    pub fn ucb1ifg_i2c(&self) -> &crate::Reg<ucb1ifg_i2c::UCB1IFG_I2C_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(44usize)
                as *const crate::Reg<ucb1ifg_i2c::UCB1IFG_I2C_SPEC>)
        }
    }
    #[doc = "0x2c - USCI B1 Interrupt Flags Register"]
    #[inline(always)]
    pub fn ucb1ifg(&self) -> &crate::Reg<ucb1ifg::UCB1IFG_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(44usize)
                as *const crate::Reg<ucb1ifg::UCB1IFG_SPEC>)
        }
    }
}
#[doc = "UCB1CTL1 register accessor: an alias for `Reg<UCB1CTL1_SPEC>`"]
pub type UCB1CTL1 = crate::Reg<ucb1ctl1::UCB1CTL1_SPEC>;
#[doc = "USCI B1 Control Register 1"]
pub mod ucb1ctl1;
#[doc = "UCB1CTL0 register accessor: an alias for `Reg<UCB1CTL0_SPEC>`"]
pub type UCB1CTL0 = crate::Reg<ucb1ctl0::UCB1CTL0_SPEC>;
#[doc = "USCI B1 Control Register 0"]
pub mod ucb1ctl0;
#[doc = "UCB1BR0 register accessor: an alias for `Reg<UCB1BR0_SPEC>`"]
pub type UCB1BR0 = crate::Reg<ucb1br0::UCB1BR0_SPEC>;
#[doc = "USCI B1 Baud Rate 0"]
pub mod ucb1br0;
#[doc = "UCB1BR1 register accessor: an alias for `Reg<UCB1BR1_SPEC>`"]
pub type UCB1BR1 = crate::Reg<ucb1br1::UCB1BR1_SPEC>;
#[doc = "USCI B1 Baud Rate 1"]
pub mod ucb1br1;
#[doc = "UCB1STAT_I2C register accessor: an alias for `Reg<UCB1STAT_I2C_SPEC>`"]
pub type UCB1STAT_I2C = crate::Reg<ucb1stat_i2c::UCB1STAT_I2C_SPEC>;
#[doc = "USCI B1 Status Register"]
pub mod ucb1stat_i2c;
#[doc = "UCB1BCNT_I2C register accessor: an alias for `Reg<UCB1BCNT_I2C_SPEC>`"]
pub type UCB1BCNT_I2C = crate::Reg<ucb1bcnt_i2c::UCB1BCNT_I2C_SPEC>;
#[doc = "USCI B1 Byte Counter Register"]
pub mod ucb1bcnt_i2c;
#[doc = "UCB1CTLW1 register accessor: an alias for `Reg<UCB1CTLW1_SPEC>`"]
pub type UCB1CTLW1 = crate::Reg<ucb1ctlw1::UCB1CTLW1_SPEC>;
#[doc = "USCI B1 Control Word Register 1"]
pub mod ucb1ctlw1;
#[doc = "UCB1TBCNT register accessor: an alias for `Reg<UCB1TBCNT_SPEC>`"]
pub type UCB1TBCNT = crate::Reg<ucb1tbcnt::UCB1TBCNT_SPEC>;
#[doc = "USCI B1 Byte Counter Threshold Register"]
pub mod ucb1tbcnt;
#[doc = "UCB1RXBUF register accessor: an alias for `Reg<UCB1RXBUF_SPEC>`"]
pub type UCB1RXBUF = crate::Reg<ucb1rxbuf::UCB1RXBUF_SPEC>;
#[doc = "USCI B1 Receive Buffer"]
pub mod ucb1rxbuf;
#[doc = "UCB1TXBUF register accessor: an alias for `Reg<UCB1TXBUF_SPEC>`"]
pub type UCB1TXBUF = crate::Reg<ucb1txbuf::UCB1TXBUF_SPEC>;
#[doc = "USCI B1 Transmit Buffer"]
pub mod ucb1txbuf;
#[doc = "UCB1I2COA0 register accessor: an alias for `Reg<UCB1I2COA0_SPEC>`"]
pub type UCB1I2COA0 = crate::Reg<ucb1i2coa0::UCB1I2COA0_SPEC>;
#[doc = "USCI B1 I2C Own Address 0"]
pub mod ucb1i2coa0;
#[doc = "UCB1I2COA1 register accessor: an alias for `Reg<UCB1I2COA1_SPEC>`"]
pub type UCB1I2COA1 = crate::Reg<ucb1i2coa1::UCB1I2COA1_SPEC>;
#[doc = "USCI B1 I2C Own Address 1"]
pub mod ucb1i2coa1;
#[doc = "UCB1I2COA2 register accessor: an alias for `Reg<UCB1I2COA2_SPEC>`"]
pub type UCB1I2COA2 = crate::Reg<ucb1i2coa2::UCB1I2COA2_SPEC>;
#[doc = "USCI B1 I2C Own Address 2"]
pub mod ucb1i2coa2;
#[doc = "UCB1I2COA3 register accessor: an alias for `Reg<UCB1I2COA3_SPEC>`"]
pub type UCB1I2COA3 = crate::Reg<ucb1i2coa3::UCB1I2COA3_SPEC>;
#[doc = "USCI B1 I2C Own Address 3"]
pub mod ucb1i2coa3;
#[doc = "UCB1ADDRX register accessor: an alias for `Reg<UCB1ADDRX_SPEC>`"]
pub type UCB1ADDRX = crate::Reg<ucb1addrx::UCB1ADDRX_SPEC>;
#[doc = "USCI B1 Received Address Register"]
pub mod ucb1addrx;
#[doc = "UCB1ADDMASK register accessor: an alias for `Reg<UCB1ADDMASK_SPEC>`"]
pub type UCB1ADDMASK = crate::Reg<ucb1addmask::UCB1ADDMASK_SPEC>;
#[doc = "USCI B1 Address Mask Register"]
pub mod ucb1addmask;
#[doc = "UCB1I2CSA register accessor: an alias for `Reg<UCB1I2CSA_SPEC>`"]
pub type UCB1I2CSA = crate::Reg<ucb1i2csa::UCB1I2CSA_SPEC>;
#[doc = "USCI B1 I2C Slave Address"]
pub mod ucb1i2csa;
#[doc = "UCB1IE register accessor: an alias for `Reg<UCB1IE_SPEC>`"]
pub type UCB1IE = crate::Reg<ucb1ie::UCB1IE_SPEC>;
#[doc = "USCI B1 Interrupt Enable Register"]
pub mod ucb1ie;
#[doc = "UCB1IE_I2C register accessor: an alias for `Reg<UCB1IE_I2C_SPEC>`"]
pub type UCB1IE_I2C = crate::Reg<ucb1ie_i2c::UCB1IE_I2C_SPEC>;
#[doc = "USCI B1 Interrupt Enable Register"]
pub mod ucb1ie_i2c;
#[doc = "UCB1IFG register accessor: an alias for `Reg<UCB1IFG_SPEC>`"]
pub type UCB1IFG = crate::Reg<ucb1ifg::UCB1IFG_SPEC>;
#[doc = "USCI B1 Interrupt Flags Register"]
pub mod ucb1ifg;
#[doc = "UCB1IFG_I2C register accessor: an alias for `Reg<UCB1IFG_I2C_SPEC>`"]
pub type UCB1IFG_I2C = crate::Reg<ucb1ifg_i2c::UCB1IFG_I2C_SPEC>;
#[doc = "USCI B1 Interrupt Flags Register"]
pub mod ucb1ifg_i2c;
#[doc = "UCB1IV register accessor: an alias for `Reg<UCB1IV_SPEC>`"]
pub type UCB1IV = crate::Reg<ucb1iv::UCB1IV_SPEC>;
#[doc = "USCI B1 Interrupt Vector Register"]
pub mod ucb1iv;
