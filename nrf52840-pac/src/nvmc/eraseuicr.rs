#[doc = "Register `ERASEUICR` writer"]
pub struct W(crate::W<ERASEUICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASEUICR_SPEC>;
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
impl From<crate::W<ERASEUICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASEUICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Register starting erase of all user information configuration registers. The erase must be enabled using CONFIG.WEN before the UICR can be erased.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERASEUICR_AW {
    #[doc = "0: No operation"]
    NO_OPERATION = 0,
    #[doc = "1: Start erase of UICR"]
    ERASE = 1,
}
impl From<ERASEUICR_AW> for bool {
    #[inline(always)]
    fn from(variant: ERASEUICR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERASEUICR` writer - Register starting erase of all user information configuration registers. The erase must be enabled using CONFIG.WEN before the UICR can be erased."]
pub type ERASEUICR_W<'a, const O: u8> = crate::BitWriter<'a, ERASEUICR_SPEC, O, ERASEUICR_AW>;
impl<'a, const O: u8> ERASEUICR_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(ERASEUICR_AW::NO_OPERATION)
    }
    #[doc = "Start erase of UICR"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ERASEUICR_AW::ERASE)
    }
}
impl W {
    #[doc = "Bit 0 - Register starting erase of all user information configuration registers. The erase must be enabled using CONFIG.WEN before the UICR can be erased."]
    #[inline(always)]
    #[must_use]
    pub fn eraseuicr(&mut self) -> ERASEUICR_W<0> {
        ERASEUICR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for erasing user information configuration registers\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eraseuicr](index.html) module"]
pub struct ERASEUICR_SPEC;
impl crate::RegisterSpec for ERASEUICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [eraseuicr::W](W) writer structure"]
impl crate::Writable for ERASEUICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERASEUICR to value 0"]
impl crate::Resettable for ERASEUICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
