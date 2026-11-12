#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - SOFTWARE RESET BIT"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - SOFTWARE RESET BIT"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - ENABLE BIT"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - ENABLE BIT"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANAEN` reader - Analog Enable"]
pub type AnaenR = crate::BitReader;
#[doc = "Field `ANAEN` writer - Analog Enable"]
pub type AnaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIPMPEN` reader - Charge Pump Enable"]
pub type AipmpenR = crate::BitReader;
#[doc = "Field `AIPMPEN` writer - Charge Pump Enable"]
pub type AipmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SOFTWARE RESET BIT"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ENABLE BIT"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Enable"]
    #[inline(always)]
    pub fn anaen(&self) -> AnaenR {
        AnaenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Charge Pump Enable"]
    #[inline(always)]
    pub fn aipmpen(&self) -> AipmpenR {
        AipmpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SOFTWARE RESET BIT"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - ENABLE BIT"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Analog Enable"]
    #[inline(always)]
    pub fn anaen(&mut self) -> AnaenW<'_, CtrlaSpec> {
        AnaenW::new(self, 2)
    }
    #[doc = "Bit 4 - Charge Pump Enable"]
    #[inline(always)]
    pub fn aipmpen(&mut self) -> AipmpenW<'_, CtrlaSpec> {
        AipmpenW::new(self, 4)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, CtrlaSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> OndemandW<'_, CtrlaSpec> {
        OndemandW::new(self, 7)
    }
}
#[doc = "CONTROL A REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLA to value 0x80"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u32 = 0x80;
}
