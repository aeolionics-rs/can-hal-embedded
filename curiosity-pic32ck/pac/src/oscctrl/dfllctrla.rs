#[doc = "Register `DFLLCTRLA` reader"]
pub type R = crate::R<DfllctrlaSpec>;
#[doc = "Register `DFLLCTRLA` writer"]
pub type W = crate::W<DfllctrlaSpec>;
#[doc = "Field `ENABLE` reader - DFLL Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - DFLL Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WrtlockR = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOWFREQ` reader - Low Frequency Mode"]
pub type LowfreqR = crate::BitReader;
#[doc = "Field `LOWFREQ` writer - Low Frequency Mode"]
pub type LowfreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - DFLL Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WrtlockR {
        WrtlockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low Frequency Mode"]
    #[inline(always)]
    pub fn lowfreq(&self) -> LowfreqR {
        LowfreqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DFLL Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, DfllctrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WrtlockW<'_, DfllctrlaSpec> {
        WrtlockW::new(self, 2)
    }
    #[doc = "Bit 3 - Low Frequency Mode"]
    #[inline(always)]
    pub fn lowfreq(&mut self) -> LowfreqW<'_, DfllctrlaSpec> {
        LowfreqW::new(self, 3)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> OndemandW<'_, DfllctrlaSpec> {
        OndemandW::new(self, 7)
    }
}
#[doc = "DFLL48M Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfllctrlaSpec;
impl crate::RegisterSpec for DfllctrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfllctrla::R`](R) reader structure"]
impl crate::Readable for DfllctrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`dfllctrla::W`](W) writer structure"]
impl crate::Writable for DfllctrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFLLCTRLA to value 0x82"]
impl crate::Resettable for DfllctrlaSpec {
    const RESET_VALUE: u32 = 0x82;
}
