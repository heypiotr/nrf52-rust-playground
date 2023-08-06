#[doc = "Register `EPIN[%s]` reader"]
pub struct R(crate::R<EPIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GETSTATUS` reader - IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub type GETSTATUS_R = crate::FieldReader<GETSTATUS_A>;
#[doc = "IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GETSTATUS_A {
    #[doc = "0: Endpoint is not halted"]
    NOT_HALTED = 0,
    #[doc = "1: Endpoint is halted"]
    HALTED = 1,
}
impl From<GETSTATUS_A> for u16 {
    #[inline(always)]
    fn from(variant: GETSTATUS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GETSTATUS_A {
    type Ux = u16;
}
impl GETSTATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GETSTATUS_A> {
        match self.bits {
            0 => Some(GETSTATUS_A::NOT_HALTED),
            1 => Some(GETSTATUS_A::HALTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_HALTED`"]
    #[inline(always)]
    pub fn is_not_halted(&self) -> bool {
        *self == GETSTATUS_A::NOT_HALTED
    }
    #[doc = "Checks if the value of the field is `HALTED`"]
    #[inline(always)]
    pub fn is_halted(&self) -> bool {
        *self == GETSTATUS_A::HALTED
    }
}
impl R {
    #[doc = "Bits 0:15 - IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub fn getstatus(&self) -> GETSTATUS_R {
        GETSTATUS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Description collection: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epin](index.html) module"]
pub struct EPIN_SPEC;
impl crate::RegisterSpec for EPIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epin::R](R) reader structure"]
impl crate::Readable for EPIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPIN[%s]
to value 0"]
impl crate::Resettable for EPIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
