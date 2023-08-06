#[doc = "Register `PRODTEST[%s]` reader"]
pub struct R(crate::R<PRODTEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRODTEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRODTEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRODTEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRODTEST` reader - Production test signature n"]
pub type PRODTEST_R = crate::FieldReader<PRODTEST_A>;
#[doc = "Production test signature n\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PRODTEST_A {
    #[doc = "3141677471: Production tests done"]
    DONE = 3141677471,
    #[doc = "4294967295: Production tests not done"]
    NOT_DONE = 4294967295,
}
impl From<PRODTEST_A> for u32 {
    #[inline(always)]
    fn from(variant: PRODTEST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRODTEST_A {
    type Ux = u32;
}
impl PRODTEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRODTEST_A> {
        match self.bits {
            3141677471 => Some(PRODTEST_A::DONE),
            4294967295 => Some(PRODTEST_A::NOT_DONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == PRODTEST_A::DONE
    }
    #[doc = "Checks if the value of the field is `NOT_DONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == PRODTEST_A::NOT_DONE
    }
}
impl R {
    #[doc = "Bits 0:31 - Production test signature n"]
    #[inline(always)]
    pub fn prodtest(&self) -> PRODTEST_R {
        PRODTEST_R::new(self.bits)
    }
}
#[doc = "Description collection: Production test signature n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prodtest](index.html) module"]
pub struct PRODTEST_SPEC;
impl crate::RegisterSpec for PRODTEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prodtest::R](R) reader structure"]
impl crate::Readable for PRODTEST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRODTEST[%s]
to value 0xffff_ffff"]
impl crate::Resettable for PRODTEST_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
