#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 9 Input"]
    pub p9in: P9IN,
    _reserved1: [u8; 1usize],
    #[doc = "0x02 - Port 9 Output"]
    pub p9out: P9OUT,
    _reserved2: [u8; 1usize],
    #[doc = "0x04 - Port 9 Direction"]
    pub p9dir: P9DIR,
    _reserved3: [u8; 1usize],
    #[doc = "0x06 - Port 9 Resistor Enable"]
    pub p9ren: P9REN,
    _reserved4: [u8; 3usize],
    #[doc = "0x0a - Port 9 Selection0"]
    pub p9sel0: P9SEL0,
    _reserved5: [u8; 1usize],
    #[doc = "0x0c - Port 9 Selection1"]
    pub p9sel1: P9SEL1,
    _reserved6: [u8; 9usize],
    #[doc = "0x16 - Port 9 Complement Selection"]
    pub p9selc: P9SELC,
}
#[doc = "Port 9 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9in](p9in) module"]
pub type P9IN = crate::Reg<u8, _P9IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9IN;
#[doc = "`read()` method returns [p9in::R](p9in::R) reader structure"]
impl crate::Readable for P9IN {}
#[doc = "`write(|w| ..)` method takes [p9in::W](p9in::W) writer structure"]
impl crate::Writable for P9IN {}
#[doc = "Port 9 Input"]
pub mod p9in;
#[doc = "Port 9 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9out](p9out) module"]
pub type P9OUT = crate::Reg<u8, _P9OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9OUT;
#[doc = "`read()` method returns [p9out::R](p9out::R) reader structure"]
impl crate::Readable for P9OUT {}
#[doc = "`write(|w| ..)` method takes [p9out::W](p9out::W) writer structure"]
impl crate::Writable for P9OUT {}
#[doc = "Port 9 Output"]
pub mod p9out;
#[doc = "Port 9 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9dir](p9dir) module"]
pub type P9DIR = crate::Reg<u8, _P9DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9DIR;
#[doc = "`read()` method returns [p9dir::R](p9dir::R) reader structure"]
impl crate::Readable for P9DIR {}
#[doc = "`write(|w| ..)` method takes [p9dir::W](p9dir::W) writer structure"]
impl crate::Writable for P9DIR {}
#[doc = "Port 9 Direction"]
pub mod p9dir;
#[doc = "Port 9 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9ren](p9ren) module"]
pub type P9REN = crate::Reg<u8, _P9REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9REN;
#[doc = "`read()` method returns [p9ren::R](p9ren::R) reader structure"]
impl crate::Readable for P9REN {}
#[doc = "`write(|w| ..)` method takes [p9ren::W](p9ren::W) writer structure"]
impl crate::Writable for P9REN {}
#[doc = "Port 9 Resistor Enable"]
pub mod p9ren;
#[doc = "Port 9 Selection0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9sel0](p9sel0) module"]
pub type P9SEL0 = crate::Reg<u8, _P9SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9SEL0;
#[doc = "`read()` method returns [p9sel0::R](p9sel0::R) reader structure"]
impl crate::Readable for P9SEL0 {}
#[doc = "`write(|w| ..)` method takes [p9sel0::W](p9sel0::W) writer structure"]
impl crate::Writable for P9SEL0 {}
#[doc = "Port 9 Selection0"]
pub mod p9sel0;
#[doc = "Port 9 Selection1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9sel1](p9sel1) module"]
pub type P9SEL1 = crate::Reg<u8, _P9SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9SEL1;
#[doc = "`read()` method returns [p9sel1::R](p9sel1::R) reader structure"]
impl crate::Readable for P9SEL1 {}
#[doc = "`write(|w| ..)` method takes [p9sel1::W](p9sel1::W) writer structure"]
impl crate::Writable for P9SEL1 {}
#[doc = "Port 9 Selection1"]
pub mod p9sel1;
#[doc = "Port 9 Complement Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9selc](p9selc) module"]
pub type P9SELC = crate::Reg<u8, _P9SELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9SELC;
#[doc = "`read()` method returns [p9selc::R](p9selc::R) reader structure"]
impl crate::Readable for P9SELC {}
#[doc = "`write(|w| ..)` method takes [p9selc::W](p9selc::W) writer structure"]
impl crate::Writable for P9SELC {}
#[doc = "Port 9 Complement Selection"]
pub mod p9selc;
