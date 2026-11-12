#[doc = "Register `CRCCTRL` reader"]
pub type R = crate::R<CrcctrlSpec>;
#[doc = "Register `CRCCTRL` writer"]
pub type W = crate::W<CrcctrlSpec>;
#[doc = "Field `CRCRST` reader - CRC Reset"]
pub type CrcrstR = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRC Reset"]
pub type CrcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - Start CRC Calculation"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - Start CRC Calculation"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - CRC Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - CRC Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLEN32` reader - Polynomial Length Select"]
pub type Plen32R = crate::BitReader;
#[doc = "Field `PLEN32` writer - Polynomial Length Select"]
pub type Plen32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOR` reader - CRC Auto Repeat"]
pub type AutorR = crate::BitReader;
#[doc = "Field `AUTOR` writer - CRC Auto Repeat"]
pub type AutorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROUT` reader - CRC Reflected Output"]
pub type RoutR = crate::BitReader;
#[doc = "Field `ROUT` writer - CRC Reflected Output"]
pub type RoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIN` reader - CRC Reflected Input"]
pub type RinR = crate::BitReader;
#[doc = "Field `RIN` writer - CRC Reflected Input"]
pub type RinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIOD` reader - Read Period in GCLK counts"]
pub type PeriodR = crate::FieldReader<u16>;
#[doc = "Field `PERIOD` writer - Read Period in GCLK counts"]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - CRC Reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CrcrstR {
        CrcrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start CRC Calculation"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - CRC Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - Polynomial Length Select"]
    #[inline(always)]
    pub fn plen32(&self) -> Plen32R {
        Plen32R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CRC Auto Repeat"]
    #[inline(always)]
    pub fn autor(&self) -> AutorR {
        AutorR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CRC Reflected Output"]
    #[inline(always)]
    pub fn rout(&self) -> RoutR {
        RoutR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CRC Reflected Input"]
    #[inline(always)]
    pub fn rin(&self) -> RinR {
        RinR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Read Period in GCLK counts"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Reset"]
    #[inline(always)]
    pub fn crcrst(&mut self) -> CrcrstW<'_, CrcctrlSpec> {
        CrcrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Start CRC Calculation"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, CrcctrlSpec> {
        CrcenW::new(self, 1)
    }
    #[doc = "Bit 5 - CRC Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, CrcctrlSpec> {
        RunstdbyW::new(self, 5)
    }
    #[doc = "Bit 12 - Polynomial Length Select"]
    #[inline(always)]
    pub fn plen32(&mut self) -> Plen32W<'_, CrcctrlSpec> {
        Plen32W::new(self, 12)
    }
    #[doc = "Bit 13 - CRC Auto Repeat"]
    #[inline(always)]
    pub fn autor(&mut self) -> AutorW<'_, CrcctrlSpec> {
        AutorW::new(self, 13)
    }
    #[doc = "Bit 14 - CRC Reflected Output"]
    #[inline(always)]
    pub fn rout(&mut self) -> RoutW<'_, CrcctrlSpec> {
        RoutW::new(self, 14)
    }
    #[doc = "Bit 15 - CRC Reflected Input"]
    #[inline(always)]
    pub fn rin(&mut self) -> RinW<'_, CrcctrlSpec> {
        RinW::new(self, 15)
    }
    #[doc = "Bits 16:31 - Read Period in GCLK counts"]
    #[inline(always)]
    pub fn period(&mut self) -> PeriodW<'_, CrcctrlSpec> {
        PeriodW::new(self, 16)
    }
}
#[doc = "CRC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcctrlSpec;
impl crate::RegisterSpec for CrcctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcctrl::R`](R) reader structure"]
impl crate::Readable for CrcctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`crcctrl::W`](W) writer structure"]
impl crate::Writable for CrcctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCCTRL to value 0"]
impl crate::Resettable for CrcctrlSpec {}
