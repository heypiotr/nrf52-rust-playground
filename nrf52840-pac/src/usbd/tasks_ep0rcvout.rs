#[doc = "Register `TASKS_EP0RCVOUT` writer"]
pub struct W(crate::W<TASKS_EP0RCVOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_EP0RCVOUT_SPEC>;
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
impl From<crate::W<TASKS_EP0RCVOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_EP0RCVOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Allows OUT data stage on control endpoint 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TASKS_EP0RCVOUT_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_EP0RCVOUT_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_EP0RCVOUT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_EP0RCVOUT` writer - Allows OUT data stage on control endpoint 0"]
pub type TASKS_EP0RCVOUT_W<'a, const O: u8> =
    crate::BitWriter<'a, TASKS_EP0RCVOUT_SPEC, O, TASKS_EP0RCVOUT_AW>;
impl<'a, const O: u8> TASKS_EP0RCVOUT_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_EP0RCVOUT_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Allows OUT data stage on control endpoint 0"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_ep0rcvout(&mut self) -> TASKS_EP0RCVOUT_W<0> {
        TASKS_EP0RCVOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Allows OUT data stage on control endpoint 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_ep0rcvout](index.html) module"]
pub struct TASKS_EP0RCVOUT_SPEC;
impl crate::RegisterSpec for TASKS_EP0RCVOUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_ep0rcvout::W](W) writer structure"]
impl crate::Writable for TASKS_EP0RCVOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKS_EP0RCVOUT to value 0"]
impl crate::Resettable for TASKS_EP0RCVOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
