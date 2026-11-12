#[doc = "Register `SMC_MODE` reader"]
pub type R = crate::R<SmcModeSpec>;
#[doc = "Register `SMC_MODE` writer"]
pub type W = crate::W<SmcModeSpec>;
#[doc = "Field `READ_MODE` reader - Read Mode"]
pub type ReadModeR = crate::BitReader;
#[doc = "Field `READ_MODE` writer - Read Mode"]
pub type ReadModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MODE` reader - Write Mode"]
pub type WriteModeR = crate::BitReader;
#[doc = "Field `WRITE_MODE` writer - Write Mode"]
pub type WriteModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "NWAIT Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ExnwModeselect {
    #[doc = "0: Disabled-The NWAIT input signal is ignored on the corresponding chip select."]
    Disabled = 0,
    #[doc = "2: Frozen Mode-If asserted, the NWAIT signal freezes the current read or write cycle. After deassertion, the read/write cycle is resumed from the point where it was stopped."]
    Frozen = 2,
    #[doc = "3: Ready Mode-The NWAIT signal indicates the availability of the external device at the end of the pulse of the controlling read or write signal, to complete the access. If high, the access normally completes. If low, the access is extended until NWAIT returns high."]
    Ready = 3,
}
impl From<ExnwModeselect> for u8 {
    #[inline(always)]
    fn from(variant: ExnwModeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ExnwModeselect {
    type Ux = u8;
}
impl crate::IsEnum for ExnwModeselect {}
#[doc = "Field `EXNW_MODE` reader - NWAIT Mode"]
pub type ExnwModeR = crate::FieldReader<ExnwModeselect>;
impl ExnwModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ExnwModeselect> {
        match self.bits {
            0 => Some(ExnwModeselect::Disabled),
            2 => Some(ExnwModeselect::Frozen),
            3 => Some(ExnwModeselect::Ready),
            _ => None,
        }
    }
    #[doc = "Disabled-The NWAIT input signal is ignored on the corresponding chip select."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ExnwModeselect::Disabled
    }
    #[doc = "Frozen Mode-If asserted, the NWAIT signal freezes the current read or write cycle. After deassertion, the read/write cycle is resumed from the point where it was stopped."]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == ExnwModeselect::Frozen
    }
    #[doc = "Ready Mode-The NWAIT signal indicates the availability of the external device at the end of the pulse of the controlling read or write signal, to complete the access. If high, the access normally completes. If low, the access is extended until NWAIT returns high."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ExnwModeselect::Ready
    }
}
#[doc = "Field `EXNW_MODE` writer - NWAIT Mode"]
pub type ExnwModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, ExnwModeselect>;
impl<'a, REG> ExnwModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled-The NWAIT input signal is ignored on the corresponding chip select."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ExnwModeselect::Disabled)
    }
    #[doc = "Frozen Mode-If asserted, the NWAIT signal freezes the current read or write cycle. After deassertion, the read/write cycle is resumed from the point where it was stopped."]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(ExnwModeselect::Frozen)
    }
    #[doc = "Ready Mode-The NWAIT signal indicates the availability of the external device at the end of the pulse of the controlling read or write signal, to complete the access. If high, the access normally completes. If low, the access is extended until NWAIT returns high."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(ExnwModeselect::Ready)
    }
}
#[doc = "Byte Access Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Batselect {
    #[doc = "0: Byte select access type:- Write operation is controlled using NCS, NWE, NBS0, NBS1.- Read operation is controlled using NCS, NRD, NBS0, NBS1."]
    ByteSelect = 0,
    #[doc = "1: Byte write access type:- Write operation is controlled using NCS, NWR0, NWR1.- Read operation is controlled using NCS and NRD."]
    ByteWrite = 1,
}
impl From<Batselect> for bool {
    #[inline(always)]
    fn from(variant: Batselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BAT` reader - Byte Access Type"]
