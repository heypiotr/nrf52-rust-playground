#[doc = "Register `RATIO` reader"]
pub struct R(crate::R<RATIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RATIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RATIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RATIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RATIO` writer"]
pub struct W(crate::W<RATIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RATIO_SPEC>;
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
impl From<crate::W<RATIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RATIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RATIO` reader - Selects the ratio between PDM_CLK and output sample rate"]
pub type RATIO_R = crate::BitReader<RATIO_A>;
#[doc = "Selects the ratio between PDM_CLK and output sample rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RATIO_A {
    #[doc = "0: Ratio of 64"]
    RATIO64 = 0,
    #[doc = "1: Ratio of 80"]
    RATIO80 = 1,
}
impl From<RATIO_A> for bool {
    #[inline(always)]
    fn from(variant: RATIO_A) -> Self {
        variant as u8 != 0
    }
}
impl RATIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RATIO_A {
        match self.bits {
            false => RATIO_A::RATIO64,
            true => RATIO_A::RATIO80,
        }
    }
    #[doc = "Checks if the value of the field is `RATIO64`"]
    #[inline(always)]
    pub fn is_ratio64(&self) -> bool {
        *self == RATIO_A::RATIO64
    }
    #[doc = "Checks if the value of the field is `RATIO80`"]
    #[inline(always)]
    pub fn is_ratio80(&self) -> bool {
        *self == RATIO_A::RATIO80
    }
}
#[doc = "Field `RATIO` writer - Selects the ratio between PDM_CLK and output sample rate"]
pub type RATIO_W<'a, const O: u8> = crate::BitWriter<'a, RATIO_SPEC, O, RATIO_A>;
impl<'a, const O: u8> RATIO_W<'a, O> {
    #[doc = "Ratio of 64"]
    #[inline(always)]
    pub fn ratio64(self) -> &'a mut W {
        self.variant(RATIO_A::RATIO64)
    }
    #[doc = "Ratio of 80"]
    #[inline(always)]
    pub fn ratio80(self) -> &'a mut W {
        self.variant(RATIO_A::RATIO80)
    }
}
impl R {
    #[doc = "Bit 0 - Selects the ratio between PDM_CLK and output sample rate"]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the ratio between PDM_CLK and output sample rate"]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RATIO_W<0> {
        RATIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratio](index.html) module"]
pub struct RATIO_SPEC;
impl crate::RegisterSpec for RATIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ratio::R](R) reader structure"]
impl crate::Readable for RATIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ratio::W](W) writer structure"]
impl crate::Writable for RATIO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RATIO to value 0"]
impl crate::Resettable for RATIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
