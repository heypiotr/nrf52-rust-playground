#[doc = "Register `POWERCLR` writer"]
pub struct W(crate::W<POWERCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWERCLR_SPEC>;
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
impl From<crate::W<POWERCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWERCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Keep RAM section S0 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S0POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S0POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0POWER` writer - Keep RAM section S0 of RAMn on or off in System ON mode"]
pub type S0POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S0POWER_AW>;
impl<'a, const O: u8> S0POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S0POWER_AW::OFF)
    }
}
#[doc = "Keep RAM section S1 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S1POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S1POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1POWER` writer - Keep RAM section S1 of RAMn on or off in System ON mode"]
pub type S1POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S1POWER_AW>;
impl<'a, const O: u8> S1POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S1POWER_AW::OFF)
    }
}
#[doc = "Keep RAM section S2 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S2POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S2POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2POWER` writer - Keep RAM section S2 of RAMn on or off in System ON mode"]
pub type S2POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S2POWER_AW>;
impl<'a, const O: u8> S2POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S2POWER_AW::OFF)
    }
}
#[doc = "Keep RAM section S3 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S3POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S3POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3POWER` writer - Keep RAM section S3 of RAMn on or off in System ON mode"]
pub type S3POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S3POWER_AW>;
impl<'a, const O: u8> S3POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S3POWER_AW::OFF)
    }
}
#[doc = "Keep RAM section S4 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S4POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S4POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S4POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S4POWER` writer - Keep RAM section S4 of RAMn on or off in System ON mode"]
pub type S4POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S4POWER_AW>;
impl<'a, const O: u8> S4POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S4POWER_AW::OFF)
    }
}
#[doc = "Keep RAM section S5 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S5POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S5POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S5POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S5POWER` writer - Keep RAM section S5 of RAMn on or off in System ON mode"]
pub type S5POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S5POWER_AW>;
impl<'a, const O: u8> S5POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S5POWER_AW::OFF)
    }
}
#[doc = "Keep RAM section S6 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S6POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S6POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S6POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S6POWER` writer - Keep RAM section S6 of RAMn on or off in System ON mode"]
pub type S6POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S6POWER_AW>;
impl<'a, const O: u8> S6POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S6POWER_AW::OFF)
    }
}
#[doc = "Keep RAM section S7 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S7POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S7POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S7POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S7POWER` writer - Keep RAM section S7 of RAMn on or off in System ON mode"]
pub type S7POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S7POWER_AW>;
impl<'a, const O: u8> S7POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S7POWER_AW::OFF)
    }
}
#[doc = "Keep RAM section S8 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S8POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S8POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S8POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S8POWER` writer - Keep RAM section S8 of RAMn on or off in System ON mode"]
pub type S8POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S8POWER_AW>;
impl<'a, const O: u8> S8POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S8POWER_AW::OFF)
    }
}
#[doc = "Keep RAM section S9 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S9POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S9POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S9POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S9POWER` writer - Keep RAM section S9 of RAMn on or off in System ON mode"]
pub type S9POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S9POWER_AW>;
impl<'a, const O: u8> S9POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S9POWER_AW::OFF)
    }
}
#[doc = "Keep RAM section S10 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S10POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S10POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S10POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S10POWER` writer - Keep RAM section S10 of RAMn on or off in System ON mode"]
pub type S10POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S10POWER_AW>;
impl<'a, const O: u8> S10POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S10POWER_AW::OFF)
    }
}
#[doc = "Keep RAM section S11 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S11POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S11POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S11POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S11POWER` writer - Keep RAM section S11 of RAMn on or off in System ON mode"]
pub type S11POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S11POWER_AW>;
impl<'a, const O: u8> S11POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S11POWER_AW::OFF)
    }
}
#[doc = "Keep RAM section S12 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S12POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S12POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S12POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S12POWER` writer - Keep RAM section S12 of RAMn on or off in System ON mode"]
pub type S12POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S12POWER_AW>;
impl<'a, const O: u8> S12POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S12POWER_AW::OFF)
    }
}
#[doc = "Keep RAM section S13 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S13POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S13POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S13POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S13POWER` writer - Keep RAM section S13 of RAMn on or off in System ON mode"]
pub type S13POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S13POWER_AW>;
impl<'a, const O: u8> S13POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S13POWER_AW::OFF)
    }
}
#[doc = "Keep RAM section S14 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S14POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S14POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S14POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S14POWER` writer - Keep RAM section S14 of RAMn on or off in System ON mode"]
pub type S14POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S14POWER_AW>;
impl<'a, const O: u8> S14POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S14POWER_AW::OFF)
    }
}
#[doc = "Keep RAM section S15 of RAMn on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S15POWER_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S15POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S15POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S15POWER` writer - Keep RAM section S15 of RAMn on or off in System ON mode"]
pub type S15POWER_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S15POWER_AW>;
impl<'a, const O: u8> S15POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S15POWER_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S0 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S0RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S0RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0RETENTION` writer - Keep retention on RAM section S0 when RAM section is switched off"]
pub type S0RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S0RETENTION_AW>;
impl<'a, const O: u8> S0RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S0RETENTION_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S1 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S1RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S1RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1RETENTION` writer - Keep retention on RAM section S1 when RAM section is switched off"]
pub type S1RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S1RETENTION_AW>;
impl<'a, const O: u8> S1RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S1RETENTION_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S2 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S2RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S2RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2RETENTION` writer - Keep retention on RAM section S2 when RAM section is switched off"]
pub type S2RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S2RETENTION_AW>;
impl<'a, const O: u8> S2RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S2RETENTION_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S3 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S3RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S3RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3RETENTION` writer - Keep retention on RAM section S3 when RAM section is switched off"]
pub type S3RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S3RETENTION_AW>;
impl<'a, const O: u8> S3RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S3RETENTION_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S4 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S4RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S4RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S4RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S4RETENTION` writer - Keep retention on RAM section S4 when RAM section is switched off"]
pub type S4RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S4RETENTION_AW>;
impl<'a, const O: u8> S4RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S4RETENTION_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S5 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S5RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S5RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S5RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S5RETENTION` writer - Keep retention on RAM section S5 when RAM section is switched off"]
pub type S5RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S5RETENTION_AW>;
impl<'a, const O: u8> S5RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S5RETENTION_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S6 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S6RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S6RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S6RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S6RETENTION` writer - Keep retention on RAM section S6 when RAM section is switched off"]
pub type S6RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S6RETENTION_AW>;
impl<'a, const O: u8> S6RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S6RETENTION_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S7 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S7RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S7RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S7RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S7RETENTION` writer - Keep retention on RAM section S7 when RAM section is switched off"]
pub type S7RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S7RETENTION_AW>;
impl<'a, const O: u8> S7RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S7RETENTION_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S8 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S8RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S8RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S8RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S8RETENTION` writer - Keep retention on RAM section S8 when RAM section is switched off"]
pub type S8RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S8RETENTION_AW>;
impl<'a, const O: u8> S8RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S8RETENTION_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S9 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S9RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S9RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S9RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S9RETENTION` writer - Keep retention on RAM section S9 when RAM section is switched off"]
pub type S9RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S9RETENTION_AW>;
impl<'a, const O: u8> S9RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S9RETENTION_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S10 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S10RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S10RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S10RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S10RETENTION` writer - Keep retention on RAM section S10 when RAM section is switched off"]
pub type S10RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S10RETENTION_AW>;
impl<'a, const O: u8> S10RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S10RETENTION_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S11 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S11RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S11RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S11RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S11RETENTION` writer - Keep retention on RAM section S11 when RAM section is switched off"]
pub type S11RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S11RETENTION_AW>;
impl<'a, const O: u8> S11RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S11RETENTION_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S12 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S12RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S12RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S12RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S12RETENTION` writer - Keep retention on RAM section S12 when RAM section is switched off"]
pub type S12RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S12RETENTION_AW>;
impl<'a, const O: u8> S12RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S12RETENTION_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S13 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S13RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S13RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S13RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S13RETENTION` writer - Keep retention on RAM section S13 when RAM section is switched off"]
pub type S13RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S13RETENTION_AW>;
impl<'a, const O: u8> S13RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S13RETENTION_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S14 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S14RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S14RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S14RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S14RETENTION` writer - Keep retention on RAM section S14 when RAM section is switched off"]
pub type S14RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S14RETENTION_AW>;
impl<'a, const O: u8> S14RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S14RETENTION_AW::OFF)
    }
}
#[doc = "Keep retention on RAM section S15 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S15RETENTION_AW {
    #[doc = "1: Off"]
    OFF = 1,
}
impl From<S15RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S15RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S15RETENTION` writer - Keep retention on RAM section S15 when RAM section is switched off"]
pub type S15RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, POWERCLR_SPEC, O, S15RETENTION_AW>;
impl<'a, const O: u8> S15RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S15RETENTION_AW::OFF)
    }
}
impl W {
    #[doc = "Bit 0 - Keep RAM section S0 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s0power(&mut self) -> S0POWER_W<0> {
        S0POWER_W::new(self)
    }
    #[doc = "Bit 1 - Keep RAM section S1 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s1power(&mut self) -> S1POWER_W<1> {
        S1POWER_W::new(self)
    }
    #[doc = "Bit 2 - Keep RAM section S2 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s2power(&mut self) -> S2POWER_W<2> {
        S2POWER_W::new(self)
    }
    #[doc = "Bit 3 - Keep RAM section S3 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s3power(&mut self) -> S3POWER_W<3> {
        S3POWER_W::new(self)
    }
    #[doc = "Bit 4 - Keep RAM section S4 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s4power(&mut self) -> S4POWER_W<4> {
        S4POWER_W::new(self)
    }
    #[doc = "Bit 5 - Keep RAM section S5 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s5power(&mut self) -> S5POWER_W<5> {
        S5POWER_W::new(self)
    }
    #[doc = "Bit 6 - Keep RAM section S6 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s6power(&mut self) -> S6POWER_W<6> {
        S6POWER_W::new(self)
    }
    #[doc = "Bit 7 - Keep RAM section S7 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s7power(&mut self) -> S7POWER_W<7> {
        S7POWER_W::new(self)
    }
    #[doc = "Bit 8 - Keep RAM section S8 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s8power(&mut self) -> S8POWER_W<8> {
        S8POWER_W::new(self)
    }
    #[doc = "Bit 9 - Keep RAM section S9 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s9power(&mut self) -> S9POWER_W<9> {
        S9POWER_W::new(self)
    }
    #[doc = "Bit 10 - Keep RAM section S10 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s10power(&mut self) -> S10POWER_W<10> {
        S10POWER_W::new(self)
    }
    #[doc = "Bit 11 - Keep RAM section S11 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s11power(&mut self) -> S11POWER_W<11> {
        S11POWER_W::new(self)
    }
    #[doc = "Bit 12 - Keep RAM section S12 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s12power(&mut self) -> S12POWER_W<12> {
        S12POWER_W::new(self)
    }
    #[doc = "Bit 13 - Keep RAM section S13 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s13power(&mut self) -> S13POWER_W<13> {
        S13POWER_W::new(self)
    }
    #[doc = "Bit 14 - Keep RAM section S14 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s14power(&mut self) -> S14POWER_W<14> {
        S14POWER_W::new(self)
    }
    #[doc = "Bit 15 - Keep RAM section S15 of RAMn on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s15power(&mut self) -> S15POWER_W<15> {
        S15POWER_W::new(self)
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s0retention(&mut self) -> S0RETENTION_W<16> {
        S0RETENTION_W::new(self)
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s1retention(&mut self) -> S1RETENTION_W<17> {
        S1RETENTION_W::new(self)
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s2retention(&mut self) -> S2RETENTION_W<18> {
        S2RETENTION_W::new(self)
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s3retention(&mut self) -> S3RETENTION_W<19> {
        S3RETENTION_W::new(self)
    }
    #[doc = "Bit 20 - Keep retention on RAM section S4 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s4retention(&mut self) -> S4RETENTION_W<20> {
        S4RETENTION_W::new(self)
    }
    #[doc = "Bit 21 - Keep retention on RAM section S5 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s5retention(&mut self) -> S5RETENTION_W<21> {
        S5RETENTION_W::new(self)
    }
    #[doc = "Bit 22 - Keep retention on RAM section S6 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s6retention(&mut self) -> S6RETENTION_W<22> {
        S6RETENTION_W::new(self)
    }
    #[doc = "Bit 23 - Keep retention on RAM section S7 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s7retention(&mut self) -> S7RETENTION_W<23> {
        S7RETENTION_W::new(self)
    }
    #[doc = "Bit 24 - Keep retention on RAM section S8 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s8retention(&mut self) -> S8RETENTION_W<24> {
        S8RETENTION_W::new(self)
    }
    #[doc = "Bit 25 - Keep retention on RAM section S9 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s9retention(&mut self) -> S9RETENTION_W<25> {
        S9RETENTION_W::new(self)
    }
    #[doc = "Bit 26 - Keep retention on RAM section S10 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s10retention(&mut self) -> S10RETENTION_W<26> {
        S10RETENTION_W::new(self)
    }
    #[doc = "Bit 27 - Keep retention on RAM section S11 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s11retention(&mut self) -> S11RETENTION_W<27> {
        S11RETENTION_W::new(self)
    }
    #[doc = "Bit 28 - Keep retention on RAM section S12 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s12retention(&mut self) -> S12RETENTION_W<28> {
        S12RETENTION_W::new(self)
    }
    #[doc = "Bit 29 - Keep retention on RAM section S13 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s13retention(&mut self) -> S13RETENTION_W<29> {
        S13RETENTION_W::new(self)
    }
    #[doc = "Bit 30 - Keep retention on RAM section S14 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s14retention(&mut self) -> S14RETENTION_W<30> {
        S14RETENTION_W::new(self)
    }
    #[doc = "Bit 31 - Keep retention on RAM section S15 when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s15retention(&mut self) -> S15RETENTION_W<31> {
        S15RETENTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: RAMn power control clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [powerclr](index.html) module"]
pub struct POWERCLR_SPEC;
impl crate::RegisterSpec for POWERCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [powerclr::W](W) writer structure"]
impl crate::Writable for POWERCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWERCLR to value 0xffff"]
impl crate::Resettable for POWERCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
