#[doc = "Register `TASKS_STARTEPIN[%s]` writer"]
pub struct W(crate::W<TASKS_STARTEPIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_STARTEPIN_SPEC>;
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
impl From<crate::W<TASKS_STARTEPIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_STARTEPIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TASKS_STARTEPIN_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_STARTEPIN_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_STARTEPIN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_STARTEPIN` writer - Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
pub type TASKS_STARTEPIN_W<'a, const O: u8> =
    crate::BitWriter<'a, TASKS_STARTEPIN_SPEC, O, TASKS_STARTEPIN_AW>;
impl<'a, const O: u8> TASKS_STARTEPIN_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_STARTEPIN_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_startepin(&mut self) -> TASKS_STARTEPIN_W<0> {
        TASKS_STARTEPIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_startepin](index.html) module"]
pub struct TASKS_STARTEPIN_SPEC;
impl crate::RegisterSpec for TASKS_STARTEPIN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_startepin::W](W) writer structure"]
impl crate::Writable for TASKS_STARTEPIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKS_STARTEPIN[%s]
to value 0"]
impl crate::Resettable for TASKS_STARTEPIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
