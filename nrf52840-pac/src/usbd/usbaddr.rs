#[doc = "Register `USBADDR` reader"]
pub struct R(crate::R<USBADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR` reader - Device USB address"]
pub type ADDR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Device USB address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Device USB address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbaddr](index.html) module"]
pub struct USBADDR_SPEC;
impl crate::RegisterSpec for USBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbaddr::R](R) reader structure"]
impl crate::Readable for USBADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBADDR to value 0"]
impl crate::Resettable for USBADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
