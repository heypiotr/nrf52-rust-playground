#[doc = "Register `PACKAGE` reader"]
pub struct R(crate::R<PACKAGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACKAGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACKAGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACKAGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PACKAGE` reader - Package option"]
pub type PACKAGE_R = crate::FieldReader<PACKAGE_A>;
#[doc = "Package option\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PACKAGE_A {
    #[doc = "8196: QIxx - 7x7 73-pin aQFN"]
    QI = 8196,
    #[doc = "8192: QFxx - 6x6 48-pin QFN"]
    QF = 8192,
    #[doc = "8197: CKxx - 3.544 x 3.607 WLCSP"]
    CK = 8197,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<PACKAGE_A> for u32 {
    #[inline(always)]
    fn from(variant: PACKAGE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PACKAGE_A {
    type Ux = u32;
}
impl PACKAGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PACKAGE_A> {
        match self.bits {
            8196 => Some(PACKAGE_A::QI),
            8192 => Some(PACKAGE_A::QF),
            8197 => Some(PACKAGE_A::CK),
            4294967295 => Some(PACKAGE_A::UNSPECIFIED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `QI`"]
    #[inline(always)]
    pub fn is_qi(&self) -> bool {
        *self == PACKAGE_A::QI
    }
    #[doc = "Checks if the value of the field is `QF`"]
    #[inline(always)]
    pub fn is_qf(&self) -> bool {
        *self == PACKAGE_A::QF
    }
    #[doc = "Checks if the value of the field is `CK`"]
    #[inline(always)]
    pub fn is_ck(&self) -> bool {
        *self == PACKAGE_A::CK
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == PACKAGE_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Package option"]
    #[inline(always)]
    pub fn package(&self) -> PACKAGE_R {
        PACKAGE_R::new(self.bits)
    }
}
#[doc = "Package option\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [package](index.html) module"]
pub struct PACKAGE_SPEC;
impl crate::RegisterSpec for PACKAGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [package::R](R) reader structure"]
impl crate::Readable for PACKAGE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PACKAGE to value 0xffff_ffff"]
impl crate::Resettable for PACKAGE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
