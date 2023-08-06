#[doc = "Register `ISOSPLIT` reader"]
pub struct R(crate::R<ISOSPLIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISOSPLIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISOSPLIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISOSPLIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISOSPLIT` writer"]
pub struct W(crate::W<ISOSPLIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISOSPLIT_SPEC>;
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
impl From<crate::W<ISOSPLIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISOSPLIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPLIT` reader - Controls the split of ISO buffers"]
pub type SPLIT_R = crate::FieldReader<SPLIT_A>;
#[doc = "Controls the split of ISO buffers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SPLIT_A {
    #[doc = "0: Full buffer dedicated to either ISO IN or OUT"]
    ONE_DIR = 0,
    #[doc = "128: Lower half for IN, upper half for OUT"]
    HALF_IN = 128,
}
impl From<SPLIT_A> for u16 {
    #[inline(always)]
    fn from(variant: SPLIT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPLIT_A {
    type Ux = u16;
}
impl SPLIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPLIT_A> {
        match self.bits {
            0 => Some(SPLIT_A::ONE_DIR),
            128 => Some(SPLIT_A::HALF_IN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONE_DIR`"]
    #[inline(always)]
    pub fn is_one_dir(&self) -> bool {
        *self == SPLIT_A::ONE_DIR
    }
    #[doc = "Checks if the value of the field is `HALF_IN`"]
    #[inline(always)]
    pub fn is_half_in(&self) -> bool {
        *self == SPLIT_A::HALF_IN
    }
}
#[doc = "Field `SPLIT` writer - Controls the split of ISO buffers"]
pub type SPLIT_W<'a, const O: u8> = crate::FieldWriter<'a, ISOSPLIT_SPEC, 16, O, SPLIT_A>;
impl<'a, const O: u8> SPLIT_W<'a, O> {
    #[doc = "Full buffer dedicated to either ISO IN or OUT"]
    #[inline(always)]
    pub fn one_dir(self) -> &'a mut W {
        self.variant(SPLIT_A::ONE_DIR)
    }
    #[doc = "Lower half for IN, upper half for OUT"]
    #[inline(always)]
    pub fn half_in(self) -> &'a mut W {
        self.variant(SPLIT_A::HALF_IN)
    }
}
impl R {
    #[doc = "Bits 0:15 - Controls the split of ISO buffers"]
    #[inline(always)]
    pub fn split(&self) -> SPLIT_R {
        SPLIT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Controls the split of ISO buffers"]
    #[inline(always)]
    #[must_use]
    pub fn split(&mut self) -> SPLIT_W<0> {
        SPLIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the split of ISO buffers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isosplit](index.html) module"]
pub struct ISOSPLIT_SPEC;
impl crate::RegisterSpec for ISOSPLIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isosplit::R](R) reader structure"]
impl crate::Readable for ISOSPLIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isosplit::W](W) writer structure"]
impl crate::Writable for ISOSPLIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISOSPLIT to value 0"]
impl crate::Resettable for ISOSPLIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
