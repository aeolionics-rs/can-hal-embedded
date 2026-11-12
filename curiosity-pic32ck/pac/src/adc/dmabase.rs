#[doc = "Register `DMABASE` reader"]
pub type R = crate::R<DmabaseSpec>;
#[doc = "Register `DMABASE` writer"]
pub type W = crate::W<DmabaseSpec>;
#[doc = "Field `DMABASE` reader - DMA Sample Value Base Address"]
pub type DmabaseR = crate::FieldReader<u32>;
#[doc = "Field `DMABASE` writer - DMA Sample Value Base Address"]
pub type DmabaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Sample Value Base Address"]
    #[inline(always)]
    pub fn dmabase(&self) -> DmabaseR {
        DmabaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Sample Value Base Address"]
    #[inline(always)]
    pub fn dmabase(&mut self) -> DmabaseW<'_, DmabaseSpec> {
        DmabaseW::new(self, 0)
    }
}
#[doc = "DMA Sample Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmabase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmabase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmabaseSpec;
impl crate::RegisterSpec for DmabaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmabase::R`](R) reader structure"]
impl crate::Readable for DmabaseSpec {}
#[doc = "`write(|w| ..)` method takes [`dmabase::W`](W) writer structure"]
impl crate::Writable for DmabaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMABASE to value 0"]
impl crate::Resettable for DmabaseSpec {}
