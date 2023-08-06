#[doc = "Register `RCCUTOFF` reader"]
pub struct R(crate::R<RCCUTOFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCCUTOFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCCUTOFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCCUTOFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RCCUTOFF` reader - Repetition counter cutoff"]
pub type RCCUTOFF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Repetition counter cutoff"]
    #[inline(always)]
    pub fn rccutoff(&self) -> RCCUTOFF_R {
        RCCUTOFF_R::new(self.bits)
    }
}
#[doc = "Repetition counter cutoff\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rccutoff](index.html) module"]
pub struct RCCUTOFF_SPEC;
impl crate::RegisterSpec for RCCUTOFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rccutoff::R](R) reader structure"]
impl crate::Readable for RCCUTOFF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RCCUTOFF to value 0xffff_ffff"]
impl crate::Resettable for RCCUTOFF_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
