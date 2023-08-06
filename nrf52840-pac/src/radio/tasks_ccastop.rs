#[doc = "Register `TASKS_CCASTOP` writer"]
pub struct W(crate::W<TASKS_CCASTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_CCASTOP_SPEC>;
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
impl From<crate::W<TASKS_CCASTOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_CCASTOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Stop the clear channel assessment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TASKS_CCASTOP_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_CCASTOP_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_CCASTOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CCASTOP` writer - Stop the clear channel assessment"]
pub type TASKS_CCASTOP_W<'a, const O: u8> =
    crate::BitWriter<'a, TASKS_CCASTOP_SPEC, O, TASKS_CCASTOP_AW>;
impl<'a, const O: u8> TASKS_CCASTOP_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_CCASTOP_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Stop the clear channel assessment"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_ccastop(&mut self) -> TASKS_CCASTOP_W<0> {
        TASKS_CCASTOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stop the clear channel assessment\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_ccastop](index.html) module"]
pub struct TASKS_CCASTOP_SPEC;
impl crate::RegisterSpec for TASKS_CCASTOP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_ccastop::W](W) writer structure"]
impl crate::Writable for TASKS_CCASTOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKS_CCASTOP to value 0"]
impl crate::Resettable for TASKS_CCASTOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
