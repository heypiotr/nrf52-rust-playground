#[doc = "Register `CINSTRCONF` reader"]
pub struct R(crate::R<CINSTRCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CINSTRCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CINSTRCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CINSTRCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CINSTRCONF` writer"]
pub struct W(crate::W<CINSTRCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CINSTRCONF_SPEC>;
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
impl From<crate::W<CINSTRCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CINSTRCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPCODE` reader - Opcode of Custom instruction."]
pub type OPCODE_R = crate::FieldReader;
#[doc = "Field `OPCODE` writer - Opcode of Custom instruction."]
pub type OPCODE_W<'a, const O: u8> = crate::FieldWriter<'a, CINSTRCONF_SPEC, 8, O>;
#[doc = "Field `LENGTH` reader - Length of custom instruction in number of bytes."]
pub type LENGTH_R = crate::FieldReader<LENGTH_A>;
#[doc = "Length of custom instruction in number of bytes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LENGTH_A {
    #[doc = "1: Send opcode only."]
    _1B = 1,
    #[doc = "2: Send opcode, CINSTRDAT0.BYTE0."]
    _2B = 2,
    #[doc = "3: Send opcode, CINSTRDAT0.BYTE0 -&amp;gt; CINSTRDAT0.BYTE1."]
    _3B = 3,
    #[doc = "4: Send opcode, CINSTRDAT0.BYTE0 -&amp;gt; CINSTRDAT0.BYTE2."]
    _4B = 4,
    #[doc = "5: Send opcode, CINSTRDAT0.BYTE0 -&amp;gt; CINSTRDAT0.BYTE3."]
    _5B = 5,
    #[doc = "6: Send opcode, CINSTRDAT0.BYTE0 -&amp;gt; CINSTRDAT1.BYTE4."]
    _6B = 6,
    #[doc = "7: Send opcode, CINSTRDAT0.BYTE0 -&amp;gt; CINSTRDAT1.BYTE5."]
    _7B = 7,
    #[doc = "8: Send opcode, CINSTRDAT0.BYTE0 -&amp;gt; CINSTRDAT1.BYTE6."]
    _8B = 8,
    #[doc = "9: Send opcode, CINSTRDAT0.BYTE0 -&amp;gt; CINSTRDAT1.BYTE7."]
    _9B = 9,
}
impl From<LENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: LENGTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LENGTH_A {
    type Ux = u8;
}
impl LENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LENGTH_A> {
        match self.bits {
            1 => Some(LENGTH_A::_1B),
            2 => Some(LENGTH_A::_2B),
            3 => Some(LENGTH_A::_3B),
            4 => Some(LENGTH_A::_4B),
            5 => Some(LENGTH_A::_5B),
            6 => Some(LENGTH_A::_6B),
            7 => Some(LENGTH_A::_7B),
            8 => Some(LENGTH_A::_8B),
            9 => Some(LENGTH_A::_9B),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1B`"]
    #[inline(always)]
    pub fn is_1b(&self) -> bool {
        *self == LENGTH_A::_1B
    }
    #[doc = "Checks if the value of the field is `_2B`"]
    #[inline(always)]
    pub fn is_2b(&self) -> bool {
        *self == LENGTH_A::_2B
    }
    #[doc = "Checks if the value of the field is `_3B`"]
    #[inline(always)]
    pub fn is_3b(&self) -> bool {
        *self == LENGTH_A::_3B
    }
    #[doc = "Checks if the value of the field is `_4B`"]
    #[inline(always)]
    pub fn is_4b(&self) -> bool {
        *self == LENGTH_A::_4B
    }
    #[doc = "Checks if the value of the field is `_5B`"]
    #[inline(always)]
    pub fn is_5b(&self) -> bool {
        *self == LENGTH_A::_5B
    }
    #[doc = "Checks if the value of the field is `_6B`"]
    #[inline(always)]
    pub fn is_6b(&self) -> bool {
        *self == LENGTH_A::_6B
    }
    #[doc = "Checks if the value of the field is `_7B`"]
    #[inline(always)]
    pub fn is_7b(&self) -> bool {
        *self == LENGTH_A::_7B
    }
    #[doc = "Checks if the value of the field is `_8B`"]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == LENGTH_A::_8B
    }
    #[doc = "Checks if the value of the field is `_9B`"]
    #[inline(always)]
    pub fn is_9b(&self) -> bool {
        *self == LENGTH_A::_9B
    }
}
#[doc = "Field `LENGTH` writer - Length of custom instruction in number of bytes."]
pub type LENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, CINSTRCONF_SPEC, 4, O, LENGTH_A>;
impl<'a, const O: u8> LENGTH_W<'a, O> {
    #[doc = "Send opcode only."]
    #[inline(always)]
    pub fn _1b(self) -> &'a mut W {
        self.variant(LENGTH_A::_1B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0."]
    #[inline(always)]
    pub fn _2b(self) -> &'a mut W {
        self.variant(LENGTH_A::_2B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&amp;gt; CINSTRDAT0.BYTE1."]
    #[inline(always)]
    pub fn _3b(self) -> &'a mut W {
        self.variant(LENGTH_A::_3B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&amp;gt; CINSTRDAT0.BYTE2."]
    #[inline(always)]
    pub fn _4b(self) -> &'a mut W {
        self.variant(LENGTH_A::_4B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&amp;gt; CINSTRDAT0.BYTE3."]
    #[inline(always)]
    pub fn _5b(self) -> &'a mut W {
        self.variant(LENGTH_A::_5B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&amp;gt; CINSTRDAT1.BYTE4."]
    #[inline(always)]
    pub fn _6b(self) -> &'a mut W {
        self.variant(LENGTH_A::_6B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&amp;gt; CINSTRDAT1.BYTE5."]
    #[inline(always)]
    pub fn _7b(self) -> &'a mut W {
        self.variant(LENGTH_A::_7B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&amp;gt; CINSTRDAT1.BYTE6."]
    #[inline(always)]
    pub fn _8b(self) -> &'a mut W {
        self.variant(LENGTH_A::_8B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&amp;gt; CINSTRDAT1.BYTE7."]
    #[inline(always)]
    pub fn _9b(self) -> &'a mut W {
        self.variant(LENGTH_A::_9B)
    }
}
#[doc = "Field `LIO2` reader - Level of the IO2 pin (if connected) during transmission of custom instruction."]
pub type LIO2_R = crate::BitReader;
#[doc = "Field `LIO2` writer - Level of the IO2 pin (if connected) during transmission of custom instruction."]
pub type LIO2_W<'a, const O: u8> = crate::BitWriter<'a, CINSTRCONF_SPEC, O>;
#[doc = "Field `LIO3` reader - Level of the IO3 pin (if connected) during transmission of custom instruction."]
pub type LIO3_R = crate::BitReader;
#[doc = "Field `LIO3` writer - Level of the IO3 pin (if connected) during transmission of custom instruction."]
pub type LIO3_W<'a, const O: u8> = crate::BitWriter<'a, CINSTRCONF_SPEC, O>;
#[doc = "Field `WIPWAIT` reader - Wait for write complete before sending command."]
pub type WIPWAIT_R = crate::BitReader<WIPWAIT_A>;
#[doc = "Wait for write complete before sending command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WIPWAIT_A {
    #[doc = "0: No wait."]
    DISABLE = 0,
    #[doc = "1: Wait."]
    ENABLE = 1,
}
impl From<WIPWAIT_A> for bool {
    #[inline(always)]
    fn from(variant: WIPWAIT_A) -> Self {
        variant as u8 != 0
    }
}
impl WIPWAIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIPWAIT_A {
        match self.bits {
            false => WIPWAIT_A::DISABLE,
            true => WIPWAIT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WIPWAIT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WIPWAIT_A::ENABLE
    }
}
#[doc = "Field `WIPWAIT` writer - Wait for write complete before sending command."]
pub type WIPWAIT_W<'a, const O: u8> = crate::BitWriter<'a, CINSTRCONF_SPEC, O, WIPWAIT_A>;
impl<'a, const O: u8> WIPWAIT_W<'a, O> {
    #[doc = "No wait."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WIPWAIT_A::DISABLE)
    }
    #[doc = "Wait."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WIPWAIT_A::ENABLE)
    }
}
#[doc = "Field `WREN` reader - Send WREN (write enable opcode 0x06) before instruction."]
pub type WREN_R = crate::BitReader<WREN_A>;
#[doc = "Send WREN (write enable opcode 0x06) before instruction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WREN_A {
    #[doc = "0: Do not send WREN."]
    DISABLE = 0,
    #[doc = "1: Send WREN."]
    ENABLE = 1,
}
impl From<WREN_A> for bool {
    #[inline(always)]
    fn from(variant: WREN_A) -> Self {
        variant as u8 != 0
    }
}
impl WREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WREN_A {
        match self.bits {
            false => WREN_A::DISABLE,
            true => WREN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WREN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WREN_A::ENABLE
    }
}
#[doc = "Field `WREN` writer - Send WREN (write enable opcode 0x06) before instruction."]
pub type WREN_W<'a, const O: u8> = crate::BitWriter<'a, CINSTRCONF_SPEC, O, WREN_A>;
impl<'a, const O: u8> WREN_W<'a, O> {
    #[doc = "Do not send WREN."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WREN_A::DISABLE)
    }
    #[doc = "Send WREN."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WREN_A::ENABLE)
    }
}
#[doc = "Field `LFEN` reader - Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field."]
pub type LFEN_R = crate::BitReader<LFEN_A>;
#[doc = "Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFEN_A {
    #[doc = "0: Long frame mode disabled"]
    DISABLE = 0,
    #[doc = "1: Long frame mode enabled"]
    ENABLE = 1,
}
impl From<LFEN_A> for bool {
    #[inline(always)]
    fn from(variant: LFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFEN_A {
        match self.bits {
            false => LFEN_A::DISABLE,
            true => LFEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LFEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LFEN_A::ENABLE
    }
}
#[doc = "Field `LFEN` writer - Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field."]
pub type LFEN_W<'a, const O: u8> = crate::BitWriter<'a, CINSTRCONF_SPEC, O, LFEN_A>;
impl<'a, const O: u8> LFEN_W<'a, O> {
    #[doc = "Long frame mode disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LFEN_A::DISABLE)
    }
    #[doc = "Long frame mode enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LFEN_A::ENABLE)
    }
}
#[doc = "Field `LFSTOP` reader - Stop (finalize) long frame transaction"]
pub type LFSTOP_R = crate::BitReader<LFSTOP_A>;
#[doc = "Stop (finalize) long frame transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFSTOP_A {
    #[doc = "1: Stop"]
    STOP = 1,
}
impl From<LFSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: LFSTOP_A) -> Self {
        variant as u8 != 0
    }
}
impl LFSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LFSTOP_A> {
        match self.bits {
            true => Some(LFSTOP_A::STOP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == LFSTOP_A::STOP
    }
}
#[doc = "Field `LFSTOP` writer - Stop (finalize) long frame transaction"]
pub type LFSTOP_W<'a, const O: u8> = crate::BitWriter<'a, CINSTRCONF_SPEC, O, LFSTOP_A>;
impl<'a, const O: u8> LFSTOP_W<'a, O> {
    #[doc = "Stop"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(LFSTOP_A::STOP)
    }
}
impl R {
    #[doc = "Bits 0:7 - Opcode of Custom instruction."]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Length of custom instruction in number of bytes."]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Level of the IO2 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    pub fn lio2(&self) -> LIO2_R {
        LIO2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the IO3 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    pub fn lio3(&self) -> LIO3_R {
        LIO3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wait for write complete before sending command."]
    #[inline(always)]
    pub fn wipwait(&self) -> WIPWAIT_R {
        WIPWAIT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field."]
    #[inline(always)]
    pub fn lfen(&self) -> LFEN_R {
        LFEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Stop (finalize) long frame transaction"]
    #[inline(always)]
    pub fn lfstop(&self) -> LFSTOP_R {
        LFSTOP_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Opcode of Custom instruction."]
    #[inline(always)]
    #[must_use]
    pub fn opcode(&mut self) -> OPCODE_W<0> {
        OPCODE_W::new(self)
    }
    #[doc = "Bits 8:11 - Length of custom instruction in number of bytes."]
    #[inline(always)]
    #[must_use]
    pub fn length(&mut self) -> LENGTH_W<8> {
        LENGTH_W::new(self)
    }
    #[doc = "Bit 12 - Level of the IO2 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    #[must_use]
    pub fn lio2(&mut self) -> LIO2_W<12> {
        LIO2_W::new(self)
    }
    #[doc = "Bit 13 - Level of the IO3 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    #[must_use]
    pub fn lio3(&mut self) -> LIO3_W<13> {
        LIO3_W::new(self)
    }
    #[doc = "Bit 14 - Wait for write complete before sending command."]
    #[inline(always)]
    #[must_use]
    pub fn wipwait(&mut self) -> WIPWAIT_W<14> {
        WIPWAIT_W::new(self)
    }
    #[doc = "Bit 15 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WREN_W<15> {
        WREN_W::new(self)
    }
    #[doc = "Bit 16 - Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field."]
    #[inline(always)]
    #[must_use]
    pub fn lfen(&mut self) -> LFEN_W<16> {
        LFEN_W::new(self)
    }
    #[doc = "Bit 17 - Stop (finalize) long frame transaction"]
    #[inline(always)]
    #[must_use]
    pub fn lfstop(&mut self) -> LFSTOP_W<17> {
        LFSTOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Custom instruction configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cinstrconf](index.html) module"]
pub struct CINSTRCONF_SPEC;
impl crate::RegisterSpec for CINSTRCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cinstrconf::R](R) reader structure"]
impl crate::Readable for CINSTRCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cinstrconf::W](W) writer structure"]
impl crate::Writable for CINSTRCONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CINSTRCONF to value 0x2000"]
impl crate::Resettable for CINSTRCONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
