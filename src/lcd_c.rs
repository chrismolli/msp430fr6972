#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LCD_C Control Register 0"]
    pub lcdcctl0: LCDCCTL0,
    #[doc = "0x02 - LCD_C Control Register 1"]
    pub lcdcctl1: LCDCCTL1,
    #[doc = "0x04 - LCD_C blinking control register"]
    pub lcdcblkctl: LCDCBLKCTL,
    #[doc = "0x06 - LCD_C memory control register"]
    pub lcdcmemctl: LCDCMEMCTL,
    #[doc = "0x08 - LCD_C Voltage Control Register"]
    pub lcdcvctl: LCDCVCTL,
    #[doc = "0x0a - LCD_C Port Control Register 0"]
    pub lcdcpctl0: LCDCPCTL0,
    #[doc = "0x0c - LCD_C Port Control Register 1"]
    pub lcdcpctl1: LCDCPCTL1,
    #[doc = "0x0e - LCD_C Port Control Register 2"]
    pub lcdcpctl2: LCDCPCTL2,
    _reserved8: [u8; 2usize],
    #[doc = "0x12 - LCD_C Charge Pump Control Register 3"]
    pub lcdccpctl: LCDCCPCTL,
    _reserved9: [u8; 10usize],
    #[doc = "0x1e - LCD_C Interrupt Vector Register"]
    pub lcdciv: LCDCIV,
    #[doc = "0x20 - LCD Memory 1"]
    pub lcdm1: LCDM1,
    #[doc = "0x21 - LCD Memory 2"]
    pub lcdm2: LCDM2,
    #[doc = "0x22 - LCD Memory 3"]
    pub lcdm3: LCDM3,
    #[doc = "0x23 - LCD Memory 4"]
    pub lcdm4: LCDM4,
    #[doc = "0x24 - LCD Memory 5"]
    pub lcdm5: LCDM5,
    #[doc = "0x25 - LCD Memory 6"]
    pub lcdm6: LCDM6,
    #[doc = "0x26 - LCD Memory 7"]
    pub lcdm7: LCDM7,
    #[doc = "0x27 - LCD Memory 8"]
    pub lcdm8: LCDM8,
    #[doc = "0x28 - LCD Memory 9"]
    pub lcdm9: LCDM9,
    #[doc = "0x29 - LCD Memory 10"]
    pub lcdm10: LCDM10,
    #[doc = "0x2a - LCD Memory 11"]
    pub lcdm11: LCDM11,
    #[doc = "0x2b - LCD Memory 12"]
    pub lcdm12: LCDM12,
    #[doc = "0x2c - LCD Memory 13"]
    pub lcdm13: LCDM13,
    #[doc = "0x2d - LCD Memory 14"]
    pub lcdm14: LCDM14,
    #[doc = "0x2e - LCD Memory 15"]
    pub lcdm15: LCDM15,
    #[doc = "0x2f - LCD Memory 16"]
    pub lcdm16: LCDM16,
    #[doc = "0x30 - LCD Memory 17"]
    pub lcdm17: LCDM17,
    #[doc = "0x31 - LCD Memory 18"]
    pub lcdm18: LCDM18,
    #[doc = "0x32 - LCD Memory 19"]
    pub lcdm19: LCDM19,
    #[doc = "0x33 - LCD Memory 20"]
    pub lcdm20: LCDM20,
    #[doc = "0x34 - LCD Memory 21"]
    pub lcdm21: LCDM21,
    #[doc = "0x35 - LCD Memory 22"]
    pub lcdm22: LCDM22,
    #[doc = "0x36 - LCD Memory 23"]
    pub lcdm23: LCDM23,
    #[doc = "0x37 - LCD Memory 24"]
    pub lcdm24: LCDM24,
    #[doc = "0x38 - LCD Memory 25"]
    pub lcdm25: LCDM25,
    #[doc = "0x39 - LCD Memory 26"]
    pub lcdm26: LCDM26,
    #[doc = "0x3a - LCD Memory 27"]
    pub lcdm27: LCDM27,
    #[doc = "0x3b - LCD Memory 28"]
    pub lcdm28: LCDM28,
    #[doc = "0x3c - LCD Memory 29"]
    pub lcdm29: LCDM29,
    #[doc = "0x3d - LCD Memory 30"]
    pub lcdm30: LCDM30,
    #[doc = "0x3e - LCD Memory 31"]
    pub lcdm31: LCDM31,
    #[doc = "0x3f - LCD Memory 32"]
    pub lcdm32: LCDM32,
    _reserved_42_lcdbm1: [u8; 1usize],
    _reserved_43_lcdbm2: [u8; 1usize],
    _reserved_44_lcdbm3: [u8; 1usize],
    _reserved_45_lcdbm4: [u8; 1usize],
    _reserved_46_lcdbm5: [u8; 1usize],
    _reserved_47_lcdbm6: [u8; 1usize],
    _reserved_48_lcdbm7: [u8; 1usize],
    _reserved_49_lcdbm8: [u8; 1usize],
    _reserved_50_lcdbm9: [u8; 1usize],
    _reserved_51_lcdm42: [u8; 1usize],
    _reserved_52_lcdm43: [u8; 1usize],
    #[doc = "0x4b - LCD Blinking Memory 12"]
    pub lcdbm12: LCDBM12,
    #[doc = "0x4c - LCD Blinking Memory 13"]
    pub lcdbm13: LCDBM13,
    #[doc = "0x4d - LCD Blinking Memory 14"]
    pub lcdbm14: LCDBM14,
    #[doc = "0x4e - LCD Blinking Memory 15"]
    pub lcdbm15: LCDBM15,
    #[doc = "0x4f - LCD Blinking Memory 16"]
    pub lcdbm16: LCDBM16,
    #[doc = "0x50 - LCD Blinking Memory 17"]
    pub lcdbm17: LCDBM17,
    #[doc = "0x51 - LCD Blinking Memory 18"]
    pub lcdbm18: LCDBM18,
    #[doc = "0x52 - LCD Blinking Memory 19"]
    pub lcdbm19: LCDBM19,
    #[doc = "0x53 - LCD Blinking Memory 20"]
    pub lcdbm20: LCDBM20,
    #[doc = "0x54 - LCD Blinking Memory 21"]
    pub lcdbm21: LCDBM21,
    #[doc = "0x55 - LCD Blinking Memory 22"]
    pub lcdbm22: LCDBM22,
}
impl RegisterBlock {
    #[doc = "0x40 - LCD Memory 33"]
    #[inline(always)]
    pub fn lcdm33(&self) -> &LCDM33 {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const LCDM33) }
    }
    #[doc = "0x40 - LCD Memory 33"]
    #[inline(always)]
    pub fn lcdm33_mut(&self) -> &mut LCDM33 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut LCDM33) }
    }
    #[doc = "0x40 - LCD Blinking Memory 1"]
    #[inline(always)]
    pub fn lcdbm1(&self) -> &LCDBM1 {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const LCDBM1) }
    }
    #[doc = "0x40 - LCD Blinking Memory 1"]
    #[inline(always)]
    pub fn lcdbm1_mut(&self) -> &mut LCDBM1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut LCDBM1) }
    }
    #[doc = "0x41 - LCD Memory 34"]
    #[inline(always)]
    pub fn lcdm34(&self) -> &LCDM34 {
        unsafe { &*(((self as *const Self) as *const u8).add(65usize) as *const LCDM34) }
    }
    #[doc = "0x41 - LCD Memory 34"]
    #[inline(always)]
    pub fn lcdm34_mut(&self) -> &mut LCDM34 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(65usize) as *mut LCDM34) }
    }
    #[doc = "0x41 - LCD Blinking Memory 2"]
    #[inline(always)]
    pub fn lcdbm2(&self) -> &LCDBM2 {
        unsafe { &*(((self as *const Self) as *const u8).add(65usize) as *const LCDBM2) }
    }
    #[doc = "0x41 - LCD Blinking Memory 2"]
    #[inline(always)]
    pub fn lcdbm2_mut(&self) -> &mut LCDBM2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(65usize) as *mut LCDBM2) }
    }
    #[doc = "0x42 - LCD Memory 35"]
    #[inline(always)]
    pub fn lcdm35(&self) -> &LCDM35 {
        unsafe { &*(((self as *const Self) as *const u8).add(66usize) as *const LCDM35) }
    }
    #[doc = "0x42 - LCD Memory 35"]
    #[inline(always)]
    pub fn lcdm35_mut(&self) -> &mut LCDM35 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(66usize) as *mut LCDM35) }
    }
    #[doc = "0x42 - LCD Blinking Memory 3"]
    #[inline(always)]
    pub fn lcdbm3(&self) -> &LCDBM3 {
        unsafe { &*(((self as *const Self) as *const u8).add(66usize) as *const LCDBM3) }
    }
    #[doc = "0x42 - LCD Blinking Memory 3"]
    #[inline(always)]
    pub fn lcdbm3_mut(&self) -> &mut LCDBM3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(66usize) as *mut LCDBM3) }
    }
    #[doc = "0x43 - LCD Memory 36"]
    #[inline(always)]
    pub fn lcdm36(&self) -> &LCDM36 {
        unsafe { &*(((self as *const Self) as *const u8).add(67usize) as *const LCDM36) }
    }
    #[doc = "0x43 - LCD Memory 36"]
    #[inline(always)]
    pub fn lcdm36_mut(&self) -> &mut LCDM36 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(67usize) as *mut LCDM36) }
    }
    #[doc = "0x43 - LCD Blinking Memory 4"]
    #[inline(always)]
    pub fn lcdbm4(&self) -> &LCDBM4 {
        unsafe { &*(((self as *const Self) as *const u8).add(67usize) as *const LCDBM4) }
    }
    #[doc = "0x43 - LCD Blinking Memory 4"]
    #[inline(always)]
    pub fn lcdbm4_mut(&self) -> &mut LCDBM4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(67usize) as *mut LCDBM4) }
    }
    #[doc = "0x44 - LCD Memory 37"]
    #[inline(always)]
    pub fn lcdm37(&self) -> &LCDM37 {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const LCDM37) }
    }
    #[doc = "0x44 - LCD Memory 37"]
    #[inline(always)]
    pub fn lcdm37_mut(&self) -> &mut LCDM37 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut LCDM37) }
    }
    #[doc = "0x44 - LCD Blinking Memory 5"]
    #[inline(always)]
    pub fn lcdbm5(&self) -> &LCDBM5 {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const LCDBM5) }
    }
    #[doc = "0x44 - LCD Blinking Memory 5"]
    #[inline(always)]
    pub fn lcdbm5_mut(&self) -> &mut LCDBM5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut LCDBM5) }
    }
    #[doc = "0x45 - LCD Memory 38"]
    #[inline(always)]
    pub fn lcdm38(&self) -> &LCDM38 {
        unsafe { &*(((self as *const Self) as *const u8).add(69usize) as *const LCDM38) }
    }
    #[doc = "0x45 - LCD Memory 38"]
    #[inline(always)]
    pub fn lcdm38_mut(&self) -> &mut LCDM38 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(69usize) as *mut LCDM38) }
    }
    #[doc = "0x45 - LCD Blinking Memory 6"]
    #[inline(always)]
    pub fn lcdbm6(&self) -> &LCDBM6 {
        unsafe { &*(((self as *const Self) as *const u8).add(69usize) as *const LCDBM6) }
    }
    #[doc = "0x45 - LCD Blinking Memory 6"]
    #[inline(always)]
    pub fn lcdbm6_mut(&self) -> &mut LCDBM6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(69usize) as *mut LCDBM6) }
    }
    #[doc = "0x46 - LCD Memory 39"]
    #[inline(always)]
    pub fn lcdm39(&self) -> &LCDM39 {
        unsafe { &*(((self as *const Self) as *const u8).add(70usize) as *const LCDM39) }
    }
    #[doc = "0x46 - LCD Memory 39"]
    #[inline(always)]
    pub fn lcdm39_mut(&self) -> &mut LCDM39 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(70usize) as *mut LCDM39) }
    }
    #[doc = "0x46 - LCD Blinking Memory 7"]
    #[inline(always)]
    pub fn lcdbm7(&self) -> &LCDBM7 {
        unsafe { &*(((self as *const Self) as *const u8).add(70usize) as *const LCDBM7) }
    }
    #[doc = "0x46 - LCD Blinking Memory 7"]
    #[inline(always)]
    pub fn lcdbm7_mut(&self) -> &mut LCDBM7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(70usize) as *mut LCDBM7) }
    }
    #[doc = "0x47 - LCD Memory 40"]
    #[inline(always)]
    pub fn lcdm40(&self) -> &LCDM40 {
        unsafe { &*(((self as *const Self) as *const u8).add(71usize) as *const LCDM40) }
    }
    #[doc = "0x47 - LCD Memory 40"]
    #[inline(always)]
    pub fn lcdm40_mut(&self) -> &mut LCDM40 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(71usize) as *mut LCDM40) }
    }
    #[doc = "0x47 - LCD Blinking Memory 8"]
    #[inline(always)]
    pub fn lcdbm8(&self) -> &LCDBM8 {
        unsafe { &*(((self as *const Self) as *const u8).add(71usize) as *const LCDBM8) }
    }
    #[doc = "0x47 - LCD Blinking Memory 8"]
    #[inline(always)]
    pub fn lcdbm8_mut(&self) -> &mut LCDBM8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(71usize) as *mut LCDBM8) }
    }
    #[doc = "0x48 - LCD Memory 41"]
    #[inline(always)]
    pub fn lcdm41(&self) -> &LCDM41 {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const LCDM41) }
    }
    #[doc = "0x48 - LCD Memory 41"]
    #[inline(always)]
    pub fn lcdm41_mut(&self) -> &mut LCDM41 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut LCDM41) }
    }
    #[doc = "0x48 - LCD Blinking Memory 9"]
    #[inline(always)]
    pub fn lcdbm9(&self) -> &LCDBM9 {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const LCDBM9) }
    }
    #[doc = "0x48 - LCD Blinking Memory 9"]
    #[inline(always)]
    pub fn lcdbm9_mut(&self) -> &mut LCDBM9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut LCDBM9) }
    }
    #[doc = "0x49 - LCD Memory 42"]
    #[inline(always)]
    pub fn lcdm42(&self) -> &LCDM42 {
        unsafe { &*(((self as *const Self) as *const u8).add(73usize) as *const LCDM42) }
    }
    #[doc = "0x49 - LCD Memory 42"]
    #[inline(always)]
    pub fn lcdm42_mut(&self) -> &mut LCDM42 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(73usize) as *mut LCDM42) }
    }
    #[doc = "0x49 - LCD Blinking Memory 10"]
    #[inline(always)]
    pub fn lcdbm10(&self) -> &LCDBM10 {
        unsafe { &*(((self as *const Self) as *const u8).add(73usize) as *const LCDBM10) }
    }
    #[doc = "0x49 - LCD Blinking Memory 10"]
    #[inline(always)]
    pub fn lcdbm10_mut(&self) -> &mut LCDBM10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(73usize) as *mut LCDBM10) }
    }
    #[doc = "0x4a - LCD Memory 43"]
    #[inline(always)]
    pub fn lcdm43(&self) -> &LCDM43 {
        unsafe { &*(((self as *const Self) as *const u8).add(74usize) as *const LCDM43) }
    }
    #[doc = "0x4a - LCD Memory 43"]
    #[inline(always)]
    pub fn lcdm43_mut(&self) -> &mut LCDM43 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(74usize) as *mut LCDM43) }
    }
    #[doc = "0x4a - LCD Blinking Memory 11"]
    #[inline(always)]
    pub fn lcdbm11(&self) -> &LCDBM11 {
        unsafe { &*(((self as *const Self) as *const u8).add(74usize) as *const LCDBM11) }
    }
    #[doc = "0x4a - LCD Blinking Memory 11"]
    #[inline(always)]
    pub fn lcdbm11_mut(&self) -> &mut LCDBM11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(74usize) as *mut LCDBM11) }
    }
}
#[doc = "LCD Memory 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm1](lcdm1) module"]
pub type LCDM1 = crate::Reg<u8, _LCDM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM1;
#[doc = "`read()` method returns [lcdm1::R](lcdm1::R) reader structure"]
impl crate::Readable for LCDM1 {}
#[doc = "`write(|w| ..)` method takes [lcdm1::W](lcdm1::W) writer structure"]
impl crate::Writable for LCDM1 {}
#[doc = "LCD Memory 1"]
pub mod lcdm1;
#[doc = "LCD Memory 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm2](lcdm2) module"]
pub type LCDM2 = crate::Reg<u8, _LCDM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM2;
#[doc = "`read()` method returns [lcdm2::R](lcdm2::R) reader structure"]
impl crate::Readable for LCDM2 {}
#[doc = "`write(|w| ..)` method takes [lcdm2::W](lcdm2::W) writer structure"]
impl crate::Writable for LCDM2 {}
#[doc = "LCD Memory 2"]
pub mod lcdm2;
#[doc = "LCD Memory 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm3](lcdm3) module"]
pub type LCDM3 = crate::Reg<u8, _LCDM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM3;
#[doc = "`read()` method returns [lcdm3::R](lcdm3::R) reader structure"]
impl crate::Readable for LCDM3 {}
#[doc = "`write(|w| ..)` method takes [lcdm3::W](lcdm3::W) writer structure"]
impl crate::Writable for LCDM3 {}
#[doc = "LCD Memory 3"]
pub mod lcdm3;
#[doc = "LCD Memory 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm4](lcdm4) module"]
pub type LCDM4 = crate::Reg<u8, _LCDM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM4;
#[doc = "`read()` method returns [lcdm4::R](lcdm4::R) reader structure"]
impl crate::Readable for LCDM4 {}
#[doc = "`write(|w| ..)` method takes [lcdm4::W](lcdm4::W) writer structure"]
impl crate::Writable for LCDM4 {}
#[doc = "LCD Memory 4"]
pub mod lcdm4;
#[doc = "LCD Memory 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm5](lcdm5) module"]
pub type LCDM5 = crate::Reg<u8, _LCDM5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM5;
#[doc = "`read()` method returns [lcdm5::R](lcdm5::R) reader structure"]
impl crate::Readable for LCDM5 {}
#[doc = "`write(|w| ..)` method takes [lcdm5::W](lcdm5::W) writer structure"]
impl crate::Writable for LCDM5 {}
#[doc = "LCD Memory 5"]
pub mod lcdm5;
#[doc = "LCD Memory 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm6](lcdm6) module"]
pub type LCDM6 = crate::Reg<u8, _LCDM6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM6;
#[doc = "`read()` method returns [lcdm6::R](lcdm6::R) reader structure"]
impl crate::Readable for LCDM6 {}
#[doc = "`write(|w| ..)` method takes [lcdm6::W](lcdm6::W) writer structure"]
impl crate::Writable for LCDM6 {}
#[doc = "LCD Memory 6"]
pub mod lcdm6;
#[doc = "LCD Memory 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm7](lcdm7) module"]
pub type LCDM7 = crate::Reg<u8, _LCDM7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM7;
#[doc = "`read()` method returns [lcdm7::R](lcdm7::R) reader structure"]
impl crate::Readable for LCDM7 {}
#[doc = "`write(|w| ..)` method takes [lcdm7::W](lcdm7::W) writer structure"]
impl crate::Writable for LCDM7 {}
#[doc = "LCD Memory 7"]
pub mod lcdm7;
#[doc = "LCD Memory 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm8](lcdm8) module"]
pub type LCDM8 = crate::Reg<u8, _LCDM8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM8;
#[doc = "`read()` method returns [lcdm8::R](lcdm8::R) reader structure"]
impl crate::Readable for LCDM8 {}
#[doc = "`write(|w| ..)` method takes [lcdm8::W](lcdm8::W) writer structure"]
impl crate::Writable for LCDM8 {}
#[doc = "LCD Memory 8"]
pub mod lcdm8;
#[doc = "LCD Memory 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm9](lcdm9) module"]
pub type LCDM9 = crate::Reg<u8, _LCDM9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM9;
#[doc = "`read()` method returns [lcdm9::R](lcdm9::R) reader structure"]
impl crate::Readable for LCDM9 {}
#[doc = "`write(|w| ..)` method takes [lcdm9::W](lcdm9::W) writer structure"]
impl crate::Writable for LCDM9 {}
#[doc = "LCD Memory 9"]
pub mod lcdm9;
#[doc = "LCD Memory 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm10](lcdm10) module"]
pub type LCDM10 = crate::Reg<u8, _LCDM10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM10;
#[doc = "`read()` method returns [lcdm10::R](lcdm10::R) reader structure"]
impl crate::Readable for LCDM10 {}
#[doc = "`write(|w| ..)` method takes [lcdm10::W](lcdm10::W) writer structure"]
impl crate::Writable for LCDM10 {}
#[doc = "LCD Memory 10"]
pub mod lcdm10;
#[doc = "LCD Memory 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm11](lcdm11) module"]
pub type LCDM11 = crate::Reg<u8, _LCDM11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM11;
#[doc = "`read()` method returns [lcdm11::R](lcdm11::R) reader structure"]
impl crate::Readable for LCDM11 {}
#[doc = "`write(|w| ..)` method takes [lcdm11::W](lcdm11::W) writer structure"]
impl crate::Writable for LCDM11 {}
#[doc = "LCD Memory 11"]
pub mod lcdm11;
#[doc = "LCD Memory 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm12](lcdm12) module"]
pub type LCDM12 = crate::Reg<u8, _LCDM12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM12;
#[doc = "`read()` method returns [lcdm12::R](lcdm12::R) reader structure"]
impl crate::Readable for LCDM12 {}
#[doc = "`write(|w| ..)` method takes [lcdm12::W](lcdm12::W) writer structure"]
impl crate::Writable for LCDM12 {}
#[doc = "LCD Memory 12"]
pub mod lcdm12;
#[doc = "LCD Memory 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm13](lcdm13) module"]
pub type LCDM13 = crate::Reg<u8, _LCDM13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM13;
#[doc = "`read()` method returns [lcdm13::R](lcdm13::R) reader structure"]
impl crate::Readable for LCDM13 {}
#[doc = "`write(|w| ..)` method takes [lcdm13::W](lcdm13::W) writer structure"]
impl crate::Writable for LCDM13 {}
#[doc = "LCD Memory 13"]
pub mod lcdm13;
#[doc = "LCD Memory 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm14](lcdm14) module"]
pub type LCDM14 = crate::Reg<u8, _LCDM14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM14;
#[doc = "`read()` method returns [lcdm14::R](lcdm14::R) reader structure"]
impl crate::Readable for LCDM14 {}
#[doc = "`write(|w| ..)` method takes [lcdm14::W](lcdm14::W) writer structure"]
impl crate::Writable for LCDM14 {}
#[doc = "LCD Memory 14"]
pub mod lcdm14;
#[doc = "LCD Memory 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm15](lcdm15) module"]
pub type LCDM15 = crate::Reg<u8, _LCDM15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM15;
#[doc = "`read()` method returns [lcdm15::R](lcdm15::R) reader structure"]
impl crate::Readable for LCDM15 {}
#[doc = "`write(|w| ..)` method takes [lcdm15::W](lcdm15::W) writer structure"]
impl crate::Writable for LCDM15 {}
#[doc = "LCD Memory 15"]
pub mod lcdm15;
#[doc = "LCD Memory 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm16](lcdm16) module"]
pub type LCDM16 = crate::Reg<u8, _LCDM16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM16;
#[doc = "`read()` method returns [lcdm16::R](lcdm16::R) reader structure"]
impl crate::Readable for LCDM16 {}
#[doc = "`write(|w| ..)` method takes [lcdm16::W](lcdm16::W) writer structure"]
impl crate::Writable for LCDM16 {}
#[doc = "LCD Memory 16"]
pub mod lcdm16;
#[doc = "LCD Memory 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm17](lcdm17) module"]
pub type LCDM17 = crate::Reg<u8, _LCDM17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM17;
#[doc = "`read()` method returns [lcdm17::R](lcdm17::R) reader structure"]
impl crate::Readable for LCDM17 {}
#[doc = "`write(|w| ..)` method takes [lcdm17::W](lcdm17::W) writer structure"]
impl crate::Writable for LCDM17 {}
#[doc = "LCD Memory 17"]
pub mod lcdm17;
#[doc = "LCD Memory 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm18](lcdm18) module"]
pub type LCDM18 = crate::Reg<u8, _LCDM18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM18;
#[doc = "`read()` method returns [lcdm18::R](lcdm18::R) reader structure"]
impl crate::Readable for LCDM18 {}
#[doc = "`write(|w| ..)` method takes [lcdm18::W](lcdm18::W) writer structure"]
impl crate::Writable for LCDM18 {}
#[doc = "LCD Memory 18"]
pub mod lcdm18;
#[doc = "LCD Memory 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm19](lcdm19) module"]
pub type LCDM19 = crate::Reg<u8, _LCDM19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM19;
#[doc = "`read()` method returns [lcdm19::R](lcdm19::R) reader structure"]
impl crate::Readable for LCDM19 {}
#[doc = "`write(|w| ..)` method takes [lcdm19::W](lcdm19::W) writer structure"]
impl crate::Writable for LCDM19 {}
#[doc = "LCD Memory 19"]
pub mod lcdm19;
#[doc = "LCD Memory 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm20](lcdm20) module"]
pub type LCDM20 = crate::Reg<u8, _LCDM20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM20;
#[doc = "`read()` method returns [lcdm20::R](lcdm20::R) reader structure"]
impl crate::Readable for LCDM20 {}
#[doc = "`write(|w| ..)` method takes [lcdm20::W](lcdm20::W) writer structure"]
impl crate::Writable for LCDM20 {}
#[doc = "LCD Memory 20"]
pub mod lcdm20;
#[doc = "LCD Memory 21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm21](lcdm21) module"]
pub type LCDM21 = crate::Reg<u8, _LCDM21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM21;
#[doc = "`read()` method returns [lcdm21::R](lcdm21::R) reader structure"]
impl crate::Readable for LCDM21 {}
#[doc = "`write(|w| ..)` method takes [lcdm21::W](lcdm21::W) writer structure"]
impl crate::Writable for LCDM21 {}
#[doc = "LCD Memory 21"]
pub mod lcdm21;
#[doc = "LCD Memory 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm22](lcdm22) module"]
pub type LCDM22 = crate::Reg<u8, _LCDM22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM22;
#[doc = "`read()` method returns [lcdm22::R](lcdm22::R) reader structure"]
impl crate::Readable for LCDM22 {}
#[doc = "`write(|w| ..)` method takes [lcdm22::W](lcdm22::W) writer structure"]
impl crate::Writable for LCDM22 {}
#[doc = "LCD Memory 22"]
pub mod lcdm22;
#[doc = "LCD Memory 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm23](lcdm23) module"]
pub type LCDM23 = crate::Reg<u8, _LCDM23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM23;
#[doc = "`read()` method returns [lcdm23::R](lcdm23::R) reader structure"]
impl crate::Readable for LCDM23 {}
#[doc = "`write(|w| ..)` method takes [lcdm23::W](lcdm23::W) writer structure"]
impl crate::Writable for LCDM23 {}
#[doc = "LCD Memory 23"]
pub mod lcdm23;
#[doc = "LCD Memory 24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm24](lcdm24) module"]
pub type LCDM24 = crate::Reg<u8, _LCDM24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM24;
#[doc = "`read()` method returns [lcdm24::R](lcdm24::R) reader structure"]
impl crate::Readable for LCDM24 {}
#[doc = "`write(|w| ..)` method takes [lcdm24::W](lcdm24::W) writer structure"]
impl crate::Writable for LCDM24 {}
#[doc = "LCD Memory 24"]
pub mod lcdm24;
#[doc = "LCD Memory 25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm25](lcdm25) module"]
pub type LCDM25 = crate::Reg<u8, _LCDM25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM25;
#[doc = "`read()` method returns [lcdm25::R](lcdm25::R) reader structure"]
impl crate::Readable for LCDM25 {}
#[doc = "`write(|w| ..)` method takes [lcdm25::W](lcdm25::W) writer structure"]
impl crate::Writable for LCDM25 {}
#[doc = "LCD Memory 25"]
pub mod lcdm25;
#[doc = "LCD Memory 26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm26](lcdm26) module"]
pub type LCDM26 = crate::Reg<u8, _LCDM26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM26;
#[doc = "`read()` method returns [lcdm26::R](lcdm26::R) reader structure"]
impl crate::Readable for LCDM26 {}
#[doc = "`write(|w| ..)` method takes [lcdm26::W](lcdm26::W) writer structure"]
impl crate::Writable for LCDM26 {}
#[doc = "LCD Memory 26"]
pub mod lcdm26;
#[doc = "LCD Memory 27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm27](lcdm27) module"]
pub type LCDM27 = crate::Reg<u8, _LCDM27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM27;
#[doc = "`read()` method returns [lcdm27::R](lcdm27::R) reader structure"]
impl crate::Readable for LCDM27 {}
#[doc = "`write(|w| ..)` method takes [lcdm27::W](lcdm27::W) writer structure"]
impl crate::Writable for LCDM27 {}
#[doc = "LCD Memory 27"]
pub mod lcdm27;
#[doc = "LCD Memory 28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm28](lcdm28) module"]
pub type LCDM28 = crate::Reg<u8, _LCDM28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM28;
#[doc = "`read()` method returns [lcdm28::R](lcdm28::R) reader structure"]
impl crate::Readable for LCDM28 {}
#[doc = "`write(|w| ..)` method takes [lcdm28::W](lcdm28::W) writer structure"]
impl crate::Writable for LCDM28 {}
#[doc = "LCD Memory 28"]
pub mod lcdm28;
#[doc = "LCD Memory 29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm29](lcdm29) module"]
pub type LCDM29 = crate::Reg<u8, _LCDM29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM29;
#[doc = "`read()` method returns [lcdm29::R](lcdm29::R) reader structure"]
impl crate::Readable for LCDM29 {}
#[doc = "`write(|w| ..)` method takes [lcdm29::W](lcdm29::W) writer structure"]
impl crate::Writable for LCDM29 {}
#[doc = "LCD Memory 29"]
pub mod lcdm29;
#[doc = "LCD Memory 30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm30](lcdm30) module"]
pub type LCDM30 = crate::Reg<u8, _LCDM30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM30;
#[doc = "`read()` method returns [lcdm30::R](lcdm30::R) reader structure"]
impl crate::Readable for LCDM30 {}
#[doc = "`write(|w| ..)` method takes [lcdm30::W](lcdm30::W) writer structure"]
impl crate::Writable for LCDM30 {}
#[doc = "LCD Memory 30"]
pub mod lcdm30;
#[doc = "LCD Memory 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm31](lcdm31) module"]
pub type LCDM31 = crate::Reg<u8, _LCDM31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM31;
#[doc = "`read()` method returns [lcdm31::R](lcdm31::R) reader structure"]
impl crate::Readable for LCDM31 {}
#[doc = "`write(|w| ..)` method takes [lcdm31::W](lcdm31::W) writer structure"]
impl crate::Writable for LCDM31 {}
#[doc = "LCD Memory 31"]
pub mod lcdm31;
#[doc = "LCD Memory 32\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm32](lcdm32) module"]
pub type LCDM32 = crate::Reg<u8, _LCDM32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM32;
#[doc = "`read()` method returns [lcdm32::R](lcdm32::R) reader structure"]
impl crate::Readable for LCDM32 {}
#[doc = "`write(|w| ..)` method takes [lcdm32::W](lcdm32::W) writer structure"]
impl crate::Writable for LCDM32 {}
#[doc = "LCD Memory 32"]
pub mod lcdm32;
#[doc = "LCD Blinking Memory 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm1](lcdbm1) module"]
pub type LCDBM1 = crate::Reg<u8, _LCDBM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM1;
#[doc = "`read()` method returns [lcdbm1::R](lcdbm1::R) reader structure"]
impl crate::Readable for LCDBM1 {}
#[doc = "`write(|w| ..)` method takes [lcdbm1::W](lcdbm1::W) writer structure"]
impl crate::Writable for LCDBM1 {}
#[doc = "LCD Blinking Memory 1"]
pub mod lcdbm1;
#[doc = "LCD Memory 33\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm33](lcdm33) module"]
pub type LCDM33 = crate::Reg<u8, _LCDM33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM33;
#[doc = "`read()` method returns [lcdm33::R](lcdm33::R) reader structure"]
impl crate::Readable for LCDM33 {}
#[doc = "`write(|w| ..)` method takes [lcdm33::W](lcdm33::W) writer structure"]
impl crate::Writable for LCDM33 {}
#[doc = "LCD Memory 33"]
pub mod lcdm33;
#[doc = "LCD Blinking Memory 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm2](lcdbm2) module"]
pub type LCDBM2 = crate::Reg<u8, _LCDBM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM2;
#[doc = "`read()` method returns [lcdbm2::R](lcdbm2::R) reader structure"]
impl crate::Readable for LCDBM2 {}
#[doc = "`write(|w| ..)` method takes [lcdbm2::W](lcdbm2::W) writer structure"]
impl crate::Writable for LCDBM2 {}
#[doc = "LCD Blinking Memory 2"]
pub mod lcdbm2;
#[doc = "LCD Memory 34\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm34](lcdm34) module"]
pub type LCDM34 = crate::Reg<u8, _LCDM34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM34;
#[doc = "`read()` method returns [lcdm34::R](lcdm34::R) reader structure"]
impl crate::Readable for LCDM34 {}
#[doc = "`write(|w| ..)` method takes [lcdm34::W](lcdm34::W) writer structure"]
impl crate::Writable for LCDM34 {}
#[doc = "LCD Memory 34"]
pub mod lcdm34;
#[doc = "LCD Blinking Memory 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm3](lcdbm3) module"]
pub type LCDBM3 = crate::Reg<u8, _LCDBM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM3;
#[doc = "`read()` method returns [lcdbm3::R](lcdbm3::R) reader structure"]
impl crate::Readable for LCDBM3 {}
#[doc = "`write(|w| ..)` method takes [lcdbm3::W](lcdbm3::W) writer structure"]
impl crate::Writable for LCDBM3 {}
#[doc = "LCD Blinking Memory 3"]
pub mod lcdbm3;
#[doc = "LCD Memory 35\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm35](lcdm35) module"]
pub type LCDM35 = crate::Reg<u8, _LCDM35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM35;
#[doc = "`read()` method returns [lcdm35::R](lcdm35::R) reader structure"]
impl crate::Readable for LCDM35 {}
#[doc = "`write(|w| ..)` method takes [lcdm35::W](lcdm35::W) writer structure"]
impl crate::Writable for LCDM35 {}
#[doc = "LCD Memory 35"]
pub mod lcdm35;
#[doc = "LCD Blinking Memory 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm4](lcdbm4) module"]
pub type LCDBM4 = crate::Reg<u8, _LCDBM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM4;
#[doc = "`read()` method returns [lcdbm4::R](lcdbm4::R) reader structure"]
impl crate::Readable for LCDBM4 {}
#[doc = "`write(|w| ..)` method takes [lcdbm4::W](lcdbm4::W) writer structure"]
impl crate::Writable for LCDBM4 {}
#[doc = "LCD Blinking Memory 4"]
pub mod lcdbm4;
#[doc = "LCD Memory 36\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm36](lcdm36) module"]
pub type LCDM36 = crate::Reg<u8, _LCDM36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM36;
#[doc = "`read()` method returns [lcdm36::R](lcdm36::R) reader structure"]
impl crate::Readable for LCDM36 {}
#[doc = "`write(|w| ..)` method takes [lcdm36::W](lcdm36::W) writer structure"]
impl crate::Writable for LCDM36 {}
#[doc = "LCD Memory 36"]
pub mod lcdm36;
#[doc = "LCD Blinking Memory 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm5](lcdbm5) module"]
pub type LCDBM5 = crate::Reg<u8, _LCDBM5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM5;
#[doc = "`read()` method returns [lcdbm5::R](lcdbm5::R) reader structure"]
impl crate::Readable for LCDBM5 {}
#[doc = "`write(|w| ..)` method takes [lcdbm5::W](lcdbm5::W) writer structure"]
impl crate::Writable for LCDBM5 {}
#[doc = "LCD Blinking Memory 5"]
pub mod lcdbm5;
#[doc = "LCD Memory 37\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm37](lcdm37) module"]
pub type LCDM37 = crate::Reg<u8, _LCDM37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM37;
#[doc = "`read()` method returns [lcdm37::R](lcdm37::R) reader structure"]
impl crate::Readable for LCDM37 {}
#[doc = "`write(|w| ..)` method takes [lcdm37::W](lcdm37::W) writer structure"]
impl crate::Writable for LCDM37 {}
#[doc = "LCD Memory 37"]
pub mod lcdm37;
#[doc = "LCD Blinking Memory 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm6](lcdbm6) module"]
pub type LCDBM6 = crate::Reg<u8, _LCDBM6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM6;
#[doc = "`read()` method returns [lcdbm6::R](lcdbm6::R) reader structure"]
impl crate::Readable for LCDBM6 {}
#[doc = "`write(|w| ..)` method takes [lcdbm6::W](lcdbm6::W) writer structure"]
impl crate::Writable for LCDBM6 {}
#[doc = "LCD Blinking Memory 6"]
pub mod lcdbm6;
#[doc = "LCD Memory 38\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm38](lcdm38) module"]
pub type LCDM38 = crate::Reg<u8, _LCDM38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM38;
#[doc = "`read()` method returns [lcdm38::R](lcdm38::R) reader structure"]
impl crate::Readable for LCDM38 {}
#[doc = "`write(|w| ..)` method takes [lcdm38::W](lcdm38::W) writer structure"]
impl crate::Writable for LCDM38 {}
#[doc = "LCD Memory 38"]
pub mod lcdm38;
#[doc = "LCD Blinking Memory 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm7](lcdbm7) module"]
pub type LCDBM7 = crate::Reg<u8, _LCDBM7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM7;
#[doc = "`read()` method returns [lcdbm7::R](lcdbm7::R) reader structure"]
impl crate::Readable for LCDBM7 {}
#[doc = "`write(|w| ..)` method takes [lcdbm7::W](lcdbm7::W) writer structure"]
impl crate::Writable for LCDBM7 {}
#[doc = "LCD Blinking Memory 7"]
pub mod lcdbm7;
#[doc = "LCD Memory 39\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm39](lcdm39) module"]
pub type LCDM39 = crate::Reg<u8, _LCDM39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM39;
#[doc = "`read()` method returns [lcdm39::R](lcdm39::R) reader structure"]
impl crate::Readable for LCDM39 {}
#[doc = "`write(|w| ..)` method takes [lcdm39::W](lcdm39::W) writer structure"]
impl crate::Writable for LCDM39 {}
#[doc = "LCD Memory 39"]
pub mod lcdm39;
#[doc = "LCD Blinking Memory 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm8](lcdbm8) module"]
pub type LCDBM8 = crate::Reg<u8, _LCDBM8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM8;
#[doc = "`read()` method returns [lcdbm8::R](lcdbm8::R) reader structure"]
impl crate::Readable for LCDBM8 {}
#[doc = "`write(|w| ..)` method takes [lcdbm8::W](lcdbm8::W) writer structure"]
impl crate::Writable for LCDBM8 {}
#[doc = "LCD Blinking Memory 8"]
pub mod lcdbm8;
#[doc = "LCD Memory 40\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm40](lcdm40) module"]
pub type LCDM40 = crate::Reg<u8, _LCDM40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM40;
#[doc = "`read()` method returns [lcdm40::R](lcdm40::R) reader structure"]
impl crate::Readable for LCDM40 {}
#[doc = "`write(|w| ..)` method takes [lcdm40::W](lcdm40::W) writer structure"]
impl crate::Writable for LCDM40 {}
#[doc = "LCD Memory 40"]
pub mod lcdm40;
#[doc = "LCD Blinking Memory 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm9](lcdbm9) module"]
pub type LCDBM9 = crate::Reg<u8, _LCDBM9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM9;
#[doc = "`read()` method returns [lcdbm9::R](lcdbm9::R) reader structure"]
impl crate::Readable for LCDBM9 {}
#[doc = "`write(|w| ..)` method takes [lcdbm9::W](lcdbm9::W) writer structure"]
impl crate::Writable for LCDBM9 {}
#[doc = "LCD Blinking Memory 9"]
pub mod lcdbm9;
#[doc = "LCD Memory 41\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm41](lcdm41) module"]
pub type LCDM41 = crate::Reg<u8, _LCDM41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM41;
#[doc = "`read()` method returns [lcdm41::R](lcdm41::R) reader structure"]
impl crate::Readable for LCDM41 {}
#[doc = "`write(|w| ..)` method takes [lcdm41::W](lcdm41::W) writer structure"]
impl crate::Writable for LCDM41 {}
#[doc = "LCD Memory 41"]
pub mod lcdm41;
#[doc = "LCD Blinking Memory 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm10](lcdbm10) module"]
pub type LCDBM10 = crate::Reg<u8, _LCDBM10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM10;
#[doc = "`read()` method returns [lcdbm10::R](lcdbm10::R) reader structure"]
impl crate::Readable for LCDBM10 {}
#[doc = "`write(|w| ..)` method takes [lcdbm10::W](lcdbm10::W) writer structure"]
impl crate::Writable for LCDBM10 {}
#[doc = "LCD Blinking Memory 10"]
pub mod lcdbm10;
#[doc = "LCD Memory 42\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm42](lcdm42) module"]
pub type LCDM42 = crate::Reg<u8, _LCDM42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM42;
#[doc = "`read()` method returns [lcdm42::R](lcdm42::R) reader structure"]
impl crate::Readable for LCDM42 {}
#[doc = "`write(|w| ..)` method takes [lcdm42::W](lcdm42::W) writer structure"]
impl crate::Writable for LCDM42 {}
#[doc = "LCD Memory 42"]
pub mod lcdm42;
#[doc = "LCD Blinking Memory 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm11](lcdbm11) module"]
pub type LCDBM11 = crate::Reg<u8, _LCDBM11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM11;
#[doc = "`read()` method returns [lcdbm11::R](lcdbm11::R) reader structure"]
impl crate::Readable for LCDBM11 {}
#[doc = "`write(|w| ..)` method takes [lcdbm11::W](lcdbm11::W) writer structure"]
impl crate::Writable for LCDBM11 {}
#[doc = "LCD Blinking Memory 11"]
pub mod lcdbm11;
#[doc = "LCD Memory 43\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdm43](lcdm43) module"]
pub type LCDM43 = crate::Reg<u8, _LCDM43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDM43;
#[doc = "`read()` method returns [lcdm43::R](lcdm43::R) reader structure"]
impl crate::Readable for LCDM43 {}
#[doc = "`write(|w| ..)` method takes [lcdm43::W](lcdm43::W) writer structure"]
impl crate::Writable for LCDM43 {}
#[doc = "LCD Memory 43"]
pub mod lcdm43;
#[doc = "LCD Blinking Memory 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm12](lcdbm12) module"]
pub type LCDBM12 = crate::Reg<u8, _LCDBM12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM12;
#[doc = "`read()` method returns [lcdbm12::R](lcdbm12::R) reader structure"]
impl crate::Readable for LCDBM12 {}
#[doc = "`write(|w| ..)` method takes [lcdbm12::W](lcdbm12::W) writer structure"]
impl crate::Writable for LCDBM12 {}
#[doc = "LCD Blinking Memory 12"]
pub mod lcdbm12;
#[doc = "LCD Blinking Memory 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm13](lcdbm13) module"]
pub type LCDBM13 = crate::Reg<u8, _LCDBM13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM13;
#[doc = "`read()` method returns [lcdbm13::R](lcdbm13::R) reader structure"]
impl crate::Readable for LCDBM13 {}
#[doc = "`write(|w| ..)` method takes [lcdbm13::W](lcdbm13::W) writer structure"]
impl crate::Writable for LCDBM13 {}
#[doc = "LCD Blinking Memory 13"]
pub mod lcdbm13;
#[doc = "LCD Blinking Memory 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm14](lcdbm14) module"]
pub type LCDBM14 = crate::Reg<u8, _LCDBM14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM14;
#[doc = "`read()` method returns [lcdbm14::R](lcdbm14::R) reader structure"]
impl crate::Readable for LCDBM14 {}
#[doc = "`write(|w| ..)` method takes [lcdbm14::W](lcdbm14::W) writer structure"]
impl crate::Writable for LCDBM14 {}
#[doc = "LCD Blinking Memory 14"]
pub mod lcdbm14;
#[doc = "LCD Blinking Memory 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm15](lcdbm15) module"]
pub type LCDBM15 = crate::Reg<u8, _LCDBM15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM15;
#[doc = "`read()` method returns [lcdbm15::R](lcdbm15::R) reader structure"]
impl crate::Readable for LCDBM15 {}
#[doc = "`write(|w| ..)` method takes [lcdbm15::W](lcdbm15::W) writer structure"]
impl crate::Writable for LCDBM15 {}
#[doc = "LCD Blinking Memory 15"]
pub mod lcdbm15;
#[doc = "LCD Blinking Memory 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm16](lcdbm16) module"]
pub type LCDBM16 = crate::Reg<u8, _LCDBM16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM16;
#[doc = "`read()` method returns [lcdbm16::R](lcdbm16::R) reader structure"]
impl crate::Readable for LCDBM16 {}
#[doc = "`write(|w| ..)` method takes [lcdbm16::W](lcdbm16::W) writer structure"]
impl crate::Writable for LCDBM16 {}
#[doc = "LCD Blinking Memory 16"]
pub mod lcdbm16;
#[doc = "LCD Blinking Memory 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm17](lcdbm17) module"]
pub type LCDBM17 = crate::Reg<u8, _LCDBM17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM17;
#[doc = "`read()` method returns [lcdbm17::R](lcdbm17::R) reader structure"]
impl crate::Readable for LCDBM17 {}
#[doc = "`write(|w| ..)` method takes [lcdbm17::W](lcdbm17::W) writer structure"]
impl crate::Writable for LCDBM17 {}
#[doc = "LCD Blinking Memory 17"]
pub mod lcdbm17;
#[doc = "LCD Blinking Memory 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm18](lcdbm18) module"]
pub type LCDBM18 = crate::Reg<u8, _LCDBM18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM18;
#[doc = "`read()` method returns [lcdbm18::R](lcdbm18::R) reader structure"]
impl crate::Readable for LCDBM18 {}
#[doc = "`write(|w| ..)` method takes [lcdbm18::W](lcdbm18::W) writer structure"]
impl crate::Writable for LCDBM18 {}
#[doc = "LCD Blinking Memory 18"]
pub mod lcdbm18;
#[doc = "LCD Blinking Memory 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm19](lcdbm19) module"]
pub type LCDBM19 = crate::Reg<u8, _LCDBM19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM19;
#[doc = "`read()` method returns [lcdbm19::R](lcdbm19::R) reader structure"]
impl crate::Readable for LCDBM19 {}
#[doc = "`write(|w| ..)` method takes [lcdbm19::W](lcdbm19::W) writer structure"]
impl crate::Writable for LCDBM19 {}
#[doc = "LCD Blinking Memory 19"]
pub mod lcdbm19;
#[doc = "LCD Blinking Memory 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm20](lcdbm20) module"]
pub type LCDBM20 = crate::Reg<u8, _LCDBM20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM20;
#[doc = "`read()` method returns [lcdbm20::R](lcdbm20::R) reader structure"]
impl crate::Readable for LCDBM20 {}
#[doc = "`write(|w| ..)` method takes [lcdbm20::W](lcdbm20::W) writer structure"]
impl crate::Writable for LCDBM20 {}
#[doc = "LCD Blinking Memory 20"]
pub mod lcdbm20;
#[doc = "LCD Blinking Memory 21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm21](lcdbm21) module"]
pub type LCDBM21 = crate::Reg<u8, _LCDBM21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM21;
#[doc = "`read()` method returns [lcdbm21::R](lcdbm21::R) reader structure"]
impl crate::Readable for LCDBM21 {}
#[doc = "`write(|w| ..)` method takes [lcdbm21::W](lcdbm21::W) writer structure"]
impl crate::Writable for LCDBM21 {}
#[doc = "LCD Blinking Memory 21"]
pub mod lcdbm21;
#[doc = "LCD Blinking Memory 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm22](lcdbm22) module"]
pub type LCDBM22 = crate::Reg<u8, _LCDBM22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDBM22;
#[doc = "`read()` method returns [lcdbm22::R](lcdbm22::R) reader structure"]
impl crate::Readable for LCDBM22 {}
#[doc = "`write(|w| ..)` method takes [lcdbm22::W](lcdbm22::W) writer structure"]
impl crate::Writable for LCDBM22 {}
#[doc = "LCD Blinking Memory 22"]
pub mod lcdbm22;
#[doc = "LCD_C Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcctl0](lcdcctl0) module"]
pub type LCDCCTL0 = crate::Reg<u16, _LCDCCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCCTL0;
#[doc = "`read()` method returns [lcdcctl0::R](lcdcctl0::R) reader structure"]
impl crate::Readable for LCDCCTL0 {}
#[doc = "`write(|w| ..)` method takes [lcdcctl0::W](lcdcctl0::W) writer structure"]
impl crate::Writable for LCDCCTL0 {}
#[doc = "LCD_C Control Register 0"]
pub mod lcdcctl0;
#[doc = "LCD_C Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcctl1](lcdcctl1) module"]
pub type LCDCCTL1 = crate::Reg<u16, _LCDCCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCCTL1;
#[doc = "`read()` method returns [lcdcctl1::R](lcdcctl1::R) reader structure"]
impl crate::Readable for LCDCCTL1 {}
#[doc = "`write(|w| ..)` method takes [lcdcctl1::W](lcdcctl1::W) writer structure"]
impl crate::Writable for LCDCCTL1 {}
#[doc = "LCD_C Control Register 1"]
pub mod lcdcctl1;
#[doc = "LCD_C blinking control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcblkctl](lcdcblkctl) module"]
pub type LCDCBLKCTL = crate::Reg<u16, _LCDCBLKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCBLKCTL;
#[doc = "`read()` method returns [lcdcblkctl::R](lcdcblkctl::R) reader structure"]
impl crate::Readable for LCDCBLKCTL {}
#[doc = "`write(|w| ..)` method takes [lcdcblkctl::W](lcdcblkctl::W) writer structure"]
impl crate::Writable for LCDCBLKCTL {}
#[doc = "LCD_C blinking control register"]
pub mod lcdcblkctl;
#[doc = "LCD_C memory control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcmemctl](lcdcmemctl) module"]
pub type LCDCMEMCTL = crate::Reg<u16, _LCDCMEMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCMEMCTL;
#[doc = "`read()` method returns [lcdcmemctl::R](lcdcmemctl::R) reader structure"]
impl crate::Readable for LCDCMEMCTL {}
#[doc = "`write(|w| ..)` method takes [lcdcmemctl::W](lcdcmemctl::W) writer structure"]
impl crate::Writable for LCDCMEMCTL {}
#[doc = "LCD_C memory control register"]
pub mod lcdcmemctl;
#[doc = "LCD_C Voltage Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcvctl](lcdcvctl) module"]
pub type LCDCVCTL = crate::Reg<u16, _LCDCVCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCVCTL;
#[doc = "`read()` method returns [lcdcvctl::R](lcdcvctl::R) reader structure"]
impl crate::Readable for LCDCVCTL {}
#[doc = "`write(|w| ..)` method takes [lcdcvctl::W](lcdcvctl::W) writer structure"]
impl crate::Writable for LCDCVCTL {}
#[doc = "LCD_C Voltage Control Register"]
pub mod lcdcvctl;
#[doc = "LCD_C Port Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcpctl0](lcdcpctl0) module"]
pub type LCDCPCTL0 = crate::Reg<u16, _LCDCPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCPCTL0;
#[doc = "`read()` method returns [lcdcpctl0::R](lcdcpctl0::R) reader structure"]
impl crate::Readable for LCDCPCTL0 {}
#[doc = "`write(|w| ..)` method takes [lcdcpctl0::W](lcdcpctl0::W) writer structure"]
impl crate::Writable for LCDCPCTL0 {}
#[doc = "LCD_C Port Control Register 0"]
pub mod lcdcpctl0;
#[doc = "LCD_C Port Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcpctl1](lcdcpctl1) module"]
pub type LCDCPCTL1 = crate::Reg<u16, _LCDCPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCPCTL1;
#[doc = "`read()` method returns [lcdcpctl1::R](lcdcpctl1::R) reader structure"]
impl crate::Readable for LCDCPCTL1 {}
#[doc = "`write(|w| ..)` method takes [lcdcpctl1::W](lcdcpctl1::W) writer structure"]
impl crate::Writable for LCDCPCTL1 {}
#[doc = "LCD_C Port Control Register 1"]
pub mod lcdcpctl1;
#[doc = "LCD_C Port Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcpctl2](lcdcpctl2) module"]
pub type LCDCPCTL2 = crate::Reg<u16, _LCDCPCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCPCTL2;
#[doc = "`read()` method returns [lcdcpctl2::R](lcdcpctl2::R) reader structure"]
impl crate::Readable for LCDCPCTL2 {}
#[doc = "`write(|w| ..)` method takes [lcdcpctl2::W](lcdcpctl2::W) writer structure"]
impl crate::Writable for LCDCPCTL2 {}
#[doc = "LCD_C Port Control Register 2"]
pub mod lcdcpctl2;
#[doc = "LCD_C Charge Pump Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdccpctl](lcdccpctl) module"]
pub type LCDCCPCTL = crate::Reg<u16, _LCDCCPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCCPCTL;
#[doc = "`read()` method returns [lcdccpctl::R](lcdccpctl::R) reader structure"]
impl crate::Readable for LCDCCPCTL {}
#[doc = "`write(|w| ..)` method takes [lcdccpctl::W](lcdccpctl::W) writer structure"]
impl crate::Writable for LCDCCPCTL {}
#[doc = "LCD_C Charge Pump Control Register 3"]
pub mod lcdccpctl;
#[doc = "LCD_C Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdciv](lcdciv) module"]
pub type LCDCIV = crate::Reg<u16, _LCDCIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCIV;
#[doc = "`read()` method returns [lcdciv::R](lcdciv::R) reader structure"]
impl crate::Readable for LCDCIV {}
#[doc = "`write(|w| ..)` method takes [lcdciv::W](lcdciv::W) writer structure"]
impl crate::Writable for LCDCIV {}
#[doc = "LCD_C Interrupt Vector Register"]
pub mod lcdciv;
