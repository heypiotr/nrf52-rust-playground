#[doc = "Register `LFRCMODE` reader"]
pub struct R(crate::R<LFRCMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFRCMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFRCMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFRCMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFRCMODE` writer"]
pub struct W(crate::W<LFRCMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFRCMODE_SPEC>;
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
impl From<crate::W<LFRCMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFRCMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Set LFRC mode"]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Set LFRC mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: Ultra-low power mode (ULP)"]
    ULP = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::NORMAL,
            true => MODE_A::ULP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `ULP`"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == MODE_A::ULP
    }
}
#[doc = "Field `MODE` writer - Set LFRC mode"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, LFRCMODE_SPEC, O, MODE_A>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MODE_A::NORMAL)
    }
    #[doc = "Ultra-low power mode (ULP)"]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut W {
        self.variant(MODE_A::ULP)
    }
}
#[doc = "Field `STATUS` reader - Active LFRC mode. This field is read only."]
pub type STATUS_R = crate::BitReader<STATUS_A>;
#[doc = "Active LFRC mode. This field is read only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STATUS_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: Ultra-low power mode (ULP)"]
    ULP = 1,
}
impl From<STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATUS_A {
        match self.bits {
            false => STATUS_A::NORMAL,
            true => STATUS_A::ULP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == STATUS_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `ULP`"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == STATUS_A::ULP
    }
}
#[doc = "Field `STATUS` writer - Active LFRC mode. This field is read only."]
pub type STATUS_W<'a, const O: u8> = crate::BitWriter<'a, LFRCMODE_SPEC, O, STATUS_A>;
impl<'a, const O: u8> STATUS_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(STATUS_A::NORMAL)
    }
    #[doc = "Ultra-low power mode (ULP)"]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut W {
        self.variant(STATUS_A::ULP)
    }
}
impl R {
    #[doc = "Bit 0 - Set LFRC mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Active LFRC mode. This field is read only."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set LFRC mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 16 - Active LFRC mode. This field is read only."]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> STATUS_W<16> {
        STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LFRC mode configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfrcmode](index.html) module"]
pub struct LFRCMODE_SPEC;
impl crate::RegisterSpec for LFRCMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfrcmode::R](R) reader structure"]
impl crate::Readable for LFRCMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfrcmode::W](W) writer structure"]
impl crate::Writable for LFRCMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFRCMODE to value 0"]
impl crate::Resettable for LFRCMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
