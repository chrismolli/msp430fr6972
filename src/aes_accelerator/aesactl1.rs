#[doc = "Reader of register AESACTL1"]
pub type R = crate::R<u16, super::AESACTL1>;
#[doc = "Writer for register AESACTL1"]
pub type W = crate::W<u16, super::AESACTL1>;
#[doc = "Register AESACTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AESACTL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AESBLKCNT0`"]
pub type AESBLKCNT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESBLKCNT0`"]
pub struct AESBLKCNT0_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT0_W<'a> {
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
#[doc = "Reader of field `AESBLKCNT1`"]
pub type AESBLKCNT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESBLKCNT1`"]
pub struct AESBLKCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT1_W<'a> {
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
#[doc = "Reader of field `AESBLKCNT2`"]
pub type AESBLKCNT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESBLKCNT2`"]
pub struct AESBLKCNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT2_W<'a> {
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
#[doc = "Reader of field `AESBLKCNT3`"]
pub type AESBLKCNT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESBLKCNT3`"]
pub struct AESBLKCNT3_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT3_W<'a> {
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
#[doc = "Reader of field `AESBLKCNT4`"]
pub type AESBLKCNT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESBLKCNT4`"]
pub struct AESBLKCNT4_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT4_W<'a> {
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
#[doc = "Reader of field `AESBLKCNT5`"]
pub type AESBLKCNT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESBLKCNT5`"]
pub struct AESBLKCNT5_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT5_W<'a> {
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
#[doc = "Reader of field `AESBLKCNT6`"]
pub type AESBLKCNT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESBLKCNT6`"]
pub struct AESBLKCNT6_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT6_W<'a> {
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
#[doc = "Reader of field `AESBLKCNT7`"]
pub type AESBLKCNT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESBLKCNT7`"]
pub struct AESBLKCNT7_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNT7_W<'a> {
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
impl R {
    #[doc = "Bit 0 - AES Cipher Block Counter Bit: 0"]
    #[inline(always)]
    pub fn aesblkcnt0(&self) -> AESBLKCNT0_R {
        AESBLKCNT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AES Cipher Block Counter Bit: 1"]
    #[inline(always)]
    pub fn aesblkcnt1(&self) -> AESBLKCNT1_R {
        AESBLKCNT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AES Cipher Block Counter Bit: 2"]
    #[inline(always)]
    pub fn aesblkcnt2(&self) -> AESBLKCNT2_R {
        AESBLKCNT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AES Cipher Block Counter Bit: 3"]
    #[inline(always)]
    pub fn aesblkcnt3(&self) -> AESBLKCNT3_R {
        AESBLKCNT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AES Cipher Block Counter Bit: 4"]
    #[inline(always)]
    pub fn aesblkcnt4(&self) -> AESBLKCNT4_R {
        AESBLKCNT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AES Cipher Block Counter Bit: 5"]
    #[inline(always)]
    pub fn aesblkcnt5(&self) -> AESBLKCNT5_R {
        AESBLKCNT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AES Cipher Block Counter Bit: 6"]
    #[inline(always)]
    pub fn aesblkcnt6(&self) -> AESBLKCNT6_R {
        AESBLKCNT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AES Cipher Block Counter Bit: 7"]
    #[inline(always)]
    pub fn aesblkcnt7(&self) -> AESBLKCNT7_R {
        AESBLKCNT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Cipher Block Counter Bit: 0"]
    #[inline(always)]
    pub fn aesblkcnt0(&mut self) -> AESBLKCNT0_W {
        AESBLKCNT0_W { w: self }
    }
    #[doc = "Bit 1 - AES Cipher Block Counter Bit: 1"]
    #[inline(always)]
    pub fn aesblkcnt1(&mut self) -> AESBLKCNT1_W {
        AESBLKCNT1_W { w: self }
    }
    #[doc = "Bit 2 - AES Cipher Block Counter Bit: 2"]
    #[inline(always)]
    pub fn aesblkcnt2(&mut self) -> AESBLKCNT2_W {
        AESBLKCNT2_W { w: self }
    }
    #[doc = "Bit 3 - AES Cipher Block Counter Bit: 3"]
    #[inline(always)]
    pub fn aesblkcnt3(&mut self) -> AESBLKCNT3_W {
        AESBLKCNT3_W { w: self }
    }
    #[doc = "Bit 4 - AES Cipher Block Counter Bit: 4"]
    #[inline(always)]
    pub fn aesblkcnt4(&mut self) -> AESBLKCNT4_W {
        AESBLKCNT4_W { w: self }
    }
    #[doc = "Bit 5 - AES Cipher Block Counter Bit: 5"]
    #[inline(always)]
    pub fn aesblkcnt5(&mut self) -> AESBLKCNT5_W {
        AESBLKCNT5_W { w: self }
    }
    #[doc = "Bit 6 - AES Cipher Block Counter Bit: 6"]
    #[inline(always)]
    pub fn aesblkcnt6(&mut self) -> AESBLKCNT6_W {
        AESBLKCNT6_W { w: self }
    }
    #[doc = "Bit 7 - AES Cipher Block Counter Bit: 7"]
    #[inline(always)]
    pub fn aesblkcnt7(&mut self) -> AESBLKCNT7_W {
        AESBLKCNT7_W { w: self }
    }
}
