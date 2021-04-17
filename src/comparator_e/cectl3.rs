#[doc = "Reader of register CECTL3"]
pub type R = crate::R<u16, super::CECTL3>;
#[doc = "Writer for register CECTL3"]
pub type W = crate::W<u16, super::CECTL3>;
#[doc = "Register CECTL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CECTL3 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CEPD0`"]
pub type CEPD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD0`"]
pub struct CEPD0_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CEPD1`"]
pub type CEPD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD1`"]
pub struct CEPD1_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CEPD2`"]
pub type CEPD2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD2`"]
pub struct CEPD2_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CEPD3`"]
pub type CEPD3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD3`"]
pub struct CEPD3_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CEPD4`"]
pub type CEPD4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD4`"]
pub struct CEPD4_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CEPD5`"]
pub type CEPD5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD5`"]
pub struct CEPD5_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CEPD6`"]
pub type CEPD6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD6`"]
pub struct CEPD6_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CEPD7`"]
pub type CEPD7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD7`"]
pub struct CEPD7_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CEPD8`"]
pub type CEPD8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD8`"]
pub struct CEPD8_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CEPD9`"]
pub type CEPD9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD9`"]
pub struct CEPD9_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CEPD10`"]
pub type CEPD10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD10`"]
pub struct CEPD10_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CEPD11`"]
pub type CEPD11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD11`"]
pub struct CEPD11_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CEPD12`"]
pub type CEPD12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD12`"]
pub struct CEPD12_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `CEPD13`"]
pub type CEPD13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD13`"]
pub struct CEPD13_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `CEPD14`"]
pub type CEPD14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD14`"]
pub struct CEPD14_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CEPD15`"]
pub type CEPD15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEPD15`"]
pub struct CEPD15_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comp. E Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    pub fn cepd0(&self) -> CEPD0_R {
        CEPD0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comp. E Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    pub fn cepd1(&self) -> CEPD1_R {
        CEPD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comp. E Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    pub fn cepd2(&self) -> CEPD2_R {
        CEPD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comp. E Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    pub fn cepd3(&self) -> CEPD3_R {
        CEPD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comp. E Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    pub fn cepd4(&self) -> CEPD4_R {
        CEPD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comp. E Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    pub fn cepd5(&self) -> CEPD5_R {
        CEPD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Comp. E Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    pub fn cepd6(&self) -> CEPD6_R {
        CEPD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Comp. E Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    pub fn cepd7(&self) -> CEPD7_R {
        CEPD7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Comp. E Disable Input Buffer of Port Register .8"]
    #[inline(always)]
    pub fn cepd8(&self) -> CEPD8_R {
        CEPD8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comp. E Disable Input Buffer of Port Register .9"]
    #[inline(always)]
    pub fn cepd9(&self) -> CEPD9_R {
        CEPD9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Comp. E Disable Input Buffer of Port Register .10"]
    #[inline(always)]
    pub fn cepd10(&self) -> CEPD10_R {
        CEPD10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Comp. E Disable Input Buffer of Port Register .11"]
    #[inline(always)]
    pub fn cepd11(&self) -> CEPD11_R {
        CEPD11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comp. E Disable Input Buffer of Port Register .12"]
    #[inline(always)]
    pub fn cepd12(&self) -> CEPD12_R {
        CEPD12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Comp. E Disable Input Buffer of Port Register .13"]
    #[inline(always)]
    pub fn cepd13(&self) -> CEPD13_R {
        CEPD13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Comp. E Disable Input Buffer of Port Register .14"]
    #[inline(always)]
    pub fn cepd14(&self) -> CEPD14_R {
        CEPD14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comp. E Disable Input Buffer of Port Register .15"]
    #[inline(always)]
    pub fn cepd15(&self) -> CEPD15_R {
        CEPD15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. E Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    pub fn cepd0(&mut self) -> CEPD0_W {
        CEPD0_W { w: self }
    }
    #[doc = "Bit 1 - Comp. E Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    pub fn cepd1(&mut self) -> CEPD1_W {
        CEPD1_W { w: self }
    }
    #[doc = "Bit 2 - Comp. E Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    pub fn cepd2(&mut self) -> CEPD2_W {
        CEPD2_W { w: self }
    }
    #[doc = "Bit 3 - Comp. E Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    pub fn cepd3(&mut self) -> CEPD3_W {
        CEPD3_W { w: self }
    }
    #[doc = "Bit 4 - Comp. E Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    pub fn cepd4(&mut self) -> CEPD4_W {
        CEPD4_W { w: self }
    }
    #[doc = "Bit 5 - Comp. E Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    pub fn cepd5(&mut self) -> CEPD5_W {
        CEPD5_W { w: self }
    }
    #[doc = "Bit 6 - Comp. E Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    pub fn cepd6(&mut self) -> CEPD6_W {
        CEPD6_W { w: self }
    }
    #[doc = "Bit 7 - Comp. E Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    pub fn cepd7(&mut self) -> CEPD7_W {
        CEPD7_W { w: self }
    }
    #[doc = "Bit 8 - Comp. E Disable Input Buffer of Port Register .8"]
    #[inline(always)]
    pub fn cepd8(&mut self) -> CEPD8_W {
        CEPD8_W { w: self }
    }
    #[doc = "Bit 9 - Comp. E Disable Input Buffer of Port Register .9"]
    #[inline(always)]
    pub fn cepd9(&mut self) -> CEPD9_W {
        CEPD9_W { w: self }
    }
    #[doc = "Bit 10 - Comp. E Disable Input Buffer of Port Register .10"]
    #[inline(always)]
    pub fn cepd10(&mut self) -> CEPD10_W {
        CEPD10_W { w: self }
    }
    #[doc = "Bit 11 - Comp. E Disable Input Buffer of Port Register .11"]
    #[inline(always)]
    pub fn cepd11(&mut self) -> CEPD11_W {
        CEPD11_W { w: self }
    }
    #[doc = "Bit 12 - Comp. E Disable Input Buffer of Port Register .12"]
    #[inline(always)]
    pub fn cepd12(&mut self) -> CEPD12_W {
        CEPD12_W { w: self }
    }
    #[doc = "Bit 13 - Comp. E Disable Input Buffer of Port Register .13"]
    #[inline(always)]
    pub fn cepd13(&mut self) -> CEPD13_W {
        CEPD13_W { w: self }
    }
    #[doc = "Bit 14 - Comp. E Disable Input Buffer of Port Register .14"]
    #[inline(always)]
    pub fn cepd14(&mut self) -> CEPD14_W {
        CEPD14_W { w: self }
    }
    #[doc = "Bit 15 - Comp. E Disable Input Buffer of Port Register .15"]
    #[inline(always)]
    pub fn cepd15(&mut self) -> CEPD15_W {
        CEPD15_W { w: self }
    }
}
