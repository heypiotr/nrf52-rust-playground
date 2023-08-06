#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - The mode of operation to be used. Settings in this register apply whenever either the KSGEN task or the CRYPT task is triggered."]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "The mode of operation to be used. Settings in this register apply whenever either the KSGEN task or the CRYPT task is triggered.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: AES CCM packet encryption mode"]
    ENCRYPTION = 0,
    #[doc = "1: AES CCM packet decryption mode"]
    DECRYPTION = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::ENCRYPTION,
            true => MODE_A::DECRYPTION,
        }
    }
    #[doc = "Checks if the value of the field is `ENCRYPTION`"]
    #[inline(always)]
    pub fn is_encryption(&self) -> bool {
        *self == MODE_A::ENCRYPTION
    }
    #[doc = "Checks if the value of the field is `DECRYPTION`"]
    #[inline(always)]
    pub fn is_decryption(&self) -> bool {
        *self == MODE_A::DECRYPTION
    }
}
#[doc = "Field `MODE` writer - The mode of operation to be used. Settings in this register apply whenever either the KSGEN task or the CRYPT task is triggered."]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, MODE_SPEC, O, MODE_A>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "AES CCM packet encryption mode"]
    #[inline(always)]
    pub fn encryption(self) -> &'a mut W {
        self.variant(MODE_A::ENCRYPTION)
    }
    #[doc = "AES CCM packet decryption mode"]
    #[inline(always)]
    pub fn decryption(self) -> &'a mut W {
        self.variant(MODE_A::DECRYPTION)
    }
}
#[doc = "Field `DATARATE` reader - Radio data rate that the CCM shall run synchronous with"]
pub type DATARATE_R = crate::FieldReader<DATARATE_A>;
#[doc = "Radio data rate that the CCM shall run synchronous with\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATARATE_A {
    #[doc = "0: 1 Mbps"]
    _1MBIT = 0,
    #[doc = "1: 2 Mbps"]
    _2MBIT = 1,
    #[doc = "2: 125 kbps"]
    _125KBPS = 2,
    #[doc = "3: 500 kbps"]
    _500KBPS = 3,
}
impl From<DATARATE_A> for u8 {
    #[inline(always)]
    fn from(variant: DATARATE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATARATE_A {
    type Ux = u8;
}
impl DATARATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATARATE_A {
        match self.bits {
            0 => DATARATE_A::_1MBIT,
            1 => DATARATE_A::_2MBIT,
            2 => DATARATE_A::_125KBPS,
            3 => DATARATE_A::_500KBPS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1MBIT`"]
    #[inline(always)]
    pub fn is_1mbit(&self) -> bool {
        *self == DATARATE_A::_1MBIT
    }
    #[doc = "Checks if the value of the field is `_2MBIT`"]
    #[inline(always)]
    pub fn is_2mbit(&self) -> bool {
        *self == DATARATE_A::_2MBIT
    }
    #[doc = "Checks if the value of the field is `_125KBPS`"]
    #[inline(always)]
    pub fn is_125kbps(&self) -> bool {
        *self == DATARATE_A::_125KBPS
    }
    #[doc = "Checks if the value of the field is `_500KBPS`"]
    #[inline(always)]
    pub fn is_500kbps(&self) -> bool {
        *self == DATARATE_A::_500KBPS
    }
}
#[doc = "Field `DATARATE` writer - Radio data rate that the CCM shall run synchronous with"]
pub type DATARATE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, MODE_SPEC, 2, O, DATARATE_A>;
impl<'a, const O: u8> DATARATE_W<'a, O> {
    #[doc = "1 Mbps"]
    #[inline(always)]
    pub fn _1mbit(self) -> &'a mut W {
        self.variant(DATARATE_A::_1MBIT)
    }
    #[doc = "2 Mbps"]
    #[inline(always)]
    pub fn _2mbit(self) -> &'a mut W {
        self.variant(DATARATE_A::_2MBIT)
    }
    #[doc = "125 kbps"]
    #[inline(always)]
    pub fn _125kbps(self) -> &'a mut W {
        self.variant(DATARATE_A::_125KBPS)
    }
    #[doc = "500 kbps"]
    #[inline(always)]
    pub fn _500kbps(self) -> &'a mut W {
        self.variant(DATARATE_A::_500KBPS)
    }
}
#[doc = "Field `LENGTH` reader - Packet length configuration"]
pub type LENGTH_R = crate::BitReader<LENGTH_A>;
#[doc = "Packet length configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LENGTH_A {
    #[doc = "0: Default length. Effective length of LENGTH field in encrypted/decrypted packet is 5 bits. A keystream for packet payloads up to 27 bytes will be generated."]
    DEFAULT = 0,
    #[doc = "1: Extended length. Effective length of LENGTH field in encrypted/decrypted packet is 8 bits. A keystream for packet payloads up to MAXPACKETSIZE bytes will be generated."]
    EXTENDED = 1,
}
impl From<LENGTH_A> for bool {
    #[inline(always)]
    fn from(variant: LENGTH_A) -> Self {
        variant as u8 != 0
    }
}
impl LENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LENGTH_A {
        match self.bits {
            false => LENGTH_A::DEFAULT,
            true => LENGTH_A::EXTENDED,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LENGTH_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == LENGTH_A::EXTENDED
    }
}
#[doc = "Field `LENGTH` writer - Packet length configuration"]
pub type LENGTH_W<'a, const O: u8> = crate::BitWriter<'a, MODE_SPEC, O, LENGTH_A>;
impl<'a, const O: u8> LENGTH_W<'a, O> {
    #[doc = "Default length. Effective length of LENGTH field in encrypted/decrypted packet is 5 bits. A keystream for packet payloads up to 27 bytes will be generated."]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(LENGTH_A::DEFAULT)
    }
    #[doc = "Extended length. Effective length of LENGTH field in encrypted/decrypted packet is 8 bits. A keystream for packet payloads up to MAXPACKETSIZE bytes will be generated."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(LENGTH_A::EXTENDED)
    }
}
impl R {
    #[doc = "Bit 0 - The mode of operation to be used. Settings in this register apply whenever either the KSGEN task or the CRYPT task is triggered."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:17 - Radio data rate that the CCM shall run synchronous with"]
    #[inline(always)]
    pub fn datarate(&self) -> DATARATE_R {
        DATARATE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Packet length configuration"]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The mode of operation to be used. Settings in this register apply whenever either the KSGEN task or the CRYPT task is triggered."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bits 16:17 - Radio data rate that the CCM shall run synchronous with"]
    #[inline(always)]
    #[must_use]
    pub fn datarate(&mut self) -> DATARATE_W<16> {
        DATARATE_W::new(self)
    }
    #[doc = "Bit 24 - Packet length configuration"]
    #[inline(always)]
    #[must_use]
    pub fn length(&mut self) -> LENGTH_W<24> {
        LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operation mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE to value 0x01"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
