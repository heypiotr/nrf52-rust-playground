#[doc = "Register `ERASEPAGEPARTIAL` writer"]
pub struct W(crate::W<ERASEPAGEPARTIAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASEPAGEPARTIAL_SPEC>;
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
impl From<crate::W<ERASEPAGEPARTIAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASEPAGEPARTIAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASEPAGEPARTIAL` writer - Register for starting partial erase of a page in code area"]
pub type ERASEPAGEPARTIAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, ERASEPAGEPARTIAL_SPEC, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Register for starting partial erase of a page in code area"]
    #[inline(always)]
    #[must_use]
    pub fn erasepagepartial(&mut self) -> ERASEPAGEPARTIAL_W<0> {
        ERASEPAGEPARTIAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for partial erase of a page in code area\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erasepagepartial](index.html) module"]
pub struct ERASEPAGEPARTIAL_SPEC;
impl crate::RegisterSpec for ERASEPAGEPARTIAL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [erasepagepartial::W](W) writer structure"]
impl crate::Writable for ERASEPAGEPARTIAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERASEPAGEPARTIAL to value 0"]
impl crate::Resettable for ERASEPAGEPARTIAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
