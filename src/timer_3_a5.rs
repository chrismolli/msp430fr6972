#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer3_A5 Control"]
    pub ta3ctl: TA3CTL,
    #[doc = "0x02 - Timer3_A5 Capture/Compare Control 0"]
    pub ta3cctl0: TA3CCTL0,
    #[doc = "0x04 - Timer3_A5 Capture/Compare Control 1"]
    pub ta3cctl1: TA3CCTL1,
    #[doc = "0x06 - Timer3_A5 Capture/Compare Control 2"]
    pub ta3cctl2: TA3CCTL2,
    #[doc = "0x08 - Timer3_A5 Capture/Compare Control 3"]
    pub ta3cctl3: TA3CCTL3,
    #[doc = "0x0a - Timer3_A5 Capture/Compare Control 4"]
    pub ta3cctl4: TA3CCTL4,
    _reserved6: [u8; 4usize],
    #[doc = "0x10 - Timer3_A5"]
    pub ta3r: TA3R,
    #[doc = "0x12 - Timer3_A5 Capture/Compare 0"]
    pub ta3ccr0: TA3CCR0,
    #[doc = "0x14 - Timer3_A5 Capture/Compare 1"]
    pub ta3ccr1: TA3CCR1,
    #[doc = "0x16 - Timer3_A5 Capture/Compare 2"]
    pub ta3ccr2: TA3CCR2,
    #[doc = "0x18 - Timer3_A5 Capture/Compare 3"]
    pub ta3ccr3: TA3CCR3,
    #[doc = "0x1a - Timer3_A5 Capture/Compare 4"]
    pub ta3ccr4: TA3CCR4,
    _reserved12: [u8; 4usize],
    #[doc = "0x20 - Timer3_A5 Expansion Register 0"]
    pub ta3ex0: TA3EX0,
    _reserved13: [u8; 12usize],
    #[doc = "0x2e - Timer3_A5 Interrupt Vector Word"]
    pub ta3iv: TA3IV,
}
#[doc = "Timer3_A5 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3ctl](ta3ctl) module"]
pub type TA3CTL = crate::Reg<u16, _TA3CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CTL;
#[doc = "`read()` method returns [ta3ctl::R](ta3ctl::R) reader structure"]
impl crate::Readable for TA3CTL {}
#[doc = "`write(|w| ..)` method takes [ta3ctl::W](ta3ctl::W) writer structure"]
impl crate::Writable for TA3CTL {}
#[doc = "Timer3_A5 Control"]
pub mod ta3ctl;
#[doc = "Timer3_A5 Capture/Compare Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3cctl0](ta3cctl0) module"]
pub type TA3CCTL0 = crate::Reg<u16, _TA3CCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CCTL0;
#[doc = "`read()` method returns [ta3cctl0::R](ta3cctl0::R) reader structure"]
impl crate::Readable for TA3CCTL0 {}
#[doc = "`write(|w| ..)` method takes [ta3cctl0::W](ta3cctl0::W) writer structure"]
impl crate::Writable for TA3CCTL0 {}
#[doc = "Timer3_A5 Capture/Compare Control 0"]
pub mod ta3cctl0;
#[doc = "Timer3_A5 Capture/Compare Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3cctl1](ta3cctl1) module"]
pub type TA3CCTL1 = crate::Reg<u16, _TA3CCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CCTL1;
#[doc = "`read()` method returns [ta3cctl1::R](ta3cctl1::R) reader structure"]
impl crate::Readable for TA3CCTL1 {}
#[doc = "`write(|w| ..)` method takes [ta3cctl1::W](ta3cctl1::W) writer structure"]
impl crate::Writable for TA3CCTL1 {}
#[doc = "Timer3_A5 Capture/Compare Control 1"]
pub mod ta3cctl1;
#[doc = "Timer3_A5 Capture/Compare Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3cctl2](ta3cctl2) module"]
pub type TA3CCTL2 = crate::Reg<u16, _TA3CCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CCTL2;
#[doc = "`read()` method returns [ta3cctl2::R](ta3cctl2::R) reader structure"]
impl crate::Readable for TA3CCTL2 {}
#[doc = "`write(|w| ..)` method takes [ta3cctl2::W](ta3cctl2::W) writer structure"]
impl crate::Writable for TA3CCTL2 {}
#[doc = "Timer3_A5 Capture/Compare Control 2"]
pub mod ta3cctl2;
#[doc = "Timer3_A5 Capture/Compare Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3cctl3](ta3cctl3) module"]
pub type TA3CCTL3 = crate::Reg<u16, _TA3CCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CCTL3;
#[doc = "`read()` method returns [ta3cctl3::R](ta3cctl3::R) reader structure"]
impl crate::Readable for TA3CCTL3 {}
#[doc = "`write(|w| ..)` method takes [ta3cctl3::W](ta3cctl3::W) writer structure"]
impl crate::Writable for TA3CCTL3 {}
#[doc = "Timer3_A5 Capture/Compare Control 3"]
pub mod ta3cctl3;
#[doc = "Timer3_A5 Capture/Compare Control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3cctl4](ta3cctl4) module"]
pub type TA3CCTL4 = crate::Reg<u16, _TA3CCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CCTL4;
#[doc = "`read()` method returns [ta3cctl4::R](ta3cctl4::R) reader structure"]
impl crate::Readable for TA3CCTL4 {}
#[doc = "`write(|w| ..)` method takes [ta3cctl4::W](ta3cctl4::W) writer structure"]
impl crate::Writable for TA3CCTL4 {}
#[doc = "Timer3_A5 Capture/Compare Control 4"]
pub mod ta3cctl4;
#[doc = "Timer3_A5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3r](ta3r) module"]
pub type TA3R = crate::Reg<u16, _TA3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3R;
#[doc = "`read()` method returns [ta3r::R](ta3r::R) reader structure"]
impl crate::Readable for TA3R {}
#[doc = "`write(|w| ..)` method takes [ta3r::W](ta3r::W) writer structure"]
impl crate::Writable for TA3R {}
#[doc = "Timer3_A5"]
pub mod ta3r;
#[doc = "Timer3_A5 Capture/Compare 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3ccr0](ta3ccr0) module"]
pub type TA3CCR0 = crate::Reg<u16, _TA3CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CCR0;
#[doc = "`read()` method returns [ta3ccr0::R](ta3ccr0::R) reader structure"]
impl crate::Readable for TA3CCR0 {}
#[doc = "`write(|w| ..)` method takes [ta3ccr0::W](ta3ccr0::W) writer structure"]
impl crate::Writable for TA3CCR0 {}
#[doc = "Timer3_A5 Capture/Compare 0"]
pub mod ta3ccr0;
#[doc = "Timer3_A5 Capture/Compare 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3ccr1](ta3ccr1) module"]
pub type TA3CCR1 = crate::Reg<u16, _TA3CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CCR1;
#[doc = "`read()` method returns [ta3ccr1::R](ta3ccr1::R) reader structure"]
impl crate::Readable for TA3CCR1 {}
#[doc = "`write(|w| ..)` method takes [ta3ccr1::W](ta3ccr1::W) writer structure"]
impl crate::Writable for TA3CCR1 {}
#[doc = "Timer3_A5 Capture/Compare 1"]
pub mod ta3ccr1;
#[doc = "Timer3_A5 Capture/Compare 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3ccr2](ta3ccr2) module"]
pub type TA3CCR2 = crate::Reg<u16, _TA3CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CCR2;
#[doc = "`read()` method returns [ta3ccr2::R](ta3ccr2::R) reader structure"]
impl crate::Readable for TA3CCR2 {}
#[doc = "`write(|w| ..)` method takes [ta3ccr2::W](ta3ccr2::W) writer structure"]
impl crate::Writable for TA3CCR2 {}
#[doc = "Timer3_A5 Capture/Compare 2"]
pub mod ta3ccr2;
#[doc = "Timer3_A5 Capture/Compare 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3ccr3](ta3ccr3) module"]
pub type TA3CCR3 = crate::Reg<u16, _TA3CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CCR3;
#[doc = "`read()` method returns [ta3ccr3::R](ta3ccr3::R) reader structure"]
impl crate::Readable for TA3CCR3 {}
#[doc = "`write(|w| ..)` method takes [ta3ccr3::W](ta3ccr3::W) writer structure"]
impl crate::Writable for TA3CCR3 {}
#[doc = "Timer3_A5 Capture/Compare 3"]
pub mod ta3ccr3;
#[doc = "Timer3_A5 Capture/Compare 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3ccr4](ta3ccr4) module"]
pub type TA3CCR4 = crate::Reg<u16, _TA3CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CCR4;
#[doc = "`read()` method returns [ta3ccr4::R](ta3ccr4::R) reader structure"]
impl crate::Readable for TA3CCR4 {}
#[doc = "`write(|w| ..)` method takes [ta3ccr4::W](ta3ccr4::W) writer structure"]
impl crate::Writable for TA3CCR4 {}
#[doc = "Timer3_A5 Capture/Compare 4"]
pub mod ta3ccr4;
#[doc = "Timer3_A5 Expansion Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3ex0](ta3ex0) module"]
pub type TA3EX0 = crate::Reg<u16, _TA3EX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3EX0;
#[doc = "`read()` method returns [ta3ex0::R](ta3ex0::R) reader structure"]
impl crate::Readable for TA3EX0 {}
#[doc = "`write(|w| ..)` method takes [ta3ex0::W](ta3ex0::W) writer structure"]
impl crate::Writable for TA3EX0 {}
#[doc = "Timer3_A5 Expansion Register 0"]
pub mod ta3ex0;
#[doc = "Timer3_A5 Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3iv](ta3iv) module"]
pub type TA3IV = crate::Reg<u16, _TA3IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3IV;
#[doc = "`read()` method returns [ta3iv::R](ta3iv::R) reader structure"]
impl crate::Readable for TA3IV {}
#[doc = "`write(|w| ..)` method takes [ta3iv::W](ta3iv::W) writer structure"]
impl crate::Writable for TA3IV {}
#[doc = "Timer3_A5 Interrupt Vector Word"]
pub mod ta3iv;
