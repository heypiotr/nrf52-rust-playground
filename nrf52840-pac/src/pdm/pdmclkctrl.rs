#[doc = "Register `PDMCLKCTRL` reader"]
pub struct R(crate::R<PDMCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMCLKCTRL` writer"]
pub struct W(crate::W<PDMCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMCLKCTRL_SPEC>;
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
impl From<crate::W<PDMCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQ` reader - PDM_CLK frequency configuration"]
pub type FREQ_R = crate::FieldReader<FREQ_A>;
#[doc = "PDM_CLK frequency configuration\n\nValue on reset: 138412032"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum FREQ_A {
    #[doc = "134217728: PDM_CLK = 32 MHz / 32 = 1.000 MHz"]
    _1000K = 134217728,
    #[doc = "138412032: PDM_CLK = 32 MHz / 31 = 1.032 MHz. Nominal clock for RATIO=Ratio64."]
    DEFAULT = 138412032,
    #[doc = "142606336: PDM_CLK = 32 MHz / 30 = 1.067 MHz"]
    _1067K = 142606336,
    #[doc = "159383552: PDM_CLK = 32 MHz / 26 = 1.231 MHz"]
    _1231K = 159383552,
    #[doc = "167772160: PDM_CLK = 32 MHz / 25 = 1.280 MHz. Nominal clock for RATIO=Ratio80."]
    _1280K = 167772160,
    #[doc = "176160768: PDM_CLK = 32 MHz / 24 = 1.333 MHz"]
    _1333K = 176160768,
}
impl From<FREQ_A> for u32 {
    #[inline(always)]
    fn from(variant: FREQ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FREQ_A {
    type Ux = u32;
}
impl FREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FREQ_A> {
        match self.bits {
            134217728 => Some(FREQ_A::_1000K),
            138412032 => Some(FREQ_A::DEFAULT),
            142606336 => Some(FREQ_A::_1067K),
            159383552 => Some(FREQ_A::_1231K),
            167772160 => Some(FREQ_A::_1280K),
            176160768 => Some(FREQ_A::_1333K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1000K`"]
    #[inline(always)]
    pub fn is_1000k(&self) -> bool {
        *self == FREQ_A::_1000K
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == FREQ_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `_1067K`"]
    #[inline(always)]
    pub fn is_1067k(&self) -> bool {
        *self == FREQ_A::_1067K
    }
    #[doc = "Checks if the value of the field is `_1231K`"]
    #[inline(always)]
    pub fn is_1231k(&self) -> bool {
        *self == FREQ_A::_1231K
    }
    #[doc = "Checks if the value of the field is `_1280K`"]
    #[inline(always)]
    pub fn is_1280k(&self) -> bool {
        *self == FREQ_A::_1280K
    }
    #[doc = "Checks if the value of the field is `_1333K`"]
    #[inline(always)]
    pub fn is_1333k(&self) -> bool {
        *self == FREQ_A::_1333K
    }
}
#[doc = "Field `FREQ` writer - PDM_CLK frequency configuration"]
pub type FREQ_W<'a, const O: u8> = crate::FieldWriter<'a, PDMCLKCTRL_SPEC, 32, O, FREQ_A>;
impl<'a, const O: u8> FREQ_W<'a, O> {
    #[doc = "PDM_CLK = 32 MHz / 32 = 1.000 MHz"]
    #[inline(always)]
    pub fn _1000k(self) -> &'a mut W {
        self.variant(FREQ_A::_1000K)
    }
    #[doc = "PDM_CLK = 32 MHz / 31 = 1.032 MHz. Nominal clock for RATIO=Ratio64."]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(FREQ_A::DEFAULT)
    }
    #[doc = "PDM_CLK = 32 MHz / 30 = 1.067 MHz"]
    #[inline(always)]
    pub fn _1067k(self) -> &'a mut W {
        self.variant(FREQ_A::_1067K)
    }
    #[doc = "PDM_CLK = 32 MHz / 26 = 1.231 MHz"]
    #[inline(always)]
    pub fn _1231k(self) -> &'a mut W {
        self.variant(FREQ_A::_1231K)
    }
    #[doc = "PDM_CLK = 32 MHz / 25 = 1.280 MHz. Nominal clock for RATIO=Ratio80."]
    #[inline(always)]
    pub fn _1280k(self) -> &'a mut W {
        self.variant(FREQ_A::_1280K)
    }
    #[doc = "PDM_CLK = 32 MHz / 24 = 1.333 MHz"]
    #[inline(always)]
    pub fn _1333k(self) -> &'a mut W {
        self.variant(FREQ_A::_1333K)
    }
}
impl R {
    #[doc = "Bits 0:31 - PDM_CLK frequency configuration"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PDM_CLK frequency configuration"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<0> {
        FREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDM clock generator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdmclkctrl](index.html) module"]
pub struct PDMCLKCTRL_SPEC;
impl crate::RegisterSpec for PDMCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdmclkctrl::R](R) reader structure"]
impl crate::Readable for PDMCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdmclkctrl::W](W) writer structure"]
impl crate::Writable for PDMCLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDMCLKCTRL to value 0x0840_0000"]
impl crate::Resettable for PDMCLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0840_0000;
}
