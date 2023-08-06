#[doc = "Register `IFCONFIG0` reader"]
pub struct R(crate::R<IFCONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFCONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFCONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFCONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFCONFIG0` writer"]
pub struct W(crate::W<IFCONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCONFIG0_SPEC>;
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
impl From<crate::W<IFCONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READOC` reader - Configure number of data lines and opcode used for reading."]
pub type READOC_R = crate::FieldReader<READOC_A>;
#[doc = "Configure number of data lines and opcode used for reading.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum READOC_A {
    #[doc = "0: Single data line SPI. FAST_READ (opcode 0x0B)."]
    FASTREAD = 0,
    #[doc = "1: Dual data line SPI. READ2O (opcode 0x3B)."]
    READ2O = 1,
    #[doc = "2: Dual data line SPI. READ2IO (opcode 0xBB)."]
    READ2IO = 2,
    #[doc = "3: Quad data line SPI. READ4O (opcode 0x6B)."]
    READ4O = 3,
    #[doc = "4: Quad data line SPI. READ4IO (opcode 0xEB)."]
    READ4IO = 4,
}
impl From<READOC_A> for u8 {
    #[inline(always)]
    fn from(variant: READOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for READOC_A {
    type Ux = u8;
}
impl READOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<READOC_A> {
        match self.bits {
            0 => Some(READOC_A::FASTREAD),
            1 => Some(READOC_A::READ2O),
            2 => Some(READOC_A::READ2IO),
            3 => Some(READOC_A::READ4O),
            4 => Some(READOC_A::READ4IO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FASTREAD`"]
    #[inline(always)]
    pub fn is_fastread(&self) -> bool {
        *self == READOC_A::FASTREAD
    }
    #[doc = "Checks if the value of the field is `READ2O`"]
    #[inline(always)]
    pub fn is_read2o(&self) -> bool {
        *self == READOC_A::READ2O
    }
    #[doc = "Checks if the value of the field is `READ2IO`"]
    #[inline(always)]
    pub fn is_read2io(&self) -> bool {
        *self == READOC_A::READ2IO
    }
    #[doc = "Checks if the value of the field is `READ4O`"]
    #[inline(always)]
    pub fn is_read4o(&self) -> bool {
        *self == READOC_A::READ4O
    }
    #[doc = "Checks if the value of the field is `READ4IO`"]
    #[inline(always)]
    pub fn is_read4io(&self) -> bool {
        *self == READOC_A::READ4IO
    }
}
#[doc = "Field `READOC` writer - Configure number of data lines and opcode used for reading."]
pub type READOC_W<'a, const O: u8> = crate::FieldWriter<'a, IFCONFIG0_SPEC, 3, O, READOC_A>;
impl<'a, const O: u8> READOC_W<'a, O> {
    #[doc = "Single data line SPI. FAST_READ (opcode 0x0B)."]
    #[inline(always)]
    pub fn fastread(self) -> &'a mut W {
        self.variant(READOC_A::FASTREAD)
    }
    #[doc = "Dual data line SPI. READ2O (opcode 0x3B)."]
    #[inline(always)]
    pub fn read2o(self) -> &'a mut W {
        self.variant(READOC_A::READ2O)
    }
    #[doc = "Dual data line SPI. READ2IO (opcode 0xBB)."]
    #[inline(always)]
    pub fn read2io(self) -> &'a mut W {
        self.variant(READOC_A::READ2IO)
    }
    #[doc = "Quad data line SPI. READ4O (opcode 0x6B)."]
    #[inline(always)]
    pub fn read4o(self) -> &'a mut W {
        self.variant(READOC_A::READ4O)
    }
    #[doc = "Quad data line SPI. READ4IO (opcode 0xEB)."]
    #[inline(always)]
    pub fn read4io(self) -> &'a mut W {
        self.variant(READOC_A::READ4IO)
    }
}
#[doc = "Field `WRITEOC` reader - Configure number of data lines and opcode used for writing."]
pub type WRITEOC_R = crate::FieldReader<WRITEOC_A>;
#[doc = "Configure number of data lines and opcode used for writing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRITEOC_A {
    #[doc = "0: Single data line SPI. PP (opcode 0x02)."]
    PP = 0,
    #[doc = "1: Dual data line SPI. PP2O (opcode 0xA2)."]
    PP2O = 1,
    #[doc = "2: Quad data line SPI. PP4O (opcode 0x32)."]
    PP4O = 2,
    #[doc = "3: Quad data line SPI. PP4IO (opcode 0x38)."]
    PP4IO = 3,
}
impl From<WRITEOC_A> for u8 {
    #[inline(always)]
    fn from(variant: WRITEOC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WRITEOC_A {
    type Ux = u8;
}
impl WRITEOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WRITEOC_A> {
        match self.bits {
            0 => Some(WRITEOC_A::PP),
            1 => Some(WRITEOC_A::PP2O),
            2 => Some(WRITEOC_A::PP4O),
            3 => Some(WRITEOC_A::PP4IO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PP`"]
    #[inline(always)]
    pub fn is_pp(&self) -> bool {
        *self == WRITEOC_A::PP
    }
    #[doc = "Checks if the value of the field is `PP2O`"]
    #[inline(always)]
    pub fn is_pp2o(&self) -> bool {
        *self == WRITEOC_A::PP2O
    }
    #[doc = "Checks if the value of the field is `PP4O`"]
    #[inline(always)]
    pub fn is_pp4o(&self) -> bool {
        *self == WRITEOC_A::PP4O
    }
    #[doc = "Checks if the value of the field is `PP4IO`"]
    #[inline(always)]
    pub fn is_pp4io(&self) -> bool {
        *self == WRITEOC_A::PP4IO
    }
}
#[doc = "Field `WRITEOC` writer - Configure number of data lines and opcode used for writing."]
pub type WRITEOC_W<'a, const O: u8> = crate::FieldWriter<'a, IFCONFIG0_SPEC, 3, O, WRITEOC_A>;
impl<'a, const O: u8> WRITEOC_W<'a, O> {
    #[doc = "Single data line SPI. PP (opcode 0x02)."]
    #[inline(always)]
    pub fn pp(self) -> &'a mut W {
        self.variant(WRITEOC_A::PP)
    }
    #[doc = "Dual data line SPI. PP2O (opcode 0xA2)."]
    #[inline(always)]
    pub fn pp2o(self) -> &'a mut W {
        self.variant(WRITEOC_A::PP2O)
    }
    #[doc = "Quad data line SPI. PP4O (opcode 0x32)."]
    #[inline(always)]
    pub fn pp4o(self) -> &'a mut W {
        self.variant(WRITEOC_A::PP4O)
    }
    #[doc = "Quad data line SPI. PP4IO (opcode 0x38)."]
    #[inline(always)]
    pub fn pp4io(self) -> &'a mut W {
        self.variant(WRITEOC_A::PP4IO)
    }
}
#[doc = "Field `ADDRMODE` reader - Addressing mode."]
pub type ADDRMODE_R = crate::BitReader<ADDRMODE_A>;
#[doc = "Addressing mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRMODE_A {
    #[doc = "0: 24-bit addressing."]
    _24BIT = 0,
    #[doc = "1: 32-bit addressing."]
    _32BIT = 1,
}
impl From<ADDRMODE_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRMODE_A {
        match self.bits {
            false => ADDRMODE_A::_24BIT,
            true => ADDRMODE_A::_32BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_24BIT`"]
    #[inline(always)]
    pub fn is_24bit(&self) -> bool {
        *self == ADDRMODE_A::_24BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == ADDRMODE_A::_32BIT
    }
}
#[doc = "Field `ADDRMODE` writer - Addressing mode."]
pub type ADDRMODE_W<'a, const O: u8> = crate::BitWriter<'a, IFCONFIG0_SPEC, O, ADDRMODE_A>;
impl<'a, const O: u8> ADDRMODE_W<'a, O> {
    #[doc = "24-bit addressing."]
    #[inline(always)]
    pub fn _24bit(self) -> &'a mut W {
        self.variant(ADDRMODE_A::_24BIT)
    }
    #[doc = "32-bit addressing."]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(ADDRMODE_A::_32BIT)
    }
}
#[doc = "Field `DPMENABLE` reader - Enable deep power-down mode (DPM) feature."]
pub type DPMENABLE_R = crate::BitReader<DPMENABLE_A>;
#[doc = "Enable deep power-down mode (DPM) feature.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPMENABLE_A {
    #[doc = "0: Disable DPM feature."]
    DISABLE = 0,
    #[doc = "1: Enable DPM feature."]
    ENABLE = 1,
}
impl From<DPMENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DPMENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl DPMENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPMENABLE_A {
        match self.bits {
            false => DPMENABLE_A::DISABLE,
            true => DPMENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DPMENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DPMENABLE_A::ENABLE
    }
}
#[doc = "Field `DPMENABLE` writer - Enable deep power-down mode (DPM) feature."]
pub type DPMENABLE_W<'a, const O: u8> = crate::BitWriter<'a, IFCONFIG0_SPEC, O, DPMENABLE_A>;
impl<'a, const O: u8> DPMENABLE_W<'a, O> {
    #[doc = "Disable DPM feature."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DPMENABLE_A::DISABLE)
    }
    #[doc = "Enable DPM feature."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DPMENABLE_A::ENABLE)
    }
}
#[doc = "Field `PPSIZE` reader - Page size for commands PP, PP2O, PP4O and PP4IO."]
pub type PPSIZE_R = crate::BitReader<PPSIZE_A>;
#[doc = "Page size for commands PP, PP2O, PP4O and PP4IO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPSIZE_A {
    #[doc = "0: 256 bytes."]
    _256BYTES = 0,
    #[doc = "1: 512 bytes."]
    _512BYTES = 1,
}
impl From<PPSIZE_A> for bool {
    #[inline(always)]
    fn from(variant: PPSIZE_A) -> Self {
        variant as u8 != 0
    }
}
impl PPSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPSIZE_A {
        match self.bits {
            false => PPSIZE_A::_256BYTES,
            true => PPSIZE_A::_512BYTES,
        }
    }
    #[doc = "Checks if the value of the field is `_256BYTES`"]
    #[inline(always)]
    pub fn is_256bytes(&self) -> bool {
        *self == PPSIZE_A::_256BYTES
    }
    #[doc = "Checks if the value of the field is `_512BYTES`"]
    #[inline(always)]
    pub fn is_512bytes(&self) -> bool {
        *self == PPSIZE_A::_512BYTES
    }
}
#[doc = "Field `PPSIZE` writer - Page size for commands PP, PP2O, PP4O and PP4IO."]
pub type PPSIZE_W<'a, const O: u8> = crate::BitWriter<'a, IFCONFIG0_SPEC, O, PPSIZE_A>;
impl<'a, const O: u8> PPSIZE_W<'a, O> {
    #[doc = "256 bytes."]
    #[inline(always)]
    pub fn _256bytes(self) -> &'a mut W {
        self.variant(PPSIZE_A::_256BYTES)
    }
    #[doc = "512 bytes."]
    #[inline(always)]
    pub fn _512bytes(self) -> &'a mut W {
        self.variant(PPSIZE_A::_512BYTES)
    }
}
impl R {
    #[doc = "Bits 0:2 - Configure number of data lines and opcode used for reading."]
    #[inline(always)]
    pub fn readoc(&self) -> READOC_R {
        READOC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Configure number of data lines and opcode used for writing."]
    #[inline(always)]
    pub fn writeoc(&self) -> WRITEOC_R {
        WRITEOC_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Addressing mode."]
    #[inline(always)]
    pub fn addrmode(&self) -> ADDRMODE_R {
        ADDRMODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable deep power-down mode (DPM) feature."]
    #[inline(always)]
    pub fn dpmenable(&self) -> DPMENABLE_R {
        DPMENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Page size for commands PP, PP2O, PP4O and PP4IO."]
    #[inline(always)]
    pub fn ppsize(&self) -> PPSIZE_R {
        PPSIZE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Configure number of data lines and opcode used for reading."]
    #[inline(always)]
    #[must_use]
    pub fn readoc(&mut self) -> READOC_W<0> {
        READOC_W::new(self)
    }
    #[doc = "Bits 3:5 - Configure number of data lines and opcode used for writing."]
    #[inline(always)]
    #[must_use]
    pub fn writeoc(&mut self) -> WRITEOC_W<3> {
        WRITEOC_W::new(self)
    }
    #[doc = "Bit 6 - Addressing mode."]
    #[inline(always)]
    #[must_use]
    pub fn addrmode(&mut self) -> ADDRMODE_W<6> {
        ADDRMODE_W::new(self)
    }
    #[doc = "Bit 7 - Enable deep power-down mode (DPM) feature."]
    #[inline(always)]
    #[must_use]
    pub fn dpmenable(&mut self) -> DPMENABLE_W<7> {
        DPMENABLE_W::new(self)
    }
    #[doc = "Bit 12 - Page size for commands PP, PP2O, PP4O and PP4IO."]
    #[inline(always)]
    #[must_use]
    pub fn ppsize(&mut self) -> PPSIZE_W<12> {
        PPSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interface configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifconfig0](index.html) module"]
pub struct IFCONFIG0_SPEC;
impl crate::RegisterSpec for IFCONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifconfig0::R](R) reader structure"]
impl crate::Readable for IFCONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifconfig0::W](W) writer structure"]
impl crate::Writable for IFCONFIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFCONFIG0 to value 0"]
impl crate::Resettable for IFCONFIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
