#[doc = "Register `EDSAMPLE` reader"]
pub struct R(crate::R<EDSAMPLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDSAMPLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDSAMPLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDSAMPLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDSAMPLE` writer"]
pub struct W(crate::W<EDSAMPLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDSAMPLE_SPEC>;
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
impl From<crate::W<EDSAMPLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDSAMPLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDLVL` reader - IEEE 802.15.4 energy detect level"]
pub type EDLVL_R = crate::FieldReader;
#[doc = "Field `EDLVL` writer - IEEE 802.15.4 energy detect level"]
pub type EDLVL_W<'a, const O: u8> = crate::FieldWriter<'a, EDSAMPLE_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub fn edlvl(&self) -> EDLVL_R {
        EDLVL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    #[must_use]
    pub fn edlvl(&mut self) -> EDLVL_W<0> {
        EDLVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IEEE 802.15.4 energy detect level\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edsample](index.html) module"]
pub struct EDSAMPLE_SPEC;
impl crate::RegisterSpec for EDSAMPLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edsample::R](R) reader structure"]
impl crate::Readable for EDSAMPLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edsample::W](W) writer structure"]
impl crate::Writable for EDSAMPLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDSAMPLE to value 0"]
impl crate::Resettable for EDSAMPLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
