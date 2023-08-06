#[doc = "Register `VARIANT` reader"]
pub struct R(crate::R<VARIANT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VARIANT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VARIANT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VARIANT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VARIANT` reader - Build code (hardware version and production configuration). Encoded as ASCII."]
pub type VARIANT_R = crate::FieldReader<VARIANT_A>;
#[doc = "Build code (hardware version and production configuration). Encoded as ASCII.\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum VARIANT_A {
    #[doc = "1094795585: AAAA"]
    AAAA = 1094795585,
    #[doc = "1111572801: BAAA"]
    BAAA = 1111572801,
    #[doc = "1128350017: CAAA"]
    CAAA = 1128350017,
    #[doc = "1094795841: AABA"]
    AABA = 1094795841,
    #[doc = "1094795842: AABB"]
    AABB = 1094795842,
    #[doc = "1094796097: AACA"]
    AACA = 1094796097,
    #[doc = "1094795586: AAAB"]
    AAAB = 1094795586,
    #[doc = "1094796080: AAC0"]
    AAC0 = 1094796080,
    #[doc = "1094796353: AADA"]
    AADA = 1094796353,
    #[doc = "1094796336: AAD0"]
    AAD0 = 1094796336,
    #[doc = "1094796337: AAD1"]
    AAD1 = 1094796337,
    #[doc = "1094796609: AAEA"]
    AAEA = 1094796609,
    #[doc = "1094796848: AAF0"]
    AAF0 = 1094796848,
    #[doc = "1094796865: AAFA"]
    AAFA = 1094796865,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<VARIANT_A> for u32 {
    #[inline(always)]
    fn from(variant: VARIANT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VARIANT_A {
    type Ux = u32;
}
impl VARIANT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VARIANT_A> {
        match self.bits {
            1094795585 => Some(VARIANT_A::AAAA),
            1111572801 => Some(VARIANT_A::BAAA),
            1128350017 => Some(VARIANT_A::CAAA),
            1094795841 => Some(VARIANT_A::AABA),
            1094795842 => Some(VARIANT_A::AABB),
            1094796097 => Some(VARIANT_A::AACA),
            1094795586 => Some(VARIANT_A::AAAB),
            1094796080 => Some(VARIANT_A::AAC0),
            1094796353 => Some(VARIANT_A::AADA),
            1094796336 => Some(VARIANT_A::AAD0),
            1094796337 => Some(VARIANT_A::AAD1),
            1094796609 => Some(VARIANT_A::AAEA),
            1094796848 => Some(VARIANT_A::AAF0),
            1094796865 => Some(VARIANT_A::AAFA),
            4294967295 => Some(VARIANT_A::UNSPECIFIED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AAAA`"]
    #[inline(always)]
    pub fn is_aaaa(&self) -> bool {
        *self == VARIANT_A::AAAA
    }
    #[doc = "Checks if the value of the field is `BAAA`"]
    #[inline(always)]
    pub fn is_baaa(&self) -> bool {
        *self == VARIANT_A::BAAA
    }
    #[doc = "Checks if the value of the field is `CAAA`"]
    #[inline(always)]
    pub fn is_caaa(&self) -> bool {
        *self == VARIANT_A::CAAA
    }
    #[doc = "Checks if the value of the field is `AABA`"]
    #[inline(always)]
    pub fn is_aaba(&self) -> bool {
        *self == VARIANT_A::AABA
    }
    #[doc = "Checks if the value of the field is `AABB`"]
    #[inline(always)]
    pub fn is_aabb(&self) -> bool {
        *self == VARIANT_A::AABB
    }
    #[doc = "Checks if the value of the field is `AACA`"]
    #[inline(always)]
    pub fn is_aaca(&self) -> bool {
        *self == VARIANT_A::AACA
    }
    #[doc = "Checks if the value of the field is `AAAB`"]
    #[inline(always)]
    pub fn is_aaab(&self) -> bool {
        *self == VARIANT_A::AAAB
    }
    #[doc = "Checks if the value of the field is `AAC0`"]
    #[inline(always)]
    pub fn is_aac0(&self) -> bool {
        *self == VARIANT_A::AAC0
    }
    #[doc = "Checks if the value of the field is `AADA`"]
    #[inline(always)]
    pub fn is_aada(&self) -> bool {
        *self == VARIANT_A::AADA
    }
    #[doc = "Checks if the value of the field is `AAD0`"]
    #[inline(always)]
    pub fn is_aad0(&self) -> bool {
        *self == VARIANT_A::AAD0
    }
    #[doc = "Checks if the value of the field is `AAD1`"]
    #[inline(always)]
    pub fn is_aad1(&self) -> bool {
        *self == VARIANT_A::AAD1
    }
    #[doc = "Checks if the value of the field is `AAEA`"]
    #[inline(always)]
    pub fn is_aaea(&self) -> bool {
        *self == VARIANT_A::AAEA
    }
    #[doc = "Checks if the value of the field is `AAF0`"]
    #[inline(always)]
    pub fn is_aaf0(&self) -> bool {
        *self == VARIANT_A::AAF0
    }
    #[doc = "Checks if the value of the field is `AAFA`"]
    #[inline(always)]
    pub fn is_aafa(&self) -> bool {
        *self == VARIANT_A::AAFA
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == VARIANT_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Build code (hardware version and production configuration). Encoded as ASCII."]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(self.bits)
    }
}
#[doc = "Build code (hardware version and production configuration)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [variant](index.html) module"]
pub struct VARIANT_SPEC;
impl crate::RegisterSpec for VARIANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [variant::R](R) reader structure"]
impl crate::Readable for VARIANT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VARIANT to value 0xffff_ffff"]
impl crate::Resettable for VARIANT_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
