#[doc = "Register `SFD` reader"]
pub struct R(crate::R<SFD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFD` writer"]
pub struct W(crate::W<SFD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFD_SPEC>;
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
impl From<crate::W<SFD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFD` reader - IEEE 802.15.4 start of frame delimiter"]
pub type SFD_R = crate::FieldReader;
#[doc = "Field `SFD` writer - IEEE 802.15.4 start of frame delimiter"]
pub type SFD_W<'a, const O: u8> = crate::FieldWriter<'a, SFD_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    pub fn sfd(&self) -> SFD_R {
        SFD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    #[must_use]
    pub fn sfd(&mut self) -> SFD_W<0> {
        SFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IEEE 802.15.4 start of frame delimiter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfd](index.html) module"]
pub struct SFD_SPEC;
impl crate::RegisterSpec for SFD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfd::R](R) reader structure"]
impl crate::Readable for SFD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfd::W](W) writer structure"]
impl crate::Writable for SFD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFD to value 0xa7"]
impl crate::Resettable for SFD_SPEC {
    const RESET_VALUE: Self::Ux = 0xa7;
}
