#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B1 Control Register 1"]
    pub ucb1ctl1: UCB1CTL1,
    #[doc = "0x01 - USCI B1 Control Register 0"]
    pub ucb1ctl0: UCB1CTL0,
    #[doc = "0x02 - USCI B1 Control Word Register 1"]
    pub ucb1ctlw1: UCB1CTLW1,
    _reserved3: [u8; 2usize],
    #[doc = "0x06 - USCI B1 Baud Rate 0"]
    pub ucb1br0: UCB1BR0,
    #[doc = "0x07 - USCI B1 Baud Rate 1"]
    pub ucb1br1: UCB1BR1,
    #[doc = "0x08 - USCI B1 Status Register"]
    pub ucb1stat_i2c: UCB1STAT_I2C,
    #[doc = "0x09 - USCI B1 Byte Counter Register"]
    pub ucb1bcnt_i2c: UCB1BCNT_I2C,
    #[doc = "0x0a - USCI B1 Byte Counter Threshold Register"]
    pub ucb1tbcnt: UCB1TBCNT,
    #[doc = "0x0c - USCI B1 Receive Buffer"]
    pub ucb1rxbuf: UCB1RXBUF,
    #[doc = "0x0e - USCI B1 Transmit Buffer"]
    pub ucb1txbuf: UCB1TXBUF,
    _reserved10: [u8; 4usize],
    #[doc = "0x14 - USCI B1 I2C Own Address 0"]
    pub ucb1i2coa0: UCB1I2COA0,
    #[doc = "0x16 - USCI B1 I2C Own Address 1"]
    pub ucb1i2coa1: UCB1I2COA1,
    #[doc = "0x18 - USCI B1 I2C Own Address 2"]
    pub ucb1i2coa2: UCB1I2COA2,
    #[doc = "0x1a - USCI B1 I2C Own Address 3"]
    pub ucb1i2coa3: UCB1I2COA3,
    #[doc = "0x1c - USCI B1 Received Address Register"]
    pub ucb1addrx: UCB1ADDRX,
    #[doc = "0x1e - USCI B1 Address Mask Register"]
    pub ucb1addmask: UCB1ADDMASK,
    #[doc = "0x20 - USCI B1 I2C Slave Address"]
    pub ucb1i2csa: UCB1I2CSA,
    _reserved17: [u8; 8usize],
    _reserved_17_ucb1: [u8; 2usize],
    _reserved_18_ucb1: [u8; 2usize],
    #[doc = "0x2e - USCI B1 Interrupt Vector Register"]
    pub ucb1iv: UCB1IV,
}
impl RegisterBlock {
    #[doc = "0x2a - USCI B1 Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb1ie_i2c(&self) -> &UCB1IE_I2C {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB1IE_I2C) }
    }
    #[doc = "0x2a - USCI B1 Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb1ie_i2c_mut(&self) -> &mut UCB1IE_I2C {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut UCB1IE_I2C) }
    }
    #[doc = "0x2a - USCI B1 Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb1ie(&self) -> &UCB1IE {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB1IE) }
    }
    #[doc = "0x2a - USCI B1 Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb1ie_mut(&self) -> &mut UCB1IE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut UCB1IE) }
    }
    #[doc = "0x2c - USCI B1 Interrupt Flags Register"]
    #[inline(always)]
    pub fn ucb1ifg_i2c(&self) -> &UCB1IFG_I2C {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB1IFG_I2C) }
    }
    #[doc = "0x2c - USCI B1 Interrupt Flags Register"]
    #[inline(always)]
    pub fn ucb1ifg_i2c_mut(&self) -> &mut UCB1IFG_I2C {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut UCB1IFG_I2C) }
    }
    #[doc = "0x2c - USCI B1 Interrupt Flags Register"]
    #[inline(always)]
    pub fn ucb1ifg(&self) -> &UCB1IFG {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB1IFG) }
    }
    #[doc = "0x2c - USCI B1 Interrupt Flags Register"]
    #[inline(always)]
    pub fn ucb1ifg_mut(&self) -> &mut UCB1IFG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut UCB1IFG) }
    }
}
#[doc = "USCI B1 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ctl1](ucb1ctl1) module"]
pub type UCB1CTL1 = crate::Reg<u8, _UCB1CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1CTL1;
#[doc = "`read()` method returns [ucb1ctl1::R](ucb1ctl1::R) reader structure"]
impl crate::Readable for UCB1CTL1 {}
#[doc = "`write(|w| ..)` method takes [ucb1ctl1::W](ucb1ctl1::W) writer structure"]
impl crate::Writable for UCB1CTL1 {}
#[doc = "USCI B1 Control Register 1"]
pub mod ucb1ctl1;
#[doc = "USCI B1 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ctl0](ucb1ctl0) module"]
pub type UCB1CTL0 = crate::Reg<u8, _UCB1CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1CTL0;
#[doc = "`read()` method returns [ucb1ctl0::R](ucb1ctl0::R) reader structure"]
impl crate::Readable for UCB1CTL0 {}
#[doc = "`write(|w| ..)` method takes [ucb1ctl0::W](ucb1ctl0::W) writer structure"]
impl crate::Writable for UCB1CTL0 {}
#[doc = "USCI B1 Control Register 0"]
pub mod ucb1ctl0;
#[doc = "USCI B1 Baud Rate 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1br0](ucb1br0) module"]
pub type UCB1BR0 = crate::Reg<u8, _UCB1BR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1BR0;
#[doc = "`read()` method returns [ucb1br0::R](ucb1br0::R) reader structure"]
impl crate::Readable for UCB1BR0 {}
#[doc = "`write(|w| ..)` method takes [ucb1br0::W](ucb1br0::W) writer structure"]
impl crate::Writable for UCB1BR0 {}
#[doc = "USCI B1 Baud Rate 0"]
pub mod ucb1br0;
#[doc = "USCI B1 Baud Rate 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1br1](ucb1br1) module"]
pub type UCB1BR1 = crate::Reg<u8, _UCB1BR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1BR1;
#[doc = "`read()` method returns [ucb1br1::R](ucb1br1::R) reader structure"]
impl crate::Readable for UCB1BR1 {}
#[doc = "`write(|w| ..)` method takes [ucb1br1::W](ucb1br1::W) writer structure"]
impl crate::Writable for UCB1BR1 {}
#[doc = "USCI B1 Baud Rate 1"]
pub mod ucb1br1;
#[doc = "USCI B1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1stat_i2c](ucb1stat_i2c) module"]
pub type UCB1STAT_I2C = crate::Reg<u8, _UCB1STAT_I2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1STAT_I2C;
#[doc = "`read()` method returns [ucb1stat_i2c::R](ucb1stat_i2c::R) reader structure"]
impl crate::Readable for UCB1STAT_I2C {}
#[doc = "`write(|w| ..)` method takes [ucb1stat_i2c::W](ucb1stat_i2c::W) writer structure"]
impl crate::Writable for UCB1STAT_I2C {}
#[doc = "USCI B1 Status Register"]
pub mod ucb1stat_i2c;
#[doc = "USCI B1 Byte Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1bcnt_i2c](ucb1bcnt_i2c) module"]
pub type UCB1BCNT_I2C = crate::Reg<u8, _UCB1BCNT_I2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1BCNT_I2C;
#[doc = "`read()` method returns [ucb1bcnt_i2c::R](ucb1bcnt_i2c::R) reader structure"]
impl crate::Readable for UCB1BCNT_I2C {}
#[doc = "`write(|w| ..)` method takes [ucb1bcnt_i2c::W](ucb1bcnt_i2c::W) writer structure"]
impl crate::Writable for UCB1BCNT_I2C {}
#[doc = "USCI B1 Byte Counter Register"]
pub mod ucb1bcnt_i2c;
#[doc = "USCI B1 Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ctlw1](ucb1ctlw1) module"]
pub type UCB1CTLW1 = crate::Reg<u16, _UCB1CTLW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1CTLW1;
#[doc = "`read()` method returns [ucb1ctlw1::R](ucb1ctlw1::R) reader structure"]
impl crate::Readable for UCB1CTLW1 {}
#[doc = "`write(|w| ..)` method takes [ucb1ctlw1::W](ucb1ctlw1::W) writer structure"]
impl crate::Writable for UCB1CTLW1 {}
#[doc = "USCI B1 Control Word Register 1"]
pub mod ucb1ctlw1;
#[doc = "USCI B1 Byte Counter Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1tbcnt](ucb1tbcnt) module"]
pub type UCB1TBCNT = crate::Reg<u16, _UCB1TBCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1TBCNT;
#[doc = "`read()` method returns [ucb1tbcnt::R](ucb1tbcnt::R) reader structure"]
impl crate::Readable for UCB1TBCNT {}
#[doc = "`write(|w| ..)` method takes [ucb1tbcnt::W](ucb1tbcnt::W) writer structure"]
impl crate::Writable for UCB1TBCNT {}
#[doc = "USCI B1 Byte Counter Threshold Register"]
pub mod ucb1tbcnt;
#[doc = "USCI B1 Receive Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1rxbuf](ucb1rxbuf) module"]
pub type UCB1RXBUF = crate::Reg<u16, _UCB1RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1RXBUF;
#[doc = "`read()` method returns [ucb1rxbuf::R](ucb1rxbuf::R) reader structure"]
impl crate::Readable for UCB1RXBUF {}
#[doc = "`write(|w| ..)` method takes [ucb1rxbuf::W](ucb1rxbuf::W) writer structure"]
impl crate::Writable for UCB1RXBUF {}
#[doc = "USCI B1 Receive Buffer"]
pub mod ucb1rxbuf;
#[doc = "USCI B1 Transmit Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1txbuf](ucb1txbuf) module"]
pub type UCB1TXBUF = crate::Reg<u16, _UCB1TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1TXBUF;
#[doc = "`read()` method returns [ucb1txbuf::R](ucb1txbuf::R) reader structure"]
impl crate::Readable for UCB1TXBUF {}
#[doc = "`write(|w| ..)` method takes [ucb1txbuf::W](ucb1txbuf::W) writer structure"]
impl crate::Writable for UCB1TXBUF {}
#[doc = "USCI B1 Transmit Buffer"]
pub mod ucb1txbuf;
#[doc = "USCI B1 I2C Own Address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1i2coa0](ucb1i2coa0) module"]
pub type UCB1I2COA0 = crate::Reg<u16, _UCB1I2COA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1I2COA0;
#[doc = "`read()` method returns [ucb1i2coa0::R](ucb1i2coa0::R) reader structure"]
impl crate::Readable for UCB1I2COA0 {}
#[doc = "`write(|w| ..)` method takes [ucb1i2coa0::W](ucb1i2coa0::W) writer structure"]
impl crate::Writable for UCB1I2COA0 {}
#[doc = "USCI B1 I2C Own Address 0"]
pub mod ucb1i2coa0;
#[doc = "USCI B1 I2C Own Address 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1i2coa1](ucb1i2coa1) module"]
pub type UCB1I2COA1 = crate::Reg<u16, _UCB1I2COA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1I2COA1;
#[doc = "`read()` method returns [ucb1i2coa1::R](ucb1i2coa1::R) reader structure"]
impl crate::Readable for UCB1I2COA1 {}
#[doc = "`write(|w| ..)` method takes [ucb1i2coa1::W](ucb1i2coa1::W) writer structure"]
impl crate::Writable for UCB1I2COA1 {}
#[doc = "USCI B1 I2C Own Address 1"]
pub mod ucb1i2coa1;
#[doc = "USCI B1 I2C Own Address 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1i2coa2](ucb1i2coa2) module"]
pub type UCB1I2COA2 = crate::Reg<u16, _UCB1I2COA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1I2COA2;
#[doc = "`read()` method returns [ucb1i2coa2::R](ucb1i2coa2::R) reader structure"]
impl crate::Readable for UCB1I2COA2 {}
#[doc = "`write(|w| ..)` method takes [ucb1i2coa2::W](ucb1i2coa2::W) writer structure"]
impl crate::Writable for UCB1I2COA2 {}
#[doc = "USCI B1 I2C Own Address 2"]
pub mod ucb1i2coa2;
#[doc = "USCI B1 I2C Own Address 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1i2coa3](ucb1i2coa3) module"]
pub type UCB1I2COA3 = crate::Reg<u16, _UCB1I2COA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1I2COA3;
#[doc = "`read()` method returns [ucb1i2coa3::R](ucb1i2coa3::R) reader structure"]
impl crate::Readable for UCB1I2COA3 {}
#[doc = "`write(|w| ..)` method takes [ucb1i2coa3::W](ucb1i2coa3::W) writer structure"]
impl crate::Writable for UCB1I2COA3 {}
#[doc = "USCI B1 I2C Own Address 3"]
pub mod ucb1i2coa3;
#[doc = "USCI B1 Received Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1addrx](ucb1addrx) module"]
pub type UCB1ADDRX = crate::Reg<u16, _UCB1ADDRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1ADDRX;
#[doc = "`read()` method returns [ucb1addrx::R](ucb1addrx::R) reader structure"]
impl crate::Readable for UCB1ADDRX {}
#[doc = "`write(|w| ..)` method takes [ucb1addrx::W](ucb1addrx::W) writer structure"]
impl crate::Writable for UCB1ADDRX {}
#[doc = "USCI B1 Received Address Register"]
pub mod ucb1addrx;
#[doc = "USCI B1 Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1addmask](ucb1addmask) module"]
pub type UCB1ADDMASK = crate::Reg<u16, _UCB1ADDMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1ADDMASK;
#[doc = "`read()` method returns [ucb1addmask::R](ucb1addmask::R) reader structure"]
impl crate::Readable for UCB1ADDMASK {}
#[doc = "`write(|w| ..)` method takes [ucb1addmask::W](ucb1addmask::W) writer structure"]
impl crate::Writable for UCB1ADDMASK {}
#[doc = "USCI B1 Address Mask Register"]
pub mod ucb1addmask;
#[doc = "USCI B1 I2C Slave Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1i2csa](ucb1i2csa) module"]
pub type UCB1I2CSA = crate::Reg<u16, _UCB1I2CSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1I2CSA;
#[doc = "`read()` method returns [ucb1i2csa::R](ucb1i2csa::R) reader structure"]
impl crate::Readable for UCB1I2CSA {}
#[doc = "`write(|w| ..)` method takes [ucb1i2csa::W](ucb1i2csa::W) writer structure"]
impl crate::Writable for UCB1I2CSA {}
#[doc = "USCI B1 I2C Slave Address"]
pub mod ucb1i2csa;
#[doc = "USCI B1 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ie](ucb1ie) module"]
pub type UCB1IE = crate::Reg<u16, _UCB1IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IE;
#[doc = "`read()` method returns [ucb1ie::R](ucb1ie::R) reader structure"]
impl crate::Readable for UCB1IE {}
#[doc = "`write(|w| ..)` method takes [ucb1ie::W](ucb1ie::W) writer structure"]
impl crate::Writable for UCB1IE {}
#[doc = "USCI B1 Interrupt Enable Register"]
pub mod ucb1ie;
#[doc = "USCI B1 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ie_i2c](ucb1ie_i2c) module"]
pub type UCB1IE_I2C = crate::Reg<u16, _UCB1IE_I2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IE_I2C;
#[doc = "`read()` method returns [ucb1ie_i2c::R](ucb1ie_i2c::R) reader structure"]
impl crate::Readable for UCB1IE_I2C {}
#[doc = "`write(|w| ..)` method takes [ucb1ie_i2c::W](ucb1ie_i2c::W) writer structure"]
impl crate::Writable for UCB1IE_I2C {}
#[doc = "USCI B1 Interrupt Enable Register"]
pub mod ucb1ie_i2c;
#[doc = "USCI B1 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ifg](ucb1ifg) module"]
pub type UCB1IFG = crate::Reg<u16, _UCB1IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IFG;
#[doc = "`read()` method returns [ucb1ifg::R](ucb1ifg::R) reader structure"]
impl crate::Readable for UCB1IFG {}
#[doc = "`write(|w| ..)` method takes [ucb1ifg::W](ucb1ifg::W) writer structure"]
impl crate::Writable for UCB1IFG {}
#[doc = "USCI B1 Interrupt Flags Register"]
pub mod ucb1ifg;
#[doc = "USCI B1 Interrupt Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ifg_i2c](ucb1ifg_i2c) module"]
pub type UCB1IFG_I2C = crate::Reg<u16, _UCB1IFG_I2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IFG_I2C;
#[doc = "`read()` method returns [ucb1ifg_i2c::R](ucb1ifg_i2c::R) reader structure"]
impl crate::Readable for UCB1IFG_I2C {}
#[doc = "`write(|w| ..)` method takes [ucb1ifg_i2c::W](ucb1ifg_i2c::W) writer structure"]
impl crate::Writable for UCB1IFG_I2C {}
#[doc = "USCI B1 Interrupt Flags Register"]
pub mod ucb1ifg_i2c;
#[doc = "USCI B1 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1iv](ucb1iv) module"]
pub type UCB1IV = crate::Reg<u16, _UCB1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IV;
#[doc = "`read()` method returns [ucb1iv::R](ucb1iv::R) reader structure"]
impl crate::Readable for UCB1IV {}
#[doc = "`write(|w| ..)` method takes [ucb1iv::W](ucb1iv::W) writer structure"]
impl crate::Writable for UCB1IV {}
#[doc = "USCI B1 Interrupt Vector Register"]
pub mod ucb1iv;
