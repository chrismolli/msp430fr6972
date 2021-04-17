#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES accelerator control register 0"]
    pub aesactl0: AESACTL0,
    #[doc = "0x02 - AES accelerator control register 1"]
    pub aesactl1: AESACTL1,
    #[doc = "0x04 - AES accelerator status register"]
    pub aesastat: AESASTAT,
    #[doc = "0x06 - AES accelerator key register"]
    pub aesakey: AESAKEY,
    #[doc = "0x08 - AES accelerator data in register"]
    pub aesadin: AESADIN,
    #[doc = "0x0a - AES accelerator data out register"]
    pub aesadout: AESADOUT,
    #[doc = "0x0c - AES accelerator XORed data in register"]
    pub aesaxdin: AESAXDIN,
    #[doc = "0x0e - AES accelerator XORed data in register (no trigger)"]
    pub aesaxin: AESAXIN,
}
#[doc = "AES accelerator control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesactl0](aesactl0) module"]
pub type AESACTL0 = crate::Reg<u16, _AESACTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESACTL0;
#[doc = "`read()` method returns [aesactl0::R](aesactl0::R) reader structure"]
impl crate::Readable for AESACTL0 {}
#[doc = "`write(|w| ..)` method takes [aesactl0::W](aesactl0::W) writer structure"]
impl crate::Writable for AESACTL0 {}
#[doc = "AES accelerator control register 0"]
pub mod aesactl0;
#[doc = "AES accelerator control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesactl1](aesactl1) module"]
pub type AESACTL1 = crate::Reg<u16, _AESACTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESACTL1;
#[doc = "`read()` method returns [aesactl1::R](aesactl1::R) reader structure"]
impl crate::Readable for AESACTL1 {}
#[doc = "`write(|w| ..)` method takes [aesactl1::W](aesactl1::W) writer structure"]
impl crate::Writable for AESACTL1 {}
#[doc = "AES accelerator control register 1"]
pub mod aesactl1;
#[doc = "AES accelerator status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesastat](aesastat) module"]
pub type AESASTAT = crate::Reg<u16, _AESASTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESASTAT;
#[doc = "`read()` method returns [aesastat::R](aesastat::R) reader structure"]
impl crate::Readable for AESASTAT {}
#[doc = "`write(|w| ..)` method takes [aesastat::W](aesastat::W) writer structure"]
impl crate::Writable for AESASTAT {}
#[doc = "AES accelerator status register"]
pub mod aesastat;
#[doc = "AES accelerator key register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesakey](aesakey) module"]
pub type AESAKEY = crate::Reg<u16, _AESAKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESAKEY;
#[doc = "`read()` method returns [aesakey::R](aesakey::R) reader structure"]
impl crate::Readable for AESAKEY {}
#[doc = "`write(|w| ..)` method takes [aesakey::W](aesakey::W) writer structure"]
impl crate::Writable for AESAKEY {}
#[doc = "AES accelerator key register"]
pub mod aesakey;
#[doc = "AES accelerator data in register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesadin](aesadin) module"]
pub type AESADIN = crate::Reg<u16, _AESADIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESADIN;
#[doc = "`read()` method returns [aesadin::R](aesadin::R) reader structure"]
impl crate::Readable for AESADIN {}
#[doc = "`write(|w| ..)` method takes [aesadin::W](aesadin::W) writer structure"]
impl crate::Writable for AESADIN {}
#[doc = "AES accelerator data in register"]
pub mod aesadin;
#[doc = "AES accelerator data out register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesadout](aesadout) module"]
pub type AESADOUT = crate::Reg<u16, _AESADOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESADOUT;
#[doc = "`read()` method returns [aesadout::R](aesadout::R) reader structure"]
impl crate::Readable for AESADOUT {}
#[doc = "`write(|w| ..)` method takes [aesadout::W](aesadout::W) writer structure"]
impl crate::Writable for AESADOUT {}
#[doc = "AES accelerator data out register"]
pub mod aesadout;
#[doc = "AES accelerator XORed data in register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesaxdin](aesaxdin) module"]
pub type AESAXDIN = crate::Reg<u16, _AESAXDIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESAXDIN;
#[doc = "`read()` method returns [aesaxdin::R](aesaxdin::R) reader structure"]
impl crate::Readable for AESAXDIN {}
#[doc = "`write(|w| ..)` method takes [aesaxdin::W](aesaxdin::W) writer structure"]
impl crate::Writable for AESAXDIN {}
#[doc = "AES accelerator XORed data in register"]
pub mod aesaxdin;
#[doc = "AES accelerator XORed data in register (no trigger)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesaxin](aesaxin) module"]
pub type AESAXIN = crate::Reg<u16, _AESAXIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESAXIN;
#[doc = "`read()` method returns [aesaxin::R](aesaxin::R) reader structure"]
impl crate::Readable for AESAXIN {}
#[doc = "`write(|w| ..)` method takes [aesaxin::W](aesaxin::W) writer structure"]
impl crate::Writable for AESAXIN {}
#[doc = "AES accelerator XORed data in register (no trigger)"]
pub mod aesaxin;
