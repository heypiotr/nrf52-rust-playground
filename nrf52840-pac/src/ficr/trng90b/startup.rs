#[doc = "Register `STARTUP` reader"]
pub struct R(crate::R<STARTUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STARTUP` reader - Amount of bytes for the startup tests"]
pub type STARTUP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Amount of bytes for the startup tests"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(self.bits)
    }
}
#[doc = "Amount of bytes for the startup tests\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [startup](index.html) module"]
pub struct STARTUP_SPEC;
impl crate::RegisterSpec for STARTUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [startup::R](R) reader structure"]
impl crate::Readable for STARTUP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STARTUP to value 0xffff_ffff"]
impl crate::Resettable for STARTUP_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
