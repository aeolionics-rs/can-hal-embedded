#[doc = "Register `CHCTRLA` reader"]
pub type R = crate::R<ChctrlaSpec>;
#[doc = "Register `CHCTRLA` writer"]
pub type W = crate::W<ChctrlaSpec>;
#[doc = "Field `ENABLE` reader - Channel Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Channel Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLEN` reader - Linked List Enable"]
pub type LlenR = crate::BitReader;
#[doc = "Field `LLEN` writer - Linked List Enable"]
pub type LlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWFRC` reader - Software Forced Trigger"]
pub type SwfrcR = crate::BitReader;
#[doc = "Field `SWFRC` writer - Software Forced Trigger"]
pub type SwfrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run In Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run In Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Linked List Enable"]
    #[inline(always)]
    pub fn llen(&self) -> LlenR {
        LlenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Software Forced Trigger"]
    #[inline(always)]
    pub fn swfrc(&self) -> SwfrcR {
        SwfrcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Run In Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, ChctrlaSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 8 - Linked List Enable"]
    #[inline(always)]
    pub fn llen(&mut self) -> LlenW<'_, ChctrlaSpec> {
        LlenW::new(self, 8)
    }
    #[doc = "Bit 16 - Software Forced Trigger"]
    #[inline(always)]
    pub fn swfrc(&mut self) -> SwfrcW<'_, ChctrlaSpec> {
        SwfrcW::new(self, 16)
    }
    #[doc = "Bit 24 - Run In Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, ChctrlaSpec> {
        RunstdbyW::new(self, 24)
    }
}
#[doc = "CHANNEL CONTROL REGISTER A\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChctrlaSpec;
impl crate::RegisterSpec for ChctrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctrla::R`](R) reader structure"]
impl crate::Readable for ChctrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`chctrla::W`](W) writer structure"]
impl crate::Writable for ChctrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTRLA to value 0"]
impl crate::Resettable for ChctrlaSpec {}
