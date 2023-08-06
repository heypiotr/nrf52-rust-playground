#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HWFC` reader - Hardware flow control"]
pub type HWFC_R = crate::BitReader<HWFC_A>;
#[doc = "Hardware flow control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWFC_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<HWFC_A> for bool {
    #[inline(always)]
    fn from(variant: HWFC_A) -> Self {
        variant as u8 != 0
    }
}
impl HWFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWFC_A {
        match self.bits {
            false => HWFC_A::DISABLED,
            true => HWFC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HWFC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HWFC_A::ENABLED
    }
}
#[doc = "Field `HWFC` writer - Hardware flow control"]
pub type HWFC_W<'a, const O: u8> = crate::BitWriter<'a, CONFIG_SPEC, O, HWFC_A>;
impl<'a, const O: u8> HWFC_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HWFC_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HWFC_A::ENABLED)
    }
}
#[doc = "Field `PARITY` reader - Parity"]
pub type PARITY_R = crate::FieldReader<PARITY_A>;
#[doc = "Parity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PARITY_A {
    #[doc = "0: Exclude parity bit"]
    EXCLUDED = 0,
    #[doc = "7: Include parity bit"]
    INCLUDED = 7,
}
impl From<PARITY_A> for u8 {
    #[inline(always)]
    fn from(variant: PARITY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PARITY_A {
    type Ux = u8;
}
impl PARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PARITY_A> {
        match self.bits {
            0 => Some(PARITY_A::EXCLUDED),
            7 => Some(PARITY_A::INCLUDED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == PARITY_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == PARITY_A::INCLUDED
    }
}
#[doc = "Field `PARITY` writer - Parity"]
pub type PARITY_W<'a, const O: u8> = crate::FieldWriter<'a, CONFIG_SPEC, 3, O, PARITY_A>;
impl<'a, const O: u8> PARITY_W<'a, O> {
    #[doc = "Exclude parity bit"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(PARITY_A::EXCLUDED)
    }
    #[doc = "Include parity bit"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(PARITY_A::INCLUDED)
    }
}
#[doc = "Field `STOP` reader - Stop bits"]
pub type STOP_R = crate::BitReader<STOP_A>;
#[doc = "Stop bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    #[doc = "0: One stop bit"]
    ONE = 0,
    #[doc = "1: Two stop bits"]
    TWO = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::ONE,
            true => STOP_A::TWO,
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == STOP_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == STOP_A::TWO
    }
}
#[doc = "Field `STOP` writer - Stop bits"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, CONFIG_SPEC, O, STOP_A>;
impl<'a, const O: u8> STOP_W<'a, O> {
    #[doc = "One stop bit"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(STOP_A::ONE)
    }
    #[doc = "Two stop bits"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(STOP_A::TWO)
    }
}
impl R {
    #[doc = "Bit 0 - Hardware flow control"]
    #[inline(always)]
    pub fn hwfc(&self) -> HWFC_R {
        HWFC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Parity"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Stop bits"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware flow control"]
    #[inline(always)]
    #[must_use]
    pub fn hwfc(&mut self) -> HWFC_W<0> {
        HWFC_W::new(self)
    }
    #[doc = "Bits 1:3 - Parity"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<1> {
        PARITY_W::new(self)
    }
    #[doc = "Bit 4 - Stop bits"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<4> {
        STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration of parity and hardware flow control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