pub type BatR = crate::BitReader<Batselect>;
impl BatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Batselect {
        match self.bits {
            false => Batselect::ByteSelect,
            true => Batselect::ByteWrite,
        }
    }
    #[doc = "Byte select access type:- Write operation is controlled using NCS, NWE, NBS0, NBS1.- Read operation is controlled using NCS, NRD, NBS0, NBS1."]
    #[inline(always)]
    pub fn is_byte_select(&self) -> bool {
        *self == Batselect::ByteSelect
    }
    #[doc = "Byte write access type:- Write operation is controlled using NCS, NWR0, NWR1.- Read operation is controlled using NCS and NRD."]
    #[inline(always)]
    pub fn is_byte_write(&self) -> bool {
        *self == Batselect::ByteWrite
    }
}
#[doc = "Field `BAT` writer - Byte Access Type"]
pub type BatW<'a, REG> = crate::BitWriter<'a, REG, Batselect>;
impl<'a, REG> BatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Byte select access type:- Write operation is controlled using NCS, NWE, NBS0, NBS1.- Read operation is controlled using NCS, NRD, NBS0, NBS1."]
    #[inline(always)]
    pub fn byte_select(self) -> &'a mut crate::W<REG> {
        self.variant(Batselect::ByteSelect)
    }
    #[doc = "Byte write access type:- Write operation is controlled using NCS, NWR0, NWR1.- Read operation is controlled using NCS and NRD."]
    #[inline(always)]
    pub fn byte_write(self) -> &'a mut crate::W<REG> {
        self.variant(Batselect::ByteWrite)
    }
}
#[doc = "Data Bus Width\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbwselect {
    #[doc = "0: 8-bit Data Bus"]
    _8Bit = 0,
    #[doc = "1: 16-bit Data Bus"]
    _16Bit = 1,
}
impl From<Dbwselect> for bool {
    #[inline(always)]
    fn from(variant: Dbwselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBW` reader - Data Bus Width"]
pub type DbwR = crate::BitReader<Dbwselect>;
impl DbwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbwselect {
        match self.bits {
            false => Dbwselect::_8Bit,
            true => Dbwselect::_16Bit,
        }
    }
    #[doc = "8-bit Data Bus"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Dbwselect::_8Bit
    }
    #[doc = "16-bit Data Bus"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == Dbwselect::_16Bit
    }
}
#[doc = "Field `DBW` writer - Data Bus Width"]
pub type DbwW<'a, REG> = crate::BitWriter<'a, REG, Dbwselect>;
impl<'a, REG> DbwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit Data Bus"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dbwselect::_8Bit)
    }
    #[doc = "16-bit Data Bus"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dbwselect::_16Bit)
    }
}
#[doc = "Field `TDF_CYCLES` reader - Data Float Time"]
pub type TdfCyclesR = crate::FieldReader;
#[doc = "Field `TDF_CYCLES` writer - Data Float Time"]
pub type TdfCyclesW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDF_MODE` reader - TDF Optimization"]
pub type TdfModeR = crate::BitReader;
#[doc = "Field `TDF_MODE` writer - TDF Optimization"]
pub type TdfModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMEN` reader - Page Mode Enabled"]
pub type PmenR = crate::BitReader;
#[doc = "Field `PMEN` writer - Page Mode Enabled"]
pub type PmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Page Size\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psselect {
    #[doc = "0: 4-byte page"]
    _4Byte = 0,
    #[doc = "1: 8-byte page"]
    _8Byte = 1,
    #[doc = "2: 16-byte page"]
    _16Byte = 2,
    #[doc = "3: 32-byte page"]
    _32Byte = 3,
}
impl From<Psselect> for u8 {
    #[inline(always)]
    fn from(variant: Psselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Psselect {
    type Ux = u8;
}
impl crate::IsEnum for Psselect {}
#[doc = "Field `PS` reader - Page Size"]
pub type PsR = crate::FieldReader<Psselect>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psselect {
        match self.bits {
            0 => Psselect::_4Byte,
            1 => Psselect::_8Byte,
            2 => Psselect::_16Byte,
            3 => Psselect::_32Byte,
            _ => unreachable!(),
        }
    }
    #[doc = "4-byte page"]
    #[inline(always)]
    pub fn is_4_byte(&self) -> bool {
        *self == Psselect::_4Byte
    }
    #[doc = "8-byte page"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == Psselect::_8Byte
    }
    #[doc = "16-byte page"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == Psselect::_16Byte
    }
    #[doc = "32-byte page"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == Psselect::_32Byte
    }
}
#[doc = "Field `PS` writer - Page Size"]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Psselect, crate::Safe>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4-byte page"]
    #[inline(always)]
    pub fn _4_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psselect::_4Byte)
    }
    #[doc = "8-byte page"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psselect::_8Byte)
    }
    #[doc = "16-byte page"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psselect::_16Byte)
    }
    #[doc = "32-byte page"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Psselect::_32Byte)
    }
}
impl R {
    #[doc = "Bit 0 - Read Mode"]
    #[inline(always)]
    pub fn read_mode(&self) -> ReadModeR {
        ReadModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Mode"]
    #[inline(always)]
    pub fn write_mode(&self) -> WriteModeR {
        WriteModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&self) -> ExnwModeR {
        ExnwModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    pub fn bat(&self) -> BatR {
        BatR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DbwR {
        DbwR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&self) -> TdfCyclesR {
        TdfCyclesR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&self) -> TdfModeR {
        TdfModeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    pub fn pmen(&self) -> PmenR {
        PmenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Read Mode"]
    #[inline(always)]
    pub fn read_mode(&mut self) -> ReadModeW<'_, SmcModeSpec> {
        ReadModeW::new(self, 0)
    }
    #[doc = "Bit 1 - Write Mode"]
    #[inline(always)]
    pub fn write_mode(&mut self) -> WriteModeW<'_, SmcModeSpec> {
        WriteModeW::new(self, 1)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&mut self) -> ExnwModeW<'_, SmcModeSpec> {
        ExnwModeW::new(self, 4)
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    pub fn bat(&mut self) -> BatW<'_, SmcModeSpec> {
        BatW::new(self, 8)
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&mut self) -> DbwW<'_, SmcModeSpec> {
        DbwW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&mut self) -> TdfCyclesW<'_, SmcModeSpec> {
        TdfCyclesW::new(self, 16)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&mut self) -> TdfModeW<'_, SmcModeSpec> {
        TdfModeW::new(self, 20)
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    pub fn pmen(&mut self) -> PmenW<'_, SmcModeSpec> {
        PmenW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, SmcModeSpec> {
        PsW::new(self, 28)
    }
}
#[doc = "SMC Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smc_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcModeSpec;
impl crate::RegisterSpec for SmcModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smc_mode::R`](R) reader structure"]
impl crate::Readable for SmcModeSpec {}
#[doc = "`write(|w| ..)` method takes [`smc_mode::W`](W) writer structure"]
impl crate::Writable for SmcModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMC_MODE to value 0x1000_1003"]
impl crate::Resettable for SmcModeSpec {
    const RESET_VALUE: u32 = 0x1000_1003;
}
