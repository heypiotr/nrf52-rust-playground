#[doc = "Register `PSELRESET[%s]` reader"]
pub struct R(crate::R<PSELRESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSELRESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSELRESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSELRESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSELRESET[%s]` writer"]
pub struct W(crate::W<PSELRESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSELRESET_SPEC>;
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
impl From<crate::W<PSELRESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSELRESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN` reader - GPIO pin number onto which nRESET is exposed"]
pub type PIN_R = crate::FieldReader;
#[doc = "Field `PIN` writer - GPIO pin number onto which nRESET is exposed"]
pub type PIN_W<'a, const O: u8> = crate::FieldWriter<'a, PSELRESET_SPEC, 5, O>;
#[doc = "Field `PORT` reader - Port number onto which nRESET is exposed"]
pub type PORT_R = crate::BitReader;
#[doc = "Field `PORT` writer - Port number onto which nRESET is exposed"]
pub type PORT_W<'a, const O: u8> = crate::BitWriter<'a, PSELRESET_SPEC, O>;
#[doc = "Field `CONNECT` reader - Connection"]
pub type CONNECT_R = crate::BitReader<CONNECT_A>;
#[doc = "Connection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONNECT_A {
    #[doc = "1: Disconnect"]
    DISCONNECTED = 1,
    #[doc = "0: Connect"]
    CONNECTED = 0,
}
impl From<CONNECT_A> for bool {
    #[inline(always)]
    fn from(variant: CONNECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CONNECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONNECT_A {
        match self.bits {
            true => CONNECT_A::DISCONNECTED,
            false => CONNECT_A::CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == CONNECT_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `CONNECTED`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == CONNECT_A::CONNECTED
    }
}
#[doc = "Field `CONNECT` writer - Connection"]
pub type CONNECT_W<'a, const O: u8> = crate::BitWriter<'a, PSELRESET_SPEC, O, CONNECT_A>;
impl<'a, const O: u8> CONNECT_W<'a, O> {
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(CONNECT_A::DISCONNECTED)
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(CONNECT_A::CONNECTED)
    }
}
impl R {
    #[doc = "Bits 0:4 - GPIO pin number onto which nRESET is exposed"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Port number onto which nRESET is exposed"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    pub fn connect(&self) -> CONNECT_R {
        CONNECT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - GPIO pin number onto which nRESET is exposed"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PIN_W<0> {
        PIN_W::new(self)
    }
    #[doc = "Bit 5 - Port number onto which nRESET is exposed"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PORT_W<5> {
        PORT_W::new(self)
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    #[must_use]
    pub fn connect(&mut self) -> CONNECT_W<31> {
        CONNECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Mapping of the nRESET function (see POWER chapter for details)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselreset](index.html) module"]
pub struct PSELRESET_SPEC;
impl crate::RegisterSpec for PSELRESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pselreset::R](R) reader structure"]
impl crate::Readable for PSELRESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pselreset::W](W) writer structure"]
impl crate::Writable for PSELRESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSELRESET[%s]
to value 0xffff_ffff"]
impl crate::Resettable for PSELRESET_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
