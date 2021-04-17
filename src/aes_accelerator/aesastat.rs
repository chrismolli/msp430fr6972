#[doc = "Reader of register AESASTAT"]
pub type R = crate::R<u16, super::AESASTAT>;
#[doc = "Writer for register AESASTAT"]
pub type W = crate::W<u16, super::AESASTAT>;
#[doc = "Register AESASTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::AESASTAT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AESBUSY`"]
pub type AESBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESBUSY`"]
pub struct AESBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBUSY_W<'a> {
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
#[doc = "Reader of field `AESKEYWR`"]
pub type AESKEYWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESKEYWR`"]
pub struct AESKEYWR_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEYWR_W<'a> {
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
#[doc = "Reader of field `AESDINWR`"]
pub type AESDINWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESDINWR`"]
pub struct AESDINWR_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDINWR_W<'a> {
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
#[doc = "Reader of field `AESDOUTRD`"]
pub type AESDOUTRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESDOUTRD`"]
pub struct AESDOUTRD_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDOUTRD_W<'a> {
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
#[doc = "Reader of field `AESKEYCNT0`"]
pub type AESKEYCNT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESKEYCNT0`"]
pub struct AESKEYCNT0_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEYCNT0_W<'a> {
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
#[doc = "Reader of field `AESKEYCNT1`"]
pub type AESKEYCNT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESKEYCNT1`"]
pub struct AESKEYCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEYCNT1_W<'a> {
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
#[doc = "Reader of field `AESKEYCNT2`"]
pub type AESKEYCNT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESKEYCNT2`"]
pub struct AESKEYCNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEYCNT2_W<'a> {
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
#[doc = "Reader of field `AESKEYCNT3`"]
pub type AESKEYCNT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESKEYCNT3`"]
pub struct AESKEYCNT3_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEYCNT3_W<'a> {
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
#[doc = "Reader of field `AESDINCNT0`"]
pub type AESDINCNT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESDINCNT0`"]
pub struct AESDINCNT0_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDINCNT0_W<'a> {
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
#[doc = "Reader of field `AESDINCNT1`"]
pub type AESDINCNT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESDINCNT1`"]
pub struct AESDINCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDINCNT1_W<'a> {
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
#[doc = "Reader of field `AESDINCNT2`"]
pub type AESDINCNT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESDINCNT2`"]
pub struct AESDINCNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDINCNT2_W<'a> {
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
#[doc = "Reader of field `AESDINCNT3`"]
pub type AESDINCNT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESDINCNT3`"]
pub struct AESDINCNT3_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDINCNT3_W<'a> {
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
#[doc = "Reader of field `AESDOUTCNT0`"]
pub type AESDOUTCNT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESDOUTCNT0`"]
pub struct AESDOUTCNT0_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDOUTCNT0_W<'a> {
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
#[doc = "Reader of field `AESDOUTCNT1`"]
pub type AESDOUTCNT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESDOUTCNT1`"]
pub struct AESDOUTCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDOUTCNT1_W<'a> {
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
#[doc = "Reader of field `AESDOUTCNT2`"]
pub type AESDOUTCNT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESDOUTCNT2`"]
pub struct AESDOUTCNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDOUTCNT2_W<'a> {
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
#[doc = "Reader of field `AESDOUTCNT3`"]
pub type AESDOUTCNT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESDOUTCNT3`"]
pub struct AESDOUTCNT3_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDOUTCNT3_W<'a> {
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
    #[doc = "Bit 0 - AES Busy"]
    #[inline(always)]
    pub fn aesbusy(&self) -> AESBUSY_R {
        AESBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AES All 16 bytes written to AESAKEY"]
    #[inline(always)]
    pub fn aeskeywr(&self) -> AESKEYWR_R {
        AESKEYWR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AES All 16 bytes written to AESADIN"]
    #[inline(always)]
    pub fn aesdinwr(&self) -> AESDINWR_R {
        AESDINWR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AES All 16 bytes read from AESADOUT"]
    #[inline(always)]
    pub fn aesdoutrd(&self) -> AESDOUTRD_R {
        AESDOUTRD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AES Bytes written via AESAKEY Bit: 0"]
    #[inline(always)]
    pub fn aeskeycnt0(&self) -> AESKEYCNT0_R {
        AESKEYCNT0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AES Bytes written via AESAKEY Bit: 1"]
    #[inline(always)]
    pub fn aeskeycnt1(&self) -> AESKEYCNT1_R {
        AESKEYCNT1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AES Bytes written via AESAKEY Bit: 2"]
    #[inline(always)]
    pub fn aeskeycnt2(&self) -> AESKEYCNT2_R {
        AESKEYCNT2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AES Bytes written via AESAKEY Bit: 3"]
    #[inline(always)]
    pub fn aeskeycnt3(&self) -> AESKEYCNT3_R {
        AESKEYCNT3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AES Bytes written via AESADIN Bit: 0"]
    #[inline(always)]
    pub fn aesdincnt0(&self) -> AESDINCNT0_R {
        AESDINCNT0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AES Bytes written via AESADIN Bit: 1"]
    #[inline(always)]
    pub fn aesdincnt1(&self) -> AESDINCNT1_R {
        AESDINCNT1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AES Bytes written via AESADIN Bit: 2"]
    #[inline(always)]
    pub fn aesdincnt2(&self) -> AESDINCNT2_R {
        AESDINCNT2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AES Bytes written via AESADIN Bit: 3"]
    #[inline(always)]
    pub fn aesdincnt3(&self) -> AESDINCNT3_R {
        AESDINCNT3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AES Bytes read via AESADOUT Bit: 0"]
    #[inline(always)]
    pub fn aesdoutcnt0(&self) -> AESDOUTCNT0_R {
        AESDOUTCNT0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AES Bytes read via AESADOUT Bit: 1"]
    #[inline(always)]
    pub fn aesdoutcnt1(&self) -> AESDOUTCNT1_R {
        AESDOUTCNT1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AES Bytes read via AESADOUT Bit: 2"]
    #[inline(always)]
    pub fn aesdoutcnt2(&self) -> AESDOUTCNT2_R {
        AESDOUTCNT2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AES Bytes read via AESADOUT Bit: 3"]
    #[inline(always)]
    pub fn aesdoutcnt3(&self) -> AESDOUTCNT3_R {
        AESDOUTCNT3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Busy"]
    #[inline(always)]
    pub fn aesbusy(&mut self) -> AESBUSY_W {
        AESBUSY_W { w: self }
    }
    #[doc = "Bit 1 - AES All 16 bytes written to AESAKEY"]
    #[inline(always)]
    pub fn aeskeywr(&mut self) -> AESKEYWR_W {
        AESKEYWR_W { w: self }
    }
    #[doc = "Bit 2 - AES All 16 bytes written to AESADIN"]
    #[inline(always)]
    pub fn aesdinwr(&mut self) -> AESDINWR_W {
        AESDINWR_W { w: self }
    }
    #[doc = "Bit 3 - AES All 16 bytes read from AESADOUT"]
    #[inline(always)]
    pub fn aesdoutrd(&mut self) -> AESDOUTRD_W {
        AESDOUTRD_W { w: self }
    }
    #[doc = "Bit 4 - AES Bytes written via AESAKEY Bit: 0"]
    #[inline(always)]
    pub fn aeskeycnt0(&mut self) -> AESKEYCNT0_W {
        AESKEYCNT0_W { w: self }
    }
    #[doc = "Bit 5 - AES Bytes written via AESAKEY Bit: 1"]
    #[inline(always)]
    pub fn aeskeycnt1(&mut self) -> AESKEYCNT1_W {
        AESKEYCNT1_W { w: self }
    }
    #[doc = "Bit 6 - AES Bytes written via AESAKEY Bit: 2"]
    #[inline(always)]
    pub fn aeskeycnt2(&mut self) -> AESKEYCNT2_W {
        AESKEYCNT2_W { w: self }
    }
    #[doc = "Bit 7 - AES Bytes written via AESAKEY Bit: 3"]
    #[inline(always)]
    pub fn aeskeycnt3(&mut self) -> AESKEYCNT3_W {
        AESKEYCNT3_W { w: self }
    }
    #[doc = "Bit 8 - AES Bytes written via AESADIN Bit: 0"]
    #[inline(always)]
    pub fn aesdincnt0(&mut self) -> AESDINCNT0_W {
        AESDINCNT0_W { w: self }
    }
    #[doc = "Bit 9 - AES Bytes written via AESADIN Bit: 1"]
    #[inline(always)]
    pub fn aesdincnt1(&mut self) -> AESDINCNT1_W {
        AESDINCNT1_W { w: self }
    }
    #[doc = "Bit 10 - AES Bytes written via AESADIN Bit: 2"]
    #[inline(always)]
    pub fn aesdincnt2(&mut self) -> AESDINCNT2_W {
        AESDINCNT2_W { w: self }
    }
    #[doc = "Bit 11 - AES Bytes written via AESADIN Bit: 3"]
    #[inline(always)]
    pub fn aesdincnt3(&mut self) -> AESDINCNT3_W {
        AESDINCNT3_W { w: self }
    }
    #[doc = "Bit 12 - AES Bytes read via AESADOUT Bit: 0"]
    #[inline(always)]
    pub fn aesdoutcnt0(&mut self) -> AESDOUTCNT0_W {
        AESDOUTCNT0_W { w: self }
    }
    #[doc = "Bit 13 - AES Bytes read via AESADOUT Bit: 1"]
    #[inline(always)]
    pub fn aesdoutcnt1(&mut self) -> AESDOUTCNT1_W {
        AESDOUTCNT1_W { w: self }
    }
    #[doc = "Bit 14 - AES Bytes read via AESADOUT Bit: 2"]
    #[inline(always)]
    pub fn aesdoutcnt2(&mut self) -> AESDOUTCNT2_W {
        AESDOUTCNT2_W { w: self }
    }
    #[doc = "Bit 15 - AES Bytes read via AESADOUT Bit: 3"]
    #[inline(always)]
    pub fn aesdoutcnt3(&mut self) -> AESDOUTCNT3_W {
        AESDOUTCNT3_W { w: self }
    }
}
