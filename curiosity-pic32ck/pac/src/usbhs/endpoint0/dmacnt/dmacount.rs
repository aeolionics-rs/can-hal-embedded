#[doc = "Register `DMACOUNT` reader"]
pub type R = crate::R<DmacountSpec>;
#[doc = "Register `DMACOUNT` writer"]
pub type W = crate::W<DmacountSpec>;
#[doc = "Field `DMACOUNT` reader - current dma count of the transfer"]
pub type DmacountR = crate::FieldReader<u32>;
#[doc = "Field `DMACOUNT` writer - current dma count of the transfer"]
pub type DmacountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - current dma count of the transfer"]
    #[inline(always)]
    pub fn dmacount(&self) -> DmacountR {
        DmacountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - current dma count of the transfer"]
    #[inline(always)]
    pub fn dmacount(&mut self) -> DmacountW<'_, DmacountSpec> {
        DmacountW::new(self, 0)
    }
}
#[doc = "Current DMA Count Of The Transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacount::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacount::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacountSpec;
impl crate::RegisterSpec for DmacountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacount::R`](R) reader structure"]
impl crate::Readable for DmacountSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacount::W`](W) writer structure"]
impl crate::Writable for DmacountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACOUNT to value 0"]
impl crate::Resettable for DmacountSpec {}
