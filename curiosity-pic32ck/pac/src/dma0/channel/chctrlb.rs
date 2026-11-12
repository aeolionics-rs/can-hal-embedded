#[doc = "Register `CHCTRLB` reader"]
pub type R = crate::R<ChctrlbSpec>;
#[doc = "Register `CHCTRLB` writer"]
pub type W = crate::W<ChctrlbSpec>;
#[doc = "Channel Write Address Sequence.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wasselect {
    #[doc = "0: Incrementing Address+1 with Transfers of Byte Operands"]
    ByteAddrIncr = 0,
    #[doc = "1: Incrementing Address+2 with Transfers of HalfWord Operands"]
    HalfWordAddrIncr = 1,
    #[doc = "2: Auto Increment Address and Transfer Size"]
    AutoAddrIncr = 2,
    #[doc = "3: Fixed Byte Address (Single Byte Address with Enable Based upon 2 LSBs)"]
    FixedByteAddrIncr = 3,
    #[doc = "4: Fixed Address of HalfWord Operand (Single HalfWord Aligned Address)"]
    FixedHalfWordAddrIncr = 4,
    #[doc = "5: Fixed Address Word Burst Transfer"]
    FixedWordAddrIncr = 5,
}
impl From<Wasselect> for u8 {
    #[inline(always)]
    fn from(variant: Wasselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wasselect {
    type Ux = u8;
}
impl crate::IsEnum for Wasselect {}
#[doc = "Field `WAS` reader - Channel Write Address Sequence."]
pub type WasR = crate::FieldReader<Wasselect>;
impl WasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wasselect> {
        match self.bits {
            0 => Some(Wasselect::ByteAddrIncr),
            1 => Some(Wasselect::HalfWordAddrIncr),
            2 => Some(Wasselect::AutoAddrIncr),
            3 => Some(Wasselect::FixedByteAddrIncr),
            4 => Some(Wasselect::FixedHalfWordAddrIncr),
            5 => Some(Wasselect::FixedWordAddrIncr),
            _ => None,
        }
    }
    #[doc = "Incrementing Address+1 with Transfers of Byte Operands"]
    #[inline(always)]
    pub fn is_byte_addr_incr(&self) -> bool {
        *self == Wasselect::ByteAddrIncr
    }
    #[doc = "Incrementing Address+2 with Transfers of HalfWord Operands"]
    #[inline(always)]
    pub fn is_half_word_addr_incr(&self) -> bool {
        *self == Wasselect::HalfWordAddrIncr
    }
    #[doc = "Auto Increment Address and Transfer Size"]
    #[inline(always)]
    pub fn is_auto_addr_incr(&self) -> bool {
        *self == Wasselect::AutoAddrIncr
    }
    #[doc = "Fixed Byte Address (Single Byte Address with Enable Based upon 2 LSBs)"]
    #[inline(always)]
    pub fn is_fixed_byte_addr_incr(&self) -> bool {
        *self == Wasselect::FixedByteAddrIncr
    }
    #[doc = "Fixed Address of HalfWord Operand (Single HalfWord Aligned Address)"]
    #[inline(always)]
    pub fn is_fixed_half_word_addr_incr(&self) -> bool {
        *self == Wasselect::FixedHalfWordAddrIncr
    }
    #[doc = "Fixed Address Word Burst Transfer"]
    #[inline(always)]
    pub fn is_fixed_word_addr_incr(&self) -> bool {
        *self == Wasselect::FixedWordAddrIncr
    }
}
#[doc = "Field `WAS` writer - Channel Write Address Sequence."]
pub type WasW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wasselect>;
impl<'a, REG> WasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Incrementing Address+1 with Transfers of Byte Operands"]
    #[inline(always)]
    pub fn byte_addr_incr(self) -> &'a mut crate::W<REG> {
        self.variant(Wasselect::ByteAddrIncr)
    }
    #[doc = "Incrementing Address+2 with Transfers of HalfWord Operands"]
    #[inline(always)]
    pub fn half_word_addr_incr(self) -> &'a mut crate::W<REG> {
        self.variant(Wasselect::HalfWordAddrIncr)
    }
    #[doc = "Auto Increment Address and Transfer Size"]
    #[inline(always)]
    pub fn auto_addr_incr(self) -> &'a mut crate::W<REG> {
        self.variant(Wasselect::AutoAddrIncr)
    }
    #[doc = "Fixed Byte Address (Single Byte Address with Enable Based upon 2 LSBs)"]
    #[inline(always)]
    pub fn fixed_byte_addr_incr(self) -> &'a mut crate::W<REG> {
        self.variant(Wasselect::FixedByteAddrIncr)
    }
    #[doc = "Fixed Address of HalfWord Operand (Single HalfWord Aligned Address)"]
    #[inline(always)]
    pub fn fixed_half_word_addr_incr(self) -> &'a mut crate::W<REG> {
        self.variant(Wasselect::FixedHalfWordAddrIncr)
    }
    #[doc = "Fixed Address Word Burst Transfer"]
    #[inline(always)]
    pub fn fixed_word_addr_incr(self) -> &'a mut crate::W<REG> {
        self.variant(Wasselect::FixedWordAddrIncr)
    }
}
#[doc = "Channel Read Address Sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rasselect {
    #[doc = "0: Incrementing Address+1 with Transfers of Byte Operands"]
    ByteAddrIncr = 0,
    #[doc = "1: Incrementing Address+2 with Transfers of HalfWord Operands"]
    HalfWordAddrIncr = 1,
    #[doc = "2: Auto Increment Address and Transfer Size"]
    AutoAddrIncr = 2,
    #[doc = "3: Fixed Byte Address (Single Byte Address with Enable Based upon 2 LSBs)"]
    FixedByteAddrIncr = 3,
    #[doc = "4: Fixed Address of HalfWord Operand (Single HalfWord Aligned Address)"]
    FixedHalfWordAddrIncr = 4,
    #[doc = "5: Fixed Address Word Burst Transfer"]
    FixedWordAddrIncr = 5,
}
impl From<Rasselect> for u8 {
    #[inline(always)]
    fn from(variant: Rasselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rasselect {
    type Ux = u8;
}
impl crate::IsEnum for Rasselect {}
#[doc = "Field `RAS` reader - Channel Read Address Sequence"]
pub type RasR = crate::FieldReader<Rasselect>;
impl RasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rasselect> {
        match self.bits {
            0 => Some(Rasselect::ByteAddrIncr),
            1 => Some(Rasselect::HalfWordAddrIncr),
            2 => Some(Rasselect::AutoAddrIncr),
            3 => Some(Rasselect::FixedByteAddrIncr),
            4 => Some(Rasselect::FixedHalfWordAddrIncr),
            5 => Some(Rasselect::FixedWordAddrIncr),
            _ => None,
        }
    }
    #[doc = "Incrementing Address+1 with Transfers of Byte Operands"]
    #[inline(always)]
    pub fn is_byte_addr_incr(&self) -> bool {
        *self == Rasselect::ByteAddrIncr
    }
    #[doc = "Incrementing Address+2 with Transfers of HalfWord Operands"]
    #[inline(always)]
    pub fn is_half_word_addr_incr(&self) -> bool {
        *self == Rasselect::HalfWordAddrIncr
    }
    #[doc = "Auto Increment Address and Transfer Size"]
    #[inline(always)]
    pub fn is_auto_addr_incr(&self) -> bool {
        *self == Rasselect::AutoAddrIncr
    }
    #[doc = "Fixed Byte Address (Single Byte Address with Enable Based upon 2 LSBs)"]
    #[inline(always)]
    pub fn is_fixed_byte_addr_incr(&self) -> bool {
        *self == Rasselect::FixedByteAddrIncr
    }
    #[doc = "Fixed Address of HalfWord Operand (Single HalfWord Aligned Address)"]
    #[inline(always)]
    pub fn is_fixed_half_word_addr_incr(&self) -> bool {
        *self == Rasselect::FixedHalfWordAddrIncr
    }
    #[doc = "Fixed Address Word Burst Transfer"]
    #[inline(always)]
    pub fn is_fixed_word_addr_incr(&self) -> bool {
        *self == Rasselect::FixedWordAddrIncr
    }
}
#[doc = "Field `RAS` writer - Channel Read Address Sequence"]
pub type RasW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rasselect>;
impl<'a, REG> RasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Incrementing Address+1 with Transfers of Byte Operands"]
    #[inline(always)]
    pub fn byte_addr_incr(self) -> &'a mut crate::W<REG> {
        self.variant(Rasselect::ByteAddrIncr)
    }
    #[doc = "Incrementing Address+2 with Transfers of HalfWord Operands"]
    #[inline(always)]
    pub fn half_word_addr_incr(self) -> &'a mut crate::W<REG> {
        self.variant(Rasselect::HalfWordAddrIncr)
    }
    #[doc = "Auto Increment Address and Transfer Size"]
    #[inline(always)]
    pub fn auto_addr_incr(self) -> &'a mut crate::W<REG> {
        self.variant(Rasselect::AutoAddrIncr)
    }
    #[doc = "Fixed Byte Address (Single Byte Address with Enable Based upon 2 LSBs)"]
    #[inline(always)]
    pub fn fixed_byte_addr_incr(self) -> &'a mut crate::W<REG> {
        self.variant(Rasselect::FixedByteAddrIncr)
    }
    #[doc = "Fixed Address of HalfWord Operand (Single HalfWord Aligned Address)"]
    #[inline(always)]
    pub fn fixed_half_word_addr_incr(self) -> &'a mut crate::W<REG> {
        self.variant(Rasselect::FixedHalfWordAddrIncr)
    }
    #[doc = "Fixed Address Word Burst Transfer"]
    #[inline(always)]
    pub fn fixed_word_addr_incr(self) -> &'a mut crate::W<REG> {
        self.variant(Rasselect::FixedWordAddrIncr)
    }
}
#[doc = "Channel Priority level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Priselect {
    #[doc = "0: Channel Priority is 1"]
    Pri1 = 0,
    #[doc = "1: Channel Priority is 2"]
    Pri2 = 1,
    #[doc = "2: Channel Priority is 3 (HIGHEST)"]
    Pri3 = 2,
}
impl From<Priselect> for u8 {
    #[inline(always)]
    fn from(variant: Priselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Priselect {
    type Ux = u8;
}
impl crate::IsEnum for Priselect {}
#[doc = "Field `PRI` reader - Channel Priority level."]
pub type PriR = crate::FieldReader<Priselect>;
impl PriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Priselect> {
        match self.bits {
            0 => Some(Priselect::Pri1),
            1 => Some(Priselect::Pri2),
            2 => Some(Priselect::Pri3),
            _ => None,
        }
    }
    #[doc = "Channel Priority is 1"]
    #[inline(always)]
    pub fn is_pri_1(&self) -> bool {
        *self == Priselect::Pri1
    }
    #[doc = "Channel Priority is 2"]
    #[inline(always)]
    pub fn is_pri_2(&self) -> bool {
        *self == Priselect::Pri2
    }
    #[doc = "Channel Priority is 3 (HIGHEST)"]
    #[inline(always)]
    pub fn is_pri_3(&self) -> bool {
        *self == Priselect::Pri3
    }
}
#[doc = "Field `PRI` writer - Channel Priority level."]
pub type PriW<'a, REG> = crate::FieldWriter<'a, REG, 2, Priselect>;
impl<'a, REG> PriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel Priority is 1"]
    #[inline(always)]
    pub fn pri_1(self) -> &'a mut crate::W<REG> {
        self.variant(Priselect::Pri1)
    }
    #[doc = "Channel Priority is 2"]
    #[inline(always)]
    pub fn pri_2(self) -> &'a mut crate::W<REG> {
        self.variant(Priselect::Pri2)
    }
    #[doc = "Channel Priority is 3 (HIGHEST)"]
    #[inline(always)]
    pub fn pri_3(self) -> &'a mut crate::W<REG> {
        self.variant(Priselect::Pri3)
    }
}
#[doc = "Field `WBOEN` reader - Write Byte Order Enable"]
pub type WboenR = crate::BitReader;
#[doc = "Field `WBOEN` writer - Write Byte Order Enable"]
pub type WboenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Byte Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bytordselect {
    #[doc = "0: Bytes are processed AS IS"]
    BytordNone = 0,
    #[doc = "1: Bytes are swapped as: BYTE3 to BYTE0, BYTE2 to BYTE1, BYTE1 to BYTE2, and BYTE0 to BYTE3"]
    BytordWordSwizzle = 1,
    #[doc = "2: Bytes are swapped as: BYTE3 to BYTE1, BYTE2 to BYTE0, BYTE1 to BYTE3, and BYTE0 to BYTE2"]
    BytordHalfWordSwizzle = 2,
    #[doc = "3: Bytes are swapped as: BYTE3 to BYTE2, BYTE2 to BYTE3, BYTE1 to BYTE0, and BYTE0 to BYTE1"]
    BytordByteSwizzle = 3,
}
impl From<Bytordselect> for u8 {
    #[inline(always)]
    fn from(variant: Bytordselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bytordselect {
    type Ux = u8;
}
impl crate::IsEnum for Bytordselect {}
#[doc = "Field `BYTORD` reader - Byte Order"]
pub type BytordR = crate::FieldReader<Bytordselect>;
impl BytordR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bytordselect {
        match self.bits {
            0 => Bytordselect::BytordNone,
            1 => Bytordselect::BytordWordSwizzle,
            2 => Bytordselect::BytordHalfWordSwizzle,
            3 => Bytordselect::BytordByteSwizzle,
            _ => unreachable!(),
        }
    }
    #[doc = "Bytes are processed AS IS"]
    #[inline(always)]
    pub fn is_bytord_none(&self) -> bool {
        *self == Bytordselect::BytordNone
    }
    #[doc = "Bytes are swapped as: BYTE3 to BYTE0, BYTE2 to BYTE1, BYTE1 to BYTE2, and BYTE0 to BYTE3"]
    #[inline(always)]
    pub fn is_bytord_word_swizzle(&self) -> bool {
        *self == Bytordselect::BytordWordSwizzle
    }
    #[doc = "Bytes are swapped as: BYTE3 to BYTE1, BYTE2 to BYTE0, BYTE1 to BYTE3, and BYTE0 to BYTE2"]
    #[inline(always)]
    pub fn is_bytord_half_word_swizzle(&self) -> bool {
        *self == Bytordselect::BytordHalfWordSwizzle
    }
    #[doc = "Bytes are swapped as: BYTE3 to BYTE2, BYTE2 to BYTE3, BYTE1 to BYTE0, and BYTE0 to BYTE1"]
    #[inline(always)]
    pub fn is_bytord_byte_swizzle(&self) -> bool {
        *self == Bytordselect::BytordByteSwizzle
    }
}
#[doc = "Field `BYTORD` writer - Byte Order"]
pub type BytordW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bytordselect, crate::Safe>;
impl<'a, REG> BytordW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bytes are processed AS IS"]
    #[inline(always)]
    pub fn bytord_none(self) -> &'a mut crate::W<REG> {
        self.variant(Bytordselect::BytordNone)
    }
    #[doc = "Bytes are swapped as: BYTE3 to BYTE0, BYTE2 to BYTE1, BYTE1 to BYTE2, and BYTE0 to BYTE3"]
    #[inline(always)]
    pub fn bytord_word_swizzle(self) -> &'a mut crate::W<REG> {
        self.variant(Bytordselect::BytordWordSwizzle)
    }
    #[doc = "Bytes are swapped as: BYTE3 to BYTE1, BYTE2 to BYTE0, BYTE1 to BYTE3, and BYTE0 to BYTE2"]
    #[inline(always)]
    pub fn bytord_half_word_swizzle(self) -> &'a mut crate::W<REG> {
        self.variant(Bytordselect::BytordHalfWordSwizzle)
    }
    #[doc = "Bytes are swapped as: BYTE3 to BYTE2, BYTE2 to BYTE3, BYTE1 to BYTE0, and BYTE0 to BYTE1"]
    #[inline(always)]
    pub fn bytord_byte_swizzle(self) -> &'a mut crate::W<REG> {
        self.variant(Bytordselect::BytordByteSwizzle)
    }
}
#[doc = "Field `TRIG` reader - Trigger that can Start a Channel Transfer"]
pub type TrigR = crate::FieldReader;
#[doc = "Field `TRIG` writer - Trigger that can Start a Channel Transfer"]
pub type TrigW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PIGNEN` reader - Enable Pattern Ignore Byte"]
pub type PignenR = crate::BitReader;
#[doc = "Field `PIGNEN` writer - Enable Pattern Ignore Byte"]
pub type PignenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATLEN` reader - Pattern Match Length"]
pub type PatlenR = crate::BitReader;
#[doc = "Field `PATLEN` writer - Pattern Match Length"]
pub type PatlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATEN` reader - Channel Pattern Match Abort Enable."]
pub type PatenR = crate::BitReader;
#[doc = "Field `PATEN` writer - Channel Pattern Match Abort Enable."]
pub type PatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CASTEN` reader - Cell Auto Start Enable of Ensuing Transfers for this channel"]
pub type CastenR = crate::BitReader;
#[doc = "Field `CASTEN` writer - Cell Auto Start Enable of Ensuing Transfers for this channel"]
pub type CastenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC Enable bit"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC Enable bit"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Channel Write Address Sequence."]
    #[inline(always)]
    pub fn was(&self) -> WasR {
        WasR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Channel Read Address Sequence"]
    #[inline(always)]
    pub fn ras(&self) -> RasR {
        RasR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Channel Priority level."]
    #[inline(always)]
    pub fn pri(&self) -> PriR {
        PriR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 13 - Write Byte Order Enable"]
    #[inline(always)]
    pub fn wboen(&self) -> WboenR {
        WboenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Byte Order"]
    #[inline(always)]
    pub fn bytord(&self) -> BytordR {
        BytordR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Trigger that can Start a Channel Transfer"]
    #[inline(always)]
    pub fn trig(&self) -> TrigR {
        TrigR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enable Pattern Ignore Byte"]
    #[inline(always)]
    pub fn pignen(&self) -> PignenR {
        PignenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Pattern Match Length"]
    #[inline(always)]
    pub fn patlen(&self) -> PatlenR {
        PatlenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel Pattern Match Abort Enable."]
    #[inline(always)]
    pub fn paten(&self) -> PatenR {
        PatenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - Cell Auto Start Enable of Ensuing Transfers for this channel"]
    #[inline(always)]
    pub fn casten(&self) -> CastenR {
        CastenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - CRC Enable bit"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel Write Address Sequence."]
    #[inline(always)]
    pub fn was(&mut self) -> WasW<'_, ChctrlbSpec> {
        WasW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Channel Read Address Sequence"]
    #[inline(always)]
    pub fn ras(&mut self) -> RasW<'_, ChctrlbSpec> {
        RasW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Channel Priority level."]
    #[inline(always)]
    pub fn pri(&mut self) -> PriW<'_, ChctrlbSpec> {
        PriW::new(self, 8)
    }
    #[doc = "Bit 13 - Write Byte Order Enable"]
    #[inline(always)]
    pub fn wboen(&mut self) -> WboenW<'_, ChctrlbSpec> {
        WboenW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Byte Order"]
    #[inline(always)]
    pub fn bytord(&mut self) -> BytordW<'_, ChctrlbSpec> {
        BytordW::new(self, 14)
    }
    #[doc = "Bits 16:23 - Trigger that can Start a Channel Transfer"]
    #[inline(always)]
    pub fn trig(&mut self) -> TrigW<'_, ChctrlbSpec> {
        TrigW::new(self, 16)
    }
    #[doc = "Bit 24 - Enable Pattern Ignore Byte"]
    #[inline(always)]
    pub fn pignen(&mut self) -> PignenW<'_, ChctrlbSpec> {
        PignenW::new(self, 24)
    }
    #[doc = "Bit 25 - Pattern Match Length"]
    #[inline(always)]
    pub fn patlen(&mut self) -> PatlenW<'_, ChctrlbSpec> {
        PatlenW::new(self, 25)
    }
    #[doc = "Bit 26 - Channel Pattern Match Abort Enable."]
    #[inline(always)]
    pub fn paten(&mut self) -> PatenW<'_, ChctrlbSpec> {
        PatenW::new(self, 26)
    }
    #[doc = "Bit 29 - Cell Auto Start Enable of Ensuing Transfers for this channel"]
    #[inline(always)]
    pub fn casten(&mut self) -> CastenW<'_, ChctrlbSpec> {
        CastenW::new(self, 29)
    }
    #[doc = "Bit 31 - CRC Enable bit"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, ChctrlbSpec> {
        CrcenW::new(self, 31)
    }
}
#[doc = "CHANNEL CONTROL REGISTER B\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChctrlbSpec;
impl crate::RegisterSpec for ChctrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctrlb::R`](R) reader structure"]
impl crate::Readable for ChctrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`chctrlb::W`](W) writer structure"]
impl crate::Writable for ChctrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTRLB to value 0"]
impl crate::Resettable for ChctrlbSpec {}
