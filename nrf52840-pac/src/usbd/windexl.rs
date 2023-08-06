#[doc = "Register `WINDEXL` reader"]
pub struct R(crate::R<WINDEXL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINDEXL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINDEXL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINDEXL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WINDEXL` reader - SETUP data, byte 4, LSB of wIndex"]
pub type WINDEXL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 4, LSB of wIndex"]
    #[inline(always)]
    pub fn windexl(&self) -> WINDEXL_R {
        WINDEXL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SETUP data, byte 4, LSB of wIndex\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [windexl](index.html) module"]
pub struct WINDEXL_SPEC;
impl crate::RegisterSpec for WINDEXL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [windexl::R](R) reader structure"]
impl crate::Readable for WINDEXL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WINDEXL to value 0"]
impl crate::Resettable for WINDEXL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
