#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Capacitive_Touch_IO 1 control register"]
    pub captio1ctl: CAPTIO1CTL,
}
#[doc = "Capacitive_Touch_IO 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [captio1ctl](captio1ctl) module"]
pub type CAPTIO1CTL = crate::Reg<u16, _CAPTIO1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPTIO1CTL;
#[doc = "`read()` method returns [captio1ctl::R](captio1ctl::R) reader structure"]
impl crate::Readable for CAPTIO1CTL {}
#[doc = "`write(|w| ..)` method takes [captio1ctl::W](captio1ctl::W) writer structure"]
impl crate::Writable for CAPTIO1CTL {}
#[doc = "Capacitive_Touch_IO 1 control register"]
pub mod captio1ctl;
