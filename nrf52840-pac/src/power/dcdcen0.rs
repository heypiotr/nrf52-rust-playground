#[doc = "Register `DCDCEN0` reader"]
pub struct R(crate::R<DCDCEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCEN0` writer"]
pub struct W(crate::W<DCDCEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCEN0_SPEC>;
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
impl From<crate::W<DCDCEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCDCEN` reader - Enable DC/DC converter for REG0 stage."]
pub type DCDCEN_R = crate::BitReader<DCDCEN_A>;
#[doc = "Enable DC/DC converter for REG0 stage.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDCEN_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<DCDCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCDCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCDCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDCEN_A {
        match self.bits {
            false => DCDCEN_A::DISABLED,
            true => DCDCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCDCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCDCEN_A::ENABLED
    }
}
#[doc = "Field `DCDCEN` writer - Enable DC/DC converter for REG0 stage."]
pub type DCDCEN_W<'a, const O: u8> = crate::BitWriter<'a, DCDCEN0_SPEC, O, DCDCEN_A>;
impl<'a, const O: u8> DCDCEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCDCEN_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCDCEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable DC/DC converter for REG0 stage."]
    #[inline(always)]
    pub fn dcdcen(&self) -> DCDCEN_R {
        DCDCEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DC/DC converter for REG0 stage."]
    #[inline(always)]
    #[must_use]
    pub fn dcdcen(&mut self) -> DCDCEN_W<0> {
        DCDCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable DC/DC converter for REG0 stage\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcen0](index.html) module"]
pub struct DCDCEN0_SPEC;
impl crate::RegisterSpec for DCDCEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdcen0::R](R) reader structure"]
impl crate::Readable for DCDCEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdcen0::W](W) writer structure"]
impl crate::Writable for DCDCEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDCEN0 to value 0"]
impl crate::Resettable for DCDCEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
