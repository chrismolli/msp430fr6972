#[doc = "Reader of register P9IN"]
pub type R = crate::R<u8, super::P9IN>;
#[doc = "Writer for register P9IN"]
pub type W = crate::W<u8, super::P9IN>;
#[doc = "Register P9IN `reset()`'s with value 0"]
impl crate::ResetValue for super::P9IN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P9IN0`"]
pub type P9IN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9IN0`"]
pub struct P9IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> P9IN0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `P9IN1`"]
pub type P9IN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9IN1`"]
pub struct P9IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> P9IN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `P9IN2`"]
pub type P9IN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9IN2`"]
pub struct P9IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> P9IN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `P9IN3`"]
pub type P9IN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9IN3`"]
pub struct P9IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> P9IN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `P9IN4`"]
pub type P9IN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9IN4`"]
pub struct P9IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> P9IN4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `P9IN5`"]
pub type P9IN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9IN5`"]
pub struct P9IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> P9IN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `P9IN6`"]
pub type P9IN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9IN6`"]
pub struct P9IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> P9IN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `P9IN7`"]
pub type P9IN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P9IN7`"]
pub struct P9IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> P9IN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - P9IN0"]
    #[inline(always)]
    pub fn p9in0(&self) -> P9IN0_R {
        P9IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P9IN1"]
    #[inline(always)]
    pub fn p9in1(&self) -> P9IN1_R {
        P9IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P9IN2"]
    #[inline(always)]
    pub fn p9in2(&self) -> P9IN2_R {
        P9IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P9IN3"]
    #[inline(always)]
    pub fn p9in3(&self) -> P9IN3_R {
        P9IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P9IN4"]
    #[inline(always)]
    pub fn p9in4(&self) -> P9IN4_R {
        P9IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P9IN5"]
    #[inline(always)]
    pub fn p9in5(&self) -> P9IN5_R {
        P9IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P9IN6"]
    #[inline(always)]
    pub fn p9in6(&self) -> P9IN6_R {
        P9IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P9IN7"]
    #[inline(always)]
    pub fn p9in7(&self) -> P9IN7_R {
        P9IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P9IN0"]
    #[inline(always)]
    pub fn p9in0(&mut self) -> P9IN0_W {
        P9IN0_W { w: self }
    }
    #[doc = "Bit 1 - P9IN1"]
    #[inline(always)]
    pub fn p9in1(&mut self) -> P9IN1_W {
        P9IN1_W { w: self }
    }
    #[doc = "Bit 2 - P9IN2"]
    #[inline(always)]
    pub fn p9in2(&mut self) -> P9IN2_W {
        P9IN2_W { w: self }
    }
    #[doc = "Bit 3 - P9IN3"]
    #[inline(always)]
    pub fn p9in3(&mut self) -> P9IN3_W {
        P9IN3_W { w: self }
    }
    #[doc = "Bit 4 - P9IN4"]
    #[inline(always)]
    pub fn p9in4(&mut self) -> P9IN4_W {
        P9IN4_W { w: self }
    }
    #[doc = "Bit 5 - P9IN5"]
    #[inline(always)]
    pub fn p9in5(&mut self) -> P9IN5_W {
        P9IN5_W { w: self }
    }
    #[doc = "Bit 6 - P9IN6"]
    #[inline(always)]
    pub fn p9in6(&mut self) -> P9IN6_W {
        P9IN6_W { w: self }
    }
    #[doc = "Bit 7 - P9IN7"]
    #[inline(always)]
    pub fn p9in7(&mut self) -> P9IN7_W {
        P9IN7_W { w: self }
    }
}
