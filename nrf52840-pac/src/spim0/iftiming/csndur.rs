#[doc = "Register `CSNDUR` reader"]
pub struct R(crate::R<CSNDUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSNDUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSNDUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSNDUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSNDUR` writer"]
pub struct W(crate::W<CSNDUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSNDUR_SPEC>;
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
impl From<crate::W<CSNDUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSNDUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSNDUR` reader - Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions. The value is specified in number of 64 MHz clock cycles (15.625 ns)."]
pub type CSNDUR_R = crate::FieldReader;
#[doc = "Field `CSNDUR` writer - Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions. The value is specified in number of 64 MHz clock cycles (15.625 ns)."]
pub type CSNDUR_W<'a, const O: u8> = crate::FieldWriter<'a, CSNDUR_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions. The value is specified in number of 64 MHz clock cycles (15.625 ns)."]
    #[inline(always)]
    pub fn csndur(&self) -> CSNDUR_R {
        CSNDUR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions. The value is specified in number of 64 MHz clock cycles (15.625 ns)."]
    #[inline(always)]
    #[must_use]
    pub fn csndur(&mut self) -> CSNDUR_W<0> {
        CSNDUR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csndur](index.html) module"]
pub struct CSNDUR_SPEC;
impl crate::RegisterSpec for CSNDUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csndur::R](R) reader structure"]
impl crate::Readable for CSNDUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csndur::W](W) writer structure"]
impl crate::Writable for CSNDUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSNDUR to value 0x02"]
impl crate::Resettable for CSNDUR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
