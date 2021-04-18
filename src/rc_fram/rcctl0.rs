#[doc = "Register `RCCTL0` reader"]
pub struct R(crate::R<RCCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RCCTL0_SPEC>> for R {
    fn from(reader: crate::R<RCCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCCTL0` writer"]
pub struct W(crate::W<RCCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<RCCTL0_SPEC>> for W {
    fn from(writer: crate::W<RCCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RAM Controller RAM Sector 0 Off Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RCRS0OFF_A {
    #[doc = "0: RAM Controller RAM Sector 0 Off : 0"]
    RCRS0OFF_0 = 0,
    #[doc = "1: RAM Controller RAM Sector 0 Off : 1"]
    RCRS0OFF_1 = 1,
    #[doc = "2: RAM Controller RAM Sector 0 Off : 2"]
    RCRS0OFF_2 = 2,
    #[doc = "3: RAM Controller RAM Sector 0 Off : 3"]
    RCRS0OFF_3 = 3,
}
impl From<RCRS0OFF_A> for u8 {
    #[inline(always)]
    fn from(variant: RCRS0OFF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RCRS0OFF` reader - RAM Controller RAM Sector 0 Off Bit: 0"]
pub struct RCRS0OFF_R(crate::FieldReader<u8, RCRS0OFF_A>);
impl RCRS0OFF_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCRS0OFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCRS0OFF_A {
        match self.bits {
            0 => RCRS0OFF_A::RCRS0OFF_0,
            1 => RCRS0OFF_A::RCRS0OFF_1,
            2 => RCRS0OFF_A::RCRS0OFF_2,
            3 => RCRS0OFF_A::RCRS0OFF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_0`"]
    #[inline(always)]
    pub fn is_rcrs0off_0(&self) -> bool {
        **self == RCRS0OFF_A::RCRS0OFF_0
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_1`"]
    #[inline(always)]
    pub fn is_rcrs0off_1(&self) -> bool {
        **self == RCRS0OFF_A::RCRS0OFF_1
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_2`"]
    #[inline(always)]
    pub fn is_rcrs0off_2(&self) -> bool {
        **self == RCRS0OFF_A::RCRS0OFF_2
    }
    #[doc = "Checks if the value of the field is `RCRS0OFF_3`"]
    #[inline(always)]
    pub fn is_rcrs0off_3(&self) -> bool {
        **self == RCRS0OFF_A::RCRS0OFF_3
    }
}
impl core::ops::Deref for RCRS0OFF_R {
    type Target = crate::FieldReader<u8, RCRS0OFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRS0OFF` writer - RAM Controller RAM Sector 0 Off Bit: 0"]
pub struct RCRS0OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS0OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRS0OFF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "RAM Controller RAM Sector 0 Off : 0"]
    #[inline(always)]
    pub fn rcrs0off_0(self) -> &'a mut W {
        self.variant(RCRS0OFF_A::RCRS0OFF_0)
    }
    #[doc = "RAM Controller RAM Sector 0 Off : 1"]
    #[inline(always)]
    pub fn rcrs0off_1(self) -> &'a mut W {
        self.variant(RCRS0OFF_A::RCRS0OFF_1)
    }
    #[doc = "RAM Controller RAM Sector 0 Off : 2"]
    #[inline(always)]
    pub fn rcrs0off_2(self) -> &'a mut W {
        self.variant(RCRS0OFF_A::RCRS0OFF_2)
    }
    #[doc = "RAM Controller RAM Sector 0 Off : 3"]
    #[inline(always)]
    pub fn rcrs0off_3(self) -> &'a mut W {
        self.variant(RCRS0OFF_A::RCRS0OFF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u16 & 0x03);
        self.w
    }
}
#[doc = "RAM Controller RAM Sector 1 Off Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RCRS1OFF_A {
    #[doc = "0: RAM Controller RAM Sector 1 Off : 0"]
    RCRS1OFF_0 = 0,
    #[doc = "1: RAM Controller RAM Sector 1 Off : 1"]
    RCRS1OFF_1 = 1,
    #[doc = "2: RAM Controller RAM Sector 1 Off : 2"]
    RCRS1OFF_2 = 2,
    #[doc = "3: RAM Controller RAM Sector 1 Off : 3"]
    RCRS1OFF_3 = 3,
}
impl From<RCRS1OFF_A> for u8 {
    #[inline(always)]
    fn from(variant: RCRS1OFF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RCRS1OFF` reader - RAM Controller RAM Sector 1 Off Bit: 0"]
pub struct RCRS1OFF_R(crate::FieldReader<u8, RCRS1OFF_A>);
impl RCRS1OFF_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCRS1OFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCRS1OFF_A {
        match self.bits {
            0 => RCRS1OFF_A::RCRS1OFF_0,
            1 => RCRS1OFF_A::RCRS1OFF_1,
            2 => RCRS1OFF_A::RCRS1OFF_2,
            3 => RCRS1OFF_A::RCRS1OFF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RCRS1OFF_0`"]
    #[inline(always)]
    pub fn is_rcrs1off_0(&self) -> bool {
        **self == RCRS1OFF_A::RCRS1OFF_0
    }
    #[doc = "Checks if the value of the field is `RCRS1OFF_1`"]
    #[inline(always)]
    pub fn is_rcrs1off_1(&self) -> bool {
        **self == RCRS1OFF_A::RCRS1OFF_1
    }
    #[doc = "Checks if the value of the field is `RCRS1OFF_2`"]
    #[inline(always)]
    pub fn is_rcrs1off_2(&self) -> bool {
        **self == RCRS1OFF_A::RCRS1OFF_2
    }
    #[doc = "Checks if the value of the field is `RCRS1OFF_3`"]
    #[inline(always)]
    pub fn is_rcrs1off_3(&self) -> bool {
        **self == RCRS1OFF_A::RCRS1OFF_3
    }
}
impl core::ops::Deref for RCRS1OFF_R {
    type Target = crate::FieldReader<u8, RCRS1OFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRS1OFF` writer - RAM Controller RAM Sector 1 Off Bit: 0"]
pub struct RCRS1OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS1OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRS1OFF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "RAM Controller RAM Sector 1 Off : 0"]
    #[inline(always)]
    pub fn rcrs1off_0(self) -> &'a mut W {
        self.variant(RCRS1OFF_A::RCRS1OFF_0)
    }
    #[doc = "RAM Controller RAM Sector 1 Off : 1"]
    #[inline(always)]
    pub fn rcrs1off_1(self) -> &'a mut W {
        self.variant(RCRS1OFF_A::RCRS1OFF_1)
    }
    #[doc = "RAM Controller RAM Sector 1 Off : 2"]
    #[inline(always)]
    pub fn rcrs1off_2(self) -> &'a mut W {
        self.variant(RCRS1OFF_A::RCRS1OFF_2)
    }
    #[doc = "RAM Controller RAM Sector 1 Off : 3"]
    #[inline(always)]
    pub fn rcrs1off_3(self) -> &'a mut W {
        self.variant(RCRS1OFF_A::RCRS1OFF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u16 & 0x03) << 2);
        self.w
    }
}
#[doc = "RAM Controller RAM Sector 2 Off Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RCRS2OFF_A {
    #[doc = "0: RAM Controller RAM Sector 2 Off : 0"]
    RCRS2OFF_0 = 0,
    #[doc = "1: RAM Controller RAM Sector 2 Off : 1"]
    RCRS2OFF_1 = 1,
    #[doc = "2: RAM Controller RAM Sector 2 Off : 2"]
    RCRS2OFF_2 = 2,
    #[doc = "3: RAM Controller RAM Sector 2 Off : 3"]
    RCRS2OFF_3 = 3,
}
impl From<RCRS2OFF_A> for u8 {
    #[inline(always)]
    fn from(variant: RCRS2OFF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RCRS2OFF` reader - RAM Controller RAM Sector 2 Off Bit: 0"]
pub struct RCRS2OFF_R(crate::FieldReader<u8, RCRS2OFF_A>);
impl RCRS2OFF_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCRS2OFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCRS2OFF_A {
        match self.bits {
            0 => RCRS2OFF_A::RCRS2OFF_0,
            1 => RCRS2OFF_A::RCRS2OFF_1,
            2 => RCRS2OFF_A::RCRS2OFF_2,
            3 => RCRS2OFF_A::RCRS2OFF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RCRS2OFF_0`"]
    #[inline(always)]
    pub fn is_rcrs2off_0(&self) -> bool {
        **self == RCRS2OFF_A::RCRS2OFF_0
    }
    #[doc = "Checks if the value of the field is `RCRS2OFF_1`"]
    #[inline(always)]
    pub fn is_rcrs2off_1(&self) -> bool {
        **self == RCRS2OFF_A::RCRS2OFF_1
    }
    #[doc = "Checks if the value of the field is `RCRS2OFF_2`"]
    #[inline(always)]
    pub fn is_rcrs2off_2(&self) -> bool {
        **self == RCRS2OFF_A::RCRS2OFF_2
    }
    #[doc = "Checks if the value of the field is `RCRS2OFF_3`"]
    #[inline(always)]
    pub fn is_rcrs2off_3(&self) -> bool {
        **self == RCRS2OFF_A::RCRS2OFF_3
    }
}
impl core::ops::Deref for RCRS2OFF_R {
    type Target = crate::FieldReader<u8, RCRS2OFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRS2OFF` writer - RAM Controller RAM Sector 2 Off Bit: 0"]
pub struct RCRS2OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS2OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRS2OFF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "RAM Controller RAM Sector 2 Off : 0"]
    #[inline(always)]
    pub fn rcrs2off_0(self) -> &'a mut W {
        self.variant(RCRS2OFF_A::RCRS2OFF_0)
    }
    #[doc = "RAM Controller RAM Sector 2 Off : 1"]
    #[inline(always)]
    pub fn rcrs2off_1(self) -> &'a mut W {
        self.variant(RCRS2OFF_A::RCRS2OFF_1)
    }
    #[doc = "RAM Controller RAM Sector 2 Off : 2"]
    #[inline(always)]
    pub fn rcrs2off_2(self) -> &'a mut W {
        self.variant(RCRS2OFF_A::RCRS2OFF_2)
    }
    #[doc = "RAM Controller RAM Sector 2 Off : 3"]
    #[inline(always)]
    pub fn rcrs2off_3(self) -> &'a mut W {
        self.variant(RCRS2OFF_A::RCRS2OFF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u16 & 0x03) << 4);
        self.w
    }
}
#[doc = "RAM Controller RAM Sector 3 Off Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RCRS3OFF_A {
    #[doc = "0: RAM Controller RAM Sector 3 Off : 0"]
    RCRS3OFF_0 = 0,
    #[doc = "1: RAM Controller RAM Sector 3 Off : 1"]
    RCRS3OFF_1 = 1,
    #[doc = "2: RAM Controller RAM Sector 3 Off : 2"]
    RCRS3OFF_2 = 2,
    #[doc = "3: RAM Controller RAM Sector 3 Off : 3"]
    RCRS3OFF_3 = 3,
}
impl From<RCRS3OFF_A> for u8 {
    #[inline(always)]
    fn from(variant: RCRS3OFF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RCRS3OFF` reader - RAM Controller RAM Sector 3 Off Bit: 0"]
pub struct RCRS3OFF_R(crate::FieldReader<u8, RCRS3OFF_A>);
impl RCRS3OFF_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCRS3OFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCRS3OFF_A {
        match self.bits {
            0 => RCRS3OFF_A::RCRS3OFF_0,
            1 => RCRS3OFF_A::RCRS3OFF_1,
            2 => RCRS3OFF_A::RCRS3OFF_2,
            3 => RCRS3OFF_A::RCRS3OFF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RCRS3OFF_0`"]
    #[inline(always)]
    pub fn is_rcrs3off_0(&self) -> bool {
        **self == RCRS3OFF_A::RCRS3OFF_0
    }
    #[doc = "Checks if the value of the field is `RCRS3OFF_1`"]
    #[inline(always)]
    pub fn is_rcrs3off_1(&self) -> bool {
        **self == RCRS3OFF_A::RCRS3OFF_1
    }
    #[doc = "Checks if the value of the field is `RCRS3OFF_2`"]
    #[inline(always)]
    pub fn is_rcrs3off_2(&self) -> bool {
        **self == RCRS3OFF_A::RCRS3OFF_2
    }
    #[doc = "Checks if the value of the field is `RCRS3OFF_3`"]
    #[inline(always)]
    pub fn is_rcrs3off_3(&self) -> bool {
        **self == RCRS3OFF_A::RCRS3OFF_3
    }
}
impl core::ops::Deref for RCRS3OFF_R {
    type Target = crate::FieldReader<u8, RCRS3OFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRS3OFF` writer - RAM Controller RAM Sector 3 Off Bit: 0"]
pub struct RCRS3OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRS3OFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCRS3OFF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "RAM Controller RAM Sector 3 Off : 0"]
    #[inline(always)]
    pub fn rcrs3off_0(self) -> &'a mut W {
        self.variant(RCRS3OFF_A::RCRS3OFF_0)
    }
    #[doc = "RAM Controller RAM Sector 3 Off : 1"]
    #[inline(always)]
    pub fn rcrs3off_1(self) -> &'a mut W {
        self.variant(RCRS3OFF_A::RCRS3OFF_1)
    }
    #[doc = "RAM Controller RAM Sector 3 Off : 2"]
    #[inline(always)]
    pub fn rcrs3off_2(self) -> &'a mut W {
        self.variant(RCRS3OFF_A::RCRS3OFF_2)
    }
    #[doc = "RAM Controller RAM Sector 3 Off : 3"]
    #[inline(always)]
    pub fn rcrs3off_3(self) -> &'a mut W {
        self.variant(RCRS3OFF_A::RCRS3OFF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u16 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RAM Controller RAM Sector 0 Off Bit: 0"]
    #[inline(always)]
    pub fn rcrs0off(&self) -> RCRS0OFF_R {
        RCRS0OFF_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - RAM Controller RAM Sector 1 Off Bit: 0"]
    #[inline(always)]
    pub fn rcrs1off(&self) -> RCRS1OFF_R {
        RCRS1OFF_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - RAM Controller RAM Sector 2 Off Bit: 0"]
    #[inline(always)]
    pub fn rcrs2off(&self) -> RCRS2OFF_R {
        RCRS2OFF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - RAM Controller RAM Sector 3 Off Bit: 0"]
    #[inline(always)]
    pub fn rcrs3off(&self) -> RCRS3OFF_R {
        RCRS3OFF_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RAM Controller RAM Sector 0 Off Bit: 0"]
    #[inline(always)]
    pub fn rcrs0off(&mut self) -> RCRS0OFF_W {
        RCRS0OFF_W { w: self }
    }
    #[doc = "Bits 2:3 - RAM Controller RAM Sector 1 Off Bit: 0"]
    #[inline(always)]
    pub fn rcrs1off(&mut self) -> RCRS1OFF_W {
        RCRS1OFF_W { w: self }
    }
    #[doc = "Bits 4:5 - RAM Controller RAM Sector 2 Off Bit: 0"]
    #[inline(always)]
    pub fn rcrs2off(&mut self) -> RCRS2OFF_W {
        RCRS2OFF_W { w: self }
    }
    #[doc = "Bits 6:7 - RAM Controller RAM Sector 3 Off Bit: 0"]
    #[inline(always)]
    pub fn rcrs3off(&mut self) -> RCRS3OFF_W {
        RCRS3OFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ram Controller Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcctl0](index.html) module"]
pub struct RCCTL0_SPEC;
impl crate::RegisterSpec for RCCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rcctl0::R](R) reader structure"]
impl crate::Readable for RCCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcctl0::W](W) writer structure"]
impl crate::Writable for RCCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCCTL0 to value 0"]
impl crate::Resettable for RCCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
