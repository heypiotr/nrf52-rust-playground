#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENDKSGEN` reader - Write '1' to enable interrupt for event ENDKSGEN"]
pub type ENDKSGEN_R = crate::BitReader<ENDKSGEN_A>;
#[doc = "Write '1' to enable interrupt for event ENDKSGEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDKSGEN_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDKSGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ENDKSGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDKSGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDKSGEN_A {
        match self.bits {
            false => ENDKSGEN_A::DISABLED,
            true => ENDKSGEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDKSGEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDKSGEN_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event ENDKSGEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDKSGEN_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ENDKSGEN_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDKSGEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDKSGEN` writer - Write '1' to enable interrupt for event ENDKSGEN"]
pub type ENDKSGEN_W<'a, const O: u8> = crate::BitWriter<'a, INTENSET_SPEC, O, ENDKSGEN_AW>;
impl<'a, const O: u8> ENDKSGEN_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ENDKSGEN_AW::SET)
    }
}
#[doc = "Field `ENDCRYPT` reader - Write '1' to enable interrupt for event ENDCRYPT"]
pub type ENDCRYPT_R = crate::BitReader<ENDCRYPT_A>;
#[doc = "Write '1' to enable interrupt for event ENDCRYPT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDCRYPT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDCRYPT_A> for bool {
    #[inline(always)]
    fn from(variant: ENDCRYPT_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDCRYPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDCRYPT_A {
        match self.bits {
            false => ENDCRYPT_A::DISABLED,
            true => ENDCRYPT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDCRYPT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDCRYPT_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event ENDCRYPT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDCRYPT_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ENDCRYPT_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDCRYPT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDCRYPT` writer - Write '1' to enable interrupt for event ENDCRYPT"]
pub type ENDCRYPT_W<'a, const O: u8> = crate::BitWriter<'a, INTENSET_SPEC, O, ENDCRYPT_AW>;
impl<'a, const O: u8> ENDCRYPT_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ENDCRYPT_AW::SET)
    }
}
#[doc = "Field `ERROR` reader - Deprecated intsetfield - Write '1' to enable interrupt for event ERROR"]
pub type ERROR_R = crate::BitReader<ERROR_A>;
#[doc = "Deprecated intsetfield - Write '1' to enable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERROR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::DISABLED,
            true => ERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERROR_A::ENABLED
    }
}
#[doc = "Deprecated intsetfield - Write '1' to enable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERROR_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ERROR_AW> for bool {
    #[inline(always)]
    fn from(variant: ERROR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` writer - Deprecated intsetfield - Write '1' to enable interrupt for event ERROR"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, INTENSET_SPEC, O, ERROR_AW>;
impl<'a, const O: u8> ERROR_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ERROR_AW::SET)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event ENDKSGEN"]
    #[inline(always)]
    pub fn endksgen(&self) -> ENDKSGEN_R {
        ENDKSGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event ENDCRYPT"]
    #[inline(always)]
    pub fn endcrypt(&self) -> ENDCRYPT_R {
        ENDCRYPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Deprecated intsetfield - Write '1' to enable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event ENDKSGEN"]
    #[inline(always)]
    #[must_use]
    pub fn endksgen(&mut self) -> ENDKSGEN_W<0> {
        ENDKSGEN_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event ENDCRYPT"]
    #[inline(always)]
    #[must_use]
    pub fn endcrypt(&mut self) -> ENDCRYPT_W<1> {
        ENDCRYPT_W::new(self)
    }
    #[doc = "Bit 2 - Deprecated intsetfield - Write '1' to enable interrupt for event ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<2> {
        ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
