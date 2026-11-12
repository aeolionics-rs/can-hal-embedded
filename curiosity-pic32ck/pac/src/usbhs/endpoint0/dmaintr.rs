#[doc = "Register `DMAINTR` reader"]
pub type R = crate::R<DmaintrSpec>;
#[doc = "Register `DMAINTR` writer"]
pub type W = crate::W<DmaintrSpec>;
#[doc = "Field `DMAINTR` reader - DMA Interrupt for each channel"]
pub type DmaintrR = crate::FieldReader;
#[doc = "Field `DMAINTR` writer - DMA Interrupt for each channel"]
pub type DmaintrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DMA Interrupt for each channel"]
    #[inline(always)]
    pub fn dmaintr(&self) -> DmaintrR {
        DmaintrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Interrupt for each channel"]
    #[inline(always)]
    pub fn dmaintr(&mut self) -> DmaintrW<'_, DmaintrSpec> {
        DmaintrW::new(self, 0)
    }
}
#[doc = "DMA Interrupt Status for Each Channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaintr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaintr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaintrSpec;
impl crate::RegisterSpec for DmaintrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaintr::R`](R) reader structure"]
impl crate::Readable for DmaintrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaintr::W`](W) writer structure"]
impl crate::Writable for DmaintrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAINTR to value 0"]
impl crate::Resettable for DmaintrSpec {}
