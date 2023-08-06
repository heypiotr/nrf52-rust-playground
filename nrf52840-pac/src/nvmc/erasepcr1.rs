#[doc = "Register `ERASEPCR1` writer"]
pub struct W(crate::W<ERASEPCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASEPCR1_SPEC>;
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
impl From<crate::W<ERASEPCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASEPCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASEPCR1` writer - Register for erasing a page in code area, equivalent to ERASEPAGE"]
pub type ERASEPCR1_W<'a, const O: u8> = crate::FieldWriter<'a, ERASEPCR1_SPEC, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Register for erasing a page in code area, equivalent to ERASEPAGE"]
    #[inline(always)]
    #[must_use]
    pub fn erasepcr1(&mut self) -> ERASEPCR1_W<0> {
        ERASEPCR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erasepcr1](index.html) module"]
pub struct ERASEPCR1_SPEC;
impl crate::RegisterSpec for ERASEPCR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [erasepcr1::W](W) writer structure"]
impl crate::Writable for ERASEPCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERASEPCR1 to value 0"]
impl crate::Resettable for ERASEPCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
