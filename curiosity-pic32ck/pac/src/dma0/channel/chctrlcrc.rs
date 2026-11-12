#[doc = "Register `CHCTRLCRC` reader"]
pub type R = crate::R<ChctrlcrcSpec>;
#[doc = "Register `CHCTRLCRC` writer"]
pub type W = crate::W<ChctrlcrcSpec>;
#[doc = "CRC/Checksum Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Crcmdselect {
    #[doc = "0: Normal (0x8005) CRC-16/CRC-16-IBM/CRC-16-ANSI"]
    Crc16 = 0,
    #[doc = "1: Normal (0x1021) CRC-16-CCITT"]
    Crc16Ccitt = 1,
    #[doc = "2: CRC-16 based on polynomial provided in CRCPOLYA (POLYA\\[15:0\\]) register"]
    _16BitCrcpolya = 2,
    #[doc = "3: CRC-16 based on polynomial provided in CRCPOLYB (POLYB\\[15:0\\]) register"]
    _16BitCrcpolyb = 3,
    #[doc = "4: Normal (0x04C11DB7) CRC-32"]
    Crc32 = 4,
    #[doc = "5: CRC-32 based on polynomial provided in CRCPOLYA register"]
    _32BitCrcpolya = 5,
    #[doc = "6: CRC-32 based on polynomial provided in CRCPOLYB register"]
    _32BitCrcpolyb = 6,
    #[doc = "7: Calculate IP header checksum"]
    IpHdrChecksum = 7,
}
impl From<Crcmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Crcmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Crcmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Crcmdselect {}
#[doc = "Field `CRCMD` reader - CRC/Checksum Mode"]
pub type CrcmdR = crate::FieldReader<Crcmdselect>;
impl CrcmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcmdselect {
        match self.bits {
            0 => Crcmdselect::Crc16,
            1 => Crcmdselect::Crc16Ccitt,
            2 => Crcmdselect::_16BitCrcpolya,
            3 => Crcmdselect::_16BitCrcpolyb,
            4 => Crcmdselect::Crc32,
            5 => Crcmdselect::_32BitCrcpolya,
            6 => Crcmdselect::_32BitCrcpolyb,
            7 => Crcmdselect::IpHdrChecksum,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal (0x8005) CRC-16/CRC-16-IBM/CRC-16-ANSI"]
    #[inline(always)]
    pub fn is_crc_16(&self) -> bool {
        *self == Crcmdselect::Crc16
    }
    #[doc = "Normal (0x1021) CRC-16-CCITT"]
    #[inline(always)]
    pub fn is_crc_16_ccitt(&self) -> bool {
        *self == Crcmdselect::Crc16Ccitt
    }
    #[doc = "CRC-16 based on polynomial provided in CRCPOLYA (POLYA\\[15:0\\]) register"]
    #[inline(always)]
    pub fn is_16_bit_crcpolya(&self) -> bool {
        *self == Crcmdselect::_16BitCrcpolya
    }
    #[doc = "CRC-16 based on polynomial provided in CRCPOLYB (POLYB\\[15:0\\]) register"]
    #[inline(always)]
    pub fn is_16_bit_crcpolyb(&self) -> bool {
        *self == Crcmdselect::_16BitCrcpolyb
    }
    #[doc = "Normal (0x04C11DB7) CRC-32"]
    #[inline(always)]
    pub fn is_crc_32(&self) -> bool {
        *self == Crcmdselect::Crc32
    }
    #[doc = "CRC-32 based on polynomial provided in CRCPOLYA register"]
    #[inline(always)]
    pub fn is_32_bit_crcpolya(&self) -> bool {
        *self == Crcmdselect::_32BitCrcpolya
    }
    #[doc = "CRC-32 based on polynomial provided in CRCPOLYB register"]
    #[inline(always)]
    pub fn is_32_bit_crcpolyb(&self) -> bool {
        *self == Crcmdselect::_32BitCrcpolyb
    }
    #[doc = "Calculate IP header checksum"]
    #[inline(always)]
    pub fn is_ip_hdr_checksum(&self) -> bool {
        *self == Crcmdselect::IpHdrChecksum
    }
}
#[doc = "Field `CRCMD` writer - CRC/Checksum Mode"]
pub type CrcmdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Crcmdselect, crate::Safe>;
impl<'a, REG> CrcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal (0x8005) CRC-16/CRC-16-IBM/CRC-16-ANSI"]
    #[inline(always)]
    pub fn crc_16(self) -> &'a mut crate::W<REG> {
        self.variant(Crcmdselect::Crc16)
    }
    #[doc = "Normal (0x1021) CRC-16-CCITT"]
    #[inline(always)]
    pub fn crc_16_ccitt(self) -> &'a mut crate::W<REG> {
        self.variant(Crcmdselect::Crc16Ccitt)
    }
    #[doc = "CRC-16 based on polynomial provided in CRCPOLYA (POLYA\\[15:0\\]) register"]
    #[inline(always)]
    pub fn _16_bit_crcpolya(self) -> &'a mut crate::W<REG> {
        self.variant(Crcmdselect::_16BitCrcpolya)
    }
    #[doc = "CRC-16 based on polynomial provided in CRCPOLYB (POLYB\\[15:0\\]) register"]
    #[inline(always)]
    pub fn _16_bit_crcpolyb(self) -> &'a mut crate::W<REG> {
        self.variant(Crcmdselect::_16BitCrcpolyb)
    }
    #[doc = "Normal (0x04C11DB7) CRC-32"]
    #[inline(always)]
    pub fn crc_32(self) -> &'a mut crate::W<REG> {
        self.variant(Crcmdselect::Crc32)
    }
    #[doc = "CRC-32 based on polynomial provided in CRCPOLYA register"]
    #[inline(always)]
    pub fn _32_bit_crcpolya(self) -> &'a mut crate::W<REG> {
        self.variant(Crcmdselect::_32BitCrcpolya)
    }
    #[doc = "CRC-32 based on polynomial provided in CRCPOLYB register"]
    #[inline(always)]
    pub fn _32_bit_crcpolyb(self) -> &'a mut crate::W<REG> {
        self.variant(Crcmdselect::_32BitCrcpolyb)
    }
    #[doc = "Calculate IP header checksum"]
    #[inline(always)]
    pub fn ip_hdr_checksum(self) -> &'a mut crate::W<REG> {
        self.variant(Crcmdselect::IpHdrChecksum)
    }
}
#[doc = "Field `CRCAPP` reader - CRC Append Mode"]
pub type CrcappR = crate::BitReader;
#[doc = "Field `CRCAPP` writer - CRC Append Mode"]
pub type CrcappW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCXOR` reader - CRC XOR Mode"]
pub type CrcxorR = crate::BitReader;
#[doc = "Field `CRCXOR` writer - CRC XOR Mode"]
pub type CrcxorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCROUT` reader - CRC Reflected Output Mode"]
pub type CrcroutR = crate::BitReader;
#[doc = "Field `CRCROUT` writer - CRC Reflected Output Mode"]
pub type CrcroutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRIN` reader - CRC Reflect Input Selection"]
pub type CrcrinR = crate::BitReader;
#[doc = "Field `CRCRIN` writer - CRC Reflect Input Selection"]
pub type CrcrinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - CRC/Checksum Mode"]
    #[inline(always)]
    pub fn crcmd(&self) -> CrcmdR {
        CrcmdR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - CRC Append Mode"]
    #[inline(always)]
    pub fn crcapp(&self) -> CrcappR {
        CrcappR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - CRC XOR Mode"]
    #[inline(always)]
    pub fn crcxor(&self) -> CrcxorR {
        CrcxorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC Reflected Output Mode"]
    #[inline(always)]
    pub fn crcrout(&self) -> CrcroutR {
        CrcroutR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC Reflect Input Selection"]
    #[inline(always)]
    pub fn crcrin(&self) -> CrcrinR {
        CrcrinR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CRC/Checksum Mode"]
    #[inline(always)]
    pub fn crcmd(&mut self) -> CrcmdW<'_, ChctrlcrcSpec> {
        CrcmdW::new(self, 0)
    }
    #[doc = "Bit 3 - CRC Append Mode"]
    #[inline(always)]
    pub fn crcapp(&mut self) -> CrcappW<'_, ChctrlcrcSpec> {
        CrcappW::new(self, 3)
    }
    #[doc = "Bit 5 - CRC XOR Mode"]
    #[inline(always)]
    pub fn crcxor(&mut self) -> CrcxorW<'_, ChctrlcrcSpec> {
        CrcxorW::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Reflected Output Mode"]
    #[inline(always)]
    pub fn crcrout(&mut self) -> CrcroutW<'_, ChctrlcrcSpec> {
        CrcroutW::new(self, 6)
    }
    #[doc = "Bit 7 - CRC Reflect Input Selection"]
    #[inline(always)]
    pub fn crcrin(&mut self) -> CrcrinW<'_, ChctrlcrcSpec> {
        CrcrinW::new(self, 7)
    }
}
#[doc = "CHANNEL CONTROL CRC\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrlcrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrlcrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChctrlcrcSpec;
impl crate::RegisterSpec for ChctrlcrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctrlcrc::R`](R) reader structure"]
impl crate::Readable for ChctrlcrcSpec {}
#[doc = "`write(|w| ..)` method takes [`chctrlcrc::W`](W) writer structure"]
impl crate::Writable for ChctrlcrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTRLCRC to value 0"]
impl crate::Resettable for ChctrlcrcSpec {}
