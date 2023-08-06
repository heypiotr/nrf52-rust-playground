#[doc = "Register `LEN` reader"]
pub struct R(crate::R<LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEN` writer"]
pub struct W(crate::W<LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEN_SPEC>;
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
impl From<crate::W<LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN` reader - LEN"]
pub type LEN_R = crate::FieldReader<LEN_A>;
#[doc = "LEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEN_A {
    #[doc = "0: Erase 4 kB block (flash command 0x20)"]
    _4KB = 0,
    #[doc = "1: Erase 64 kB block (flash command 0xD8)"]
    _64KB = 1,
    #[doc = "2: Erase all (flash command 0xC7)"]
    ALL = 2,
}
impl From<LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: LEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LEN_A {
    type Ux = u8;
}
impl LEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LEN_A> {
        match self.bits {
            0 => Some(LEN_A::_4KB),
            1 => Some(LEN_A::_64KB),
            2 => Some(LEN_A::ALL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_4KB`"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == LEN_A::_4KB
    }
    #[doc = "Checks if the value of the field is `_64KB`"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == LEN_A::_64KB
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == LEN_A::ALL
    }
}
#[doc = "Field `LEN` writer - LEN"]
pub type LEN_W<'a, const O: u8> = crate::FieldWriter<'a, LEN_SPEC, 2, O, LEN_A>;
impl<'a, const O: u8> LEN_W<'a, O> {
    #[doc = "Erase 4 kB block (flash command 0x20)"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut W {
        self.variant(LEN_A::_4KB)
    }
    #[doc = "Erase 64 kB block (flash command 0xD8)"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut W {
        self.variant(LEN_A::_64KB)
    }
    #[doc = "Erase all (flash command 0xC7)"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(LEN_A::ALL)
    }
}
impl R {
    #[doc = "Bits 0:1 - LEN"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LEN"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<0> {
        LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Size of block to be erased.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [len](index.html) module"]
pub struct LEN_SPEC;
impl crate::RegisterSpec for LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [len::R](R) reader structure"]
impl crate::Readable for LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [len::W](W) writer structure"]
impl crate::Writable for LEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LEN to value 0"]
impl crate::Resettable for LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
