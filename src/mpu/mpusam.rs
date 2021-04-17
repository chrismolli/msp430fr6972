#[doc = "Reader of register MPUSAM"]
pub type R = crate::R<u16, super::MPUSAM>;
#[doc = "Writer for register MPUSAM"]
pub type W = crate::W<u16, super::MPUSAM>;
#[doc = "Register MPUSAM `reset()`'s with value 0"]
impl crate::ResetValue for super::MPUSAM {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPUSEG1RE`"]
pub type MPUSEG1RE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEG1RE`"]
pub struct MPUSEG1RE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG1RE_W<'a> {
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
#[doc = "Reader of field `MPUSEG1WE`"]
pub type MPUSEG1WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEG1WE`"]
pub struct MPUSEG1WE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG1WE_W<'a> {
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
#[doc = "Reader of field `MPUSEG1XE`"]
pub type MPUSEG1XE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEG1XE`"]
pub struct MPUSEG1XE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG1XE_W<'a> {
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
#[doc = "Reader of field `MPUSEG1VS`"]
pub type MPUSEG1VS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEG1VS`"]
pub struct MPUSEG1VS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG1VS_W<'a> {
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
#[doc = "Reader of field `MPUSEG2RE`"]
pub type MPUSEG2RE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEG2RE`"]
pub struct MPUSEG2RE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG2RE_W<'a> {
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
#[doc = "Reader of field `MPUSEG2WE`"]
pub type MPUSEG2WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEG2WE`"]
pub struct MPUSEG2WE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG2WE_W<'a> {
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
#[doc = "Reader of field `MPUSEG2XE`"]
pub type MPUSEG2XE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEG2XE`"]
pub struct MPUSEG2XE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG2XE_W<'a> {
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
#[doc = "Reader of field `MPUSEG2VS`"]
pub type MPUSEG2VS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEG2VS`"]
pub struct MPUSEG2VS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG2VS_W<'a> {
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
#[doc = "Reader of field `MPUSEG3RE`"]
pub type MPUSEG3RE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEG3RE`"]
pub struct MPUSEG3RE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG3RE_W<'a> {
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
#[doc = "Reader of field `MPUSEG3WE`"]
pub type MPUSEG3WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEG3WE`"]
pub struct MPUSEG3WE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG3WE_W<'a> {
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
#[doc = "Reader of field `MPUSEG3XE`"]
pub type MPUSEG3XE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEG3XE`"]
pub struct MPUSEG3XE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG3XE_W<'a> {
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
#[doc = "Reader of field `MPUSEG3VS`"]
pub type MPUSEG3VS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEG3VS`"]
pub struct MPUSEG3VS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEG3VS_W<'a> {
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
#[doc = "Reader of field `MPUSEGIRE`"]
pub type MPUSEGIRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGIRE`"]
pub struct MPUSEGIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIRE_W<'a> {
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
#[doc = "Reader of field `MPUSEGIWE`"]
pub type MPUSEGIWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGIWE`"]
pub struct MPUSEGIWE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIWE_W<'a> {
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
#[doc = "Reader of field `MPUSEGIXE`"]
pub type MPUSEGIXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGIXE`"]
pub struct MPUSEGIXE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIXE_W<'a> {
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
#[doc = "Reader of field `MPUSEGIVS`"]
pub type MPUSEGIVS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPUSEGIVS`"]
pub struct MPUSEGIVS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSEGIVS_W<'a> {
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
    #[doc = "Bit 0 - MPU Main memory Segment 1 Read enable"]
    #[inline(always)]
    pub fn mpuseg1re(&self) -> MPUSEG1RE_R {
        MPUSEG1RE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU Main memory Segment 1 Write enable"]
    #[inline(always)]
    pub fn mpuseg1we(&self) -> MPUSEG1WE_R {
        MPUSEG1WE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPU Main memory Segment 1 Execute enable"]
    #[inline(always)]
    pub fn mpuseg1xe(&self) -> MPUSEG1XE_R {
        MPUSEG1XE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPU Main memory Segment 1 Violation select"]
    #[inline(always)]
    pub fn mpuseg1vs(&self) -> MPUSEG1VS_R {
        MPUSEG1VS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPU Main memory Segment 2 Read enable"]
    #[inline(always)]
    pub fn mpuseg2re(&self) -> MPUSEG2RE_R {
        MPUSEG2RE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPU Main memory Segment 2 Write enable"]
    #[inline(always)]
    pub fn mpuseg2we(&self) -> MPUSEG2WE_R {
        MPUSEG2WE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPU Main memory Segment 2 Execute enable"]
    #[inline(always)]
    pub fn mpuseg2xe(&self) -> MPUSEG2XE_R {
        MPUSEG2XE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPU Main memory Segment 2 Violation select"]
    #[inline(always)]
    pub fn mpuseg2vs(&self) -> MPUSEG2VS_R {
        MPUSEG2VS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MPU Main memory Segment 3 Read enable"]
    #[inline(always)]
    pub fn mpuseg3re(&self) -> MPUSEG3RE_R {
        MPUSEG3RE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MPU Main memory Segment 3 Write enable"]
    #[inline(always)]
    pub fn mpuseg3we(&self) -> MPUSEG3WE_R {
        MPUSEG3WE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MPU Main memory Segment 3 Execute enable"]
    #[inline(always)]
    pub fn mpuseg3xe(&self) -> MPUSEG3XE_R {
        MPUSEG3XE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MPU Main memory Segment 3 Violation select"]
    #[inline(always)]
    pub fn mpuseg3vs(&self) -> MPUSEG3VS_R {
        MPUSEG3VS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MPU Info memory Segment Read enable"]
    #[inline(always)]
    pub fn mpusegire(&self) -> MPUSEGIRE_R {
        MPUSEGIRE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MPU Info memory Segment Write enable"]
    #[inline(always)]
    pub fn mpusegiwe(&self) -> MPUSEGIWE_R {
        MPUSEGIWE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MPU Info memory Segment Execute enable"]
    #[inline(always)]
    pub fn mpusegixe(&self) -> MPUSEGIXE_R {
        MPUSEGIXE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MPU Info memory Segment Violation select"]
    #[inline(always)]
    pub fn mpusegivs(&self) -> MPUSEGIVS_R {
        MPUSEGIVS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU Main memory Segment 1 Read enable"]
    #[inline(always)]
    pub fn mpuseg1re(&mut self) -> MPUSEG1RE_W {
        MPUSEG1RE_W { w: self }
    }
    #[doc = "Bit 1 - MPU Main memory Segment 1 Write enable"]
    #[inline(always)]
    pub fn mpuseg1we(&mut self) -> MPUSEG1WE_W {
        MPUSEG1WE_W { w: self }
    }
    #[doc = "Bit 2 - MPU Main memory Segment 1 Execute enable"]
    #[inline(always)]
    pub fn mpuseg1xe(&mut self) -> MPUSEG1XE_W {
        MPUSEG1XE_W { w: self }
    }
    #[doc = "Bit 3 - MPU Main memory Segment 1 Violation select"]
    #[inline(always)]
    pub fn mpuseg1vs(&mut self) -> MPUSEG1VS_W {
        MPUSEG1VS_W { w: self }
    }
    #[doc = "Bit 4 - MPU Main memory Segment 2 Read enable"]
    #[inline(always)]
    pub fn mpuseg2re(&mut self) -> MPUSEG2RE_W {
        MPUSEG2RE_W { w: self }
    }
    #[doc = "Bit 5 - MPU Main memory Segment 2 Write enable"]
    #[inline(always)]
    pub fn mpuseg2we(&mut self) -> MPUSEG2WE_W {
        MPUSEG2WE_W { w: self }
    }
    #[doc = "Bit 6 - MPU Main memory Segment 2 Execute enable"]
    #[inline(always)]
    pub fn mpuseg2xe(&mut self) -> MPUSEG2XE_W {
        MPUSEG2XE_W { w: self }
    }
    #[doc = "Bit 7 - MPU Main memory Segment 2 Violation select"]
    #[inline(always)]
    pub fn mpuseg2vs(&mut self) -> MPUSEG2VS_W {
        MPUSEG2VS_W { w: self }
    }
    #[doc = "Bit 8 - MPU Main memory Segment 3 Read enable"]
    #[inline(always)]
    pub fn mpuseg3re(&mut self) -> MPUSEG3RE_W {
        MPUSEG3RE_W { w: self }
    }
    #[doc = "Bit 9 - MPU Main memory Segment 3 Write enable"]
    #[inline(always)]
    pub fn mpuseg3we(&mut self) -> MPUSEG3WE_W {
        MPUSEG3WE_W { w: self }
    }
    #[doc = "Bit 10 - MPU Main memory Segment 3 Execute enable"]
    #[inline(always)]
    pub fn mpuseg3xe(&mut self) -> MPUSEG3XE_W {
        MPUSEG3XE_W { w: self }
    }
    #[doc = "Bit 11 - MPU Main memory Segment 3 Violation select"]
    #[inline(always)]
    pub fn mpuseg3vs(&mut self) -> MPUSEG3VS_W {
        MPUSEG3VS_W { w: self }
    }
    #[doc = "Bit 12 - MPU Info memory Segment Read enable"]
    #[inline(always)]
    pub fn mpusegire(&mut self) -> MPUSEGIRE_W {
        MPUSEGIRE_W { w: self }
    }
    #[doc = "Bit 13 - MPU Info memory Segment Write enable"]
    #[inline(always)]
    pub fn mpusegiwe(&mut self) -> MPUSEGIWE_W {
        MPUSEGIWE_W { w: self }
    }
    #[doc = "Bit 14 - MPU Info memory Segment Execute enable"]
    #[inline(always)]
    pub fn mpusegixe(&mut self) -> MPUSEGIXE_W {
        MPUSEGIXE_W { w: self }
    }
    #[doc = "Bit 15 - MPU Info memory Segment Violation select"]
    #[inline(always)]
    pub fn mpusegivs(&mut self) -> MPUSEGIVS_W {
        MPUSEGIVS_W { w: self }
    }
}
