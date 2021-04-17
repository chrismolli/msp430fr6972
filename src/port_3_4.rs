#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 3 Input"]
    pub p3in: P3IN,
    #[doc = "0x01 - Port 4 Input"]
    pub p4in: P4IN,
    #[doc = "0x02 - Port 3 Output"]
    pub p3out: P3OUT,
    #[doc = "0x03 - Port 4 Output"]
    pub p4out: P4OUT,
    #[doc = "0x04 - Port 3 Direction"]
    pub p3dir: P3DIR,
    #[doc = "0x05 - Port 4 Direction"]
    pub p4dir: P4DIR,
    #[doc = "0x06 - Port 3 Resistor Enable"]
    pub p3ren: P3REN,
    #[doc = "0x07 - Port 4 Resistor Enable"]
    pub p4ren: P4REN,
    _reserved8: [u8; 2usize],
    #[doc = "0x0a - Port 3 Selection 0"]
    pub p3sel0: P3SEL0,
    #[doc = "0x0b - Port 4 Selection 0"]
    pub p4sel0: P4SEL0,
    #[doc = "0x0c - Port 3 Selection 1"]
    pub p3sel1: P3SEL1,
    #[doc = "0x0d - Port 4 Selection 1"]
    pub p4sel1: P4SEL1,
    #[doc = "0x0e - Port 3 Interrupt Vector Word"]
    pub p3iv: P3IV,
    _reserved13: [u8; 6usize],
    #[doc = "0x16 - Port 3 Complement Selection"]
    pub p3selc: P3SELC,
    #[doc = "0x17 - Port 4 Complement Selection"]
    pub p4selc: P4SELC,
    #[doc = "0x18 - Port 3 Interrupt Edge Select"]
    pub p3ies: P3IES,
    #[doc = "0x19 - Port 4 Interrupt Edge Select"]
    pub p4ies: P4IES,
    #[doc = "0x1a - Port 3 Interrupt Enable"]
    pub p3ie: P3IE,
    #[doc = "0x1b - Port 4 Interrupt Enable"]
    pub p4ie: P4IE,
    #[doc = "0x1c - Port 3 Interrupt Flag"]
    pub p3ifg: P3IFG,
    #[doc = "0x1d - Port 4 Interrupt Flag"]
    pub p4ifg: P4IFG,
    #[doc = "0x1e - Port 4 Interrupt Vector Word"]
    pub p4iv: P4IV,
}
#[doc = "Port 3 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3in](p3in) module"]
pub type P3IN = crate::Reg<u8, _P3IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3IN;
#[doc = "`read()` method returns [p3in::R](p3in::R) reader structure"]
impl crate::Readable for P3IN {}
#[doc = "`write(|w| ..)` method takes [p3in::W](p3in::W) writer structure"]
impl crate::Writable for P3IN {}
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "Port 4 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4in](p4in) module"]
pub type P4IN = crate::Reg<u8, _P4IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4IN;
#[doc = "`read()` method returns [p4in::R](p4in::R) reader structure"]
impl crate::Readable for P4IN {}
#[doc = "`write(|w| ..)` method takes [p4in::W](p4in::W) writer structure"]
impl crate::Writable for P4IN {}
#[doc = "Port 4 Input"]
pub mod p4in;
#[doc = "Port 3 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3out](p3out) module"]
pub type P3OUT = crate::Reg<u8, _P3OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3OUT;
#[doc = "`read()` method returns [p3out::R](p3out::R) reader structure"]
impl crate::Readable for P3OUT {}
#[doc = "`write(|w| ..)` method takes [p3out::W](p3out::W) writer structure"]
impl crate::Writable for P3OUT {}
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "Port 4 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4out](p4out) module"]
pub type P4OUT = crate::Reg<u8, _P4OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4OUT;
#[doc = "`read()` method returns [p4out::R](p4out::R) reader structure"]
impl crate::Readable for P4OUT {}
#[doc = "`write(|w| ..)` method takes [p4out::W](p4out::W) writer structure"]
impl crate::Writable for P4OUT {}
#[doc = "Port 4 Output"]
pub mod p4out;
#[doc = "Port 3 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3dir](p3dir) module"]
pub type P3DIR = crate::Reg<u8, _P3DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3DIR;
#[doc = "`read()` method returns [p3dir::R](p3dir::R) reader structure"]
impl crate::Readable for P3DIR {}
#[doc = "`write(|w| ..)` method takes [p3dir::W](p3dir::W) writer structure"]
impl crate::Writable for P3DIR {}
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "Port 4 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4dir](p4dir) module"]
pub type P4DIR = crate::Reg<u8, _P4DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4DIR;
#[doc = "`read()` method returns [p4dir::R](p4dir::R) reader structure"]
impl crate::Readable for P4DIR {}
#[doc = "`write(|w| ..)` method takes [p4dir::W](p4dir::W) writer structure"]
impl crate::Writable for P4DIR {}
#[doc = "Port 4 Direction"]
pub mod p4dir;
#[doc = "Port 3 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3ren](p3ren) module"]
pub type P3REN = crate::Reg<u8, _P3REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3REN;
#[doc = "`read()` method returns [p3ren::R](p3ren::R) reader structure"]
impl crate::Readable for P3REN {}
#[doc = "`write(|w| ..)` method takes [p3ren::W](p3ren::W) writer structure"]
impl crate::Writable for P3REN {}
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "Port 4 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4ren](p4ren) module"]
pub type P4REN = crate::Reg<u8, _P4REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4REN;
#[doc = "`read()` method returns [p4ren::R](p4ren::R) reader structure"]
impl crate::Readable for P4REN {}
#[doc = "`write(|w| ..)` method takes [p4ren::W](p4ren::W) writer structure"]
impl crate::Writable for P4REN {}
#[doc = "Port 4 Resistor Enable"]
pub mod p4ren;
#[doc = "Port 3 Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3sel0](p3sel0) module"]
pub type P3SEL0 = crate::Reg<u8, _P3SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3SEL0;
#[doc = "`read()` method returns [p3sel0::R](p3sel0::R) reader structure"]
impl crate::Readable for P3SEL0 {}
#[doc = "`write(|w| ..)` method takes [p3sel0::W](p3sel0::W) writer structure"]
impl crate::Writable for P3SEL0 {}
#[doc = "Port 3 Selection 0"]
pub mod p3sel0;
#[doc = "Port 4 Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4sel0](p4sel0) module"]
pub type P4SEL0 = crate::Reg<u8, _P4SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4SEL0;
#[doc = "`read()` method returns [p4sel0::R](p4sel0::R) reader structure"]
impl crate::Readable for P4SEL0 {}
#[doc = "`write(|w| ..)` method takes [p4sel0::W](p4sel0::W) writer structure"]
impl crate::Writable for P4SEL0 {}
#[doc = "Port 4 Selection 0"]
pub mod p4sel0;
#[doc = "Port 3 Selection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3sel1](p3sel1) module"]
pub type P3SEL1 = crate::Reg<u8, _P3SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3SEL1;
#[doc = "`read()` method returns [p3sel1::R](p3sel1::R) reader structure"]
impl crate::Readable for P3SEL1 {}
#[doc = "`write(|w| ..)` method takes [p3sel1::W](p3sel1::W) writer structure"]
impl crate::Writable for P3SEL1 {}
#[doc = "Port 3 Selection 1"]
pub mod p3sel1;
#[doc = "Port 4 Selection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4sel1](p4sel1) module"]
pub type P4SEL1 = crate::Reg<u8, _P4SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4SEL1;
#[doc = "`read()` method returns [p4sel1::R](p4sel1::R) reader structure"]
impl crate::Readable for P4SEL1 {}
#[doc = "`write(|w| ..)` method takes [p4sel1::W](p4sel1::W) writer structure"]
impl crate::Writable for P4SEL1 {}
#[doc = "Port 4 Selection 1"]
pub mod p4sel1;
#[doc = "Port 3 Complement Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3selc](p3selc) module"]
pub type P3SELC = crate::Reg<u8, _P3SELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3SELC;
#[doc = "`read()` method returns [p3selc::R](p3selc::R) reader structure"]
impl crate::Readable for P3SELC {}
#[doc = "`write(|w| ..)` method takes [p3selc::W](p3selc::W) writer structure"]
impl crate::Writable for P3SELC {}
#[doc = "Port 3 Complement Selection"]
pub mod p3selc;
#[doc = "Port 4 Complement Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4selc](p4selc) module"]
pub type P4SELC = crate::Reg<u8, _P4SELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4SELC;
#[doc = "`read()` method returns [p4selc::R](p4selc::R) reader structure"]
impl crate::Readable for P4SELC {}
#[doc = "`write(|w| ..)` method takes [p4selc::W](p4selc::W) writer structure"]
impl crate::Writable for P4SELC {}
#[doc = "Port 4 Complement Selection"]
pub mod p4selc;
#[doc = "Port 3 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3ies](p3ies) module"]
pub type P3IES = crate::Reg<u8, _P3IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3IES;
#[doc = "`read()` method returns [p3ies::R](p3ies::R) reader structure"]
impl crate::Readable for P3IES {}
#[doc = "`write(|w| ..)` method takes [p3ies::W](p3ies::W) writer structure"]
impl crate::Writable for P3IES {}
#[doc = "Port 3 Interrupt Edge Select"]
pub mod p3ies;
#[doc = "Port 4 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4ies](p4ies) module"]
pub type P4IES = crate::Reg<u8, _P4IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4IES;
#[doc = "`read()` method returns [p4ies::R](p4ies::R) reader structure"]
impl crate::Readable for P4IES {}
#[doc = "`write(|w| ..)` method takes [p4ies::W](p4ies::W) writer structure"]
impl crate::Writable for P4IES {}
#[doc = "Port 4 Interrupt Edge Select"]
pub mod p4ies;
#[doc = "Port 3 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3ie](p3ie) module"]
pub type P3IE = crate::Reg<u8, _P3IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3IE;
#[doc = "`read()` method returns [p3ie::R](p3ie::R) reader structure"]
impl crate::Readable for P3IE {}
#[doc = "`write(|w| ..)` method takes [p3ie::W](p3ie::W) writer structure"]
impl crate::Writable for P3IE {}
#[doc = "Port 3 Interrupt Enable"]
pub mod p3ie;
#[doc = "Port 4 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4ie](p4ie) module"]
pub type P4IE = crate::Reg<u8, _P4IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4IE;
#[doc = "`read()` method returns [p4ie::R](p4ie::R) reader structure"]
impl crate::Readable for P4IE {}
#[doc = "`write(|w| ..)` method takes [p4ie::W](p4ie::W) writer structure"]
impl crate::Writable for P4IE {}
#[doc = "Port 4 Interrupt Enable"]
pub mod p4ie;
#[doc = "Port 3 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3ifg](p3ifg) module"]
pub type P3IFG = crate::Reg<u8, _P3IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3IFG;
#[doc = "`read()` method returns [p3ifg::R](p3ifg::R) reader structure"]
impl crate::Readable for P3IFG {}
#[doc = "`write(|w| ..)` method takes [p3ifg::W](p3ifg::W) writer structure"]
impl crate::Writable for P3IFG {}
#[doc = "Port 3 Interrupt Flag"]
pub mod p3ifg;
#[doc = "Port 4 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4ifg](p4ifg) module"]
pub type P4IFG = crate::Reg<u8, _P4IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4IFG;
#[doc = "`read()` method returns [p4ifg::R](p4ifg::R) reader structure"]
impl crate::Readable for P4IFG {}
#[doc = "`write(|w| ..)` method takes [p4ifg::W](p4ifg::W) writer structure"]
impl crate::Writable for P4IFG {}
#[doc = "Port 4 Interrupt Flag"]
pub mod p4ifg;
#[doc = "Port 3 Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3iv](p3iv) module"]
pub type P3IV = crate::Reg<u16, _P3IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3IV;
#[doc = "`read()` method returns [p3iv::R](p3iv::R) reader structure"]
impl crate::Readable for P3IV {}
#[doc = "`write(|w| ..)` method takes [p3iv::W](p3iv::W) writer structure"]
impl crate::Writable for P3IV {}
#[doc = "Port 3 Interrupt Vector Word"]
pub mod p3iv;
#[doc = "Port 4 Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4iv](p4iv) module"]
pub type P4IV = crate::Reg<u16, _P4IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4IV;
#[doc = "`read()` method returns [p4iv::R](p4iv::R) reader structure"]
impl crate::Readable for P4IV {}
#[doc = "`write(|w| ..)` method takes [p4iv::W](p4iv::W) writer structure"]
impl crate::Writable for P4IV {}
#[doc = "Port 4 Interrupt Vector Word"]
pub mod p4iv;
