#[doc = "Register `TASKS_STOPTX` writer"]
pub struct W(crate::W<TASKS_STOPTX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_STOPTX_SPEC>;
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
impl From<crate::W<TASKS_STOPTX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_STOPTX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Stop UART transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TASKS_STOPTX_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_STOPTX_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_STOPTX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_STOPTX` writer - Stop UART transmitter"]
pub type TASKS_STOPTX_W<'a, const O: u8> =
    crate::BitWriter<'a, TASKS_STOPTX_SPEC, O, TASKS_STOPTX_AW>;
impl<'a, const O: u8> TASKS_STOPTX_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_STOPTX_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Stop UART transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_stoptx(&mut self) -> TASKS_STOPTX_W<0> {
        TASKS_STOPTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stop UART transmitter\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stoptx](index.html) module"]
pub struct TASKS_STOPTX_SPEC;
impl crate::RegisterSpec for TASKS_STOPTX_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_stoptx::W](W) writer structure"]
impl crate::Writable for TASKS_STOPTX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKS_STOPTX to value 0"]
impl crate::Resettable for TASKS_STOPTX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
