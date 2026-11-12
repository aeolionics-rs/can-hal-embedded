#[doc = "Register `SPICTRL` reader"]
pub type R = crate::R<SpictrlSpec>;
#[doc = "Register `SPICTRL` writer"]
pub type W = crate::W<SpictrlSpec>;
#[doc = "Serial Word Length bits for AUDEN=0, not used when AUDEN=1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modeselect {
    #[doc = "0: 8 Bits Mode"]
    Mode8 = 0,
    #[doc = "1: 16 Bits Mode"]
    Mode16 = 1,
    #[doc = "2: 32 Bits Mode"]
    Mode32 = 2,
}
impl From<Modeselect> for u8 {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modeselect {
    type Ux = u8;
}
impl crate::IsEnum for Modeselect {}
#[doc = "Field `MODE` reader - Serial Word Length bits for AUDEN=0, not used when AUDEN=1"]
pub type ModeR = crate::FieldReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modeselect> {
        match self.bits {
            0 => Some(Modeselect::Mode8),
            1 => Some(Modeselect::Mode16),
            2 => Some(Modeselect::Mode32),
            _ => None,
        }
    }
    #[doc = "8 Bits Mode"]
    #[inline(always)]
    pub fn is_mode8(&self) -> bool {
        *self == Modeselect::Mode8
    }
    #[doc = "16 Bits Mode"]
    #[inline(always)]
    pub fn is_mode16(&self) -> bool {
        *self == Modeselect::Mode16
    }
    #[doc = "32 Bits Mode"]
    #[inline(always)]
    pub fn is_mode32(&self) -> bool {
        *self == Modeselect::Mode32
    }
}
#[doc = "Field `MODE` writer - Serial Word Length bits for AUDEN=0, not used when AUDEN=1"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 Bits Mode"]
    #[inline(always)]
    pub fn mode8(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Mode8)
    }
    #[doc = "16 Bits Mode"]
    #[inline(always)]
    pub fn mode16(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Mode16)
    }
    #[doc = "32 Bits Mode"]
    #[inline(always)]
    pub fn mode32(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Mode32)
    }
}
#[doc = "Field `DISSDI` reader - Disable SDI bit"]
pub type DissdiR = crate::BitReader;
#[doc = "Field `DISSDI` writer - Disable SDI bit"]
pub type DissdiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISSDO` reader - Disable SDO bit"]
pub type DissdoR = crate::BitReader;
#[doc = "Field `DISSDO` writer - Disable SDO bit"]
pub type DissdoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTEN` reader - Master Mode Enable bit"]
pub type MstenR = crate::BitReader;
#[doc = "Field `MSTEN` writer - Master Mode Enable bit"]
pub type MstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEN` reader - Host/Client Mode Select Enable bit"]
pub type CsenR = crate::BitReader;
#[doc = "Field `CSEN` writer - Host/Client Mode Select Enable bit"]
pub type CsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPISGNEXT` reader - Sign Extend Read Data from the RX FIFO"]
pub type SpisgnextR = crate::BitReader;
#[doc = "Field `SPISGNEXT` writer - Sign Extend Read Data from the RX FIFO"]
pub type SpisgnextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMP` reader - SPI Data Input Sample Phase bit"]
pub type SmpR = crate::BitReader;
#[doc = "Field `SMP` writer - SPI Data Input Sample Phase bit"]
pub type SmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Serial Word Length bits for AUDEN=0, not used when AUDEN=1"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Disable SDI bit"]
    #[inline(always)]
    pub fn dissdi(&self) -> DissdiR {
        DissdiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disable SDO bit"]
    #[inline(always)]
    pub fn dissdo(&self) -> DissdoR {
        DissdoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master Mode Enable bit"]
    #[inline(always)]
    pub fn msten(&self) -> MstenR {
        MstenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Host/Client Mode Select Enable bit"]
    #[inline(always)]
    pub fn csen(&self) -> CsenR {
        CsenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Sign Extend Read Data from the RX FIFO"]
    #[inline(always)]
    pub fn spisgnext(&self) -> SpisgnextR {
        SpisgnextR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI Data Input Sample Phase bit"]
    #[inline(always)]
    pub fn smp(&self) -> SmpR {
        SmpR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Serial Word Length bits for AUDEN=0, not used when AUDEN=1"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, SpictrlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 3 - Disable SDI bit"]
    #[inline(always)]
    pub fn dissdi(&mut self) -> DissdiW<'_, SpictrlSpec> {
        DissdiW::new(self, 3)
    }
    #[doc = "Bit 4 - Disable SDO bit"]
    #[inline(always)]
    pub fn dissdo(&mut self) -> DissdoW<'_, SpictrlSpec> {
        DissdoW::new(self, 4)
    }
    #[doc = "Bit 5 - Master Mode Enable bit"]
    #[inline(always)]
    pub fn msten(&mut self) -> MstenW<'_, SpictrlSpec> {
        MstenW::new(self, 5)
    }
    #[doc = "Bit 6 - Host/Client Mode Select Enable bit"]
    #[inline(always)]
    pub fn csen(&mut self) -> CsenW<'_, SpictrlSpec> {
        CsenW::new(self, 6)
    }
    #[doc = "Bit 9 - Sign Extend Read Data from the RX FIFO"]
    #[inline(always)]
    pub fn spisgnext(&mut self) -> SpisgnextW<'_, SpictrlSpec> {
        SpisgnextW::new(self, 9)
    }
    #[doc = "Bit 10 - SPI Data Input Sample Phase bit"]
    #[inline(always)]
    pub fn smp(&mut self) -> SmpW<'_, SpictrlSpec> {
        SmpW::new(self, 10)
    }
}
#[doc = "SPI Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spictrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spictrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpictrlSpec;
impl crate::RegisterSpec for SpictrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spictrl::R`](R) reader structure"]
impl crate::Readable for SpictrlSpec {}
#[doc = "`write(|w| ..)` method takes [`spictrl::W`](W) writer structure"]
impl crate::Writable for SpictrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPICTRL to value 0"]
impl crate::Resettable for SpictrlSpec {}
