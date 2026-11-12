#[doc = "Register `BDCON` reader"]
pub type R = crate::R<BdconSpec>;
#[doc = "Register `BDCON` writer"]
pub type W = crate::W<BdconSpec>;
#[doc = "Field `DMAEN` reader - DMA Enable Bit 1 - Enabled, 0 - Disabled"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA Enable Bit 1 - Enabled, 0 - Disabled"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLLEN` reader - Setting this bit would cause Buffer Descriptor processor (BDP) to poll for descriptor valid till descriptor valid bit is set or till poll counter expires based on value programmed in BD_POLL_CTRL register."]
pub type PollenR = crate::BitReader;
#[doc = "Field `POLLEN` writer - Setting this bit would cause Buffer Descriptor processor (BDP) to poll for descriptor valid till descriptor valid bit is set or till poll counter expires based on value programmed in BD_POLL_CTRL register."]
pub type PollenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Setting this bit to '1' would start BDP (Buffer Descriptor Processor) to fetch a descriptor and hence this bit should be enabled only after all the DMA descriptor programming is done. Hardware clears this bit automatically after one clock cycle."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Setting this bit to '1' would start BDP (Buffer Descriptor Processor) to fetch a descriptor and hence this bit should be enabled only after all the DMA descriptor programming is done. Hardware clears this bit automatically after one clock cycle."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Enable Bit 1 - Enabled, 0 - Disabled"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit would cause Buffer Descriptor processor (BDP) to poll for descriptor valid till descriptor valid bit is set or till poll counter expires based on value programmed in BD_POLL_CTRL register."]
    #[inline(always)]
    pub fn pollen(&self) -> PollenR {
        PollenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Setting this bit to '1' would start BDP (Buffer Descriptor Processor) to fetch a descriptor and hence this bit should be enabled only after all the DMA descriptor programming is done. Hardware clears this bit automatically after one clock cycle."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable Bit 1 - Enabled, 0 - Disabled"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, BdconSpec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Setting this bit would cause Buffer Descriptor processor (BDP) to poll for descriptor valid till descriptor valid bit is set or till poll counter expires based on value programmed in BD_POLL_CTRL register."]
    #[inline(always)]
    pub fn pollen(&mut self) -> PollenW<'_, BdconSpec> {
        PollenW::new(self, 1)
    }
    #[doc = "Bit 2 - Setting this bit to '1' would start BDP (Buffer Descriptor Processor) to fetch a descriptor and hence this bit should be enabled only after all the DMA descriptor programming is done. Hardware clears this bit automatically after one clock cycle."]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, BdconSpec> {
        StartW::new(self, 2)
    }
}
#[doc = "BD_CTRL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdconSpec;
impl crate::RegisterSpec for BdconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdcon::R`](R) reader structure"]
impl crate::Readable for BdconSpec {}
#[doc = "`write(|w| ..)` method takes [`bdcon::W`](W) writer structure"]
impl crate::Writable for BdconSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BDCON to value 0"]
impl crate::Resettable for BdconSpec {}
