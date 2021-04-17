#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 7 Input"]
    pub p7in: P7IN,
    _reserved1: [u8; 1usize],
    #[doc = "0x02 - Port 7 Output"]
    pub p7out: P7OUT,
    _reserved2: [u8; 1usize],
    #[doc = "0x04 - Port 7 Direction"]
    pub p7dir: P7DIR,
    _reserved3: [u8; 1usize],
    #[doc = "0x06 - Port 7 Resistor Enable"]
    pub p7ren: P7REN,
    _reserved4: [u8; 3usize],
    #[doc = "0x0a - Port 7 Selection0"]
    pub p7sel0: P7SEL0,
    _reserved5: [u8; 1usize],
    #[doc = "0x0c - Port 7 Selection1"]
    pub p7sel1: P7SEL1,
    _reserved6: [u8; 9usize],
    #[doc = "0x16 - Port 7 Complement Selection"]
    pub p7selc: P7SELC,
}
#[doc = "Port 7 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7in](p7in) module"]
pub type P7IN = crate::Reg<u8, _P7IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7IN;
#[doc = "`read()` method returns [p7in::R](p7in::R) reader structure"]
impl crate::Readable for P7IN {}
#[doc = "`write(|w| ..)` method takes [p7in::W](p7in::W) writer structure"]
impl crate::Writable for P7IN {}
#[doc = "Port 7 Input"]
pub mod p7in;
#[doc = "Port 7 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7out](p7out) module"]
pub type P7OUT = crate::Reg<u8, _P7OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7OUT;
#[doc = "`read()` method returns [p7out::R](p7out::R) reader structure"]
impl crate::Readable for P7OUT {}
#[doc = "`write(|w| ..)` method takes [p7out::W](p7out::W) writer structure"]
impl crate::Writable for P7OUT {}
#[doc = "Port 7 Output"]
pub mod p7out;
#[doc = "Port 7 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7dir](p7dir) module"]
pub type P7DIR = crate::Reg<u8, _P7DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7DIR;
#[doc = "`read()` method returns [p7dir::R](p7dir::R) reader structure"]
impl crate::Readable for P7DIR {}
#[doc = "`write(|w| ..)` method takes [p7dir::W](p7dir::W) writer structure"]
impl crate::Writable for P7DIR {}
#[doc = "Port 7 Direction"]
pub mod p7dir;
#[doc = "Port 7 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7ren](p7ren) module"]
pub type P7REN = crate::Reg<u8, _P7REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7REN;
#[doc = "`read()` method returns [p7ren::R](p7ren::R) reader structure"]
impl crate::Readable for P7REN {}
#[doc = "`write(|w| ..)` method takes [p7ren::W](p7ren::W) writer structure"]
impl crate::Writable for P7REN {}
#[doc = "Port 7 Resistor Enable"]
pub mod p7ren;
#[doc = "Port 7 Selection0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7sel0](p7sel0) module"]
pub type P7SEL0 = crate::Reg<u8, _P7SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7SEL0;
#[doc = "`read()` method returns [p7sel0::R](p7sel0::R) reader structure"]
impl crate::Readable for P7SEL0 {}
#[doc = "`write(|w| ..)` method takes [p7sel0::W](p7sel0::W) writer structure"]
impl crate::Writable for P7SEL0 {}
#[doc = "Port 7 Selection0"]
pub mod p7sel0;
#[doc = "Port 7 Selection1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7sel1](p7sel1) module"]
pub type P7SEL1 = crate::Reg<u8, _P7SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7SEL1;
#[doc = "`read()` method returns [p7sel1::R](p7sel1::R) reader structure"]
impl crate::Readable for P7SEL1 {}
#[doc = "`write(|w| ..)` method takes [p7sel1::W](p7sel1::W) writer structure"]
impl crate::Writable for P7SEL1 {}
#[doc = "Port 7 Selection1"]
pub mod p7sel1;
#[doc = "Port 7 Complement Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p7selc](p7selc) module"]
pub type P7SELC = crate::Reg<u8, _P7SELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P7SELC;
#[doc = "`read()` method returns [p7selc::R](p7selc::R) reader structure"]
impl crate::Readable for P7SELC {}
#[doc = "`write(|w| ..)` method takes [p7selc::W](p7selc::W) writer structure"]
impl crate::Writable for P7SELC {}
#[doc = "Port 7 Complement Selection"]
pub mod p7selc;
